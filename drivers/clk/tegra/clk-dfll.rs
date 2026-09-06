//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/clk/tegra/clk-dfll.h
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
// clk-dfll.h - prototypes and macros for the Tegra DFLL clocksource driver
// Copyright (C) 2013-2019 NVIDIA Corporation.  All rights reserved.
//
// Aleksandr Frid <afrid@nvidia.com>
// Paul Walmsley <pwalmsley@nvidia.com>
//

//
// struct tegra_dfll_soc_data - SoC-specific hooks/integration for the DFLL driver
// @dev: struct device * that holds the OPP table for the DFLL
// @max_freq: maximum frequency supported on this SoC
// @cvb: CPU frequency table for this SoC
// @alignment: parameters of the regulator step and offset
// @init_clock_trimmers: callback to initialize clock trimmers
// @set_clock_trimmers_high: callback to tune clock trimmers for high voltage
// @set_clock_trimmers_low: callback to tune clock trimmers for low voltage
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_dfll_soc_data {
    pub dev: *mut device,
    pub max_freq: c_ulong,
    pub cvb: *const cvb_table,
    pub alignment: rail_alignment,
    pub (*init_clock_trimmers)(void): *mut c_void,
    pub (*set_clock_trimmers_high)(void): *mut c_void,
    pub (*set_clock_trimmers_low)(void): *mut c_void,
}

extern "C" {
    pub fn tegra_dfll_runtime_suspend(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn tegra_dfll_runtime_resume(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn tegra_dfll_suspend(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn tegra_dfll_resume(dev: *mut device) -> c_int;
}
