// This file is part of magic-ring-buffer. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/magic-ring-buffer/master/COPYRIGHT. No part of magic-ring-buffer, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of magic-ring-buffer. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/magic-ring-buffer/master/COPYRIGHT.


#[repr(C, align(16))]
struct CompareExchangeOnlyEverIncreasesMonotonicallyOffset(AtomicU64);

impl Default for CompareExchangeOnlyEverIncreasesMonotonicallyOffset
{
	#[inline(always)]
	fn default() -> Self
	{
		Self(AtomicU64::new(OnlyEverIncreasesMonotonicallyOffset::default().0))
	}
}

impl Debug for CompareExchangeOnlyEverIncreasesMonotonicallyOffset
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "CompareExchangeOnlyEverIncreasesMonotonicallyOffset({:?})", self.get())
	}
}

impl CompareExchangeOnlyEverIncreasesMonotonicallyOffset
{
	#[inline(always)]
	pub(crate) fn get(&self) -> OnlyEverIncreasesMonotonicallyOffset
	{
		OnlyEverIncreasesMonotonicallyOffset(self.0.load(Acquire))
	}

	#[inline(always)]
	pub(crate) fn set(&self, offset: OnlyEverIncreasesMonotonicallyOffset)
	{
		self.0.store(offset.0, Release)
	}

	#[inline(always)]
	pub(crate) fn try_to_update(&self, current_value: OnlyEverIncreasesMonotonicallyOffset, new_value: OnlyEverIncreasesMonotonicallyOffset) -> Result<(), OnlyEverIncreasesMonotonicallyOffset>
	{
		match self.0.compare_exchange(current_value.0, new_value.0, AcqRel, AcqRel)
		{
			Ok(_current) => Ok(()),
			Err(was) => Err(OnlyEverIncreasesMonotonicallyOffset(was))
		}
	}

	#[inline(always)]
	pub(crate) fn fetch_add(&self, increment: Size) -> (OnlyEverIncreasesMonotonicallyOffset, OnlyEverIncreasesMonotonicallyOffset)
	{
		let previous = OnlyEverIncreasesMonotonicallyOffset(self.0.fetch_add(increment.into(), Acquire));
		(previous, previous + increment)
	}
}
