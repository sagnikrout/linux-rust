//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/vfio/mdev/mdev_private.h
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
// Mediated device internal definitions
//
// Copyright (c) 2016, NVIDIA CORPORATION. All rights reserved.
// Author: Neo Jia <cjia@nvidia.com>
// Kirti Wankhede <kwankhede@nvidia.com>
//

extern "C" {
    pub fn parent_create_sysfs_files(parent: *mut mdev_parent) -> c_int;
}
extern "C" {
    pub fn parent_remove_sysfs_files(parent: *mut mdev_parent);
}
extern "C" {
    pub fn mdev_create_sysfs_files(mdev: *mut mdev_device) -> c_int;
}
extern "C" {
    pub fn mdev_remove_sysfs_files(mdev: *mut mdev_device);
}
extern "C" {
    pub fn mdev_device_create(kobj: *mut mdev_type, uuid: *const guid_t) -> c_int;
}
extern "C" {
    pub fn mdev_device_remove(dev: *mut mdev_device) -> c_int;
}
