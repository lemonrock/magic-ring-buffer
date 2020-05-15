// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Will be relinquished (freed and made available again) on drop.
///
/// Not thread safe.
#[derive(Debug)]
pub struct ReferenceCountedLargeRingQueueElement<Element>
{
	element: NonNull<Element>,
	reference_counted_large_ring_queue: ReferenceCountedLargeRingQueue<Element>,
}

impl<Element> Drop for ReferenceCountedLargeRingQueueElement<Element>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		self.reference_counted_large_ring_queue.relinquish(self.element)
	}
}

impl<Element> ReferenceCountedLargeRingQueueElement<Element>
{
	/// Element.
	///
	/// Be careful to not let a reference to `NonNull<Element>` outlive the lifetime of `self`.
	#[inline(always)]
	pub unsafe fn element(&self) -> NonNull<Element>
	{
		self.element
	}
}
