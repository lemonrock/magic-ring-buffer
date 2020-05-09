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


use errno::errno;
use likely::*;
use libc::c_void;
use libc::EACCES;
use libc::EAGAIN;
use libc::EBADF;
use libc::EINVAL;
use libc::ENFILE;
use libc::ENODEV;
use libc::ENOMEM;
use libc::EOVERFLOW;
use libc::EPERM;
use libc::ETXTBSY;
use libc::ftruncate;
use libc::MAP_ANONYMOUS;
use libc::MAP_FAILED;
use libc::MAP_FIXED;
use libc::MAP_NORESERVE;
use libc::MAP_PRIVATE;
use libc::MAP_SHARED;
use libc::mkstemps;
use libc::mlock;
use libc::mmap;
use libc::munmap;
use libc::PROT_NONE;
use libc::PROT_READ;
use libc::PROT_WRITE;
use libc::unlink;
use linux_support::file_descriptors::RawFdExt;
use std::error;
use std::ffi::CString;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Display;
use std::io;
use std::io::ErrorKind;
use std::ops::{Add, Deref};
use std::ops::Mul;
use std::ops::Rem;
use std::ops::Sub;
use std::os::unix::io::AsRawFd;
use std::os::unix::io::RawFd;
use std::os::unix::ffi::OsStrExt;
use std::path::Path;
use std::ptr::null_mut;
use std::slice::from_raw_parts_mut;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering::*;
use std::sync::atomic::spin_loop_hint;
use linux_support::file_descriptors::memfd::MemoryFileDescriptor;
use linux_support::memory::huge_pages::{HugePageSize, DefaultPageSizeAndHugePageSizes, PageSizeOrHugePageSize};
use linux_support::strings::ConstCStr;
use linux_support::memory::mapping::{MappedMemory, AddressHint, Protection, Sharing};
use std::mem::forget;


include!("CompareExchangeOnlyEverIncreasesMonotonicallyOffset.rs");
include!("MagicRingBuffer.rs");
include!("MirroredMemoryMap.rs");
include!("MirroredMemoryMapCreationError.rs");
include!("OnlyEverIncreasesMonotonicallyOffset.rs");
include!("VirtualAddress.rs");

