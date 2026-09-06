//! Automatically rewritten from C Header to Rust Module
//! Source: fs/btrfs/async-thread.h
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
// Copyright (C) 2014 Fujitsu.  All rights reserved.
//

extern "C" {
    pub fn void(arg: *mut *mut btrfs_func_t)(struct btrfs_work) -> typedef;
}
extern "C" {
    pub fn void(arg: *mut *mut btrfs_ordered_func_t)(struct btrfs_work, _arg: bool) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_work {
    pub func: btrfs_func_t,
    pub ordered_func: btrfs_ordered_func_t,
// Don't touch things below
    pub normal_work: work_struct,
    pub ordered_list: list_head,
    pub wq: *mut btrfs_workqueue,
    pub flags: c_ulong,
}

extern "C" {
    pub fn btrfs_destroy_workqueue(wq: *mut btrfs_workqueue);
}
extern "C" {
    pub fn btrfs_workqueue_set_max(wq: *mut btrfs_workqueue, max: c_int);
}
extern "C" {
    pub fn btrfs_work_owner(work: *const btrfs_work) -> *mut btrfs_fs_info  __pure;
}
extern "C" {
    pub fn btrfs_workqueue_owner(wq: *const btrfs_workqueue) -> *mut btrfs_fs_info  __pure;
}
extern "C" {
    pub fn btrfs_workqueue_normal_congested(wq: *const btrfs_workqueue) -> bool;
}
extern "C" {
    pub fn btrfs_flush_workqueue(wq: *mut btrfs_workqueue);
}
