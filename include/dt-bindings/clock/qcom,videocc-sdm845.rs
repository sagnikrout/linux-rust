//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/qcom,videocc-sdm845.h
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
// VIDEO_CC clock registers
pub const VIDEO_CC_APB_CLK: c_int = 0;
pub const VIDEO_CC_AT_CLK: c_int = 1;
pub const VIDEO_CC_QDSS_TRIG_CLK: c_int = 2;
pub const VIDEO_CC_QDSS_TSCTR_DIV8_CLK: c_int = 3;
pub const VIDEO_CC_VCODEC0_AXI_CLK: c_int = 4;
pub const VIDEO_CC_VCODEC0_CORE_CLK: c_int = 5;
pub const VIDEO_CC_VCODEC1_AXI_CLK: c_int = 6;
pub const VIDEO_CC_VCODEC1_CORE_CLK: c_int = 7;
pub const VIDEO_CC_VENUS_AHB_CLK: c_int = 8;
pub const VIDEO_CC_VENUS_CLK_SRC: c_int = 9;
pub const VIDEO_CC_VENUS_CTL_AXI_CLK: c_int = 10;
pub const VIDEO_CC_VENUS_CTL_CORE_CLK: c_int = 11;
pub const VIDEO_PLL0: c_int = 12;
// VIDEO_CC Resets
pub const VIDEO_CC_VENUS_BCR: c_int = 0;
pub const VIDEO_CC_VCODEC0_BCR: c_int = 1;
pub const VIDEO_CC_VCODEC1_BCR: c_int = 2;
pub const VIDEO_CC_INTERFACE_BCR: c_int = 3;
// VIDEO_CC GDSCRs
pub const VENUS_GDSC: c_int = 0;
pub const VCODEC0_GDSC: c_int = 1;
pub const VCODEC1_GDSC: c_int = 2;
