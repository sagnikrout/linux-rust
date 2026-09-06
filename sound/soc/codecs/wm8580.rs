//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/wm8580.h
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
// wm8580.h  --  audio driver for WM8580
//
// Copyright 2008 Samsung Electronics.
// Author: Ryu Euiyoul
// ryu.real@gmail.com
//
pub const WM8580_PLLA: c_int = 1;
pub const WM8580_PLLB: c_int = 2;
pub const WM8580_MCLK: c_int = 1;
pub const WM8580_CLKOUTSRC: c_int = 2;
pub const WM8580_CLKSRC_MCLK: c_int = 1;
pub const WM8580_CLKSRC_PLLA: c_int = 2;
pub const WM8580_CLKSRC_PLLB: c_int = 3;
pub const WM8580_CLKSRC_OSC: c_int = 4;
pub const WM8580_CLKSRC_NONE: c_int = 5;
pub const WM8580_CLKSRC_ADCMCLK: c_int = 6;
pub const WM8580_DAI_PAIFRX: c_int = 0;
pub const WM8580_DAI_PAIFTX: c_int = 1;
