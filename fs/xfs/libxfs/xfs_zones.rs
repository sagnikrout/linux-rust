//! Automatically rewritten from C Header to Rust Module
//! Source: fs/xfs/libxfs/xfs_zones.h
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
// In order to guarantee forward progress for GC we need to reserve at least
// two zones:  one that will be used for moving data into and one spare zone
// making sure that we have enough space to relocate a nearly-full zone.
// To allow for slightly sloppy accounting for when we need to reserve the
// second zone, we actually reserve three as that is easier than doing fully
// accurate bookkeeping.
//

//
// In addition we need two zones for user writes, one open zone for writing
// and one to still have available blocks without resetting the open zone
// when data in the open zone has been freed.
//

//
// Always keep one zone out of the general open zone pool to allow for GC to
// happen while other writers are waiting for free space.
//

//
// For zoned devices that do not have a limit on the number of open zones, and
// for regular devices using the zoned allocator, use the most common SMR disks
// limit (128) as the default limit on the number of open zones.
//
pub const XFS_DEFAULT_MAX_OPEN_ZONES: c_int = 128;
