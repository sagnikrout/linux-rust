//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/pinctrl/amlogic,pinctrl.h
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
// Copyright (c) 2024 Amlogic, Inc. All rights reserved.
// Author: Xianwei Zhao <xianwei.zhao@amlogic.com>
//
// Normal PIN bank
pub const AMLOGIC_GPIO_A: c_int = 0;
pub const AMLOGIC_GPIO_B: c_int = 1;
pub const AMLOGIC_GPIO_C: c_int = 2;
pub const AMLOGIC_GPIO_D: c_int = 3;
pub const AMLOGIC_GPIO_E: c_int = 4;
pub const AMLOGIC_GPIO_F: c_int = 5;
pub const AMLOGIC_GPIO_G: c_int = 6;
pub const AMLOGIC_GPIO_H: c_int = 7;
pub const AMLOGIC_GPIO_I: c_int = 8;
pub const AMLOGIC_GPIO_J: c_int = 9;
pub const AMLOGIC_GPIO_K: c_int = 10;
pub const AMLOGIC_GPIO_L: c_int = 11;
pub const AMLOGIC_GPIO_M: c_int = 12;
pub const AMLOGIC_GPIO_N: c_int = 13;
pub const AMLOGIC_GPIO_O: c_int = 14;
pub const AMLOGIC_GPIO_P: c_int = 15;
pub const AMLOGIC_GPIO_Q: c_int = 16;
pub const AMLOGIC_GPIO_R: c_int = 17;
pub const AMLOGIC_GPIO_S: c_int = 18;
pub const AMLOGIC_GPIO_T: c_int = 19;
pub const AMLOGIC_GPIO_U: c_int = 20;
pub const AMLOGIC_GPIO_V: c_int = 21;
pub const AMLOGIC_GPIO_W: c_int = 22;
pub const AMLOGIC_GPIO_X: c_int = 23;
pub const AMLOGIC_GPIO_Y: c_int = 24;
pub const AMLOGIC_GPIO_Z: c_int = 25;
// Special PIN bank
pub const AMLOGIC_GPIO_DV: c_int = 26;
pub const AMLOGIC_GPIO_AO: c_int = 27;
pub const AMLOGIC_GPIO_CC: c_int = 28;
pub const AMLOGIC_GPIO_TEST_N: c_int = 29;
pub const AMLOGIC_GPIO_ANALOG: c_int = 30;

