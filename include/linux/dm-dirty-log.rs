//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/dm-dirty-log.h
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
// Copyright (C) 2003 Sistina Software
// Copyright (C) 2004-2008 Red Hat, Inc. All rights reserved.
//
// Device-Mapper dirty region log.
//
// This file is released under the LGPL.
//

pub type region_t = sector_t;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dm_dirty_log {
    pub type: *mut dm_dirty_log_type,
    pub ti): *mut *mut int (flush_callback_fn)(struct dm_target,
    pub context: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dm_dirty_log_type {
    pub name: *const c_char,
    pub module: *mut module,
// For internal device-mapper use
    pub list: list_head,
    pub argv): *mut unsigned int argc, char,
    pub log): *mut *mut void (dtr)(struct dm_dirty_log,
//
// There are times when we don't want the log to touch
// the disk.
//
    pub log): *mut *mut int (presuspend)(struct dm_dirty_log,
    pub log): *mut *mut int (postsuspend)(struct dm_dirty_log,
    pub log): *mut *mut int (resume)(struct dm_dirty_log,
//
// Retrieves the smallest size of region that the log can
// deal with.
//
    pub log): *mut *mut uint32_t (get_region_size)(struct dm_dirty_log,
//
// A predicate to say whether a region is clean or not.
// May block.
//
    pub region): *mut *mut *mut int (is_clean)(struct dm_dirty_log log, region_t,
//
// Returns: 0, 1, -EWOULDBLOCK, < 0
//
// A predicate function to check the area given by
// [sector, sector + len) is in sync.
//
// If -EWOULDBLOCK is returned the state of the region is
// unknown, typically this will result in a read being
// passed to a daemon to deal with, since a daemon is
// allowed to block.
//
    pub can_block): c_int,
//
// Flush the current log state (eg, to disk).  This
// function may block.
//
    pub log): *mut *mut int (flush)(struct dm_dirty_log,
//
// Mark an area as clean or dirty.  These functions may
// block, though for performance reasons blocking should
// be extremely rare (eg, allocating another chunk of
// memory for some reason).
//
    pub region): *mut *mut *mut void (mark_region)(struct dm_dirty_log log, region_t,
    pub region): *mut *mut *mut void (clear_region)(struct dm_dirty_log log, region_t,
//
// Returns: <0 (error), 0 (no region), 1 (region)
//
// The mirrord will need perform recovery on regions of
// the mirror that are in the NOSYNC state.  This
// function asks the log to tell the caller about the
// next region that this machine should recover.
//
// Do not confuse this function with 'in_sync()', one
// tells you if an area is synchronised, the other
// assigns recovery work.
//
    pub region): *mut *mut *mut int (get_resync_work)(struct dm_dirty_log log, region_t,
//
// This notifies the log that the resync status of a region
// has changed.  It also clears the region from the recovering
// list (if present).
//
    pub in_sync): region_t region, int,
//
// Returns the number of regions that are in sync.
//
    pub log): *mut *mut region_t (get_sync_count)(struct dm_dirty_log,
//
// Support function for mirror status requests.
//
    pub maxlen): *mut *mut char result, unsigned int,
//
// is_remote_recovering is necessary for cluster mirroring. It provides
// a way to detect recovery on another node, so we aren't writing
// concurrently.  This function is likely to block (when a cluster log
// is used).
//
// Returns: 0, 1
//
    pub region): *mut *mut *mut int (is_remote_recovering)(struct dm_dirty_log log, region_t,
}

extern "C" {
    pub fn dm_dirty_log_type_register(type: *mut dm_dirty_log_type) -> c_int;
}
extern "C" {
    pub fn dm_dirty_log_type_unregister(type: *mut dm_dirty_log_type) -> c_int;
}
//
// Make sure you use these two functions, rather than calling
// type->constructor/destructor() directly.
//
extern "C" {
    pub fn dm_dirty_log_destroy(log: *mut dm_dirty_log);
}

