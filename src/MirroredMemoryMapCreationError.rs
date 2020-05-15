// This file is part of magic-ring-buffer. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/magic-ring-buffer/master/COPYRIGHT. No part of magic-ring-buffer, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of magic-ring-buffer. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/magic-ring-buffer/master/COPYRIGHT.


/// An error that can occur during creation of a file descriptor instance.
#[derive(Debug)]
pub enum MirroredMemoryMapCreationError
{
	/// ie can not exceed `2^63`.
	#[allow(missing_docs)]
	BufferSizeWouldBeLargerThanTheLargestPowerOfTwoInAnU64(NonZeroU64),
	
	/// ie can not exceed `2^62`.
	#[allow(missing_docs)]
	BufferSizeRequiredMirrorSizeLargerThanTheLargestPowerOfTwoInAnU64(NonZeroU64),

	#[allow(missing_docs)]
	CouldNotOpenMemFd(CreationError),

	#[allow(missing_docs)]
	CouldNotSetLength(io::Error),

	#[allow(missing_docs)]
	CouldNotCreateFirstMemoryMapping(CreationError),

	#[allow(missing_docs)]
	CouldNotCreateSecondMemoryMapping(CreationError),

	#[allow(missing_docs)]
	CouldNotLockMemory(io::Error),

	#[allow(missing_docs)]
	CouldNotLockAllMemory,

	#[allow(missing_docs)]
	CouldNotAdviseMemory(io::Error),
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
			&BufferSizeWouldBeLargerThanTheLargestPowerOfTwoInAnU64(..) => None,
			
			&BufferSizeRequiredMirrorSizeLargerThanTheLargestPowerOfTwoInAnU64(..) => None,
			
			&CouldNotOpenMemFd(ref error) => Some(error),

			&CouldNotSetLength(ref error) => Some(error),

			&CouldNotCreateFirstMemoryMapping(ref error) => Some(error),

			&CouldNotCreateSecondMemoryMapping(ref error) => Some(error),

			&CouldNotLockMemory(ref error) => Some(error),

			&CouldNotLockAllMemory => None,

			&CouldNotAdviseMemory(ref error) => Some(error),
		}
	}
}
