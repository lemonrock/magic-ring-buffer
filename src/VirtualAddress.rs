// This file is part of magic-ring-buffer. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/magic-ring-buffer/master/COPYRIGHT. No part of magic-ring-buffer, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of magic-ring-buffer. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/magic-ring-buffer/master/COPYRIGHT.


/// Represents a virtual address.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct VirtualAddress(*mut c_void);

impl Default for VirtualAddress
{
	#[inline(always)]
	fn default() -> Self
	{
		VirtualAddress(null_mut())
	}
}

impl Into<*mut u8> for VirtualAddress
{
	#[inline(always)]
	fn into(self) -> *mut u8
	{
		self.0 as *mut u8
	}
}

impl VirtualAddress
{
	/// Is not null.
	#[inline(always)]
	pub(crate) fn is_not_null(self) -> bool
	{
		!self.0.is_null()
	}

	#[inline(always)]
	pub(crate) fn add(self, offset: Size) -> VirtualAddress
	{
		Self(unsafe { self.0.add(offset.into()) })
	}
}
