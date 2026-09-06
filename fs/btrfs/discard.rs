//! Automatically rewritten from C Header to Rust Module
//! Source: fs/btrfs/discard.h
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

// Discard size limits

// List operations
extern "C" {
    pub fn btrfs_discard_check_filter(block_group: *mut btrfs_block_group, bytes: u64);
}
// Work operations
// Update operations
extern "C" {
    pub fn btrfs_discard_calc_delay(discard_ctl: *mut btrfs_discard_ctl);
}
extern "C" {
    pub fn btrfs_discard_update_discardable(block_group: *mut btrfs_block_group);
}
// Setup/cleanup operations
extern "C" {
    pub fn btrfs_discard_punt_unused_bgs_list(fs_info: *mut btrfs_fs_info);
}
extern "C" {
    pub fn btrfs_discard_resume(fs_info: *mut btrfs_fs_info);
}
extern "C" {
    pub fn btrfs_discard_stop(fs_info: *mut btrfs_fs_info);
}
extern "C" {
    pub fn btrfs_discard_init(fs_info: *mut btrfs_fs_info);
}
extern "C" {
    pub fn btrfs_discard_cleanup(fs_info: *mut btrfs_fs_info);
}
