//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/lpc_ich.h
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
// linux/drivers/mfd/lpc_ich.h
//
// Copyright (c) 2012 Extreme Engineering Solution, Inc.
// Author: Aaron Sierra <asierra@xes-inc.com>
//

// GPIO resources
pub const ICH_RES_GPIO: c_int = 0;
pub const ICH_RES_GPE0: c_int = 1;
// GPIO compatibility
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lpc_gpio_versions {
    ICH_I3100_GPIO,
    ICH_V5_GPIO,
    ICH_V6_GPIO,
    ICH_V7_GPIO,
    ICH_V9_GPIO,
    ICH_V10CORP_GPIO,
    ICH_V10CONS_GPIO,
    AVOTON_GPIO,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpc_ich_info {
    pub name: [c_char; 32],
    pub iTCO_version: c_uint,
    pub gpio_version: lpc_gpio_versions,
    pub spi_type: intel_spi_type,
    pub gpio_info: *const lpc_ich_gpio_info,
    pub use_gpio: u8,
}
