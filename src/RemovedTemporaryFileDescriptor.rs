// This file is part of magic-ring-buffer. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/magic-ring-buffer/master/COPYRIGHT. No part of magic-ring-buffer, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of magic-ring-buffer. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/magic-ring-buffer/master/COPYRIGHT.


#[derive(Debug)]
pub(crate) struct RemovedTemporaryFileDescriptor(RawFd);

impl Drop for RemovedTemporaryFileDescriptor
{
	#[inline(always)]
	fn drop(&mut self)
	{
		self.0.close();
	}
}

impl AsRawFd for RemovedTemporaryFileDescriptor
{
	#[inline(always)]
	fn as_raw_fd(&self) -> RawFd
	{
		self.0
	}
}

impl RemovedTemporaryFileDescriptor
{
	#[inline(always)]
	pub(crate) fn create_temporary_file_and_remove_it(temporary_directory_path: impl AsRef<Path>, file_extension: &str) -> Result<Self, MirroredMemoryMapCreationError>
	{
		const DotOfFileExtension: i32 = 1;

		let mutable_temporary_file_path_template_pointer =
		{
			let path = temporary_directory_path.as_ref();
			let joined = path.join(format!("XXXXXX.{}", file_extension));
			CString::new(joined.as_os_str().as_bytes()).unwrap().into_raw()
		};

		let result = unsafe { mkstemps(mutable_temporary_file_path_template_pointer, DotOfFileExtension + (file_extension.len() as i32)) };

		let temporary_file_path = unsafe { CString::from_raw(mutable_temporary_file_path_template_pointer) };

		use self::MirroredMemoryMapCreationError::*;

		let file_descriptor = if likely!(result >= 0)
		{
			result
		}
		else if likely!(result == -1)
		{
			return Err(CouldNotOpenMemoryMappingFile(io::Error::last_os_error()))
		}
		else
		{
			unreachable!()
		};

		let result = unsafe { unlink(temporary_file_path.as_ptr()) };

		if likely!(result == 0)
		{
			Ok(RemovedTemporaryFileDescriptor(file_descriptor))
		}
		else if likely!(result == -1)
		{
			file_descriptor.close();
			Err(CouldNotUnlinkMemoryMappingFile(io::Error::last_os_error()))
		}
		else
		{
			unreachable!()
		}
	}

	#[inline(always)]
	pub(crate) fn truncate(&self, length: Size) -> Result<(), MirroredMemoryMapCreationError>
	{
		let result = unsafe { ftruncate(self.0, length.into()) };

		loop
		{
			if likely!(result == 0)
			{
				return Ok(())
			}
			else if likely!(result == -1)
			{
				let error = io::Error::last_os_error();
				if error.kind() == ErrorKind::Interrupted
				{
					continue
				}
				else
				{
					return Err(MirroredMemoryMapCreationError::CouldNotTruncateMemoryMappingFile(error))
				}
			}
			else
			{
				unreachable!()
			}
		}
	}

	#[inline(always)]
	pub(crate) fn into(this: Option<&Self>) -> i32
	{
		match this
		{
			Some(this) => this.as_raw_fd(),
			None => -1,
		}
	}
}
