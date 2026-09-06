//! Automatically rewritten from C Header to Rust Module
//! Source: fs/btrfs/disk-io.h
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
// Copyright (C) 2007 Oracle.  All rights reserved.
//

pub const BTRFS_SUPER_MIRROR_MAX: c_int = 3;
pub const BTRFS_SUPER_MIRROR_SHIFT: c_int = 12;
//
// Fixed blocksize for all devices, applies to specific ways of reading
// metadata like superblock. Must meet the set_blocksize requirements.
//
// Do not change.
//

extern "C" {
    pub fn btrfs_check_leaked_roots(fs_info: *const btrfs_fs_info);
}
extern "C" {
    pub fn btrfs_init_fs_info(fs_info: *mut btrfs_fs_info);
}
extern "C" {
    pub fn btrfs_start_pre_rw_mount(fs_info: *mut btrfs_fs_info) -> c_int;
}
extern "C" {
    pub fn open_ctree(sb: *mut super_block, fs_devices: *mut btrfs_fs_devices) -> int __cold;
}
extern "C" {
    pub fn close_ctree(fs_info: *mut btrfs_fs_info) -> void __cold;
}
extern "C" {
    pub fn btrfs_check_features(fs_info: *mut btrfs_fs_info, is_rw_mount: bool) -> c_int;
}
extern "C" {
    pub fn write_all_supers(trans: *mut btrfs_trans_handle) -> c_int;
}
extern "C" {
    pub fn btrfs_commit_super(fs_info: *mut btrfs_fs_info) -> c_int;
}
extern "C" {
    pub fn btrfs_free_fs_roots(fs_info: *mut btrfs_fs_info);
}
extern "C" {
    pub fn btrfs_global_root_insert(root: *mut btrfs_root) -> c_int;
}
extern "C" {
    pub fn btrfs_global_root_delete(root: *mut btrfs_root);
}
extern "C" {
    pub fn btrfs_free_fs_info(fs_info: *mut btrfs_fs_info);
}
extern "C" {
    pub fn btrfs_btree_balance_dirty(fs_info: *mut btrfs_fs_info);
}
extern "C" {
    pub fn btrfs_btree_balance_dirty_nodelay(fs_info: *mut btrfs_fs_info);
}

//
// This function is used to grab the root, and avoid it is freed when we
// access it. But it doesn't ensure that the tree is not dropped.
//
extern "C" {
    pub fn btrfs_put_root(root: *mut btrfs_root);
}
extern "C" {
    pub fn btree_csum_one_bio(bbio: *mut btrfs_bio) -> c_int;
}
extern "C" {
    pub fn btrfs_cleanup_one_transaction(trans: *mut btrfs_transaction);
}
extern "C" {
    pub fn btrfs_get_num_tolerated_disk_barrier_failures(flags: u64) -> c_int;
}
extern "C" {
    pub fn btrfs_get_free_objectid(root: *mut btrfs_root, objectid: *mut u64) -> c_int;
}
extern "C" {
    pub fn btrfs_init_root_free_objectid(root: *mut btrfs_root) -> c_int;
}
