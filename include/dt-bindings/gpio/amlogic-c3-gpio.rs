//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/gpio/amlogic-c3-gpio.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR MIT)
//
// Copyright (c) 2021 Amlogic, Inc. All rights reserved.
// Author: Huqiang Qin <huqiang.qin@amlogic.com>
//
pub const GPIOE_0: c_int = 0;
pub const GPIOE_1: c_int = 1;
pub const GPIOE_2: c_int = 2;
pub const GPIOE_3: c_int = 3;
pub const GPIOE_4: c_int = 4;
pub const GPIOB_0: c_int = 5;
pub const GPIOB_1: c_int = 6;
pub const GPIOB_2: c_int = 7;
pub const GPIOB_3: c_int = 8;
pub const GPIOB_4: c_int = 9;
pub const GPIOB_5: c_int = 10;
pub const GPIOB_6: c_int = 11;
pub const GPIOB_7: c_int = 12;
pub const GPIOB_8: c_int = 13;
pub const GPIOB_9: c_int = 14;
pub const GPIOB_10: c_int = 15;
pub const GPIOB_11: c_int = 16;
pub const GPIOB_12: c_int = 17;
pub const GPIOB_13: c_int = 18;
pub const GPIOB_14: c_int = 19;
pub const GPIOC_0: c_int = 20;
pub const GPIOC_1: c_int = 21;
pub const GPIOC_2: c_int = 22;
pub const GPIOC_3: c_int = 23;
pub const GPIOC_4: c_int = 24;
pub const GPIOC_5: c_int = 25;
pub const GPIOC_6: c_int = 26;
pub const GPIOX_0: c_int = 27;
pub const GPIOX_1: c_int = 28;
pub const GPIOX_2: c_int = 29;
pub const GPIOX_3: c_int = 30;
pub const GPIOX_4: c_int = 31;
pub const GPIOX_5: c_int = 32;
pub const GPIOX_6: c_int = 33;
pub const GPIOX_7: c_int = 34;
pub const GPIOX_8: c_int = 35;
pub const GPIOX_9: c_int = 36;
pub const GPIOX_10: c_int = 37;
pub const GPIOX_11: c_int = 38;
pub const GPIOX_12: c_int = 39;
pub const GPIOX_13: c_int = 40;
pub const GPIOD_0: c_int = 41;
pub const GPIOD_1: c_int = 42;
pub const GPIOD_2: c_int = 43;
pub const GPIOD_3: c_int = 44;
pub const GPIOD_4: c_int = 45;
pub const GPIOD_5: c_int = 46;
pub const GPIOD_6: c_int = 47;
pub const GPIOA_0: c_int = 48;
pub const GPIOA_1: c_int = 49;
pub const GPIOA_2: c_int = 50;
pub const GPIOA_3: c_int = 51;
pub const GPIOA_4: c_int = 52;
pub const GPIOA_5: c_int = 53;
pub const GPIO_TEST_N: c_int = 54;
