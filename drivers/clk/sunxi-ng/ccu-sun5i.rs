//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/clk/sunxi-ng/ccu-sun5i.h
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
// Copyright 2016 Maxime Ripard
//
// Maxime Ripard <maxime.ripard@free-electrons.com>
//

// The HOSC is exported
pub const CLK_PLL_CORE: c_int = 2;
pub const CLK_PLL_AUDIO_BASE: c_int = 3;
pub const CLK_PLL_AUDIO: c_int = 4;
pub const CLK_PLL_AUDIO_2X: c_int = 5;
pub const CLK_PLL_AUDIO_4X: c_int = 6;
pub const CLK_PLL_AUDIO_8X: c_int = 7;
pub const CLK_PLL_VIDEO0: c_int = 8;
// The PLL_VIDEO0_2X is exported for HDMI
pub const CLK_PLL_VE: c_int = 10;
pub const CLK_PLL_DDR_BASE: c_int = 11;
pub const CLK_PLL_DDR: c_int = 12;
pub const CLK_PLL_DDR_OTHER: c_int = 13;
pub const CLK_PLL_PERIPH: c_int = 14;
pub const CLK_PLL_VIDEO1: c_int = 15;
// The PLL_VIDEO1_2X is exported for HDMI
// The CPU clock is exported
pub const CLK_AXI: c_int = 18;
pub const CLK_AHB: c_int = 19;
pub const CLK_APB0: c_int = 20;
pub const CLK_APB1: c_int = 21;
pub const CLK_DRAM_AXI: c_int = 22;
// AHB gates are exported
// APB0 gates are exported
// APB1 gates are exported
// Modules clocks are exported
// USB clocks are exported
// GPS clock is exported
// DRAM gates are exported
// More display modules clocks are exported
pub const CLK_TCON_CH1_SCLK: c_int = 91;
// The rest of the module clocks are exported

