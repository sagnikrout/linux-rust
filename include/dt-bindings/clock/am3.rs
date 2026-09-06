//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/am3.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright 2017 Texas Instruments, Inc.
//
pub const AM3_CLKCTRL_OFFSET: c_uint = 0x0;

// l4ls clocks
pub const AM3_L4LS_CLKCTRL_OFFSET: c_uint = 0x38;

// l3s clocks
pub const AM3_L3S_CLKCTRL_OFFSET: c_uint = 0x1c;

// l3 clocks
pub const AM3_L3_CLKCTRL_OFFSET: c_uint = 0x24;

// l4hs clocks
pub const AM3_L4HS_CLKCTRL_OFFSET: c_uint = 0x120;

// pruss_ocp clocks
pub const AM3_PRUSS_OCP_CLKCTRL_OFFSET: c_uint = 0xe8;

// cpsw_125mhz clocks

// lcdc clocks
pub const AM3_LCDC_CLKCTRL_OFFSET: c_uint = 0x18;

// clk_24mhz clocks
pub const AM3_CLK_24MHZ_CLKCTRL_OFFSET: c_uint = 0x14c;

// l4_wkup clocks

// l3_aon clocks
pub const AM3_L3_AON_CLKCTRL_OFFSET: c_uint = 0x14;

// l4_wkup_aon clocks
pub const AM3_L4_WKUP_AON_CLKCTRL_OFFSET: c_uint = 0xb0;

// mpu clocks

// l4_rtc clocks

// gfx_l3 clocks

// l4_cefuse clocks

