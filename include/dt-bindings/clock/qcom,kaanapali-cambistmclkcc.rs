//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/qcom,kaanapali-cambistmclkcc.h
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
// Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
//
// CAM_BIST_MCLK_CC clocks
pub const CAM_BIST_MCLK_CC_DEBUG_CLK: c_int = 0;
pub const CAM_BIST_MCLK_CC_DEBUG_DIV_CLK_SRC: c_int = 1;
pub const CAM_BIST_MCLK_CC_MCLK0_CLK: c_int = 2;
pub const CAM_BIST_MCLK_CC_MCLK0_CLK_SRC: c_int = 3;
pub const CAM_BIST_MCLK_CC_MCLK1_CLK: c_int = 4;
pub const CAM_BIST_MCLK_CC_MCLK1_CLK_SRC: c_int = 5;
pub const CAM_BIST_MCLK_CC_MCLK2_CLK: c_int = 6;
pub const CAM_BIST_MCLK_CC_MCLK2_CLK_SRC: c_int = 7;
pub const CAM_BIST_MCLK_CC_MCLK3_CLK: c_int = 8;
pub const CAM_BIST_MCLK_CC_MCLK3_CLK_SRC: c_int = 9;
pub const CAM_BIST_MCLK_CC_MCLK4_CLK: c_int = 10;
pub const CAM_BIST_MCLK_CC_MCLK4_CLK_SRC: c_int = 11;
pub const CAM_BIST_MCLK_CC_MCLK5_CLK: c_int = 12;
pub const CAM_BIST_MCLK_CC_MCLK5_CLK_SRC: c_int = 13;
pub const CAM_BIST_MCLK_CC_MCLK6_CLK: c_int = 14;
pub const CAM_BIST_MCLK_CC_MCLK6_CLK_SRC: c_int = 15;
pub const CAM_BIST_MCLK_CC_MCLK7_CLK: c_int = 16;
pub const CAM_BIST_MCLK_CC_MCLK7_CLK_SRC: c_int = 17;
pub const CAM_BIST_MCLK_CC_PLL0: c_int = 18;
pub const CAM_BIST_MCLK_CC_PLL_TEST_CLK: c_int = 19;
pub const CAM_BIST_MCLK_CC_PLL_TEST_DIV_CLK_SRC: c_int = 20;
pub const CAM_BIST_MCLK_CC_SLEEP_CLK: c_int = 21;
