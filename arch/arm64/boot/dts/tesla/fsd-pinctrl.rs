//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/boot/dts/tesla/fsd-pinctrl.h
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


// SPDX-License-Identifier: GPL-2.0
//
// Tesla FSD DTS pinctrl constants
//
// Copyright (c) 2016 Samsung Electronics Co., Ltd.
// http://www.samsung.com
// Copyright (c) 2022 Linaro Ltd
// Author: Krzysztof Kozlowski <krzk@kernel.org>
//
pub const FSD_PIN_PULL_NONE: c_int = 0;
pub const FSD_PIN_PULL_DOWN: c_int = 1;
pub const FSD_PIN_PULL_UP: c_int = 3;
pub const FSD_PIN_DRV_LV1: c_int = 0;
pub const FSD_PIN_DRV_LV2: c_int = 1;
pub const FSD_PIN_DRV_LV4: c_int = 2;
pub const FSD_PIN_DRV_LV6: c_int = 3;
pub const FSD_PIN_FUNC_INPUT: c_int = 0;
pub const FSD_PIN_FUNC_OUTPUT: c_int = 1;
pub const FSD_PIN_FUNC_2: c_int = 2;
pub const FSD_PIN_FUNC_3: c_int = 3;
pub const FSD_PIN_FUNC_4: c_int = 4;
pub const FSD_PIN_FUNC_5: c_int = 5;
pub const FSD_PIN_FUNC_6: c_int = 6;
pub const FSD_PIN_FUNC_EINT: c_uint = 0xf;

