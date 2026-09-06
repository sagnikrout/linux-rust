//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/qcom,rpmh.h
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
// Copyright (c) 2018, 2020, The Linux Foundation. All rights reserved.
// RPMh controlled clocks
pub const RPMH_CXO_CLK: c_int = 0;
pub const RPMH_CXO_CLK_A: c_int = 1;
pub const RPMH_LN_BB_CLK2: c_int = 2;
pub const RPMH_LN_BB_CLK2_A: c_int = 3;
pub const RPMH_LN_BB_CLK3: c_int = 4;
pub const RPMH_LN_BB_CLK3_A: c_int = 5;
pub const RPMH_RF_CLK1: c_int = 6;
pub const RPMH_RF_CLK1_A: c_int = 7;
pub const RPMH_RF_CLK2: c_int = 8;
pub const RPMH_RF_CLK2_A: c_int = 9;
pub const RPMH_RF_CLK3: c_int = 10;
pub const RPMH_RF_CLK3_A: c_int = 11;
pub const RPMH_IPA_CLK: c_int = 12;
pub const RPMH_LN_BB_CLK1: c_int = 13;
pub const RPMH_LN_BB_CLK1_A: c_int = 14;
pub const RPMH_CE_CLK: c_int = 15;
pub const RPMH_QPIC_CLK: c_int = 16;
pub const RPMH_DIV_CLK1: c_int = 17;
pub const RPMH_DIV_CLK1_A: c_int = 18;
pub const RPMH_RF_CLK4: c_int = 19;
pub const RPMH_RF_CLK4_A: c_int = 20;
pub const RPMH_RF_CLK5: c_int = 21;
pub const RPMH_RF_CLK5_A: c_int = 22;
pub const RPMH_PKA_CLK: c_int = 23;
pub const RPMH_HWKM_CLK: c_int = 24;
pub const RPMH_QLINK_CLK: c_int = 25;
pub const RPMH_QLINK_CLK_A: c_int = 26;
pub const RPMH_LN_BB_CLK4: c_int = 27;
pub const RPMH_LN_BB_CLK4_A: c_int = 28;
