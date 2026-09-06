//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/mc13892.h
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
// Copyright 2010 Yong Shen <yong.shen@linaro.org>
//

pub const MC13892_SW1: c_int = 0;
pub const MC13892_SW2: c_int = 1;
pub const MC13892_SW3: c_int = 2;
pub const MC13892_SW4: c_int = 3;
pub const MC13892_SWBST: c_int = 4;
pub const MC13892_VIOHI: c_int = 5;
pub const MC13892_VPLL: c_int = 6;
pub const MC13892_VDIG: c_int = 7;
pub const MC13892_VSD: c_int = 8;
pub const MC13892_VUSB2: c_int = 9;
pub const MC13892_VVIDEO: c_int = 10;
pub const MC13892_VAUDIO: c_int = 11;
pub const MC13892_VCAM: c_int = 12;
pub const MC13892_VGEN1: c_int = 13;
pub const MC13892_VGEN2: c_int = 14;
pub const MC13892_VGEN3: c_int = 15;
pub const MC13892_VUSB: c_int = 16;
pub const MC13892_GPO1: c_int = 17;
pub const MC13892_GPO2: c_int = 18;
pub const MC13892_GPO3: c_int = 19;
pub const MC13892_GPO4: c_int = 20;
pub const MC13892_PWGT1SPI: c_int = 21;
pub const MC13892_PWGT2SPI: c_int = 22;
pub const MC13892_VCOINCELL: c_int = 23;
