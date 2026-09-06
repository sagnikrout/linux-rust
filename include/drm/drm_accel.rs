//! Automatically rewritten from C Header to Rust Module
//! Source: include/drm/drm_accel.h
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
// Copyright 2022 HabanaLabs, Ltd.
// All Rights Reserved.
//

pub const ACCEL_MAJOR: c_int = 261;
pub const ACCEL_MAX_MINORS: c_int = 256;
//
// DRM_ACCEL_FOPS - Default drm accelerators file operations
//
// This macro provides a shorthand for setting the accelerator file ops in the
// &file_operations structure.  If all you need are the default ops, use
// DEFINE_DRM_ACCEL_FOPS instead.
//

//
// DEFINE_DRM_ACCEL_FOPS() - macro to generate file operations for accelerators drivers
// @name: name for the generated structure
//
// This macro autogenerates a suitable &struct file_operations for accelerators based
// drivers, which can be assigned to &drm_driver.fops. Note that this structure
// cannot be shared between drivers, because it contains a reference to the
// current module using THIS_MODULE.
//
// Note that the declaration is already marked as static - if you need a
// non-static version of this you're probably doing it wrong and will break the
// THIS_MODULE reference by accident.
//

extern "C" {
    pub fn accel_core_exit();
}
extern "C" {
    pub fn accel_core_init() -> c_int;
}
extern "C" {
    pub fn accel_set_device_instance_params(kdev: *mut device, index: c_int);
}
extern "C" {
    pub fn accel_open(inode: *mut inode, filp: *mut file) -> c_int;
}
extern "C" {
    pub fn accel_debugfs_register(dev: *mut drm_device);
}

// Return 0 to allow drm_core_init to complete successfully

