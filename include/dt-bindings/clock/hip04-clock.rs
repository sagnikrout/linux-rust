//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/hip04-clock.h
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
// Copyright (c) 2013-2014 Hisilicon Limited.
// Copyright (c) 2013-2014 Linaro Limited.
//
// Author: Haojian Zhuang <haojian.zhuang@linaro.org>
//
pub const HIP04_NONE_CLOCK: c_int = 0;
// fixed rate & fixed factor clocks
pub const HIP04_OSC50M: c_int = 1;
pub const HIP04_CLK_50M: c_int = 2;
pub const HIP04_CLK_168M: c_int = 3;
pub const HIP04_NR_CLKS: c_int = 64;
