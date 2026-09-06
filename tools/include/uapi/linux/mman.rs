//! Automatically rewritten from C Header to Rust Module
//! Source: tools/include/uapi/linux/mman.h
#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use core::ffi::*;

// --- Linux Kernel Primitives Prelude ---
pub type uid_t = u32;
pub type gid_t = u32;
pub type uid16_t = u16;
pub type gid16_t = u16;
pub type pid_t = i32;
pub type mode_t = u32;
pub type umode_t = u16;
pub type nlink_t = u32;
pub type off_t = i64;
pub type loff_t = i64;
pub type dev_t = u32;
pub type ino_t = u64;
pub type size_t = usize;
pub type ssize_t = isize;
pub type uintptr_t = usize;
pub type intptr_t = isize;
pub type ptrdiff_t = isize;
pub type clockid_t = i32;
pub type timer_t = i32;
pub type time64_t = i64;
pub type atomic_t = core::sync::atomic::AtomicI32;
pub type atomic64_t = core::sync::atomic::AtomicI64;
// ---------------------------------------


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note

pub const MREMAP_MAYMOVE: c_int = 1;
pub const MREMAP_FIXED: c_int = 2;
pub const MREMAP_DONTUNMAP: c_int = 4;
pub const OVERCOMMIT_GUESS: c_int = 0;
pub const OVERCOMMIT_ALWAYS: c_int = 1;
pub const OVERCOMMIT_NEVER: c_int = 2;
pub const MAP_SHARED: c_uint = 0x01		/* Share changes */;
pub const MAP_PRIVATE: c_uint = 0x02		/* Changes are private */;
pub const MAP_SHARED_VALIDATE: c_uint = 0x03	/* share + validate extension flags */;
pub const MAP_DROPPABLE: c_uint = 0x08		/* Zero memory under memory pressure. */;
//
// Huge page size encoding when MAP_HUGETLB is specified, and a huge page
// size other than the default is desired.  See hugetlb_encode.h.
// All known huge page size encodings are provided here.  It is the
// responsibility of the application to know which sizes are supported on
// the running system.  See mmap(2) man page for details.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cachestat_range {
    pub off: __u64,
    pub len: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cachestat {
    pub nr_cache: __u64,
    pub nr_dirty: __u64,
    pub nr_writeback: __u64,
    pub nr_evicted: __u64,
    pub nr_recently_evicted: __u64,
}
