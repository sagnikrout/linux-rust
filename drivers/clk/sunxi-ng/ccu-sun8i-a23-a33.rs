//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/clk/sunxi-ng/ccu-sun8i-a23-a33.h
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

pub const CLK_PLL_CPUX: c_int = 0;
pub const CLK_PLL_AUDIO_BASE: c_int = 1;
pub const CLK_PLL_AUDIO: c_int = 2;
pub const CLK_PLL_AUDIO_2X: c_int = 3;
pub const CLK_PLL_AUDIO_4X: c_int = 4;
pub const CLK_PLL_AUDIO_8X: c_int = 5;
pub const CLK_PLL_VIDEO: c_int = 6;
pub const CLK_PLL_VIDEO_2X: c_int = 7;
pub const CLK_PLL_VE: c_int = 8;
pub const CLK_PLL_DDR0: c_int = 9;
pub const CLK_PLL_PERIPH: c_int = 10;
pub const CLK_PLL_PERIPH_2X: c_int = 11;
pub const CLK_PLL_GPU: c_int = 12;
// The PLL MIPI clock is exported
pub const CLK_PLL_HSIC: c_int = 14;
pub const CLK_PLL_DE: c_int = 15;
pub const CLK_PLL_DDR1: c_int = 16;
pub const CLK_PLL_DDR: c_int = 17;
// The CPUX clock is exported
pub const CLK_AXI: c_int = 19;
pub const CLK_AHB1: c_int = 20;
pub const CLK_APB1: c_int = 21;
pub const CLK_APB2: c_int = 22;
// All the bus gates are exported
// The first part of the mod clocks is exported
pub const CLK_DRAM: c_int = 79;
// Some more module clocks are exported
pub const CLK_MBUS: c_int = 95;
// And the last module clocks are exported

