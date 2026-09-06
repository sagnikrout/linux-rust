//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/cix,sky1-audss-cru.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
//
// Copyright 2026 Cix Technology Group Co., Ltd.
//
pub const CLK_AUD_CLK4_DIV2: c_int = 0;
pub const CLK_AUD_CLK4_DIV4: c_int = 1;
pub const CLK_AUD_CLK5_DIV2: c_int = 2;
pub const CLK_DSP_CLK: c_int = 3;
pub const CLK_DSP_BCLK: c_int = 4;
pub const CLK_DSP_PBCLK: c_int = 5;
pub const CLK_SRAM_AXI: c_int = 6;
pub const CLK_HDA_SYS: c_int = 7;
pub const CLK_HDA_HDA: c_int = 8;
pub const CLK_DMAC_AXI: c_int = 9;
pub const CLK_WDG_APB: c_int = 10;
pub const CLK_WDG_WDG: c_int = 11;
pub const CLK_TIMER_APB: c_int = 12;
pub const CLK_TIMER_TIMER: c_int = 13;

pub const CLK_I2S0_APB: c_int = 16;
pub const CLK_I2S1_APB: c_int = 17;
pub const CLK_I2S2_APB: c_int = 18;
pub const CLK_I2S3_APB: c_int = 19;
pub const CLK_I2S4_APB: c_int = 20;
pub const CLK_I2S5_APB: c_int = 21;
pub const CLK_I2S6_APB: c_int = 22;
pub const CLK_I2S7_APB: c_int = 23;
pub const CLK_I2S8_APB: c_int = 24;
pub const CLK_I2S9_APB: c_int = 25;
pub const CLK_I2S0: c_int = 26;
pub const CLK_I2S1: c_int = 27;
pub const CLK_I2S2: c_int = 28;
pub const CLK_I2S3: c_int = 29;
pub const CLK_I2S4: c_int = 30;
pub const CLK_I2S5: c_int = 31;
pub const CLK_I2S6: c_int = 32;
pub const CLK_I2S7: c_int = 33;
pub const CLK_I2S8: c_int = 34;
pub const CLK_I2S9: c_int = 35;
pub const CLK_MCLK0: c_int = 36;
pub const CLK_MCLK1: c_int = 37;
pub const CLK_MCLK2: c_int = 38;
pub const CLK_MCLK3: c_int = 39;
pub const CLK_MCLK4: c_int = 40;
