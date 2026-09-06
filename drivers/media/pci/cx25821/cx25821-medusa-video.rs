//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/pci/cx25821/cx25821-medusa-video.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Driver for the Conexant CX25821 PCIe bridge
//
// Copyright (C) 2009 Conexant Systems Inc.
// Authors  <shu.lin@conexant.com>, <hiep.huynh@conexant.com>
//

// Color control constants
pub const VIDEO_PROCAMP_MIN: c_int = 0;
pub const VIDEO_PROCAMP_MAX: c_int = 10000;
pub const UNSIGNED_BYTE_MIN: c_int = 0;
pub const UNSIGNED_BYTE_MAX: c_uint = 0xFF;

pub const SIGNED_BYTE_MAX: c_int = 127;
// Default video color settings
pub const SHARPNESS_DEFAULT: c_int = 50;
pub const SATURATION_DEFAULT: c_int = 5000;
pub const BRIGHTNESS_DEFAULT: c_int = 6200;
pub const CONTRAST_DEFAULT: c_int = 5000;
pub const HUE_DEFAULT: c_int = 5000;
