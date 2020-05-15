// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Error when constructing.
#[derive(Debug)]
pub enum LargeRingQueueCreationError
{
	/// ie can not exceed `2^63`.
	#[allow(missing_docs)]
	MaximumNumberOfElementsRoundedUpToAPowerOfTwoWouldBeLargerThanTheLargestPowerOfTwoInAnU64,
	
	/// ie can not exceed `2^63`.
	#[allow(missing_docs)]
	MaximumNumberOfElementsRoundedUpToAPowerOfTwoAndScaledByTheSizeOfEachElementWouldBeLargerThanTheLargestPowerOfTwoInAnU64,
	
	/// ie can not exceed `2^63`.
	#[allow(missing_docs)]
	BufferSizeWouldBeLargerThanTheLargestPowerOfTwoInAnU64,
	
	#[allow(missing_docs)]
	CouldNotCreateMemoryMapping(CreationError),
	
	#[allow(missing_docs)]
	CouldNotLockMemory(io::Error),
	
	#[allow(missing_docs)]
	CouldNotLockAllMemory,
	
	#[allow(missing_docs)]
	CouldNotAdviseMemory(io::Error),
}

impl Display for LargeRingQueueCreationError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		<LargeRingQueueCreationError as Debug>::fmt(self, f)
	}
}

impl error::Error for LargeRingQueueCreationError
{
	#[inline(always)]
	fn source(&self) ->  Option<&(dyn error::Error + 'static)>
	{
		use self::LargeRingQueueCreationError::*;

		match self
		{
			&MaximumNumberOfElementsRoundedUpToAPowerOfTwoWouldBeLargerThanTheLargestPowerOfTwoInAnU64 => None,
			
			&MaximumNumberOfElementsRoundedUpToAPowerOfTwoAndScaledByTheSizeOfEachElementWouldBeLargerThanTheLargestPowerOfTwoInAnU64 => None,
			
			&BufferSizeWouldBeLargerThanTheLargestPowerOfTwoInAnU64 => None,

			&CouldNotCreateMemoryMapping(ref error) => Some(error),

			&CouldNotLockMemory(ref error) => Some(error),

			&CouldNotLockAllMemory => None,

			&CouldNotAdviseMemory(ref error) => Some(error),
		}
	}
}
