// This file is part of magic-ring-buffer. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/magic-ring-buffer/master/COPYRIGHT. No part of magic-ring-buffer, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of magic-ring-buffer. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/magic-ring-buffer/master/COPYRIGHT.


#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) struct MirroredMemoryMap
{
	address: VirtualAddress,
	length: Size,
	unmirrored_buffer_size: Size,
}

impl Drop for MirroredMemoryMap
{
	#[inline(always)]
	fn drop(&mut self)
	{
		unsafe { munmap(self.address.0, self.length.into()) };
	}
}

impl MirroredMemoryMap
{
	#[inline(always)]
	pub(crate) fn allocate_mirrored_and_not_swappable_from_dev_shm(file_extension: &str, buffer_size_not_page_aligned: Size) -> Result<Self, MirroredMemoryMapCreationError>
	{
		Self::allocate_mirrored_and_not_swappable("/dev/shm", file_extension, buffer_size_not_page_aligned)
	}
	
	pub(crate) fn allocate_mirrored_and_not_swappable(temporary_directory_path: impl AsRef<Path>, file_extension: &str, buffer_size_not_page_aligned: Size) -> Result<Self, MirroredMemoryMapCreationError>
	{
		debug_assert_ne!(file_extension.len(), 0, "file_extension can not be empty");
		debug_assert_ne!(buffer_size_not_page_aligned, Size::default(), "buffer_size_not_page_aligned can not be zero");
		
		let temporary_file = RemovedTemporaryFileDescriptor::create_temporary_file_and_remove_it(temporary_directory_path, file_extension)?;
		temporary_file.truncate(buffer_size_not_page_aligned)?;

		let this = Self::map_using_file(buffer_size_not_page_aligned, &temporary_file)?;
		
		Ok(this)
	}

	pub(crate) fn map_using_file(buffer_size_not_page_aligned: Size, temporary_file: &RemovedTemporaryFileDescriptor) -> Result<Self, MirroredMemoryMapCreationError>
	{
		let buffer_size = buffer_size_not_page_aligned.round_up_to_page_size();
		let mirror_length = buffer_size * 2;

		let address = Self::memory_map(VirtualAddress::default(), mirror_length, PROT_NONE, MAP_NORESERVE | MAP_ANONYMOUS | MAP_PRIVATE, None, 0)?;
		let this = Self
		{
			address,
			length: mirror_length,
			unmirrored_buffer_size: buffer_size,
		};
		debug_assert!(address.is_not_null(), "Memory mapping address is null");

		const ProtectionConstants: i32 = PROT_READ | PROT_WRITE;
		const MapConstants: i32 = MAP_NORESERVE | MAP_FIXED | MAP_SHARED;

		let address_of_real_memory = Self::memory_map(address, buffer_size, ProtectionConstants, MapConstants, Some(&temporary_file), 0)?;
		debug_assert_eq!(address, address_of_real_memory, "First fixed mapping failed");

		let address_of_mirrored_memory = Self::memory_map(Self::mirror_address(address, buffer_size), buffer_size, ProtectionConstants, MapConstants, Some(&temporary_file), 0)?;
		debug_assert_eq!(address, address_of_mirrored_memory, "Second fixed mapping failed");

		let _memory_locked = Self::try_to_memory_lock(address, mirror_length);

		Ok(this)
	}

	#[inline(always)]
	fn memory_map(address: VirtualAddress, length: Size, protection: i32, flags: i32, temporary_file: Option<&RemovedTemporaryFileDescriptor>, offset: usize) -> Result<VirtualAddress, MirroredMemoryMapCreationError>
	{
		let address = unsafe { mmap(address.0, length.into(), protection, flags, RemovedTemporaryFileDescriptor::into(temporary_file), offset as i64) };

		if likely!(address != MAP_FAILED)
		{
			Ok(VirtualAddress(address))
		}
		else
		{
			use self::MirroredMemoryMapCreationError::*;

			Err
			(
				match errno().0
				{
					ENFILE => PerProcessLimitOnNumberOfFileDescriptorsWouldBeExceeded,
					ENOMEM => KernelWouldBeOutOfMemory,

					EAGAIN => panic!("File locked (or too much memory has been locked)"),
					EBADF => panic!("`fd` is not a valid file descriptor (and `MAP_ANONYMOUS` was not set)"),
					EACCES => panic!(" A file descriptor refers to a non-regular file; or `MAP_PRIVATE` was requested, but `fd` is not open for reading; or `MAP_SHARED` was requested and `PROT_WRITE` is set, but `fd` is not open for read/write (`O_RDWR`) mode; or `PROT_WRITE` is set, but the file is append-only"),
					EINVAL => panic!("`addr`, `length` or `offset` were too large or not page aligned, or `length` was zero, or `flags` contained either both or or none of `MAP_PRIVATE` and `MAP_SHARED`"),
					ENODEV => panic!("The underlying file system of the specified file does not support memory mapping"),
					EPERM => panic!("The `prot` argument asks for `PROT_EXEC` but the mapped area belongs to a file on a file system that was mounted `no-exec`"),
					ETXTBSY => panic!("MAP_DENYWRITE was set but the object specified by `fd` is open for writing"),
					EOVERFLOW => panic!("On 32-bit architecture together with the large file extension the number of pages used for length plus number of pages used for offset would overflow a 32-bit unsigned long"),

					_ => unreachable!(),
				}
			)
		}
	}
	
	#[inline(always)]
	fn try_to_memory_lock(address: VirtualAddress, length: Size) -> bool
	{
		let result = unsafe { mlock(address.0, length.into()) };

		if likely!(result == 0)
		{
			true
		}
		else if likely!(result == -1)
		{
			let error = io::Error::last_os_error();
			match error.raw_os_error().unwrap()
			{
				ENOMEM | EAGAIN | EPERM => false,

				EINVAL => panic!("EINVAL for memory lock"),

				_ => unreachable!(),
			}
		}
		else
		{
			unreachable!()
		}
	}
	
	#[inline(always)]
	fn mirror_address(address: VirtualAddress, offset: Size) -> VirtualAddress
	{
		address.add(offset)
	}

	#[inline(always)]
	fn pointer(&self, offset: OnlyEverIncreasesMonotonicallyOffset) -> *mut u8
	{
		self.address.add(offset % self.unmirrored_buffer_size).into()
	}
}
