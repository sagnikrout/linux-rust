//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/platform_data/max3421-hcd.h
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
// Copyright (c) 2014 eGauge Systems LLC
// Contributed by David Mosberger-Tang <davidm@egauge.net>
//
// Platform-data structure for MAX3421 USB HCD driver.
//

// Macro flag: #define MAX3421_HCD_PLAT_H_INCLUDED
//
// This structure defines the mapping of certain auxiliary functions to the
// MAX3421E GPIO pins.  The chip has eight GP inputs and eight GP outputs.
// A value of 0 indicates that the pin is not used/wired to anything.
//
// At this point, the only control the max3421-hcd driver cares about is
// to control Vbus (5V to the peripheral).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct max3421_hcd_platform_data {
    pub /: *mut *mut u8 vbus_gpout; / pin controlling Vbus,
    pub /: *mut *mut u8 vbus_active_level; / level that turns on power,
}
