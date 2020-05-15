// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Reference-counted with automatic relinquish of elements on drop of element.
///
/// Not thread safe.
#[derive(Debug)]
pub struct ReferenceCountedLargeRingQueue<Element>(Rc<UnsafeCell<LargeRingQueue<Element>>>);

impl<Element> ReferenceCountedLargeRingQueue<Element>
{
	/// New instance.
	///
	/// Suitable for coroutine memory allocators.
	pub fn new_exact_fit(ideal_maximum_number_of_elements: NonZeroU64, defaults: &DefaultPageSizeAndHugePageSizes) -> Result<Self, LargeRingQueueCreationError>
	{
		Self::new(ideal_maximum_number_of_elements, defaults, 0, true)
	}
	
	/// New instance.
	pub fn new(ideal_maximum_number_of_elements: NonZeroU64, defaults: &DefaultPageSizeAndHugePageSizes, inclusive_maximum_bytes_wasted: u64, create_full_of_uninitialized_elements: bool) -> Result<Self, LargeRingQueueCreationError>
	{
		Ok(Self(Rc::new(UnsafeCell::new(LargeRingQueue::new(ideal_maximum_number_of_elements, defaults, inclusive_maximum_bytes_wasted, create_full_of_uninitialized_elements)?))))
	}
}

impl<Element> Clone for ReferenceCountedLargeRingQueue<Element>
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		Self(self.0.clone())
	}
}

impl<Element> ReferenceCountedLargeRingQueue<Element>
{
	/// Obtain.
	#[inline(always)]
	pub fn obtain<EmptyHandler: FnOnce() -> Error, Error>(&self, empty_handler: EmptyHandler) -> Result<ReferenceCountedLargeRingQueueElement<Element>, Error>
	{
		self.use_large_ring_queue(|large_ring_queue|
		{
			large_ring_queue.obtain_and_map
			(
				|element| ReferenceCountedLargeRingQueueElement
				{
					element,
					reference_counted_large_ring_queue: self.clone(),
				},
				empty_handler
			)
		})
	}
	
	#[inline(always)]
	fn relinquish(&self, element: NonNull<Element>)
	{
		self.use_large_ring_queue(|large_ring_queue| large_ring_queue.relinquish(element))
	}
	
	#[inline(always)]
	fn use_large_ring_queue<R>(&self, large_ring_queue_user: impl FnOnce(&mut LargeRingQueue<Element>) -> R) -> R
	{
		unsafe { large_ring_queue_user(&mut * self.0.get()) }
	}
}
