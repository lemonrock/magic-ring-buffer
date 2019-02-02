// This file is part of magic-ring-buffer. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/magic-ring-buffer/master/COPYRIGHT. No part of magic-ring-buffer, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of magic-ring-buffer. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/magic-ring-buffer/master/COPYRIGHT.


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

impl Size
{
	/// Rounds up to a page size multiple.
	#[inline(always)]
	pub(crate) fn round_up_to_page_size(self) -> Self
	{
		let page_size = Self::page_size().0;
		Self((self.0 + page_size - 1) / page_size)
	}

	#[cfg(any(target_os = "android", target_os = "emscripten", target_os = "fuschia", target_os = "linux"))]
	pub(crate) fn page_size() -> Self
	{
		extern "C"
		{
			fn getpagesize() -> i32;
		}

		Self(unsafe { getpagesize() } as u32 as u64)
	}

	#[cfg(not(any(target_os = "android", target_os = "emscripten", target_os = "fuschia", target_os = "linux")))]
	pub(crate) fn page_size() -> Self
	{
		use ::libc::_SC_PAGESIZE;
		use ::libc::sysconf;

		Self((unsafe { sysconf(_SC_PAGESIZE) }) as u64)
	}
}
