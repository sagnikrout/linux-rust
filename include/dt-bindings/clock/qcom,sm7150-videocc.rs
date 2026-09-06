//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/qcom,sm7150-videocc.h
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
// Copyright (c) 2018, The Linux Foundation. All rights reserved.
// Copyright (c) 2024, Danila Tikhonov <danila@jiaxyga.com>
//
pub const VIDEOCC_PLL0: c_int = 0;
pub const VIDEOCC_IRIS_AHB_CLK: c_int = 1;
pub const VIDEOCC_IRIS_CLK_SRC: c_int = 2;
pub const VIDEOCC_MVS0_AXI_CLK: c_int = 3;
pub const VIDEOCC_MVS0_CORE_CLK: c_int = 4;
pub const VIDEOCC_MVS1_AXI_CLK: c_int = 5;
pub const VIDEOCC_MVS1_CORE_CLK: c_int = 6;
pub const VIDEOCC_MVSC_CORE_CLK: c_int = 7;
pub const VIDEOCC_MVSC_CTL_AXI_CLK: c_int = 8;
pub const VIDEOCC_VENUS_AHB_CLK: c_int = 9;
pub const VIDEOCC_XO_CLK: c_int = 10;
pub const VIDEOCC_XO_CLK_SRC: c_int = 11;
// VIDEOCC GDSCRs
pub const VENUS_GDSC: c_int = 0;
pub const VCODEC0_GDSC: c_int = 1;
pub const VCODEC1_GDSC: c_int = 2;
