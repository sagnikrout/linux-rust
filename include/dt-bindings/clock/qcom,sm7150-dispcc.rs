//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/qcom,sm7150-dispcc.h
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
// Copyright (c) 2024, David Wronek <david@mainlining.org>
//
// DISPCC clock registers
pub const DISPCC_PLL0: c_int = 0;
pub const DISPCC_MDSS_AHB_CLK: c_int = 1;
pub const DISPCC_MDSS_AHB_CLK_SRC: c_int = 2;
pub const DISPCC_MDSS_BYTE0_CLK: c_int = 3;
pub const DISPCC_MDSS_BYTE0_CLK_SRC: c_int = 4;
pub const DISPCC_MDSS_BYTE0_DIV_CLK_SRC: c_int = 5;
pub const DISPCC_MDSS_BYTE0_INTF_CLK: c_int = 6;
pub const DISPCC_MDSS_BYTE1_CLK: c_int = 7;
pub const DISPCC_MDSS_BYTE1_CLK_SRC: c_int = 8;
pub const DISPCC_MDSS_BYTE1_DIV_CLK_SRC: c_int = 9;
pub const DISPCC_MDSS_BYTE1_INTF_CLK: c_int = 10;
pub const DISPCC_MDSS_DP_AUX_CLK: c_int = 11;
pub const DISPCC_MDSS_DP_AUX_CLK_SRC: c_int = 12;
pub const DISPCC_MDSS_DP_CRYPTO_CLK: c_int = 13;
pub const DISPCC_MDSS_DP_CRYPTO_CLK_SRC: c_int = 14;
pub const DISPCC_MDSS_DP_LINK_CLK: c_int = 15;
pub const DISPCC_MDSS_DP_LINK_CLK_SRC: c_int = 16;
pub const DISPCC_MDSS_DP_LINK_INTF_CLK: c_int = 17;
pub const DISPCC_MDSS_DP_PIXEL1_CLK: c_int = 18;
pub const DISPCC_MDSS_DP_PIXEL1_CLK_SRC: c_int = 19;
pub const DISPCC_MDSS_DP_PIXEL_CLK: c_int = 20;
pub const DISPCC_MDSS_DP_PIXEL_CLK_SRC: c_int = 21;
pub const DISPCC_MDSS_ESC0_CLK: c_int = 22;
pub const DISPCC_MDSS_ESC0_CLK_SRC: c_int = 23;
pub const DISPCC_MDSS_ESC1_CLK: c_int = 24;
pub const DISPCC_MDSS_ESC1_CLK_SRC: c_int = 25;
pub const DISPCC_MDSS_MDP_CLK: c_int = 26;
pub const DISPCC_MDSS_MDP_CLK_SRC: c_int = 27;
pub const DISPCC_MDSS_MDP_LUT_CLK: c_int = 28;
pub const DISPCC_MDSS_NON_GDSC_AHB_CLK: c_int = 29;
pub const DISPCC_MDSS_PCLK0_CLK: c_int = 30;
pub const DISPCC_MDSS_PCLK0_CLK_SRC: c_int = 31;
pub const DISPCC_MDSS_PCLK1_CLK: c_int = 32;
pub const DISPCC_MDSS_PCLK1_CLK_SRC: c_int = 33;
pub const DISPCC_MDSS_ROT_CLK: c_int = 34;
pub const DISPCC_MDSS_ROT_CLK_SRC: c_int = 35;
pub const DISPCC_MDSS_RSCC_AHB_CLK: c_int = 36;
pub const DISPCC_MDSS_RSCC_VSYNC_CLK: c_int = 37;
pub const DISPCC_MDSS_VSYNC_CLK: c_int = 38;
pub const DISPCC_MDSS_VSYNC_CLK_SRC: c_int = 39;
pub const DISPCC_XO_CLK_SRC: c_int = 40;
pub const DISPCC_SLEEP_CLK: c_int = 41;
pub const DISPCC_SLEEP_CLK_SRC: c_int = 42;
// DISPCC resets
pub const DISPCC_MDSS_CORE_BCR: c_int = 0;
// DISPCC GDSCR
pub const MDSS_GDSC: c_int = 0;
