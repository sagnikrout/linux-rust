//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/soundwire/sdw_type.h
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
// Copyright(c) 2015-17 Intel Corporation.

extern "C" {
    pub fn __sdw_register_driver(drv: *mut sdw_driver, owner: *mut module) -> c_int;
}
extern "C" {
    pub fn sdw_unregister_driver(drv: *mut sdw_driver);
}
extern "C" {
    pub fn sdw_slave_uevent(dev: *const device, env: *mut kobj_uevent_env) -> c_int;
}
//
// module_sdw_driver() - Helper macro for registering a Soundwire driver
// @__sdw_driver: soundwire slave driver struct
//
// Helper macro for Soundwire drivers which do not do anything special in
// module init/exit. This eliminates a lot of boilerplate. Each module may only
// use this macro once, and calling it replaces module_init() and module_exit()
//

