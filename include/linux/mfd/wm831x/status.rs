//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/wm831x/status.h
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
// include/linux/mfd/wm831x/status.h -- Status LEDs for WM831x
//
// Copyright 2009 Wolfson Microelectronics PLC.
//
// Author: Mark Brown <broonie@opensource.wolfsonmicro.com>
//
pub const WM831X_LED_SRC_MASK: c_uint = 0xC000  /* LED_SRC - [15:14] */;

pub const WM831X_LED_MODE_MASK: c_uint = 0x0300  /* LED_MODE - [9:8] */;

pub const WM831X_LED_SEQ_LEN_MASK: c_uint = 0x0030  /* LED_SEQ_LEN - [5:4] */;

pub const WM831X_LED_DUR_MASK: c_uint = 0x000C  /* LED_DUR - [3:2] */;

pub const WM831X_LED_DUTY_CYC_MASK: c_uint = 0x0003  /* LED_DUTY_CYC - [1:0] */;

