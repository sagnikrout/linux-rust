//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/sed-opal.h
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
// Copyright © 2016 Intel Corporation
//
// Authors:
// Rafael Antognolli <rafael.antognolli@intel.com>
// Scott  Bauer      <scott.bauer@intel.com>
//

extern "C" {
    pub fn free_opal_dev(dev: *mut opal_dev);
}
extern "C" {
    pub fn opal_unlock_from_suspend(dev: *mut opal_dev) -> bool;
}
extern "C" {
    pub fn sed_ioctl(dev: *mut opal_dev, cmd: c_uint, ioctl_ptr: *mut void __user) -> c_int;
}

