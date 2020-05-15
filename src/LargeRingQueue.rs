// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Not thread safe.
///
/// Only works because the size of memory allocated is a power-of-two.
#[derive(Debug)]
pub struct LargeRingQueue<Element>
{
	mapped_memory: MappedMemory,
	ring_mask: u64,
	tail: OnlyEverIncreasesMonotonicallyOffset,
	head: OnlyEverIncreasesMonotonicallyOffset,
	maximum_number_of_elements: NonZeroU64,
	marker: PhantomData<Element>,
}

impl<Element> Drop for LargeRingQueue<Element>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		while !self.is_empty()
		{
			let element = unsafe { &mut * self.real_pointer(self.tail) };
			unsafe { drop_in_place(element) };
			self.tail += 1;
		}
	}
}

impl<Element: Sized + Copy> LargeRingQueue<Element>
{
	/// Enqueues and copies.
	///
	/// Returns `false` if enqueue failed because the queue is full.
	#[inline(always)]
	pub fn enqueue_checked(&mut self, value: Element) -> bool
	{
		if unlikely!(self.is_full())
		{
			false
		}
		else
		{
			unsafe { self.enqueue_unchecked(value) };
			true
		}
	}
	
	/// Enqueues and copies without checking for capacity.
	#[inline(always)]
	pub unsafe fn enqueue_unchecked(&mut self, value: Element)
	{
		debug_assert!(self.head >= self.tail);
		
		* self.real_pointer(self.head) = value;
		
		self.head += 1;
	}
	
	/// Dequeues and copies without checking for capacity.
	#[inline(always)]
	pub fn dequeue(&mut self) -> Option<Element>
	{
		debug_assert!(self.head >= self.tail);
		
		if unlikely!(self.tail == self.head)
		{
			return None
		}
		
		let value = unsafe { * self.real_pointer(self.tail) };
		
		self.tail += 1;
		
		Some(value)
	}
}

impl<Element> LargeRingQueue<Element>
{
	const ElementSize: u64 = size_of::<Element>() as u64;
	
	/// Creates a new queue.
	///
	/// `create_full_of_uninitialized_elements` is dangerous!
	/// It's only really suitable for things like buffer slices (eg `&[u8]`).
	pub fn new(ideal_maximum_number_of_elements: NonZeroU64, defaults: &DefaultPageSizeAndHugePageSizes, inclusive_maximum_bytes_wasted: u64, create_full_of_uninitialized_elements: bool) -> Result<Self, LargeRingQueueCreationError>
	{
		use self::LargeRingQueueCreationError::*;
		
		let maximum_number_of_elements_power_of_two = ideal_maximum_number_of_elements.get().checked_next_power_of_two().ok_or(MaximumNumberOfElementsRoundedUpToAPowerOfTwoWouldBeLargerThanTheLargestPowerOfTwoInAnU64)?;
		
		let preferred_buffer_size = maximum_number_of_elements_power_of_two.checked_mul(Self::ElementSize).ok_or(MaximumNumberOfElementsRoundedUpToAPowerOfTwoAndScaledByTheSizeOfEachElementWouldBeLargerThanTheLargestPowerOfTwoInAnU64)?;
		
		let (buffer_size, huge_page_size) = MappedMemory::size_suitable_for_a_power_of_two_ring_queue(unsafe { NonZeroU64::new_unchecked(preferred_buffer_size) }, defaults, inclusive_maximum_bytes_wasted).ok_or(BufferSizeWouldBeLargerThanTheLargestPowerOfTwoInAnU64)?;
		
		let mapped_memory = MappedMemory::anonymous(unsafe { NonZeroU64::new_unchecked(buffer_size) }, AddressHint::any(), Protection::Inaccessible, Sharing::Private, huge_page_size, false, false, &defaults).map_err(CouldNotCreateMemoryMapping)?;
		
		Self::lock_memory(&mapped_memory)?;
		mapped_memory.advise(MemoryAdvice::DontFork).map_err(CouldNotAdviseMemory)?;
		
		let maximum_number_of_elements =
		{
			debug_assert_eq!(buffer_size % Self::ElementSize, 0);
			let maximum_number_of_elements = buffer_size / Self::ElementSize;
			debug_assert_eq!(Some(maximum_number_of_elements), maximum_number_of_elements.checked_next_power_of_two());
			maximum_number_of_elements
		};
		
		Ok
		(
			Self
			{
				mapped_memory,
				ring_mask: maximum_number_of_elements - 1,
				tail: OnlyEverIncreasesMonotonicallyOffset::default(),
				head: if create_full_of_uninitialized_elements
				{
					OnlyEverIncreasesMonotonicallyOffset::default() + maximum_number_of_elements
				}
				else
				{
					OnlyEverIncreasesMonotonicallyOffset::default()
				},
				maximum_number_of_elements: unsafe { NonZeroU64::new_unchecked(maximum_number_of_elements) },
				marker: PhantomData,
			}
		)
	}
	
	/// Is empty?
	#[inline(always)]
	pub fn is_empty(&self) -> bool
	{
		debug_assert!(self.head >= self.tail);
		
		self.head == self.tail
	}
	
	/// Is full?
	#[inline(always)]
	pub fn is_full(&self) -> bool
	{
		self.available() == 0
	}
	
	/// Available capacity.
	#[inline(always)]
	pub fn available(&self) -> u64
	{
		debug_assert!(self.head >= self.tail);
		
		let available = (self.head - self.tail).u64();
		debug_assert!(available <= self.maximum_number_of_elements.get());
		available
	}
	
	/// Enqueues without checking for capacity or copying data.
	#[inline(always)]
	pub fn relinquish(&mut self, non_null_owned_by_us: NonNull<Element>)
	{
		debug_assert!(self.head >= self.tail);
		debug_assert!(self.mapped_memory.owns_non_null(non_null_owned_by_us));
		
		self.head += 1;
	}
	
	/// Dequeues uninitialized memory.
	///
	/// Maps dequeued element to avoid need for separate non-performant `.map()` and `.map_err()` operations.
	#[inline(always)]
	pub fn obtain_and_map<Mapper: FnOnce(NonNull<Element>) -> Mapped, Mapped, EmptyHandler: FnOnce() -> Error, Error>(&mut self, mapper: Mapper, empty_handler: EmptyHandler) -> Result<Mapped, Error>
	{
		debug_assert!(self.head >= self.tail);
		
		if unlikely!(self.tail == self.head)
		{
			return Err(empty_handler())
		}
		
		let value = unsafe { NonNull::new_unchecked(self.real_pointer(self.tail)) };
		
		self.tail += 1;
		
		Ok(mapper(value))
	}
	
	/// Virtual address.
	#[inline(always)]
	pub fn virtual_address(&self) -> VirtualAddress
	{
		self.mapped_memory.virtual_address()
	}
	
	/// Size in bytes.
	#[inline(always)]
	pub fn size_in_bytes(&self) -> u64
	{
		self.mapped_memory.mapped_size_in_bytes() as u64
	}
	
	/// Size in bytes.
	#[inline(always)]
	pub fn raw_backing_memory_slice(&self) -> &mut [u8]
	{
		let pointer: *mut u8 = self.virtual_address().into();
		unsafe { from_raw_parts_mut(pointer, self.mapped_memory.mapped_size_in_bytes()) }
	}
	
	#[inline(always)]
	fn real_pointer(&self, offset: OnlyEverIncreasesMonotonicallyOffset) -> *mut Element
	{
		// This calculation can be simplified to (offset << Self::ElementSize) & (self.ring_mask << Self::ElementSize) if Self::ElementSize is a power of 2.
		// Which allows one to cache (self.ring_mask << Self::ElementSize); a micro-optimization for the cost of multiply, which the complier can optimize anyway as Self::ElementSize is a constant.
		let offset_in_bytes = (offset & self.ring_mask) * Self::ElementSize;
		(self.mapped_memory.virtual_address() + offset_in_bytes).into()
	}
	
	#[inline(always)]
	fn lock_memory(mapped_memory: &MappedMemory) -> Result<(), LargeRingQueueCreationError>
	{
		use self::LargeRingQueueCreationError::*;
		
		let locked_all_memory = mapped_memory.lock(MemoryLockSettings::Normal).map_err(CouldNotLockMemory)?;
		if likely!(locked_all_memory)
		{
			Ok(())
		}
		else
		{
			Err(CouldNotLockAllMemory)
		}
	}
}
