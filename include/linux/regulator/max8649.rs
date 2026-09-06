//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/regulator/max8649.h
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
// Interface of Maxim max8649
//
// Copyright (C) 2009-2010 Marvell International Ltd.
// Haojian Zhuang <haojian.zhuang@marvell.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct max8649_platform_data {
    pub regulator: *mut regulator_init_data,
    pub /: *mut *mut unsigned mode:2; / bit[1:0] = VID1,VID0,
    pub extclk_freq:2: unsigned,
    pub extclk:1: unsigned,
    pub ramp_timing:3: unsigned,
    pub ramp_down:1: unsigned,
}
