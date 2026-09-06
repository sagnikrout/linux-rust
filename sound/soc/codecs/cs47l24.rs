//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/cs47l24.h
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
// cs47l24.h  --  ALSA SoC Audio driver for Cirrus Logic CS47L24
//
// Copyright 2015 Cirrus Logic Inc.
//
// Author: Richard Fitzgerald <rf@opensource.wolfsonmicro.com>
//

pub const CS47L24_FLL1: c_int = 1;
pub const CS47L24_FLL2: c_int = 2;
pub const CS47L24_FLL1_REFCLK: c_int = 3;
pub const CS47L24_FLL2_REFCLK: c_int = 4;
