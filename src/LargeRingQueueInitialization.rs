// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Initialization choice.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LargeRingQueueInitialization<Element: LargeRingQueueElement>
{
	/// Empty.
	Empty,
	
	/// Full of uninitialized elements.
	CreateFullOfUninitializedElements,
	
	/// Full of zeroed elements, each consisting of zeros.
	CreateFullOfZeroedElements,

	/// Full using an initializer.
	CreateFullUsingInitializer(unsafe fn(u64, NonNull<Element>))
}

impl<Element: LargeRingQueueElement> LargeRingQueueInitialization<Element>
{
	#[inline(always)]
	fn apply(self, mapped_memory: &MappedMemory, maximum_number_of_elements: u64) -> OnlyEverIncreasesMonotonicallyOffset
	{
		use self::LargeRingQueueInitialization::*;
		
		match self
		{
			Empty => Self::empty(),
			
			CreateFullOfUninitializedElements => Self::full(maximum_number_of_elements),
			
			CreateFullOfZeroedElements =>
			{
				let pointer: *mut u8 = mapped_memory.virtual_address().into();
				let size = mapped_memory.mapped_size_in_bytes();
				unsafe { pointer.write_bytes(0x00, size) };
				
				Self::full(maximum_number_of_elements)
			}
			
			CreateFullUsingInitializer(initializer) =>
			{
				let mut pointer: *mut Element = mapped_memory.virtual_address().into();
				for index in 0 .. maximum_number_of_elements
				{
					unsafe
					{
						initializer(index, NonNull::new_unchecked(pointer));
						pointer = pointer.add(index as usize)
					}
				}
				Self::full(maximum_number_of_elements)
			}
		}
	}
	
	#[inline(always)]
	fn full(maximum_number_of_elements: u64) -> OnlyEverIncreasesMonotonicallyOffset
	{
		Self::empty() + maximum_number_of_elements
	}
	
	#[inline(always)]
	fn empty() -> OnlyEverIncreasesMonotonicallyOffset
	{
		OnlyEverIncreasesMonotonicallyOffset::default()
	}
}
