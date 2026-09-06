//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/lsi,axm5516-clks.h
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
// Copyright (c) 2014 LSI Corporation
//
pub const AXXIA_CLK_FAB_PLL: c_int = 0;
pub const AXXIA_CLK_CPU_PLL: c_int = 1;
pub const AXXIA_CLK_SYS_PLL: c_int = 2;
pub const AXXIA_CLK_SM0_PLL: c_int = 3;
pub const AXXIA_CLK_SM1_PLL: c_int = 4;
pub const AXXIA_CLK_FAB_DIV: c_int = 5;
pub const AXXIA_CLK_SYS_DIV: c_int = 6;
pub const AXXIA_CLK_NRCP_DIV: c_int = 7;
pub const AXXIA_CLK_CPU0_DIV: c_int = 8;
pub const AXXIA_CLK_CPU1_DIV: c_int = 9;
pub const AXXIA_CLK_CPU2_DIV: c_int = 10;
pub const AXXIA_CLK_CPU3_DIV: c_int = 11;
pub const AXXIA_CLK_PER_DIV: c_int = 12;
pub const AXXIA_CLK_MMC_DIV: c_int = 13;
pub const AXXIA_CLK_FAB: c_int = 14;
pub const AXXIA_CLK_SYS: c_int = 15;
pub const AXXIA_CLK_NRCP: c_int = 16;
pub const AXXIA_CLK_CPU0: c_int = 17;
pub const AXXIA_CLK_CPU1: c_int = 18;
pub const AXXIA_CLK_CPU2: c_int = 19;
pub const AXXIA_CLK_CPU3: c_int = 20;
pub const AXXIA_CLK_PER: c_int = 21;
pub const AXXIA_CLK_MMC: c_int = 22;
