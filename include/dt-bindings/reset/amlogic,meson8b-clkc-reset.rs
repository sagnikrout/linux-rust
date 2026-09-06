//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/reset/amlogic,meson8b-clkc-reset.h
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


//
// Copyright (c) 2017 Martin Blumenstingl <martin.blumenstingl@googlemail.com>.
//
// SPDX-License-Identifier: (GPL-2.0+ OR MIT)
//
pub const CLKC_RESET_L2_CACHE_SOFT_RESET: c_int = 0;
pub const CLKC_RESET_AXI_64_TO_128_BRIDGE_A5_SOFT_RESET: c_int = 1;
pub const CLKC_RESET_SCU_SOFT_RESET: c_int = 2;
pub const CLKC_RESET_CPU0_SOFT_RESET: c_int = 3;
pub const CLKC_RESET_CPU1_SOFT_RESET: c_int = 4;
pub const CLKC_RESET_CPU2_SOFT_RESET: c_int = 5;
pub const CLKC_RESET_CPU3_SOFT_RESET: c_int = 6;
pub const CLKC_RESET_A5_GLOBAL_RESET: c_int = 7;
pub const CLKC_RESET_A5_AXI_SOFT_RESET: c_int = 8;
pub const CLKC_RESET_A5_ABP_SOFT_RESET: c_int = 9;
pub const CLKC_RESET_AXI_64_TO_128_BRIDGE_MMC_SOFT_RESET: c_int = 10;
pub const CLKC_RESET_VID_CLK_CNTL_SOFT_RESET: c_int = 11;
pub const CLKC_RESET_VID_DIVIDER_CNTL_SOFT_RESET_POST: c_int = 12;
pub const CLKC_RESET_VID_DIVIDER_CNTL_SOFT_RESET_PRE: c_int = 13;
pub const CLKC_RESET_VID_DIVIDER_CNTL_RESET_N_POST: c_int = 14;
pub const CLKC_RESET_VID_DIVIDER_CNTL_RESET_N_PRE: c_int = 15;
