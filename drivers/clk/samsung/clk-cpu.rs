//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/clk/samsung/clk-cpu.h
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
// Copyright (c) 2014 Samsung Electronics Co., Ltd.
//
// Common Clock Framework support for all PLL's in Samsung platforms
//
// The CPU clock registers have DIV1 configuration register

// When ALT parent is active, debug clocks need safe divider values

//
// enum exynos_cpuclk_layout - CPU clock registers layout compatibility
// @CPUCLK_LAYOUT_E4210: Exynos4210 compatible layout
// @CPUCLK_LAYOUT_E5433: Exynos5433 compatible layout
// @CPUCLK_LAYOUT_E850_CL0: Exynos850 cluster 0 compatible layout
// @CPUCLK_LAYOUT_E850_CL1: Exynos850 cluster 1 compatible layout
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum exynos_cpuclk_layout {
    CPUCLK_LAYOUT_E4210,
    CPUCLK_LAYOUT_E5433,
    CPUCLK_LAYOUT_E850_CL0,
    CPUCLK_LAYOUT_E850_CL1,
}

//
// struct exynos_cpuclk_cfg_data - config data to setup cpu clocks
// @prate: frequency of the primary parent clock (in KHz)
// @div0: value to be programmed in the div_cpu0 register
// @div1: value to be programmed in the div_cpu1 register
//
// This structure holds the divider configuration data for dividers in the CPU
// clock domain. The parent frequency at which these divider values are valid is
// specified in @prate. The @prate is the frequency of the primary parent clock.
// For CPU clock domains that do not have a DIV1 register, the @div1 member
// value is not used.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct exynos_cpuclk_cfg_data {
    pub prate: c_ulong,
    pub div0: c_ulong,
    pub div1: c_ulong,
}
