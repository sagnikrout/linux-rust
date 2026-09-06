//! Automatically rewritten from C Header to Rust Module
//! Source: fs/btrfs/free-space-tree.h
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
// Copyright (C) 2015 Facebook.  All rights reserved.
//

//
// The default size for new free space bitmap items. The last bitmap in a block
// group may be truncated, and none of the free space tree code assumes that
// existing bitmaps are this size.
//
pub const BTRFS_FREE_SPACE_BITMAP_SIZE: c_int = 256;

extern "C" {
    pub fn btrfs_set_free_space_tree_thresholds(block_group: *mut btrfs_block_group);
}
extern "C" {
    pub fn btrfs_create_free_space_tree(fs_info: *mut btrfs_fs_info) -> c_int;
}
extern "C" {
    pub fn btrfs_delete_free_space_tree(fs_info: *mut btrfs_fs_info) -> c_int;
}
extern "C" {
    pub fn btrfs_rebuild_free_space_tree(fs_info: *mut btrfs_fs_info) -> c_int;
}
extern "C" {
    pub fn btrfs_load_free_space_tree(caching_ctl: *mut btrfs_caching_control) -> c_int;
}
extern "C" {
    pub fn btrfs_delete_orphan_free_space_entries(fs_info: *mut btrfs_fs_info) -> c_int;
}

