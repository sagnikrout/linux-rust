//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/am4.h
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
pub const AM4_CLKCTRL_OFFSET: c_uint = 0x20;

// l3s_tsc clocks
pub const AM4_L3S_TSC_CLKCTRL_OFFSET: c_uint = 0x120;

// l4_wkup_aon clocks
pub const AM4_L4_WKUP_AON_CLKCTRL_OFFSET: c_uint = 0x228;

// l4_wkup clocks
pub const AM4_L4_WKUP_CLKCTRL_OFFSET: c_uint = 0x220;

// mpu clocks

// gfx_l3 clocks

// l4_rtc clocks

// l3 clocks

// l3s clocks
pub const AM4_L3S_CLKCTRL_OFFSET: c_uint = 0x68;

// pruss_ocp clocks
pub const AM4_PRUSS_OCP_CLKCTRL_OFFSET: c_uint = 0x320;

// l4ls clocks
pub const AM4_L4LS_CLKCTRL_OFFSET: c_uint = 0x420;

// emif clocks
pub const AM4_EMIF_CLKCTRL_OFFSET: c_uint = 0x720;

// dss clocks
pub const AM4_DSS_CLKCTRL_OFFSET: c_uint = 0xa20;

// cpsw_125mhz clocks
pub const AM4_CPSW_125MHZ_CLKCTRL_OFFSET: c_uint = 0xb20;

