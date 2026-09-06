//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/clk/sunxi-ng/ccu-sun8i-a83t.h
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
pub const CLK_PLL_AUDIO: c_int = 2;
pub const CLK_PLL_VIDEO0: c_int = 3;
pub const CLK_PLL_VE: c_int = 4;
pub const CLK_PLL_DDR: c_int = 5;
// pll-periph is exported to the PRCM block
pub const CLK_PLL_GPU: c_int = 7;
pub const CLK_PLL_HSIC: c_int = 8;
// pll-de is exported for the display engine
pub const CLK_PLL_VIDEO1: c_int = 10;
// The CPUX clocks are exported
pub const CLK_AXI0: c_int = 13;
pub const CLK_AXI1: c_int = 14;
pub const CLK_AHB1: c_int = 15;
pub const CLK_AHB2: c_int = 16;
pub const CLK_APB1: c_int = 17;
pub const CLK_APB2: c_int = 18;
// bus gates exported
pub const CLK_CCI400: c_int = 58;
// module and usb clocks exported
pub const CLK_DRAM: c_int = 82;
// dram gates and more module clocks exported
pub const CLK_MBUS: c_int = 95;
// more module clocks exported

