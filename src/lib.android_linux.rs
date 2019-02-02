// This file is part of magic-ring-buffer. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/magic-ring-buffer/master/COPYRIGHT. No part of magic-ring-buffer, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of magic-ring-buffer. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/magic-ring-buffer/master/COPYRIGHT.


extern crate errno;
extern crate file_descriptors;
extern crate libc;
#[macro_use] extern crate likely;


use ::errno::errno;
use ::file_descriptors::RawFdExt;
use ::libc::c_void;
use ::libc::EACCES;
use ::libc::EAGAIN;
use ::libc::EBADF;
use ::libc::EINVAL;
use ::libc::ENFILE;
use ::libc::ENODEV;
use ::libc::ENOMEM;
use ::libc::EOVERFLOW;
use ::libc::EPERM;
use ::libc::ETXTBSY;
use ::libc::ftruncate;
use ::libc::MAP_ANONYMOUS;
use ::libc::MAP_FAILED;
use ::libc::MAP_FIXED;
use ::libc::MAP_NORESERVE;
use ::libc::MAP_PRIVATE;
use ::libc::MAP_SHARED;
use ::libc::mkstemps;
use ::libc::mlock;
use ::libc::mmap;
use ::libc::munmap;
use ::libc::PROT_NONE;
use ::libc::PROT_READ;
use ::libc::PROT_WRITE;
use ::libc::unlink;
use ::std::error;
use ::std::ffi::CString;
use ::std::fmt;
use ::std::fmt::Debug;
use ::std::fmt::Formatter;
use ::std::fmt::Display;
use ::std::io;
use ::std::io::ErrorKind;
use ::std::ops::Add;
use ::std::ops::Mul;
use ::std::ops::Rem;
use ::std::ops::Sub;
use ::std::os::unix::io::AsRawFd;
use ::std::os::unix::io::RawFd;
use ::std::os::unix::ffi::OsStrExt;
use ::std::path::Path;
use ::std::ptr::null_mut;
use ::std::slice::from_raw_parts_mut;
use ::std::sync::atomic::AtomicU64;
use ::std::sync::atomic::Ordering::*;
use ::std::sync::atomic::spin_loop_hint;


include!("CompareExchangeOnlyEverIncreasesMonotonicallyOffset.rs");
include!("MagicRingBuffer.rs");
include!("MirroredMemoryMap.rs");
include!("MirroredMemoryMapCreationError.rs");
include!("RemovedTemporaryFileDescriptor.rs");
include!("OnlyEverIncreasesMonotonicallyOffset.rs");
include!("Size.rs");
include!("VirtualAddress.rs");
