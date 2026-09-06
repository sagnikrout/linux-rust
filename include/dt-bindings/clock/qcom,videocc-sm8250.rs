//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/qcom,videocc-sm8250.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (c) 2018-2020, The Linux Foundation. All rights reserved.
//
// VIDEO_CC clocks
pub const VIDEO_CC_MVS0_CLK_SRC: c_int = 0;
pub const VIDEO_CC_MVS0C_CLK: c_int = 1;
pub const VIDEO_CC_MVS0C_DIV2_DIV_CLK_SRC: c_int = 2;
pub const VIDEO_CC_MVS1_CLK_SRC: c_int = 3;
pub const VIDEO_CC_MVS1_DIV2_CLK: c_int = 4;
pub const VIDEO_CC_MVS1C_CLK: c_int = 5;
pub const VIDEO_CC_MVS1C_DIV2_DIV_CLK_SRC: c_int = 6;
pub const VIDEO_CC_PLL0: c_int = 7;
pub const VIDEO_CC_PLL1: c_int = 8;
pub const VIDEO_CC_MVS0_DIV_CLK_SRC: c_int = 9;
pub const VIDEO_CC_MVS0_CLK: c_int = 10;
// VIDEO_CC resets
pub const VIDEO_CC_CVP_INTERFACE_BCR: c_int = 0;
pub const VIDEO_CC_CVP_MVS0_BCR: c_int = 1;
pub const VIDEO_CC_MVS0C_CLK_ARES: c_int = 2;
pub const VIDEO_CC_CVP_MVS0C_BCR: c_int = 3;
pub const VIDEO_CC_CVP_MVS1_BCR: c_int = 4;
pub const VIDEO_CC_MVS1C_CLK_ARES: c_int = 5;
pub const VIDEO_CC_CVP_MVS1C_BCR: c_int = 6;
pub const MVS0C_GDSC: c_int = 0;
pub const MVS1C_GDSC: c_int = 1;
pub const MVS0_GDSC: c_int = 2;
pub const MVS1_GDSC: c_int = 3;
