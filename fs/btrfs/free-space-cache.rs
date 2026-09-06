//! Automatically rewritten from C Header to Rust Module
//! Source: fs/btrfs/free-space-cache.h
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
// Copyright (C) 2009 Oracle.  All rights reserved.
//

//
// This is the trim state of an extent or bitmap.
//
// BTRFS_TRIM_STATE_TRIMMING is special and used to maintain the state of a
// bitmap as we may need several trims to fully trim a single bitmap entry.
// This is reset should any free space other than trimmed space be added to the
// bitmap.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum btrfs_trim_state {
    BTRFS_TRIM_STATE_UNTRIMMED,
    BTRFS_TRIM_STATE_TRIMMED,
    BTRFS_TRIM_STATE_TRIMMING,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_free_space {
    pub offset_index: rb_node,
    pub bytes_index: rb_node,
    pub offset: u64,
    pub bytes: u64,
    pub max_extent_size: u64,
    pub bitmap: *mut c_ulong,
    pub list: list_head,
    pub trim_state: btrfs_trim_state,
    pub bitmap_extents: i32,
}

extern "C" {
    pub fn fatal_signal_pending(freezing(current: current) ||) -> return;
}
//
// Deltas are an effective way to populate global statistics.  Give macro names
// to make it clear what we're doing.  An example is discard_extents in
// btrfs_free_space_ctl.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_free_space_ctl {
    pub free_space_offset: rb_root,
    pub free_space_bytes: rb_root_cached,
    pub tree_lock: spinlock_t,
    pub extents_thresh: c_int,
    pub free_extents: c_int,
    pub total_bitmaps: c_int,
    pub free_space: u64,
    pub discardable_extents: [i32; BTRFS_STAT_NR_ENTRIES],
    pub discardable_bytes: [i64; BTRFS_STAT_NR_ENTRIES],
    pub block_group: *mut btrfs_block_group,
    pub cache_writeout_mutex: mutex,
    pub trimming_ranges: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_io_ctl {
    pub orig: *mut *mut void cur,,
    pub page: *mut page,
    pub pages: *mut page,
    pub fs_info: *mut btrfs_fs_info,
    pub inode: *mut inode,
    pub size: c_ulong,
    pub index: c_int,
    pub num_pages: c_int,
    pub entries: c_int,
    pub bitmaps: c_int,
}

extern "C" {
    pub fn btrfs_free_space_init() -> int __init;
}
extern "C" {
    pub fn btrfs_free_space_exit() -> void __cold;
}
extern "C" {
    pub fn load_free_space_cache(block_group: *mut btrfs_block_group) -> c_int;
}
extern "C" {
    pub fn btrfs_remove_free_space_cache(block_group: *mut btrfs_block_group);
}
extern "C" {
    pub fn btrfs_is_free_space_trimmed(block_group: *mut btrfs_block_group) -> bool;
}
extern "C" {
    pub fn btrfs_init_free_cluster(cluster: *mut btrfs_free_cluster);
}
extern "C" {
    pub fn btrfs_trim_fully_remapped_block_group(bg: *mut btrfs_block_group);
}
extern "C" {
    pub fn btrfs_free_space_cache_v1_active(fs_info: *mut btrfs_fs_info) -> bool;
}
extern "C" {
    pub fn btrfs_set_free_space_cache_v1_active(fs_info: *mut btrfs_fs_info, active: bool) -> c_int;
}
// Support functions for running our sanity tests

extern "C" {
    pub fn test_check_exists(cache: *mut btrfs_block_group, offset: u64, bytes: u64) -> c_int;
}

