// This file is part of magic-ring-buffer. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/magic-ring-buffer/master/COPYRIGHT. No part of magic-ring-buffer, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of magic-ring-buffer. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/magic-ring-buffer/master/COPYRIGHT.


#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C, align(8))]
struct OnlyEverIncreasesMonotonicallyOffset(u64);

impl Into<usize> for OnlyEverIncreasesMonotonicallyOffset
{
	#[inline(always)]
	fn into(self) -> usize
	{
		self.0 as usize
	}
}

impl Add<Size> for OnlyEverIncreasesMonotonicallyOffset
{
	type Output = Self;

	#[inline(always)]
	fn add(self, rhs: Size) -> Self::Output
	{
		OnlyEverIncreasesMonotonicallyOffset(self.0 + rhs.0)
	}
}

impl AddAssign<Size> for OnlyEverIncreasesMonotonicallyOffset
{
	#[inline(always)]
	fn add_assign(&mut self, rhs: Size)
	{
		self.0 += rhs.0
	}
}

impl Add<u64> for OnlyEverIncreasesMonotonicallyOffset
{
	type Output = Self;
	
	#[inline(always)]
	fn add(self, rhs: u64) -> Self::Output
	{
		OnlyEverIncreasesMonotonicallyOffset(self.0 + rhs)
	}
}

impl AddAssign<u64> for OnlyEverIncreasesMonotonicallyOffset
{
	#[inline(always)]
	fn add_assign(&mut self, rhs: u64)
	{
		self.0 += rhs
	}
}

impl Add<NonZeroU64> for OnlyEverIncreasesMonotonicallyOffset
{
	type Output = Self;
	
	#[inline(always)]
	fn add(self, rhs: NonZeroU64) -> Self::Output
	{
		OnlyEverIncreasesMonotonicallyOffset(self.0 + rhs.get())
	}
}

impl AddAssign<NonZeroU64> for OnlyEverIncreasesMonotonicallyOffset
{
	#[inline(always)]
	fn add_assign(&mut self, rhs: NonZeroU64)
	{
		self.0 += rhs.get()
	}
}

impl BitAnd<u64> for OnlyEverIncreasesMonotonicallyOffset
{
	type Output = u64;
	
	#[inline(always)]
	fn bitand(self, rhs: u64) -> Self::Output
	{
		self.0 & rhs
	}
}

impl Sub<Self> for OnlyEverIncreasesMonotonicallyOffset
{
	type Output = Size;

	#[inline(always)]
	fn sub(self, rhs: Self) -> Self::Output
	{
		Size(self.0 - rhs.0)
	}
}

impl Sub<NonZeroU64> for OnlyEverIncreasesMonotonicallyOffset
{
	type Output = Self;

	#[inline(always)]
	fn sub(self, rhs: NonZeroU64) -> Self::Output
	{
		Self(self.0 - rhs.get())
	}
}
