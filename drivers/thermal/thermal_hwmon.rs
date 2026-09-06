//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/thermal/thermal_hwmon.h
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
// thermal_hwmon.h - Generic Thermal Management hwmon support.
//
// Code based on Intel thermal_core.c. Copyrights of the original code:
// Copyright (C) 2008 Intel Corp
// Copyright (C) 2008 Zhang Rui <rui.zhang@intel.com>
// Copyright (C) 2008 Sujith Thomas <sujith.thomas@intel.com>
//
// Copyright (C) 2013 Texas Instruments
// Copyright (C) 2013 Eduardo Valentin <eduardo.valentin@ti.com>
//

extern "C" {
    pub fn thermal_add_hwmon_sysfs(tz: *mut thermal_zone_device) -> c_int;
}
extern "C" {
    pub fn devm_thermal_add_hwmon_sysfs(dev: *mut device, tz: *mut thermal_zone_device) -> c_int;
}
extern "C" {
    pub fn thermal_remove_hwmon_sysfs(tz: *mut thermal_zone_device);
}

