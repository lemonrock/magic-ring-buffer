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


use static_assertions::assert_cfg;
assert_cfg!(target_os = "linux");
assert_cfg!(target_pointer_width = "64");


use likely::*;
use linux_support::file_descriptors::CreationError;
use linux_support::file_descriptors::memfd::MemoryFileDescriptor;
use linux_support::memory::VirtualAddress;
use linux_support::memory::huge_pages::*;
use linux_support::memory::mapping::*;
use std::cell::UnsafeCell;
use std::error;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Display;
use std::io;
use std::marker::PhantomData;
use std::mem::forget;
use std::mem::MaybeUninit;
use std::mem::needs_drop;
use std::mem::size_of;
use std::num::NonZeroU64;
use std::ops::*;
use std::panic::RefUnwindSafe;
use std::ptr::drop_in_place;
use std::ptr::NonNull;
use std::rc::Rc;
use std::slice::from_raw_parts_mut;
use std::sync::atomic::*;
use std::sync::atomic::Ordering::*;
use swiss_army_knife::hardware_optimized_spin_lock::busy_wait_spin_loop_hint;
use swiss_army_knife::non_zero::new_non_null;
use swiss_army_knife::non_zero::new_non_zero_u64;
use swiss_army_knife::strings::ConstCStr;


/// Convenience trait and types to accommodate the lack of const generics in Rust as of May 2020.
pub mod memory_sizes;


include!("CompareExchangeOnlyEverIncreasesMonotonicallyOffset.rs");
include!("LargeRingQueue.rs");
include!("LargeRingQueueCreationError.rs");
include!("LargeRingQueueElement.rs");
include!("LargeRingQueueInitialization.rs");
include!("MagicRingBuffer.rs");
include!("MirroredMemoryMap.rs");
include!("MirroredMemoryMapCreationError.rs");
include!("OnlyEverIncreasesMonotonicallyOffset.rs");
include!("ReferenceCountedLargeRingQueue.rs");
include!("ReferenceCountedLargeRingQueueElement.rs");
include!("Size.rs");

