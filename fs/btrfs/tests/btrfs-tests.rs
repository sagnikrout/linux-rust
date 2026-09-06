//! Automatically rewritten from C Header to Rust Module
//! Source: fs/btrfs/tests/btrfs-tests.h
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
// Copyright (C) 2013 Fusion IO.  All rights reserved.
//

extern "C" {
    pub fn btrfs_run_sanity_tests() -> c_int;
}

extern "C" {
    pub fn btrfs_test_extent_buffer_operations(sectorsize: u32, nodesize: u32) -> c_int;
}
extern "C" {
    pub fn btrfs_test_free_space_cache(sectorsize: u32, nodesize: u32) -> c_int;
}
extern "C" {
    pub fn btrfs_test_extent_io(sectorsize: u32, nodesize: u32) -> c_int;
}
extern "C" {
    pub fn btrfs_test_inodes(sectorsize: u32, nodesize: u32) -> c_int;
}
extern "C" {
    pub fn btrfs_test_qgroups(sectorsize: u32, nodesize: u32) -> c_int;
}
extern "C" {
    pub fn btrfs_test_free_space_tree(sectorsize: u32, nodesize: u32) -> c_int;
}
extern "C" {
    pub fn btrfs_test_raid_stripe_tree(sectorsize: u32, nodesize: u32) -> c_int;
}
extern "C" {
    pub fn btrfs_test_extent_map() -> c_int;
}
extern "C" {
    pub fn btrfs_test_delayed_refs(sectorsize: u32, nodesize: u32) -> c_int;
}
extern "C" {
    pub fn btrfs_test_chunk_allocation(sectorsize: u32, nodesize: u32) -> c_int;
}
extern "C" {
    pub fn btrfs_free_dummy_fs_info(fs_info: *mut btrfs_fs_info);
}
extern "C" {
    pub fn btrfs_free_dummy_root(root: *mut btrfs_root);
}
extern "C" {
    pub fn btrfs_free_dummy_block_group(cache: *mut btrfs_block_group);
}
extern "C" {
    pub fn btrfs_init_dummy_transaction(trans: *mut btrfs_transaction, fs_info: *mut btrfs_fs_info);
}

extern "C" {
    pub fn btrfs_test_zoned() -> c_int;
}

