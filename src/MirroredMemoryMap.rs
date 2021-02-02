// This file is part of magic-ring-buffer. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/magic-ring-buffer/master/COPYRIGHT. No part of magic-ring-buffer, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of magic-ring-buffer. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/magic-ring-buffer/master/COPYRIGHT.


#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) struct MirroredMemoryMap
{
	mapped_memory: MappedMemory,
	buffer_size: Size,
	ring_mask: u64,
}

impl MirroredMemoryMap
{
	#[inline(always)]
	pub(crate) fn new(defaults: &DefaultHugePageSizes, preferred_buffer_size: NonZeroU64, inclusive_maximum_bytes_wasted: u64) -> Result<Self, MirroredMemoryMapCreationError>
	{
		use self::MirroredMemoryMapCreationError::*;

		let (buffer_length, page_size_or_huge_page_size_settings, mirror_length) = Self::round_up_to_huge_page_size(preferred_buffer_size, defaults, inclusive_maximum_bytes_wasted)?;
		
		let mapped_memory = MappedMemory::anonymous(mirror_length, AddressHint::any(), Protection::Inaccessible, Sharing::Private, false, false, &page_size_or_huge_page_size_settings).map_err(CouldNotCreateFirstMemoryMapping)?;
		
		const NonUniqueNameForDebuggingPurposes: ConstCStr = ConstCStr(b"mirror\0");
		const AllowSealingOperations: bool = false;
		let memory_file_descriptor  = MemoryFileDescriptor::open_anonymous_memory_as_file(NonUniqueNameForDebuggingPurposes.as_cstr(), AllowSealingOperations, &page_size_or_huge_page_size_settings).map_err(CouldNotOpenMemFd)?;
		memory_file_descriptor.set_non_zero_length(buffer_length).map_err(CouldNotSetLength)?;
		
		Self::map_file_over_memory_reservation(&memory_file_descriptor, &mapped_memory, 0, buffer_length, &page_size_or_huge_page_size_settings)?;
		Self::map_file_over_memory_reservation(&memory_file_descriptor, &mapped_memory, buffer_length.get(), buffer_length, &page_size_or_huge_page_size_settings)?;

		Self::lock_memory(&mapped_memory)?;
		mapped_memory.advise(MemoryAdvice::DontFork).map_err(CouldNotAdviseMemory)?;
		
		let buffer_size = Size::from(buffer_length);
		Ok
		(
			MirroredMemoryMap
			{
				mapped_memory,
				buffer_size,
				ring_mask: buffer_size.to_ring_mask(),
			}
		)
	}

	#[inline(always)]
	fn buffer_size(&self) -> Size
	{
		self.buffer_size
	}

	#[inline(always)]
	fn pointer(&self, offset: OnlyEverIncreasesMonotonicallyOffset) -> *mut u8
	{
		let index = offset & self.ring_mask;
		(self.mapped_memory.virtual_address() + index).into()
	}
	
	#[inline(always)]
	fn round_up_to_huge_page_size(preferred_buffer_size: NonZeroU64, defaults: &DefaultHugePageSizes, inclusive_maximum_bytes_wasted: u64) -> Result<(NonZeroU64, PageSizeOrHugePageSizeSettings, NonZeroU64), MirroredMemoryMapCreationError>
	{
		use self::MirroredMemoryMapCreationError::*;
		
		let (buffer_size, page_size_or_huge_page_size_settings) = MappedMemory::size_suitable_for_a_power_of_two_ring_queue(preferred_buffer_size, defaults, inclusive_maximum_bytes_wasted).ok_or(BufferSizeWouldBeLargerThanTheLargestPowerOfTwoInAnU64(preferred_buffer_size))?;
		let mirror_size = buffer_size.checked_mul(2).ok_or(BufferSizeRequiredMirrorSizeLargerThanTheLargestPowerOfTwoInAnU64(preferred_buffer_size))?;
		
		let buffer_length = new_non_zero_u64(buffer_size);
		let mirror_length = new_non_zero_u64(mirror_size);
		
		Ok((buffer_length, page_size_or_huge_page_size_settings, mirror_length))
	}
	
	#[inline(always)]
	fn map_file_over_memory_reservation(memory_file_descriptor: &MemoryFileDescriptor, mapped_memory: &MappedMemory, mirror_fragment_offset: u64, buffer_length: NonZeroU64, page_size_or_huge_page_size_settings: &PageSizeOrHugePageSizeSettings) -> Result<(), MirroredMemoryMapCreationError>
	{
		const NoOffset: u64 = 0;
		
		let address_hint = AddressHint::fixed(mapped_memory.virtual_address(), mirror_fragment_offset);
		let mirror_fragment = MappedMemory::from_file(memory_file_descriptor, NoOffset, buffer_length, address_hint, Protection::ReadWrite, Sharing::Shared, false, false, page_size_or_huge_page_size_settings).map_err(MirroredMemoryMapCreationError::CouldNotCreateSecondMemoryMapping)?;
		forget(mirror_fragment);
		Ok(())
	}
	
	#[inline(always)]
	fn lock_memory(mapped_memory: &MappedMemory) -> Result<(), MirroredMemoryMapCreationError>
	{
		use self::MirroredMemoryMapCreationError::*;
		
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
