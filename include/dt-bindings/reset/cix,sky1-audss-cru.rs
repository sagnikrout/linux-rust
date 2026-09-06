//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/reset/cix,sky1-audss-cru.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
//
// Copyright 2026 Cix Technology Group Co., Ltd.
//
pub const AUDSS_I2S0_SW_RST: c_int = 0;
pub const AUDSS_I2S1_SW_RST: c_int = 1;
pub const AUDSS_I2S2_SW_RST: c_int = 2;
pub const AUDSS_I2S3_SW_RST: c_int = 3;
pub const AUDSS_I2S4_SW_RST: c_int = 4;
pub const AUDSS_I2S5_SW_RST: c_int = 5;
pub const AUDSS_I2S6_SW_RST: c_int = 6;
pub const AUDSS_I2S7_SW_RST: c_int = 7;
pub const AUDSS_I2S8_SW_RST: c_int = 8;
pub const AUDSS_I2S9_SW_RST: c_int = 9;
pub const AUDSS_WDT_SW_RST: c_int = 10;
pub const AUDSS_TIMER_SW_RST: c_int = 11;
pub const AUDSS_MB0_SW_RST: c_int = 12;
pub const AUDSS_MB1_SW_RST: c_int = 13;
pub const AUDSS_HDA_SW_RST: c_int = 14;
pub const AUDSS_DMAC_SW_RST: c_int = 15;
