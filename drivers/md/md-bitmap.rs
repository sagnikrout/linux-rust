//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/md/md-bitmap.h
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
// bitmap.h: Copyright (C) Peter T. Breuer (ptb@ot.uc3m.es) 2003
//
// additions: Copyright (C) 2003-2004, Paul Clements, SteelEye Technology, Inc.
//
pub const BITMAP_H: c_int = 1;
pub const BITMAP_MAGIC: c_uint = 0x6d746962;
//
// version 3 is host-endian order, this is deprecated and not used for new
// array
//
pub const BITMAP_MAJOR_LO: c_int = 3;
pub const BITMAP_MAJOR_HOSTENDIAN: c_int = 3;
// version 4 is little-endian order, the default value
pub const BITMAP_MAJOR_HI: c_int = 4;
// version 5 is only used for cluster
pub const BITMAP_MAJOR_CLUSTERED: c_int = 5;
// version 6 is only used for lockless bitmap
pub const BITMAP_MAJOR_LOCKLESS: c_int = 6;
// use these for bitmap->flags and bitmap->sb->state bit-fields
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bitmap_state {
    BITMAP_STALE	   = 1, /* the bitmap file is out of date or had -EIO */
    BITMAP_WRITE_ERROR = 2, /* A write error has occurred */
    BITMAP_FIRST_USE   = 3, /* llbitmap is just created */
    BITMAP_CLEAN       = 4, /* llbitmap is created with assume_clean */
    BITMAP_DAEMON_BUSY = 5, /* llbitmap daemon is not finished after daemon_sleep */
    BITMAP_SHUTDOWN    = 6, /* llbitmap is being destroyed */
    BITMAP_HOSTENDIAN  =15,
}

// the superblock at the front of the bitmap file -- little endian
// reserved for the bitmap.
// notes:
// (1) This event counter is updated before the eventcounter in the md superblock
// When a bitmap is loaded, it is only accepted if this event counter is equal
// to, or one greater than, the event counter in the superblock.
// (2) This event counter is updated when the other one is *if*and*only*if* the
// array is not degraded.  As bits are not cleared when the array is degraded,
// this represents the last time that any bits were cleared.
// If a device is being added that has an event count with this value or
// higher, it is accepted as conforming to the bitmap.
// (3)This is the number of sectors represented by the bitmap, and is the range that
// resync happens across.  For raid1 and raid5/6 it is the size of individual
// devices.  For raid10 it is the size of the array.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct md_bitmap_stats {
    pub events_cleared: u64,
    pub behind_writes: c_int,
    pub behind_wait: bool,
    pub missing_pages: c_ulong,
    pub file_pages: c_ulong,
    pub sync_size: c_ulong,
    pub pages: c_ulong,
    pub file: *mut file,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bitmap_operations {
    pub head: md_submodule_head,
    pub flush): *mut *mut *mut bool (enabled)(void data, bool,
    pub mddev): *mut *mut int (create)(struct mddev,
    pub chunksize): *mut *mut *mut int (resize)(struct mddev mddev, sector_t blocks, int,
    pub mddev): *mut *mut int (load)(struct mddev,
    pub mddev): *mut *mut void (destroy)(struct mddev,
    pub mddev): *mut *mut void (flush)(struct mddev,
    pub mddev): *mut *mut void (write_all)(struct mddev,
    pub e): c_ulong,
// Prepare a range for this bitmap implementation.
    pub discard): bool,
    pub mddev): *mut *mut void (reshape_finish)(struct mddev,
    pub mddev): *mut *mut int (reshape_can_start)(struct mddev,
    pub new_pos): sector_t,
    pub sync): *mut *mut *mut void (unplug)(struct mddev mddev, bool,
    pub mddev): *mut *mut void (daemon_work)(struct mddev,
    pub mddev): *mut *mut void (start_behind_write)(struct mddev,
    pub mddev): *mut *mut void (end_behind_write)(struct mddev,
    pub mddev): *mut *mut void (wait_behind_writes)(struct mddev,
    pub start_write: *mut md_bitmap_fn,
    pub end_write: *mut md_bitmap_fn,
    pub start_discard: *mut md_bitmap_fn,
    pub end_discard: *mut md_bitmap_fn,
    pub offset): *mut *mut *mut sector_t (skip_sync_blocks)(struct mddev mddev, sector_t,
    pub offset): *mut *mut *mut bool (blocks_synced)(struct mddev mddev, sector_t,
    pub degraded): *mut *mut sector_t blocks, bool,
    pub blocks): *mut *mut *mut void (end_sync)(struct mddev mddev, sector_t offset, sector_t,
    pub force): *mut *mut *mut void (cond_end_sync)(struct mddev mddev, sector_t sector, bool,
    pub mddev): *mut *mut void (close_sync)(struct mddev,
    pub data): *mut *mut void (update_sb)(void,
    pub stats): *mut *mut *mut int (get_stats)(void data, struct md_bitmap_stats,
    pub new_hi): sector_t new_lo, sector_t,
    pub slot): *mut *mut *mut *mut void (get_from_slot)(struct mddev mddev, int,
    pub clear_bits): *mut *mut sector_t hi, bool,
    pub pages): *mut *mut *mut void (set_pages)(void data, unsigned long,
    pub data): *mut *mut void (free)(void,
    pub groups: *const attribute_group,
}

// the bitmap API
// bitmap_ops must be registered before creating bitmap.
// always resync if no bitmap
// blocks = 1024;

extern "C" {
    pub fn md_bitmap_init() -> c_int;
}
extern "C" {
    pub fn md_bitmap_exit();
}

extern "C" {
    pub fn md_llbitmap_init() -> c_int;
}
extern "C" {
    pub fn md_llbitmap_exit();
}

