//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/clk/sunxi-ng/ccu-sun6i-a31.h
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

pub const CLK_PLL_CPU: c_int = 0;
pub const CLK_PLL_AUDIO_BASE: c_int = 1;
pub const CLK_PLL_AUDIO: c_int = 2;
pub const CLK_PLL_AUDIO_2X: c_int = 3;
pub const CLK_PLL_AUDIO_4X: c_int = 4;
pub const CLK_PLL_AUDIO_8X: c_int = 5;
pub const CLK_PLL_VIDEO0: c_int = 6;
// The PLL_VIDEO0_2X clock is exported
pub const CLK_PLL_VE: c_int = 8;
pub const CLK_PLL_DDR: c_int = 9;
// The PLL_PERIPH clock is exported
pub const CLK_PLL_PERIPH_2X: c_int = 11;
pub const CLK_PLL_VIDEO1: c_int = 12;
// The PLL_VIDEO1_2X clock is exported
pub const CLK_PLL_GPU: c_int = 14;
// The PLL_VIDEO1_2X clock is exported
pub const CLK_PLL9: c_int = 16;
pub const CLK_PLL10: c_int = 17;
// The CPUX clock is exported
pub const CLK_AXI: c_int = 19;
pub const CLK_AHB1: c_int = 20;
pub const CLK_APB1: c_int = 21;
pub const CLK_APB2: c_int = 22;
// All the bus gates are exported
// The first bunch of module clocks are exported
// EMAC clock is not implemented
pub const CLK_MDFS: c_int = 107;
pub const CLK_SDRAM0: c_int = 108;
pub const CLK_SDRAM1: c_int = 109;
// All the DRAM gates are exported
// Some more module clocks are exported
pub const CLK_MBUS0: c_int = 141;
pub const CLK_MBUS1: c_int = 142;
// Some more module clocks and external clock outputs are exported

