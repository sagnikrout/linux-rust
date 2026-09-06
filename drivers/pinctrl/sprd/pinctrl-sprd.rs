//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/pinctrl/sprd/pinctrl-sprd.h
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
// Driver header file for pin controller driver
// Copyright (C) 2017 Spreadtrum  - http://www.spreadtrum.com
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pin_type {
    GLOBAL_CTRL_PIN,
    COMMON_PIN,
    MISC_PIN,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sprd_pins_info {
    pub name: *const c_char,
    pub num: c_uint,
    pub type: pin_type,
// for global control pins configuration
    pub bit_offset: c_ulong,
    pub bit_width: c_ulong,
    pub reg: c_uint,
}

extern "C" {
    pub fn sprd_pinctrl_remove(pdev: *mut platform_device);
}
extern "C" {
    pub fn sprd_pinctrl_shutdown(pdev: *mut platform_device);
}
