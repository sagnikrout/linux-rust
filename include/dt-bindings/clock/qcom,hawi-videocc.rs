//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/qcom,hawi-videocc.h
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
// VIDEO_CC clocks
pub const VIDEO_CC_AHB_CLK: c_int = 0;
pub const VIDEO_CC_AHB_CLK_SRC: c_int = 1;
pub const VIDEO_CC_CX_AXI0_CLK: c_int = 2;
pub const VIDEO_CC_CX_DBGCH_XO_CLK: c_int = 3;
pub const VIDEO_CC_CX_XO_CLK: c_int = 4;
pub const VIDEO_CC_DBGCH_XO_CLK: c_int = 5;
pub const VIDEO_CC_MVS0_CLK: c_int = 6;
pub const VIDEO_CC_MVS0_CLK_SRC: c_int = 7;
pub const VIDEO_CC_MVS0_SHIFT_CLK: c_int = 8;
pub const VIDEO_CC_MVS0_VPP0_CLK: c_int = 9;
pub const VIDEO_CC_MVS0_VPP0_VPP1_GATING_CLK: c_int = 10;
pub const VIDEO_CC_MVS0_VPP1_CLK: c_int = 11;
pub const VIDEO_CC_MVS0A_CLK: c_int = 12;
pub const VIDEO_CC_MVS0A_CLK_SRC: c_int = 13;
pub const VIDEO_CC_MVS0B_CLK: c_int = 14;
pub const VIDEO_CC_MVS0B_CLK_SRC: c_int = 15;
pub const VIDEO_CC_MVS0C_CLK: c_int = 16;
pub const VIDEO_CC_MVS0C_CLK_SRC: c_int = 17;
pub const VIDEO_CC_MVS0C_CTL_FREERUN_CLK: c_int = 18;
pub const VIDEO_CC_MVS0C_DEBUG_CLK: c_int = 19;
pub const VIDEO_CC_MVS0C_FREERUN_CLK: c_int = 20;
pub const VIDEO_CC_MVS0C_SHIFT_CLK: c_int = 21;
pub const VIDEO_CC_PLL0: c_int = 22;
pub const VIDEO_CC_PLL0_OUT_EVEN: c_int = 23;
pub const VIDEO_CC_PLL1: c_int = 24;
pub const VIDEO_CC_PLL2: c_int = 25;
pub const VIDEO_CC_PLL3: c_int = 26;
pub const VIDEO_CC_SLEEP_CLK: c_int = 27;
pub const VIDEO_CC_XO_CLK: c_int = 28;
pub const VIDEO_CC_XO_CLK_SRC: c_int = 29;
// VIDEO_CC power domains
pub const VIDEO_CC_AXI0_CX_INT_GDSC: c_int = 0;
pub const VIDEO_CC_MM_INT_GDSC: c_int = 1;
pub const VIDEO_CC_MVS0_GDSC: c_int = 2;
pub const VIDEO_CC_MVS0_VPP0_GDSC: c_int = 3;
pub const VIDEO_CC_MVS0_VPP1_GDSC: c_int = 4;
pub const VIDEO_CC_MVS0A_GDSC: c_int = 5;
pub const VIDEO_CC_MVS0C_GDSC: c_int = 6;
// VIDEO_CC resets
pub const VIDEO_CC_AXI0_CX_INT_BCR: c_int = 0;
pub const VIDEO_CC_INTERFACE_BCR: c_int = 1;
pub const VIDEO_CC_MM_INT_BCR: c_int = 2;
pub const VIDEO_CC_MVS0_BCR: c_int = 3;
pub const VIDEO_CC_MVS0_VPP0_BCR: c_int = 4;
pub const VIDEO_CC_MVS0_VPP1_BCR: c_int = 5;
pub const VIDEO_CC_MVS0A_BCR: c_int = 6;
pub const VIDEO_CC_MVS0C_CLK_ARES: c_int = 7;
pub const VIDEO_CC_MVS0C_BCR: c_int = 8;
pub const VIDEO_CC_MVS0C_CTL_FREERUN_CLK_ARES: c_int = 9;
pub const VIDEO_CC_MVS0C_FREERUN_CLK_ARES: c_int = 10;
pub const VIDEO_CC_XO_CLK_ARES: c_int = 11;
