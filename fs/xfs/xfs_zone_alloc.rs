//! Automatically rewritten from C Header to Rust Module
//! Source: fs/xfs/xfs_zone_alloc.h
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
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_zone_alloc_ctx {
    pub open_zone: *mut xfs_open_zone,
    pub reserved_blocks: xfs_filblks_t,
}

//
// Grab any available space, even if it is less than what the caller asked for.
//

//
// Only grab instantly available space, don't wait or GC.
//

//
// Dip into the reserved pool.
//

extern "C" {
    pub fn xfs_zoned_add_available(mp: *mut xfs_mount, count_fsb: xfs_filblks_t);
}
extern "C" {
    pub fn xfs_open_zone_put(oz: *mut xfs_open_zone);
}
extern "C" {
    pub fn xfs_zoned_wake_all(mp: *mut xfs_mount);
}
extern "C" {
    pub fn xfs_zone_rgbno_is_valid(rtg: *mut xfs_rtgroup, rgbno: xfs_rgnumber_t) -> bool;
}
extern "C" {
    pub fn xfs_mark_rtg_boundary(ioend: *mut iomap_ioend);
}
extern "C" {
    pub fn xfs_zone_mark_free(rtg: *mut xfs_rtgroup);
}
extern "C" {
    pub fn xfs_zoned_show_stats(m: *mut seq_file, mp: *mut xfs_mount);
}

extern "C" {
    pub fn xfs_mount_zones(mp: *mut xfs_mount) -> c_int;
}
extern "C" {
    pub fn xfs_unmount_zones(mp: *mut xfs_mount);
}
extern "C" {
    pub fn xfs_zone_gc_start(mp: *mut xfs_mount);
}
extern "C" {
    pub fn xfs_zone_gc_stop(mp: *mut xfs_mount);
}
extern "C" {
    pub fn xfs_zone_gc_wakeup(mp: *mut xfs_mount);
}

