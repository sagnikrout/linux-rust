//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/pinctrl/stm32-pinfunc.h
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


// SPDX-License-Identifier: (GPL-2.0+ OR BSD-3-Clause)
//
// Copyright (C) STMicroelectronics 2017 - All Rights Reserved
// Author: Torgue Alexandre <alexandre.torgue@st.com> for STMicroelectronics.
//
// define PIN modes
pub const GPIO: c_uint = 0x0;
pub const AF0: c_uint = 0x1;
pub const AF1: c_uint = 0x2;
pub const AF2: c_uint = 0x3;
pub const AF3: c_uint = 0x4;
pub const AF4: c_uint = 0x5;
pub const AF5: c_uint = 0x6;
pub const AF6: c_uint = 0x7;
pub const AF7: c_uint = 0x8;
pub const AF8: c_uint = 0x9;
pub const AF9: c_uint = 0xa;
pub const AF10: c_uint = 0xb;
pub const AF11: c_uint = 0xc;
pub const AF12: c_uint = 0xd;
pub const AF13: c_uint = 0xe;
pub const AF14: c_uint = 0xf;
pub const AF15: c_uint = 0x10;
pub const ANALOG: c_uint = 0x11;
pub const RSVD: c_uint = 0x12;
// define Pins number

// package information
pub const STM32MP_PKG_AA: c_uint = 0x1;
pub const STM32MP_PKG_AB: c_uint = 0x2;
pub const STM32MP_PKG_AC: c_uint = 0x4;
pub const STM32MP_PKG_AD: c_uint = 0x8;
pub const STM32MP_PKG_AI: c_uint = 0x100;
pub const STM32MP_PKG_AK: c_uint = 0x400;
pub const STM32MP_PKG_AL: c_uint = 0x800;
