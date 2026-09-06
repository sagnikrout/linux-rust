//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/amlogic,c3-scmi-clkc.h
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
// Copyright (c) 2023 Amlogic, Inc. All rights reserved.
// Author: Chuan Liu <chuan.liu@amlogic.com>
//
pub const CLKID_DDR_PLL_OSC: c_int = 0;
pub const CLKID_DDR_PHY: c_int = 1;
pub const CLKID_TOP_PLL_OSC: c_int = 2;
pub const CLKID_USB_PLL_OSC: c_int = 3;
pub const CLKID_MIPIISP_VOUT: c_int = 4;
pub const CLKID_MCLK_PLL_OSC: c_int = 5;
pub const CLKID_USB_CTRL: c_int = 6;
pub const CLKID_ETH_PLL_OSC: c_int = 7;
pub const CLKID_OSC: c_int = 8;
pub const CLKID_SYS_CLK: c_int = 9;
pub const CLKID_AXI_CLK: c_int = 10;
pub const CLKID_CPU_CLK: c_int = 11;
pub const CLKID_FIXED_PLL_OSC: c_int = 12;
pub const CLKID_GP1_PLL_OSC: c_int = 13;
pub const CLKID_SYS_PLL_DIV16: c_int = 14;
pub const CLKID_CPU_CLK_DIV16: c_int = 15;
