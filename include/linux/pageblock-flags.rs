//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/pageblock-flags.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Macros for manipulating and testing flags related to a
// pageblock_nr_pages number of pages.
//
// Copyright (C) IBM Corporation, 2006
//
// Original author, Mel Gorman
// Major cleanups and reduction of bit operations, Andy Whitcroft
//

// Bit indices that affect a whole block of pages
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pageblock_bits {
    PB_migrate_0,
    PB_migrate_1,
    PB_migrate_2,
    PB_compact_skip,/* If set the block is skipped by compaction */

//
// Pageblock isolation is represented with a separate bit, so that
// the migratetype of a block is not overwritten by isolation.
//
    PB_migrate_isolate, /* If set the block is isolated */

//
// Assume the bits will always align on a word. If this assumption
// changes then get/set pageblock needs updating.
//
    __NR_PAGEBLOCK_BITS
}

pub const PAGEBLOCK_ISO_MASK: c_int = 0;

// Huge page sizes are variable

//
// Huge pages are a constant size, but don't exceed the maximum allocation
// granularity.
//

// If huge pages are not used, group by PAGE_BLOCK_MAX_ORDER

// Forward declaration
// Declarations for getting and setting flags. See mm/page_alloc.c

