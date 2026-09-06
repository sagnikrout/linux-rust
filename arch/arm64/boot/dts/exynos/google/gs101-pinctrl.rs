//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/boot/dts/exynos/google/gs101-pinctrl.h
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
// Pinctrl binding constants for GS101
//
// Copyright 2020-2023 Google LLC
//
pub const GS101_PIN_PULL_NONE: c_int = 0;
pub const GS101_PIN_PULL_DOWN: c_int = 1;
pub const GS101_PIN_PULL_UP: c_int = 3;
// Pin function in power down mode
pub const GS101_PIN_PDN_OUT0: c_int = 0;
pub const GS101_PIN_PDN_OUT1: c_int = 1;
pub const GS101_PIN_PDN_INPUT: c_int = 2;
pub const GS101_PIN_PDN_PREV: c_int = 3;
// GS101 drive strengths
pub const GS101_PIN_DRV_2_5_MA: c_int = 0;
pub const GS101_PIN_DRV_5_MA: c_int = 1;
pub const GS101_PIN_DRV_7_5_MA: c_int = 2;
pub const GS101_PIN_DRV_10_MA: c_int = 3;
pub const GS101_PIN_FUNC_INPUT: c_int = 0;
pub const GS101_PIN_FUNC_OUTPUT: c_int = 1;
pub const GS101_PIN_FUNC_2: c_int = 2;
pub const GS101_PIN_FUNC_3: c_int = 3;
pub const GS101_PIN_FUNC_EINT: c_uint = 0xf;
