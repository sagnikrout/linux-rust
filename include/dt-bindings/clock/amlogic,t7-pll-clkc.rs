//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/amlogic,t7-pll-clkc.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR MIT)
//
// Copyright (C) 2024-2025 Amlogic, Inc. All rights reserved
//
// GP0
pub const CLKID_GP0_PLL_DCO: c_int = 0;
pub const CLKID_GP0_PLL: c_int = 1;
// GP1
pub const CLKID_GP1_PLL_DCO: c_int = 0;
pub const CLKID_GP1_PLL: c_int = 1;
// HIFI
pub const CLKID_HIFI_PLL_DCO: c_int = 0;
pub const CLKID_HIFI_PLL: c_int = 1;
// PCIE
pub const CLKID_PCIE_PLL_DCO: c_int = 0;
pub const CLKID_PCIE_PLL_DCO_DIV2: c_int = 1;
pub const CLKID_PCIE_PLL_OD: c_int = 2;
pub const CLKID_PCIE_PLL: c_int = 3;
// MPLL
pub const CLKID_MPLL_PREDIV: c_int = 0;
pub const CLKID_MPLL0_DIV: c_int = 1;
pub const CLKID_MPLL0: c_int = 2;
pub const CLKID_MPLL1_DIV: c_int = 3;
pub const CLKID_MPLL1: c_int = 4;
pub const CLKID_MPLL2_DIV: c_int = 5;
pub const CLKID_MPLL2: c_int = 6;
pub const CLKID_MPLL3_DIV: c_int = 7;
pub const CLKID_MPLL3: c_int = 8;
// HDMI
pub const CLKID_HDMI_PLL_DCO: c_int = 0;
pub const CLKID_HDMI_PLL_OD: c_int = 1;
pub const CLKID_HDMI_PLL: c_int = 2;
// MCLK
pub const CLKID_MCLK_PLL_DCO: c_int = 0;
pub const CLKID_MCLK_PRE: c_int = 1;
pub const CLKID_MCLK_PLL: c_int = 2;
pub const CLKID_MCLK_0_SEL: c_int = 3;
pub const CLKID_MCLK_0_DIV2: c_int = 4;
pub const CLKID_MCLK_0_PRE: c_int = 5;
pub const CLKID_MCLK_0: c_int = 6;
pub const CLKID_MCLK_1_SEL: c_int = 7;
pub const CLKID_MCLK_1_DIV2: c_int = 8;
pub const CLKID_MCLK_1_PRE: c_int = 9;
pub const CLKID_MCLK_1: c_int = 10;
