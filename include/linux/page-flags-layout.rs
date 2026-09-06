//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/page-flags-layout.h
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


// SPDX-License-Identifier: GPL-2.0

//
// When a memory allocation must conform to specific limitations (such
// as being suitable for DMA) the caller will pass in hints to the
// allocator in the gfp_mask, in the zone modifier bits.  These bits
// are used to select a priority ordered list of memory zones which
// match the requested limits. See gfp_zone() in include/linux/gfp.h
//

pub const ZONES_SHIFT: c_int = 0;

pub const ZONES_SHIFT: c_int = 1;

pub const ZONES_SHIFT: c_int = 2;

pub const ZONES_SHIFT: c_int = 3;

pub const SECTIONS_SHIFT: c_int = 0;

//
// page->flags layout:
//
// There are five possibilities for how page->flags get laid out.  The first
// pair is for the normal case without sparsemem. The second pair is for
// sparsemem when there is plenty of space for node and section information.
// The last is when there is insufficient space in page->flags and a separate
// lookup is necessary.
//
// No sparsemem or sparsemem vmemmap: |       NODE     | ZONE |             ... | FLAGS |
// " plus space for last_cpupid: |       NODE     | ZONE | LAST_CPUPID ... | FLAGS |
// classic sparse with space for node:| SECTION | NODE | ZONE |             ... | FLAGS |
// " plus space for last_cpupid: | SECTION | NODE | ZONE | LAST_CPUPID ... | FLAGS |
// classic sparse no space for node:  | SECTION |     ZONE    | ... | FLAGS |
//

pub const SECTIONS_WIDTH: c_int = 0;

pub const NODES_WIDTH: c_int = 0;

//
// Note that this #define MUST have a value so that it can be tested with
// the IS_ENABLED() macro.
//

pub const NODE_NOT_IN_PAGE_FLAGS: c_int = 1;

pub const KASAN_TAG_WIDTH: c_int = 8;

pub const KASAN_TAG_WIDTH: c_int = 4;

pub const KASAN_TAG_WIDTH: c_int = 0;

pub const LAST__PID_SHIFT: c_int = 8;

pub const LAST_CPUPID_SHIFT: c_int = 0;

pub const LAST_CPUPID_WIDTH: c_int = 0;

// Macro flag: #define LAST_CPUPID_NOT_IN_PAGE_FLAGS

// see the comment on MAX_NR_TIERS

