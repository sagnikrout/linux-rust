//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/mfd/intel-lpss.h
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
// Intel LPSS core support.
//
// Copyright (C) 2015, Intel Corporation
//
// Authors: Andy Shevchenko <andriy.shevchenko@linux.intel.com>
// Mika Westerberg <mika.westerberg@linux.intel.com>
//

//
// Some DSDTs have an unused GEXP ACPI device conflicting with I2C4 resources.
// Set to ignore resource conflicts with ACPI declared SystemMemory regions.
//

//
// Some devices have misconfigured clock divider due to a firmware bug.
// Set this to force the clock divider to 1:1 ratio.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_lpss_platform_info {
    pub mem: *mut resource,
    pub irq: c_int,
    pub quirks: c_uint,
    pub clk_rate: c_ulong,
    pub clk_con_id: *const c_char,
    pub swnode: *const software_node,
}

extern "C" {
    pub fn intel_lpss_remove(dev: *mut device);
}
