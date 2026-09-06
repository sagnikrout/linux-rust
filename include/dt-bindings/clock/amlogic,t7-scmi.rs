//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/amlogic,t7-scmi.h
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
pub const CLKID_DDR_PLL_OSC: c_int = 0;
pub const CLKID_AUD_PLL_OSC: c_int = 1;
pub const CLKID_TOP_PLL_OSC: c_int = 2;
pub const CLKID_TCON_PLL_OSC: c_int = 3;
pub const CLKID_USB_PLL0_OSC: c_int = 4;
pub const CLKID_USB_PLL1_OSC: c_int = 5;
pub const CLKID_MCLK_PLL_OSC: c_int = 6;
pub const CLKID_PCIE_OSC: c_int = 7;
pub const CLKID_ETH_PLL_OSC: c_int = 8;
pub const CLKID_PCIE_REFCLK_PLL_OSC: c_int = 9;
pub const CLKID_EARC_OSC: c_int = 10;
pub const CLKID_SYS1_PLL_OSC: c_int = 11;
pub const CLKID_HDMI_PLL_OSC: c_int = 12;
pub const CLKID_SYS_CLK: c_int = 13;
pub const CLKID_AXI_CLK: c_int = 14;
pub const CLKID_FIXED_PLL_DCO: c_int = 15;
pub const CLKID_FIXED_PLL: c_int = 16;
pub const CLKID_FCLK_DIV2_DIV: c_int = 17;
pub const CLKID_FCLK_DIV2: c_int = 18;
pub const CLKID_FCLK_DIV2P5_DIV: c_int = 19;
pub const CLKID_FCLK_DIV2P5: c_int = 20;
pub const CLKID_FCLK_DIV3_DIV: c_int = 21;
pub const CLKID_FCLK_DIV3: c_int = 22;
pub const CLKID_FCLK_DIV4_DIV: c_int = 23;
pub const CLKID_FCLK_DIV4: c_int = 24;
pub const CLKID_FCLK_DIV5_DIV: c_int = 25;
pub const CLKID_FCLK_DIV5: c_int = 26;
pub const CLKID_FCLK_DIV7_DIV: c_int = 27;
pub const CLKID_FCLK_DIV7: c_int = 28;
pub const CLKID_FCLK_50M_DIV: c_int = 29;
pub const CLKID_FCLK_50M: c_int = 30;
pub const CLKID_CPU_CLK: c_int = 31;
pub const CLKID_A73_CLK: c_int = 32;
pub const CLKID_CPU_CLK_DIV16_DIV: c_int = 33;
pub const CLKID_CPU_CLK_DIV16: c_int = 34;
pub const CLKID_A73_CLK_DIV16_DIV: c_int = 35;
pub const CLKID_A73_CLK_DIV16: c_int = 36;
