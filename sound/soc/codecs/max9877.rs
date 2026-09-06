//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/max9877.h
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
// max9877.h  --  amp driver for max9877
//
// Copyright (C) 2009 Samsung Electronics Co.Ltd
// Author: Joonyoung Shim <jy0922.shim@samsung.com>
//
pub const MAX9877_INPUT_MODE: c_uint = 0x00;
pub const MAX9877_SPK_VOLUME: c_uint = 0x01;
pub const MAX9877_HPL_VOLUME: c_uint = 0x02;
pub const MAX9877_HPR_VOLUME: c_uint = 0x03;
pub const MAX9877_OUTPUT_MODE: c_uint = 0x04;
// MAX9877_INPUT_MODE

// MAX9877_OUTPUT_MODE

pub const MAX9877_OSC_OFFSET: c_int = 4;

