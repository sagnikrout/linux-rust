//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/clk/sunxi-ng/ccu-sun8i-v3s.h
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
// Copyright (c) 2016 Icenowy Zheng <icenowy@aosc.xyz>
//
// Based on ccu-sun8i-h3.h, which is:
// Copyright (c) 2016 Maxime Ripard <maxime.ripard@free-electrons.com>
//

pub const CLK_PLL_CPU: c_int = 0;
pub const CLK_PLL_AUDIO_BASE: c_int = 1;
pub const CLK_PLL_AUDIO: c_int = 2;
pub const CLK_PLL_AUDIO_2X: c_int = 3;
pub const CLK_PLL_AUDIO_4X: c_int = 4;
pub const CLK_PLL_AUDIO_8X: c_int = 5;
pub const CLK_PLL_VIDEO: c_int = 6;
pub const CLK_PLL_VE: c_int = 7;
pub const CLK_PLL_DDR0: c_int = 8;
pub const CLK_PLL_PERIPH0: c_int = 9;
pub const CLK_PLL_PERIPH0_2X: c_int = 10;
pub const CLK_PLL_ISP: c_int = 11;
pub const CLK_PLL_PERIPH1: c_int = 12;
// Reserve one number for not implemented and not used PLL_DDR1
// The CPU clock is exported
pub const CLK_AXI: c_int = 15;
pub const CLK_AHB1: c_int = 16;
pub const CLK_APB1: c_int = 17;
pub const CLK_APB2: c_int = 18;
pub const CLK_AHB2: c_int = 19;
// All the bus gates are exported
// The first bunch of module clocks are exported
pub const CLK_DRAM: c_int = 58;
// All the DRAM gates are exported
// Some more module clocks are exported
pub const CLK_MBUS: c_int = 72;
// And the GPU module clock is exported
pub const CLK_PLL_DDR1: c_int = 74;
