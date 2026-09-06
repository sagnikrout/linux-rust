//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/boot/dts/freescale/imx95-clock.h
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


// SPDX-License-Identifier: GPL-2.0-only OR MIT
//
// Copyright 2024 NXP
//
// The index should match i.MX95 SCMI Firmware
pub const IMX95_CLK_32K: c_int = 1;
pub const IMX95_CLK_24M: c_int = 2;
pub const IMX95_CLK_FRO: c_int = 3;
pub const IMX95_CLK_SYSPLL1_VCO: c_int = 4;
pub const IMX95_CLK_SYSPLL1_PFD0_UNGATED: c_int = 5;
pub const IMX95_CLK_SYSPLL1_PFD0: c_int = 6;
pub const IMX95_CLK_SYSPLL1_PFD0_DIV2: c_int = 7;
pub const IMX95_CLK_SYSPLL1_PFD1_UNGATED: c_int = 8;
pub const IMX95_CLK_SYSPLL1_PFD1: c_int = 9;
pub const IMX95_CLK_SYSPLL1_PFD1_DIV2: c_int = 10;
pub const IMX95_CLK_SYSPLL1_PFD2_UNGATED: c_int = 11;
pub const IMX95_CLK_SYSPLL1_PFD2: c_int = 12;
pub const IMX95_CLK_SYSPLL1_PFD2_DIV2: c_int = 13;
pub const IMX95_CLK_AUDIOPLL1_VCO: c_int = 14;
pub const IMX95_CLK_AUDIOPLL1: c_int = 15;
pub const IMX95_CLK_AUDIOPLL2_VCO: c_int = 16;
pub const IMX95_CLK_AUDIOPLL2: c_int = 17;
pub const IMX95_CLK_VIDEOPLL1_VCO: c_int = 18;
pub const IMX95_CLK_VIDEOPLL1: c_int = 19;
pub const IMX95_CLK_RESERVED20: c_int = 20;
pub const IMX95_CLK_RESERVED21: c_int = 21;
pub const IMX95_CLK_RESERVED22: c_int = 22;
pub const IMX95_CLK_RESERVED23: c_int = 23;
pub const IMX95_CLK_ARMPLL_VCO: c_int = 24;
pub const IMX95_CLK_ARMPLL_PFD0_UNGATED: c_int = 25;
pub const IMX95_CLK_ARMPLL_PFD0: c_int = 26;
pub const IMX95_CLK_ARMPLL_PFD1_UNGATED: c_int = 27;
pub const IMX95_CLK_ARMPLL_PFD1: c_int = 28;
pub const IMX95_CLK_ARMPLL_PFD2_UNGATED: c_int = 29;
pub const IMX95_CLK_ARMPLL_PFD2: c_int = 30;
pub const IMX95_CLK_ARMPLL_PFD3_UNGATED: c_int = 31;
pub const IMX95_CLK_ARMPLL_PFD3: c_int = 32;
pub const IMX95_CLK_DRAMPLL_VCO: c_int = 33;
pub const IMX95_CLK_DRAMPLL: c_int = 34;
pub const IMX95_CLK_HSIOPLL_VCO: c_int = 35;
pub const IMX95_CLK_HSIOPLL: c_int = 36;
pub const IMX95_CLK_LDBPLL_VCO: c_int = 37;
pub const IMX95_CLK_LDBPLL: c_int = 38;
pub const IMX95_CLK_EXT1: c_int = 39;
pub const IMX95_CLK_EXT2: c_int = 40;
pub const IMX95_CCM_NUM_CLK_SRC: c_int = 41;

