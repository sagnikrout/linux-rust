//! Automatically rewritten from C Header to Rust Module
//! Source: fs/btrfs/dev-replace.h
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
// Copyright (C) STRATO AG 2012.  All rights reserved.
//

extern "C" {
    pub fn btrfs_init_dev_replace(fs_info: *mut btrfs_fs_info) -> c_int;
}
extern "C" {
    pub fn btrfs_run_dev_replace(trans: *mut btrfs_trans_handle) -> c_int;
}
extern "C" {
    pub fn btrfs_dev_replace_cancel(fs_info: *mut btrfs_fs_info) -> c_int;
}
extern "C" {
    pub fn btrfs_dev_replace_suspend_for_unmount(fs_info: *mut btrfs_fs_info);
}
extern "C" {
    pub fn btrfs_resume_dev_replace_async(fs_info: *mut btrfs_fs_info) -> c_int;
}
extern "C" {
    pub fn btrfs_dev_replace_is_ongoing(dev_replace: *mut btrfs_dev_replace) -> bool __pure;
}
extern "C" {
    pub fn btrfs_bio_counter_inc_blocked(fs_info: *mut btrfs_fs_info);
}
extern "C" {
    pub fn btrfs_bio_counter_sub(fs_info: *mut btrfs_fs_info, amount: i64);
}
