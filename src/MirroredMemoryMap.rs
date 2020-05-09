// This file is part of magic-ring-buffer. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/magic-ring-buffer/master/COPYRIGHT. No part of magic-ring-buffer, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of magic-ring-buffer. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/magic-ring-buffer/master/COPYRIGHT.


#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) struct MirroredMemoryMap
{
	mapped_memory: MappedMemory,
	buffer_size: NonZeroU64,
}

impl MirroredMemoryMap
{
	#[inline(always)]
	pub(crate) fn new(defaults: &DefaultPageSizeAndHugePageSizes, buffer_size_not_page_aligned: NonZeroU64, page_size: PageSizeOrHugePageSize) -> Result<Self, MirroredMemoryMapCreationError>
	{
		const non_unique_name_for_debugging_purposes: ConstCStr = ConstCStr(b"mirror\0");
		const allow_sealing_operations: bool = false;

		let buffer_size = page_size.non_zero_number_of_bytes_rounded_up_to_multiple_of_page_size(buffer_size_not_page_aligned);
		use self::PageSizeOrHugePageSize::*;
		let huge_page_size = match page_size
		{
			PageSize(_) => None,
			HugePageSize(huge_page_size) => Some(Some(huge_page_size)),
		};

		let (memory_file_descriptor, _huge_page_size) = MemoryFileDescriptor::open_anonymous_memory_as_file(non_unique_name_for_debugging_purposes.as_cstr(), allow_sealing_operations, huge_page_size, defaults)?;
		memory_file_descriptor.deref().set_len(buffer_size.get())?;

		let mirror_length = unsafe { NonZeroU64::new_unchecked(buffer_size.get() * 2) };

		let mapped_memory = MappedMemory::anonymous(mirror_length, AddressHint::Any { constrain_to_first_2Gb: false }, Protection::Unaccessible, Sharing::Private, huge_page_size, false, false, &defaults)?;

		// The logic above ensure a memory reservation of twice the size of the anonymous file.
		{
			let first = MappedMemory::from_file(&memory_file_descriptor, 0, buffer_size, AddressHint::Fixed { virtual_address_required: mapped_memory.virtual_address() }, Protection::ReadWrite, Sharing::Shared, huge_page_size, false, false, defaults)?;
			forget(second);
			let second = MappedMemory::from_file(&memory_file_descriptor, 0, buffer_size, AddressHint::Fixed { virtual_address_required: mapped_memory.virtual_address() + buffer_size }, Protection::ReadWrite, Sharing::Shared, huge_page_size, false, false, defaults)?;
			forget(second);
		}

		loop
		{
			let locked = mapped_memory.lock(false)?;
			if locked
			{
				continue
			}
			else
			{
				break
			}
		}

		Ok
		(
			MirroredMemoryMap
			{
				mapped_memory,
				buffer_size,
			}
		)
	}

	#[inline(always)]
	fn pointer(&self, offset: OnlyEverIncreasesMonotonicallyOffset) -> *mut u8
	{
		self.0.virtual_address().add(offset % self.buffer_size).into()
	}
}
