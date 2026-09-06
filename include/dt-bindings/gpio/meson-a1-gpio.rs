//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/gpio/meson-a1-gpio.h
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


// SPDX-License-Identifier: (GPL-2.0+ OR MIT)
//
// Copyright (c) 2019 Amlogic, Inc. All rights reserved.
// Author: Qianggui Song <qianggui.song@amlogic.com>
//
pub const GPIOP_0: c_int = 0;
pub const GPIOP_1: c_int = 1;
pub const GPIOP_2: c_int = 2;
pub const GPIOP_3: c_int = 3;
pub const GPIOP_4: c_int = 4;
pub const GPIOP_5: c_int = 5;
pub const GPIOP_6: c_int = 6;
pub const GPIOP_7: c_int = 7;
pub const GPIOP_8: c_int = 8;
pub const GPIOP_9: c_int = 9;
pub const GPIOP_10: c_int = 10;
pub const GPIOP_11: c_int = 11;
pub const GPIOP_12: c_int = 12;
pub const GPIOB_0: c_int = 13;
pub const GPIOB_1: c_int = 14;
pub const GPIOB_2: c_int = 15;
pub const GPIOB_3: c_int = 16;
pub const GPIOB_4: c_int = 17;
pub const GPIOB_5: c_int = 18;
pub const GPIOB_6: c_int = 19;
pub const GPIOX_0: c_int = 20;
pub const GPIOX_1: c_int = 21;
pub const GPIOX_2: c_int = 22;
pub const GPIOX_3: c_int = 23;
pub const GPIOX_4: c_int = 24;
pub const GPIOX_5: c_int = 25;
pub const GPIOX_6: c_int = 26;
pub const GPIOX_7: c_int = 27;
pub const GPIOX_8: c_int = 28;
pub const GPIOX_9: c_int = 29;
pub const GPIOX_10: c_int = 30;
pub const GPIOX_11: c_int = 31;
pub const GPIOX_12: c_int = 32;
pub const GPIOX_13: c_int = 33;
pub const GPIOX_14: c_int = 34;
pub const GPIOX_15: c_int = 35;
pub const GPIOX_16: c_int = 36;
pub const GPIOF_0: c_int = 37;
pub const GPIOF_1: c_int = 38;
pub const GPIOF_2: c_int = 39;
pub const GPIOF_3: c_int = 40;
pub const GPIOF_4: c_int = 41;
pub const GPIOF_5: c_int = 42;
pub const GPIOF_6: c_int = 43;
pub const GPIOF_7: c_int = 44;
pub const GPIOF_8: c_int = 45;
pub const GPIOF_9: c_int = 46;
pub const GPIOF_10: c_int = 47;
pub const GPIOF_11: c_int = 48;
pub const GPIOF_12: c_int = 49;
pub const GPIOA_0: c_int = 50;
pub const GPIOA_1: c_int = 51;
pub const GPIOA_2: c_int = 52;
pub const GPIOA_3: c_int = 53;
pub const GPIOA_4: c_int = 54;
pub const GPIOA_5: c_int = 55;
pub const GPIOA_6: c_int = 56;
pub const GPIOA_7: c_int = 57;
pub const GPIOA_8: c_int = 58;
pub const GPIOA_9: c_int = 59;
pub const GPIOA_10: c_int = 60;
pub const GPIOA_11: c_int = 61;
