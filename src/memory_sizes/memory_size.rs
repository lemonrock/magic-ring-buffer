// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


macro_rules! memory_size
{
	($name: ident, $size_in_kb: expr) =>
	{
		/// Fixed size.
		pub struct $name([u8; $size_in_kb * 1024]);
		
		impl Debug for $name
		{
			#[inline(always)]
			fn fmt(&self, f: &mut Formatter) -> fmt::Result
			{
				write!(f, "{}", stringify!($name))
			}
		}
		
		impl MemorySize for $name
		{
		}
	}
}
