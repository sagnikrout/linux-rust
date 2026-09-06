//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/clk/sunxi-ng/ccu-sun50i-h6.h
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


// SPDX-License-Identifier: GPL-2.0
//
// Copyright 2016 Icenowy Zheng <icenowy@aosc.io>
//

pub const CLK_OSC12M: c_int = 0;
pub const CLK_PLL_CPUX: c_int = 1;
pub const CLK_PLL_DDR0: c_int = 2;
// PLL_PERIPH0 exported for PRCM
pub const CLK_PLL_PERIPH0_2X: c_int = 4;
pub const CLK_PLL_PERIPH0_4X: c_int = 5;
pub const CLK_PLL_PERIPH1: c_int = 6;
pub const CLK_PLL_PERIPH1_2X: c_int = 7;
pub const CLK_PLL_PERIPH1_4X: c_int = 8;
pub const CLK_PLL_GPU: c_int = 9;
pub const CLK_PLL_VIDEO0: c_int = 10;
pub const CLK_PLL_VIDEO0_4X: c_int = 11;
pub const CLK_PLL_VIDEO1: c_int = 12;
pub const CLK_PLL_VIDEO1_4X: c_int = 13;
pub const CLK_PLL_VE: c_int = 14;
pub const CLK_PLL_DE: c_int = 15;
pub const CLK_PLL_HSIC: c_int = 16;
pub const CLK_PLL_AUDIO_BASE: c_int = 17;
pub const CLK_PLL_AUDIO: c_int = 18;
pub const CLK_PLL_AUDIO_2X: c_int = 19;
pub const CLK_PLL_AUDIO_4X: c_int = 20;
// CPUX clock exported for DVFS
pub const CLK_AXI: c_int = 22;
pub const CLK_CPUX_APB: c_int = 23;
pub const CLK_PSI_AHB1_AHB2: c_int = 24;
pub const CLK_AHB3: c_int = 25;
// APB1 clock exported for PIO
pub const CLK_APB2: c_int = 27;
pub const CLK_MBUS: c_int = 28;
// All module clocks and bus gates are exported except DRAM
pub const CLK_DRAM: c_int = 52;
pub const CLK_BUS_DRAM: c_int = 60;

