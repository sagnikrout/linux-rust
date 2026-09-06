//! Automatically rewritten from C Header to Rust Module
//! Source: include/soc/amlogic/meson_ddr_pmu.h
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
// Copyright (c) 2022 Amlogic, Inc. All rights reserved.
//
pub const MAX_CHANNEL_NUM: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmc_counter {
    pub /: *mut *mut u64 all_cnt; / The count of all requests come in/out ddr controller,
    pub all_req: u64,
    pub all_idle_cnt: u64,
    pub all_16bit_cnt: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmc_hw_info {
    pub info): *mut *mut void (enable)(struct dmc_info,
    pub info): *mut *mut void (disable)(struct dmc_info,
// Bind an axi line to a bandwidth-monitor channel
    pub chann): *mut *mut *mut void (set_axi_filter)(struct dmc_info info, int axi_id, int,
    pub counter): *mut dmc_counter,
    pub counter): *mut dmc_counter,
    pub /: *mut *mut int dmc_nr; / The number of dmc controller,
    pub /: *mut *mut int chann_nr; / The number of dmc bandwidth monitor channels,
    pub fmt_attr: *mut attribute,
    pub capability: [u64; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmc_info {
    pub hw_info: *const dmc_hw_info,
    pub ddr_reg: [*mut void __iomem; 4],
    pub /: *mut *mut unsigned long timer_value; / Timer value in TIMER register,
    pub pll_reg: *mut void __iomem,
    pub /: *mut *mut int irq_num; / irq vector number,
}

extern "C" {
    pub fn meson_ddr_pmu_create(pdev: *mut platform_device) -> c_int;
}
extern "C" {
    pub fn meson_ddr_pmu_remove(pdev: *mut platform_device) -> c_int;
}
