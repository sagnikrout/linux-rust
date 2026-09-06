//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/greybus.h
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
// Greybus driver and device API
//
// Copyright 2014-2015 Google Inc.
// Copyright 2014-2015 Linaro Ltd.
//

// Matches up with the Greybus Protocol specification document
pub const GREYBUS_VERSION_MAJOR: c_uint = 0x00;
pub const GREYBUS_VERSION_MINOR: c_uint = 0x01;

// Maximum number of CPorts

#[repr(C)]
#[derive(Copy, Clone)]
pub struct greybus_driver {
    pub name: *const c_char,
    pub id): *const greybus_bundle_id,
    pub bundle): *mut *mut void (disconnect)(struct gb_bundle,
    pub id_table: *const greybus_bundle_id,
    pub driver: device_driver,
}

extern "C" {
    pub fn dev_get_drvdata(_arg: &bundle->dev) -> return;
}
// Don't call these directly, use the module_greybus_driver() macro instead
extern "C" {
    pub fn greybus_deregister_driver(driver: *mut greybus_driver);
}
// define to get proper THIS_MODULE and KBUILD_MODNAME values

//
// module_greybus_driver() - Helper macro for registering a Greybus driver
// @__greybus_driver: greybus_driver structure
//
// Helper macro for Greybus drivers to set up proper module init / exit
// functions.  Replaces module_init() and module_exit() and keeps people from
// printing pointless things to the kernel log when their driver is loaded.
//

extern "C" {
    pub fn greybus_disabled() -> c_int;
}
extern "C" {
    pub fn gb_debugfs_init();
}
extern "C" {
    pub fn gb_debugfs_cleanup();
}

