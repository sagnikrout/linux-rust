//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/qcom,ipq5332-cmn-pll.h
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
// CMN PLL core clock.
pub const IPQ5332_CMN_PLL_CLK: c_int = 0;
// The output clocks from CMN PLL of IPQ5332.
pub const IPQ5332_XO_24MHZ_CLK: c_int = 1;
pub const IPQ5332_SLEEP_32KHZ_CLK: c_int = 2;
pub const IPQ5332_PCS_31P25MHZ_CLK: c_int = 3;
pub const IPQ5332_NSS_300MHZ_CLK: c_int = 4;
pub const IPQ5332_PPE_200MHZ_CLK: c_int = 5;
pub const IPQ5332_ETH_50MHZ_CLK: c_int = 6;
