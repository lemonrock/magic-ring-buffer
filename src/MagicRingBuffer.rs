// This file is part of magic-ring-buffer. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/magic-ring-buffer/master/COPYRIGHT. No part of magic-ring-buffer, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of magic-ring-buffer. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/magic-ring-buffer/master/COPYRIGHT.


/// A magic ring buffer (also known as virtual ring buffer, VRB, or a mirrored ring buffer).
#[derive(Debug)]
pub struct MagicRingBuffer
{
	writer_offset: CompareExchangeOnlyEverIncreasesMonotonicallyOffset,
	unread_offset: CompareExchangeOnlyEverIncreasesMonotonicallyOffset,
	read_offset: CompareExchangeOnlyEverIncreasesMonotonicallyOffset,
	mirrored_memory_map: MirroredMemoryMap,
}

impl MagicRingBuffer
{
	/// Creates a new instance.
	///
	/// Rounds `preferred_buffer_size` to page size.
	#[inline(always)]
	pub fn allocate(defaults: &DefaultHugePageSizes, preferred_buffer_size: NonZeroU64, inclusive_maximum_bytes_wasted: u64) -> Result<Self, MirroredMemoryMapCreationError>
	{
		Ok
		(
			Self
			{
				writer_offset: CompareExchangeOnlyEverIncreasesMonotonicallyOffset::default(),
				unread_offset: CompareExchangeOnlyEverIncreasesMonotonicallyOffset::default(),
				read_offset: CompareExchangeOnlyEverIncreasesMonotonicallyOffset::default(),
				mirrored_memory_map: MirroredMemoryMap::new(defaults, preferred_buffer_size, inclusive_maximum_bytes_wasted)?,
			}
		)
	}

	/// In a recovery scenario, we can potentially (a) re-read a message and (b) will lose all messages written but not committed between `unread_offset` and `writer_offset`.
	#[inline(always)]
	pub fn recovery_if_using_persistent_memory(&self)
	{
		self.writer_offset.set(self.unread_offset.get())
	}

	/// The logic in `writer` must not panic; if it does, then the entire queue is effectively corrupt and irrecoverable.
	#[inline(always)]
	pub fn write_some_data(&self, amount_we_want_to_write: usize, writer: impl FnOnce(&mut [u8]))
	{
		let amount_we_want_to_write = Size::from(amount_we_want_to_write);
		debug_assert!(amount_we_want_to_write <= self.unmirrored_buffer_size(), "Can not write amounts large than then ring buffer's size");

		// Get a new offset to write to.
		let (current_writer_state_write_offset, next_writer_state_write_offset) = self.writer_offset.fetch_add(amount_we_want_to_write);

		// We exit this loop when the reader has made enough forward progress to free up space to accommodate our write (and any predecessors on other threads).
		let mut current_unread_offset = loop
		{
			let (current_unread_offset, _current_read_offset, unread) = self.current_unread_offset_and_current_read_offset_and_unread();

			// This value decrements or stays the same with every loop iteration; it can never increase.
			let total_size_required_for_writes_in_progress = next_writer_state_write_offset - current_unread_offset;

			let available_for_writes = self.unmirrored_buffer_size() - unread;

			debug_assert!(available_for_writes <= self.unmirrored_buffer_size());

			if likely!(available_for_writes >= total_size_required_for_writes_in_progress)
			{
				break current_unread_offset
			}
			
			busy_wait_spin_loop_hint();
		};

		// Write data.
		writer(self.write_to_buffer(current_writer_state_write_offset, amount_we_want_to_write));

		// Serialize order of writers so that they only commit their writes in ascending order with no 'holes', ie later before earlier.
		loop
		{
			current_unread_offset = match self.unread_offset.try_to_update(current_unread_offset, current_writer_state_write_offset)
			{
				Ok(()) => break,
				Err(was_reader_state) => was_reader_state,
			};
			busy_wait_spin_loop_hint();
		}
	}

	/// Read data, assuming a single reader is active.
	///
	/// This is *NOT* enforced.
	///
	/// Returns true if there is more data to read.
	#[inline(always)]
	pub fn single_reader_read_some_data<E, Reader: FnOnce(&mut [u8]) -> (usize, Result<(), E>)>(&self, reader: Reader) -> Result<bool, E>
	{
		let (_current_unread_offset, current_read_offset, unread) = self.current_unread_offset_and_current_read_offset_and_unread();

		let (actually_read, outcome) = reader(self.read_from_buffer(current_read_offset, unread));
		let actually_read = Size::from(actually_read);

		let updated_read_offset = current_read_offset + actually_read;
		self.read_offset.set(updated_read_offset);

		match outcome
		{
			Err(error) => Err(error),
			Ok(()) =>
			{
				let (_current_unread_offset, _current_read_offset, unread) = self.current_unread_offset_and_current_read_offset_and_unread();
				Ok(unread != Size::default())
			}
		}
	}

	// Multiple readers can be implemented using a mutual exclusion lock.
	// But is there the possibility of an `unwriter_offset` - similar to that used to linearize writers - a sort of unwriter_offset

	#[inline(always)]
	fn unmirrored_buffer_size(&self) -> Size
	{
		self.mirrored_memory_map.buffer_size()
	}

	#[inline(always)]
	fn current_unread_offset_and_current_read_offset_and_unread(&self) -> (OnlyEverIncreasesMonotonicallyOffset, OnlyEverIncreasesMonotonicallyOffset, Size)
	{
		let current_unread_offset = self.unread_offset.get();
		let current_read_offset = self.read_offset.get();
		debug_assert!(current_unread_offset >= current_read_offset);
		let unread = current_unread_offset - current_read_offset;

		(current_unread_offset, current_read_offset, unread)
	}

	#[inline(always)]
	fn real_pointer(&self, offset: OnlyEverIncreasesMonotonicallyOffset) -> *mut u8
	{
		self.mirrored_memory_map.pointer(offset)
	}

	#[inline(always)]
	fn write_to_buffer(&self, current_writer_state_write_offset: OnlyEverIncreasesMonotonicallyOffset, amount_we_want_to_write: Size) -> &mut [u8]
	{
		let write_pointer = self.real_pointer(current_writer_state_write_offset);
		unsafe { from_raw_parts_mut(write_pointer, amount_we_want_to_write.into()) }
	}

	#[inline(always)]
	fn read_from_buffer(&self, current_read_offset: OnlyEverIncreasesMonotonicallyOffset, unread: Size) -> &mut [u8]
	{
		let read_pointer = self.real_pointer(current_read_offset);
		unsafe { from_raw_parts_mut(read_pointer, unread.into()) }
	}
}
