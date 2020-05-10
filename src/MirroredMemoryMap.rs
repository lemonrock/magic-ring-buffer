// This file is part of magic-ring-buffer. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/magic-ring-buffer/master/COPYRIGHT. No part of magic-ring-buffer, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of magic-ring-buffer. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/magic-ring-buffer/master/COPYRIGHT.


#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) struct MirroredMemoryMap
{
	mapped_memory: MappedMemory,
	buffer_size: Size,
}

impl MirroredMemoryMap
{
	#[inline(always)]
	pub(crate) fn new(defaults: &DefaultPageSizeAndHugePageSizes, buffer_size_not_page_aligned: NonZeroU64, inclusive_maximum_bytes_wasted: usize) -> Result<Self, MirroredMemoryMapCreationError>
	{
		use self::MirroredMemoryMapCreationError::*;

		const non_unique_name_for_debugging_purposes: ConstCStr = ConstCStr(b"mirror\0");
		const allow_sealing_operations: bool = false;

		let (buffer_size, huge_page_size) = match defaults.best_fit_huge_page_size_if_any(buffer_size_not_page_aligned.get() as usize, inclusive_maximum_bytes_wasted)
		{
			None => (memory::PageSize::current().non_zero_number_of_bytes_rounded_up_to_multiple_of_page_size(buffer_size_not_page_aligned), None),
			Some(huge_page_size) => (huge_page_size.non_zero_number_of_bytes_rounded_up_to_multiple_of_page_size(buffer_size_not_page_aligned), Some(Some(huge_page_size)))
		};

		let (memory_file_descriptor, _huge_page_size) = MemoryFileDescriptor::open_anonymous_memory_as_file(non_unique_name_for_debugging_purposes.as_cstr(), allow_sealing_operations, huge_page_size, defaults).map_err(CouldNotOpenMemFd)?;
		memory_file_descriptor.deref().set_len(buffer_size.get()).map_err(CouldNotSetLength)?;

		let mirror_length = unsafe { NonZeroU64::new_unchecked(buffer_size.get() * 2) };

		let mapped_memory = MappedMemory::anonymous(mirror_length, AddressHint::any(), Protection::Inaccessible, Sharing::Private, huge_page_size, false, false, &defaults).map_err(CouldNotCreateFirstMemoryMapping)?;

		// The logic above ensure a memory reservation of twice the size of the anonymous file.
		{
			let first = MappedMemory::from_file(&memory_file_descriptor, 0, buffer_size, AddressHint::Fixed { virtual_address_required: mapped_memory.virtual_address() }, Protection::ReadWrite, Sharing::Shared, huge_page_size, false, false, defaults).map_err(CouldNotCreateSecondMemoryMapping)?;
			forget(first);
			let second = MappedMemory::from_file(&memory_file_descriptor, 0, buffer_size, AddressHint::Fixed { virtual_address_required: mapped_memory.virtual_address() + buffer_size }, Protection::ReadWrite, Sharing::Shared, huge_page_size, false, false, defaults).map_err(CouldNotCreateThirdMemoryMapping)?;
			forget(second);
		}

		let locked_all_memory = mapped_memory.lock(MemoryLockSettings::Normal).map_err(CouldNotLockMemory)?;
		if unlikely!(!locked_all_memory)
		{
			return Err(CouldNotLockAllMemory)
		}

		mapped_memory.advise(MemoryAdvice::DontFork).map_err(CouldNotAdviseMemory)?;

		Ok
		(
			MirroredMemoryMap
			{
				mapped_memory,
				buffer_size: Size(buffer_size.get()),
			}
		)
	}

	#[inline(always)]
	fn pointer(&self, offset: OnlyEverIncreasesMonotonicallyOffset) -> *mut u8
	{
		self.mapped_memory.virtual_address().offset_in_bytes((offset % self.buffer_size).0 as usize).into()
	}
}
