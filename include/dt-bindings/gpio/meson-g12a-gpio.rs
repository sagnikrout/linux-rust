//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/gpio/meson-g12a-gpio.h
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
// Copyright (c) 2018 Amlogic, Inc. All rights reserved.
// Author: Xingyu Chen <xingyu.chen@amlogic.com>
//
// First GPIO chip
pub const GPIOAO_0: c_int = 0;
pub const GPIOAO_1: c_int = 1;
pub const GPIOAO_2: c_int = 2;
pub const GPIOAO_3: c_int = 3;
pub const GPIOAO_4: c_int = 4;
pub const GPIOAO_5: c_int = 5;
pub const GPIOAO_6: c_int = 6;
pub const GPIOAO_7: c_int = 7;
pub const GPIOAO_8: c_int = 8;
pub const GPIOAO_9: c_int = 9;
pub const GPIOAO_10: c_int = 10;
pub const GPIOAO_11: c_int = 11;
pub const GPIOE_0: c_int = 12;
pub const GPIOE_1: c_int = 13;
pub const GPIOE_2: c_int = 14;
// Second GPIO chip
pub const GPIOZ_0: c_int = 0;
pub const GPIOZ_1: c_int = 1;
pub const GPIOZ_2: c_int = 2;
pub const GPIOZ_3: c_int = 3;
pub const GPIOZ_4: c_int = 4;
pub const GPIOZ_5: c_int = 5;
pub const GPIOZ_6: c_int = 6;
pub const GPIOZ_7: c_int = 7;
pub const GPIOZ_8: c_int = 8;
pub const GPIOZ_9: c_int = 9;
pub const GPIOZ_10: c_int = 10;
pub const GPIOZ_11: c_int = 11;
pub const GPIOZ_12: c_int = 12;
pub const GPIOZ_13: c_int = 13;
pub const GPIOZ_14: c_int = 14;
pub const GPIOZ_15: c_int = 15;
pub const GPIOH_0: c_int = 16;
pub const GPIOH_1: c_int = 17;
pub const GPIOH_2: c_int = 18;
pub const GPIOH_3: c_int = 19;
pub const GPIOH_4: c_int = 20;
pub const GPIOH_5: c_int = 21;
pub const GPIOH_6: c_int = 22;
pub const GPIOH_7: c_int = 23;
pub const GPIOH_8: c_int = 24;
pub const BOOT_0: c_int = 25;
pub const BOOT_1: c_int = 26;
pub const BOOT_2: c_int = 27;
pub const BOOT_3: c_int = 28;
pub const BOOT_4: c_int = 29;
pub const BOOT_5: c_int = 30;
pub const BOOT_6: c_int = 31;
pub const BOOT_7: c_int = 32;
pub const BOOT_8: c_int = 33;
pub const BOOT_9: c_int = 34;
pub const BOOT_10: c_int = 35;
pub const BOOT_11: c_int = 36;
pub const BOOT_12: c_int = 37;
pub const BOOT_13: c_int = 38;
pub const BOOT_14: c_int = 39;
pub const BOOT_15: c_int = 40;
pub const GPIOC_0: c_int = 41;
pub const GPIOC_1: c_int = 42;
pub const GPIOC_2: c_int = 43;
pub const GPIOC_3: c_int = 44;
pub const GPIOC_4: c_int = 45;
pub const GPIOC_5: c_int = 46;
pub const GPIOC_6: c_int = 47;
pub const GPIOC_7: c_int = 48;
pub const GPIOA_0: c_int = 49;
pub const GPIOA_1: c_int = 50;
pub const GPIOA_2: c_int = 51;
pub const GPIOA_3: c_int = 52;
pub const GPIOA_4: c_int = 53;
pub const GPIOA_5: c_int = 54;
pub const GPIOA_6: c_int = 55;
pub const GPIOA_7: c_int = 56;
pub const GPIOA_8: c_int = 57;
pub const GPIOA_9: c_int = 58;
pub const GPIOA_10: c_int = 59;
pub const GPIOA_11: c_int = 60;
pub const GPIOA_12: c_int = 61;
pub const GPIOA_13: c_int = 62;
pub const GPIOA_14: c_int = 63;
pub const GPIOA_15: c_int = 64;
pub const GPIOX_0: c_int = 65;
pub const GPIOX_1: c_int = 66;
pub const GPIOX_2: c_int = 67;
pub const GPIOX_3: c_int = 68;
pub const GPIOX_4: c_int = 69;
pub const GPIOX_5: c_int = 70;
pub const GPIOX_6: c_int = 71;
pub const GPIOX_7: c_int = 72;
pub const GPIOX_8: c_int = 73;
pub const GPIOX_9: c_int = 74;
pub const GPIOX_10: c_int = 75;
pub const GPIOX_11: c_int = 76;
pub const GPIOX_12: c_int = 77;
pub const GPIOX_13: c_int = 78;
pub const GPIOX_14: c_int = 79;
pub const GPIOX_15: c_int = 80;
pub const GPIOX_16: c_int = 81;
pub const GPIOX_17: c_int = 82;
pub const GPIOX_18: c_int = 83;
pub const GPIOX_19: c_int = 84;
