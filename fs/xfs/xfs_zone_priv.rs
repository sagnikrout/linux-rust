//! Automatically rewritten from C Header to Rust Module
//! Source: fs/xfs/xfs_zone_priv.h
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
pub struct xfs_open_zone {
//
// Entry in the open zone list and refcount.  Protected by
// zi_open_zones_lock in struct xfs_zone_info.
//
    pub oz_entry: list_head,
    pub oz_ref: core::sync::atomic::AtomicI32,
//
// oz_allocated is the amount of space already allocated out of the zone
// and is protected by oz_alloc_lock.
//
// For conventional zones it also is the offset of the next write.
//
    pub oz_alloc_lock: spinlock_t,
    pub oz_allocated: xfs_rgblock_t,
//
// oz_written is the number of blocks for which we've received a write
// completion.  oz_written must always be <= oz_allocated and is
// protected by the ILOCK of the rmap inode.
//
    pub oz_written: xfs_rgblock_t,
//
// Write hint (data temperature) assigned to this zone, or
// WRITE_LIFE_NOT_SET if none was set.
//
    pub oz_write_hint: rw_hint,
// Is this open zone used for garbage collection?
    pub oz_is_gc: bool,
//
// Pointer to the RT groups structure for this open zone.  Constant over
// the life time of an open zone.
//
    pub oz_rtg: *mut xfs_rtgroup,
    pub oz_rcu: rcu_head,
}

//
// Number of bitmap buckets to track reclaimable zones.  There are 10 buckets
// so that each 10% of the usable capacity get their own bucket and GC can
// only has to walk the bitmaps of the lesser used zones if there are any.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_zone_info {
//
// List of pending space reservations:
//
    pub zi_reservation_lock: spinlock_t,
    pub zi_reclaim_reservations: list_head,
//
// List and number of open zones:
//
    pub zi_open_zones_lock: spinlock_t,
    pub zi_open_zones: list_head,
    pub zi_nr_open_zones: c_uint,
    pub zi_nr_open_gc_zones: c_uint,
//
// Free zone search cursor and number of free zones:
//
    pub zi_nr_free_zones: core::sync::atomic::AtomicI32,
//
// Wait queue to wait for free zones or open zone resources to become
// available:
//
    pub zi_zone_wait: wait_queue_head_t,
//
// Pointer to the GC thread.
//
    pub zi_gc_thread: *mut task_struct,
//
// List of zones that need a reset:
//
    pub zi_reset_list_lock: spinlock_t,
    pub zi_reset_list: *mut xfs_group,
//
// A set of bitmaps to bucket-sort reclaimable zones by used blocks to help
// garbage collection to quickly find the best candidate for reclaim.
//
    pub zi_used_buckets_lock: spinlock_t,
    pub zi_used_bucket_entries: [c_uint; XFS_ZONE_USED_BUCKETS],
    pub zi_used_bucket_bitmap: [*mut c_ulong; XFS_ZONE_USED_BUCKETS],
}

extern "C" {
    pub fn xfs_zone_gc_reset_sync(rtg: *mut xfs_rtgroup) -> c_int;
}
extern "C" {
    pub fn xfs_zoned_need_gc(mp: *mut xfs_mount) -> bool;
}
extern "C" {
    pub fn xfs_zoned_have_reclaimable(zi: *mut xfs_zone_info) -> bool;
}
extern "C" {
    pub fn xfs_zone_gc_mount(mp: *mut xfs_mount) -> c_int;
}
extern "C" {
    pub fn xfs_zone_gc_unmount(mp: *mut xfs_mount);
}
extern "C" {
    pub fn xfs_zoned_resv_wake_all(mp: *mut xfs_mount);
}
