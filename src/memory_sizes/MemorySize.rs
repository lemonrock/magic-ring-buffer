// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Memory size marker trait.
pub trait MemorySize: LargeRingQueueElement + Sized + RefUnwindSafe + Send + Sync + Debug + Clone
{
}

impl<MS: MemorySize> LargeRingQueueElement for MS
{
	const Initialization: LargeRingQueueInitialization<Self> = LargeRingQueueInitialization::CreateFullOfUninitializedElements;

	const ElementsAllocatedFromQueueDropWhenQueueIsDropped: bool = false;

	const ElementsLeftOnQueueDropWhenQueueIsDropped: bool = false;
}
