// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A large ring queue element.
pub trait LargeRingQueueElement: Sized
{
	/// How to initialize when the queue is created.
	const Initialization: LargeRingQueueInitialization<Self>;
	
	/// When the `LargeRingQueue` is dropped, are its allocated elements safe to drop.
	///
	/// In other words, after `initialization()`'s  `LargeRingQueueInitialization<Self>` has been used, are the elements NOT on the queue safe to drop (or even need to be dropped).
	///
	/// Defaults to `needs_drop::<Self>()`.
	/// If the queue elements are `impl Copy`, this *will* be `false`.
	const ElementsAllocatedFromQueueDropWhenQueueIsDropped: bool = needs_drop::<Self>();
	
	/// When the `LargeRingQueue` is dropped, are its elements safe to drop.
	///
	/// In other words, after `initialization()`'s  `LargeRingQueueInitialization<Self>` has been used, are the elements on the queue safe to drop (or even need to be dropped).
	///
	/// Defaults to `needs_drop::<Self>()`.
	/// If the queue elements are `impl Copy`, this *will* be `false`.
	const ElementsLeftOnQueueDropWhenQueueIsDropped: bool = needs_drop::<Self>();
}
