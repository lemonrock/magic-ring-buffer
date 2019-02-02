// This file is part of magic-ring-buffer. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/magic-ring-buffer/master/COPYRIGHT. No part of magic-ring-buffer, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of magic-ring-buffer. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/magic-ring-buffer/master/COPYRIGHT.


#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![deny(missing_docs)]
#![deny(unreachable_patterns)]
#![feature(core_intrinsics)]
#![feature(integer_atomics)]


//! #magic-ring-buffer
//! 
//! This is a Rust crate providing a magic ring buffer (also known as a virtual ring buffer, VRB, or mirrored buffer) which is lock-free for multiple producers and a single consumer.
//!
//! A magic ring buffer allows 'wrap-around' of a ring buffer without the need to use two separate read or two separate writes; it exploits the fact that virtual memory does not need to be implemented using contiguous physical memory.
//!
//! The current implementation only works on Android and Linux, as it relies on mapping anonymous shared memory in `/dev/shm`.
//! It should be possible to make implementations that work on Mac OS X, the BSDs and Windows.


#[cfg(any(target_os = "android", target_os = "linux"))] include!("lib.android_linux.rs");
