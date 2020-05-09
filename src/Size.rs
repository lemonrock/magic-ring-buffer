// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C, align(8))]
pub(crate) struct Size(u64);

impl From<usize> for Size
{
	#[inline(always)]
	fn from(value: usize) -> Self
	{
		 Self(value as u64)
	}
}
impl From<NonZeroU64> for Size
{
	#[inline(always)]
	fn from(value: NonZeroU64) -> Self
	{
		 Self(value.get())
	}
}

impl Into<usize> for Size
{
	#[inline(always)]
	fn into(self) -> usize
	{
		 self.0 as usize
	}
}

impl Into<u64> for Size
{
	#[inline(always)]
	fn into(self) -> u64
	{
		 self.0
	}
}

impl Into<i64> for Size
{
	#[inline(always)]
	fn into(self) -> i64
	{
		 self.0 as i64
	}
}

impl Sub<Self> for Size
{
	type Output = Self;

	#[inline(always)]
	fn sub(self, rhs: Self) -> Self::Output
	{
		 Self(self.0 - rhs.0)
	}
}

impl Mul<usize> for Size
{
	type Output = Size;

	#[inline(always)]
	fn mul(self, rhs: usize) -> Self::Output
	{
		 Self(self.0 * rhs as u64)
	}
}
