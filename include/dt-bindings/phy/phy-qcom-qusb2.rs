//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/phy/phy-qcom-qusb2.h
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
// Copyright (c) 2018, The Linux Foundation. All rights reserved.
//
// PHY HSTX TRIM bit values (24mA to 15mA)
pub const QUSB2_V2_HSTX_TRIM_24_0_MA: c_uint = 0x0;
pub const QUSB2_V2_HSTX_TRIM_23_4_MA: c_uint = 0x1;
pub const QUSB2_V2_HSTX_TRIM_22_8_MA: c_uint = 0x2;
pub const QUSB2_V2_HSTX_TRIM_22_2_MA: c_uint = 0x3;
pub const QUSB2_V2_HSTX_TRIM_21_6_MA: c_uint = 0x4;
pub const QUSB2_V2_HSTX_TRIM_21_0_MA: c_uint = 0x5;
pub const QUSB2_V2_HSTX_TRIM_20_4_MA: c_uint = 0x6;
pub const QUSB2_V2_HSTX_TRIM_19_8_MA: c_uint = 0x7;
pub const QUSB2_V2_HSTX_TRIM_19_2_MA: c_uint = 0x8;
pub const QUSB2_V2_HSTX_TRIM_18_6_MA: c_uint = 0x9;
pub const QUSB2_V2_HSTX_TRIM_18_0_MA: c_uint = 0xa;
pub const QUSB2_V2_HSTX_TRIM_17_4_MA: c_uint = 0xb;
pub const QUSB2_V2_HSTX_TRIM_16_8_MA: c_uint = 0xc;
pub const QUSB2_V2_HSTX_TRIM_16_2_MA: c_uint = 0xd;
pub const QUSB2_V2_HSTX_TRIM_15_6_MA: c_uint = 0xe;
pub const QUSB2_V2_HSTX_TRIM_15_0_MA: c_uint = 0xf;
// PHY PREEMPHASIS bit values
pub const QUSB2_V2_PREEMPHASIS_NONE: c_int = 0;
pub const QUSB2_V2_PREEMPHASIS_5_PERCENT: c_int = 1;
pub const QUSB2_V2_PREEMPHASIS_10_PERCENT: c_int = 2;
pub const QUSB2_V2_PREEMPHASIS_15_PERCENT: c_int = 3;
// PHY PREEMPHASIS-WIDTH bit values
pub const QUSB2_V2_PREEMPHASIS_WIDTH_FULL_BIT: c_int = 0;
pub const QUSB2_V2_PREEMPHASIS_WIDTH_HALF_BIT: c_int = 1;
