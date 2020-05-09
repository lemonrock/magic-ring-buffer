// This file is part of magic-ring-buffer. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/magic-ring-buffer/master/COPYRIGHT. No part of magic-ring-buffer, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of magic-ring-buffer. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/magic-ring-buffer/master/COPYRIGHT.


/// An error that can occur during creation of a file descriptor instance.
#[derive(Debug)]
pub enum MirroredMemoryMapCreationError
{
	/// Could not open memory mapping file.
	CouldNotOpenMemoryMappingFile(io::Error),

	/// Could not truncate memory mapping file.
	CouldNotTruncateMemoryMappingFile(io::Error),
}

impl Display for MirroredMemoryMapCreationError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		<MirroredMemoryMapCreationError as Debug>::fmt(self, f)
	}
}

impl error::Error for MirroredMemoryMapCreationError
{
	#[inline(always)]
	fn source(&self) ->  Option<&(dyn error::Error + 'static)>
	{
		use self::MirroredMemoryMapCreationError::*;

		match self
		{
			&CouldNotOpenMemoryMappingFile(ref error) => Some(error),

			&CouldNotUnlinkMemoryMappingFile(ref error) => Some(error),

			&CouldNotTruncateMemoryMappingFile(ref error) => Some(error),

			&PerProcessLimitOnNumberOfFileDescriptorsWouldBeExceeded => None,

			&KernelWouldBeOutOfMemory => None,
		}
	}
}
