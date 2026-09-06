//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/clk/sunxi-ng/ccu-sun9i-a80.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright 2016 Chen-Yu Tsai
//
// Chen-Yu Tsai <wens@csie.org>
//

pub const CLK_PLL_C0CPUX: c_int = 0;
pub const CLK_PLL_C1CPUX: c_int = 1;
// pll-audio and pll-periph0 are exported to the PRCM block
pub const CLK_PLL_VE: c_int = 4;
pub const CLK_PLL_DDR: c_int = 5;
pub const CLK_PLL_VIDEO0: c_int = 6;
pub const CLK_PLL_VIDEO1: c_int = 7;
pub const CLK_PLL_GPU: c_int = 8;
pub const CLK_PLL_DE: c_int = 9;
pub const CLK_PLL_ISP: c_int = 10;
pub const CLK_PLL_PERIPH1: c_int = 11;
// The CPUX clocks are exported
pub const CLK_ATB0: c_int = 14;
pub const CLK_AXI0: c_int = 15;
pub const CLK_ATB1: c_int = 16;
pub const CLK_AXI1: c_int = 17;
pub const CLK_GTBUS: c_int = 18;
pub const CLK_AHB0: c_int = 19;
pub const CLK_AHB1: c_int = 20;
pub const CLK_AHB2: c_int = 21;
pub const CLK_APB0: c_int = 22;
pub const CLK_APB1: c_int = 23;
pub const CLK_CCI400: c_int = 24;
pub const CLK_ATS: c_int = 25;
pub const CLK_TRACE: c_int = 26;
// module clocks and bus gates exported

