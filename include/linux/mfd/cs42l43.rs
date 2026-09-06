//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/cs42l43.h
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
// CS42L43 core driver external data
//
// Copyright (C) 2022-2023 Cirrus Logic, Inc. and
// Cirrus Logic International Semiconductor Ltd.
//

pub const CS42L43_N_SUPPLIES: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cs42l43_irq_numbers {
    CS42L43_PLL_LOST_LOCK,
    CS42L43_PLL_READY,

    CS42L43_HP_STARTUP_DONE,
    CS42L43_HP_SHUTDOWN_DONE,
    CS42L43_HSDET_DONE,
    CS42L43_TIPSENSE_UNPLUG_DB,
    CS42L43_TIPSENSE_PLUG_DB,
    CS42L43_RINGSENSE_UNPLUG_DB,
    CS42L43_RINGSENSE_PLUG_DB,
    CS42L43_TIPSENSE_UNPLUG_PDET,
    CS42L43_TIPSENSE_PLUG_PDET,
    CS42L43_RINGSENSE_UNPLUG_PDET,
    CS42L43_RINGSENSE_PLUG_PDET,

    CS42L43_HS2_BIAS_SENSE,
    CS42L43_HS1_BIAS_SENSE,
    CS42L43_DC_DETECT1_FALSE,
    CS42L43_DC_DETECT1_TRUE,
    CS42L43_HSBIAS_CLAMPED,
    CS42L43_HS3_4_BIAS_SENSE,

    CS42L43_AMP2_CLK_STOP_FAULT,
    CS42L43_AMP1_CLK_STOP_FAULT,
    CS42L43_AMP2_VDDSPK_FAULT,
    CS42L43_AMP1_VDDSPK_FAULT,
    CS42L43_AMP2_SHUTDOWN_DONE,
    CS42L43_AMP1_SHUTDOWN_DONE,
    CS42L43_AMP2_STARTUP_DONE,
    CS42L43_AMP1_STARTUP_DONE,
    CS42L43_AMP2_THERM_SHDN,
    CS42L43_AMP1_THERM_SHDN,
    CS42L43_AMP2_THERM_WARN,
    CS42L43_AMP1_THERM_WARN,
    CS42L43_AMP2_SCDET,
    CS42L43_AMP1_SCDET,

    CS42L43_GPIO3_FALL,
    CS42L43_GPIO3_RISE,
    CS42L43_GPIO2_FALL,
    CS42L43_GPIO2_RISE,
    CS42L43_GPIO1_FALL,
    CS42L43_GPIO1_RISE,

    CS42L43_HP_ILIMIT,
    CS42L43_HP_LOADDET_DONE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cs42l43 {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub sdw: *mut sdw_slave,
    pub vdd_p: *mut regulator,
    pub vdd_d: *mut regulator,
    pub core_supplies: [regulator_bulk_data; CS42L43_N_SUPPLIES],
    pub reset: *mut gpio_desc,
    pub irq: c_int,
    pub irq_chip: regmap_irq_chip,
    pub irq_data: *mut regmap_irq_chip_data,
    pub boot_work: work_struct,
    pub device_detach: completion,
    pub firmware_download: completion,
    pub firmware_error: c_int,
    pub sdw_freq: c_uint,
// Lock to gate control of the PLL and its sources.
    pub pll_lock: mutex,
    pub sdw_pll_active: bool,
    pub hw_lock: bool,
    pub variant_id: c_long,
}
