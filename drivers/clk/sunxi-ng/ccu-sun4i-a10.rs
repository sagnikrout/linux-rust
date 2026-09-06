//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/clk/sunxi-ng/ccu-sun4i-a10.h
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
// Copyright 2017 Priit Laes
//
// Priit Laes <plaes@plaes.org>
//

// The HOSC is exported
pub const CLK_PLL_CORE: c_int = 2;
pub const CLK_PLL_AUDIO_BASE: c_int = 3;
pub const CLK_PLL_AUDIO: c_int = 4;
pub const CLK_PLL_AUDIO_2X: c_int = 5;
pub const CLK_PLL_AUDIO_4X: c_int = 6;
pub const CLK_PLL_AUDIO_8X: c_int = 7;
pub const CLK_PLL_VIDEO0: c_int = 8;
// The PLL_VIDEO0_2X clock is exported
pub const CLK_PLL_VE: c_int = 10;
pub const CLK_PLL_DDR_BASE: c_int = 11;
pub const CLK_PLL_DDR: c_int = 12;
pub const CLK_PLL_DDR_OTHER: c_int = 13;
pub const CLK_PLL_PERIPH_BASE: c_int = 14;
pub const CLK_PLL_PERIPH: c_int = 15;
pub const CLK_PLL_PERIPH_SATA: c_int = 16;
pub const CLK_PLL_VIDEO1: c_int = 17;
// The PLL_VIDEO1_2X clock is exported
pub const CLK_PLL_GPU: c_int = 19;
// The CPU clock is exported
pub const CLK_AXI: c_int = 21;
pub const CLK_AXI_DRAM: c_int = 22;
pub const CLK_AHB: c_int = 23;
pub const CLK_APB0: c_int = 24;
pub const CLK_APB1: c_int = 25;
// AHB gates are exported (23..68)
// APB0 gates are exported (69..78)
// APB1 gates are exported (79..95)
// IP module clocks are exported (96..128)
// DRAM gates are exported (129..142)
// Media (display engine clocks & etc) are exported (143..169)

