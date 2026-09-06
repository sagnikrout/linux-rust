//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/soc/samsung/exynos-pmu.h
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
// Copyright (c) 2015 Samsung Electronics Co., Ltd.
// http://www.samsung.com
//
// Header for Exynos PMU Driver support
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct exynos_pmu_conf {
    pub offset: c_uint,
    pub val: [u8; NUM_SYS_POWERDOWN],
}

//
// struct exynos_pmu_data - of_device_id (match) data
//
// @pmu_config: Optional table detailing register writes for target system
// states: SYS_AFTR, SYS_LPA, SYS_SLEEP.
// @pmu_config_extra: Optional secondary table detailing additional register
// writes for target system states: SYS_AFTR, SYS_LPA,
// SYS_SLEEP.
// @pmu_secure: Whether or not PMU register writes need to be done via SMC call.
// @pmu_cpuhp: Whether or not extra handling is required for CPU hotplug and
// CPUidle outside of standard PSCI calls, due to non-compliant
// firmware.
// @pmu_init: Optional init function.
// @powerdown_conf: Optional callback before entering target system states:
// SYS_AFTR, SYS_LPA, SYS_SLEEP. This will be invoked before
// the registers from @pmu_config are written.
// @powerdown_conf_extra: Optional secondary callback before entering
// target system states: SYS_AFTR, SYS_LPA, SYS_SLEEP.
// This will be invoked after @pmu_config registers have
// been written.
// @rd_table: A table of readable register ranges in case a custom regmap is
// used (i.e. when @pmu_secure is @true).
// @wr_table: A table of writable register ranges in case a custom regmap is
// used (i.e. when @pmu_secure is @true).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct exynos_pmu_data {
    pub pmu_config: *const exynos_pmu_conf,
    pub pmu_config_extra: *const exynos_pmu_conf,
    pub pmu_secure: bool,
    pub pmu_cpuhp: bool,
    pub (*pmu_init)(void): *mut c_void,
    pub sys_powerdown): *mut *mut void (powerdown_conf)(enum,
    pub sys_powerdown): *mut *mut void (powerdown_conf_extra)(enum,
    pub rd_table: *const regmap_access_table,
    pub wr_table: *const regmap_access_table,
}

// list of all exported SoC specific data

extern "C" {
    pub fn pmu_raw_writel(val: u32, offset: u32);
}
extern "C" {
    pub fn pmu_raw_readl(offset: u32) -> u32;
}
extern "C" {
    pub fn tensor_sec_reg_write(context: *mut c_void, reg: c_uint, val: c_uint) -> c_int;
}
extern "C" {
    pub fn tensor_sec_reg_read(context: *mut c_void, reg: c_uint, val: *mut c_uint) -> c_int;
}
