//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/intel_soc_pmic.h
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
// Intel SoC PMIC Driver
//
// Copyright (C) 2012-2014 Intel Corporation. All rights reserved.
//
// Author: Yang, Bin <bin.yang@intel.com>
// Author: Zhu, Lejun <lejun.zhu@linux.intel.com>
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum intel_cht_wc_models {
    INTEL_CHT_WC_UNKNOWN,
    INTEL_CHT_WC_GPD_WIN_POCKET,
    INTEL_CHT_WC_XIAOMI_MIPAD2,
    INTEL_CHT_WC_LENOVO_YOGABOOK1,
    INTEL_CHT_WC_LENOVO_YT3_X90,
}

//
// struct intel_soc_pmic - Intel SoC PMIC data
// @irq: Master interrupt number of the parent PMIC device
// @regmap: Pointer to the parent PMIC device regmap structure
// @irq_chip_data: IRQ chip data for the PMIC itself
// @irq_chip_data_pwrbtn: Chained IRQ chip data for the Power Button
// @irq_chip_data_tmu: Chained IRQ chip data for the Time Management Unit
// @irq_chip_data_bcu: Chained IRQ chip data for the Burst Control Unit
// @irq_chip_data_adc: Chained IRQ chip data for the General Purpose ADC
// @irq_chip_data_chgr: Chained IRQ chip data for the External Charger
// @irq_chip_data_crit: Chained IRQ chip data for the Critical Event Handler
// @dev: Pointer to the parent PMIC device
// @scu: Pointer to the SCU IPC device data structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_soc_pmic {
    pub irq: c_int,
    pub regmap: *mut regmap,
    pub irq_chip_data: *mut regmap_irq_chip_data,
    pub irq_chip_data_pwrbtn: *mut regmap_irq_chip_data,
    pub irq_chip_data_tmu: *mut regmap_irq_chip_data,
    pub irq_chip_data_bcu: *mut regmap_irq_chip_data,
    pub irq_chip_data_adc: *mut regmap_irq_chip_data,
    pub irq_chip_data_chgr: *mut regmap_irq_chip_data,
    pub irq_chip_data_crit: *mut regmap_irq_chip_data,
    pub dev: *mut device,
    pub scu: *mut intel_scu_ipc_dev,
    pub cht_wc_model: intel_cht_wc_models,
}
