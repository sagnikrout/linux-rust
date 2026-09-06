//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/power/max77705_charger.h
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
// Maxim MAX77705 definitions.
//
// Copyright (C) 2015 Samsung Electronics, Inc.
// Copyright (C) 2025 Dzmitry Sankouski <dsankouski@gmail.com>
//

// MAX77705_CHG_REG_CHG_INT

// MAX77705_CHG_REG_CHG_INT_OK

// MAX77705_CHG_REG_DETAILS_00

pub const MAX77705_WCIN_DTLS_SHIFT: c_int = 3;

pub const MAX77705_CHGIN_DTLS_SHIFT: c_int = 5;
// MAX77705_CHG_REG_DETAILS_01

pub const MAX77705_CHG_DTLS_SHIFT: c_int = 0;

pub const MAX77705_BAT_DTLS_SHIFT: c_int = 4;
// MAX77705_CHG_REG_DETAILS_02

pub const MAX77705_BYP_DTLS_SHIFT: c_int = 0;
// MAX77705_CHG_REG_CNFG_00
pub const MAX77705_CHG_SHIFT: c_int = 0;
pub const MAX77705_UNO_SHIFT: c_int = 1;
pub const MAX77705_OTG_SHIFT: c_int = 1;
pub const MAX77705_BUCK_SHIFT: c_int = 2;
pub const MAX77705_BOOST_SHIFT: c_int = 3;
pub const MAX77705_WDTEN_SHIFT: c_int = 4;

// MAX77705_CHG_REG_CNFG_01
pub const MAX77705_FCHGTIME_DISABLE: c_int = 0;
pub const MAX77705_CHG_RSTRT_DISABLE: c_uint = 0x3;
pub const MAX77705_CHG_PQEN_DISABLE: c_int = 0;
pub const MAX77705_CHG_PQEN_ENABLE: c_int = 1;
// MAX77705_CHG_REG_CNFG_02
pub const MAX77705_OTG_ILIM_500: c_int = 0;
pub const MAX77705_OTG_ILIM_900: c_int = 1;
pub const MAX77705_OTG_ILIM_1200: c_int = 2;
pub const MAX77705_OTG_ILIM_1500: c_int = 3;
// MAX77705_CHG_REG_CNFG_03
pub const MAX77705_TO_ITH_150MA: c_int = 0;
pub const MAX77705_TO_TIME_30M: c_int = 3;
pub const MAX77705_SYS_TRACK_ENABLE: c_int = 0;
pub const MAX77705_SYS_TRACK_DISABLE: c_int = 1;
// MAX77705_CHG_REG_CNFG_04
pub const MAX77705_CHG_MINVSYS_SHIFT: c_int = 6;

// MAX77705_CHG_REG_CNFG_05
pub const MAX77705_B2SOVRC_DISABLE: c_int = 0;
pub const MAX77705_B2SOVRC_4_5A: c_int = 6;
pub const MAX77705_B2SOVRC_4_8A: c_int = 8;
pub const MAX77705_B2SOVRC_5_0A: c_int = 9;
// MAX77705_CHG_CNFG_06
pub const MAX77705_WDTCLR_SHIFT: c_int = 0;

pub const MAX77705_WDTCLR: c_int = 1;
pub const MAX77705_CHGPROT_UNLOCKED: c_int = 3;
pub const MAX77705_SLOWEST_LX_SLOPE: c_int = 3;
// MAX77705_CHG_REG_CNFG_07
pub const MAX77705_CHG_FMBST: c_int = 4;
pub const MAX77705_REG_FMBST_SHIFT: c_int = 2;

pub const MAX77705_REG_FGSRC_SHIFT: c_int = 1;

// MAX77705_CHG_REG_CNFG_08
pub const MAX77705_CHG_FSW_3MHz: c_int = 0;
pub const MAX77705_CHG_FSW_2MHz: c_int = 1;
pub const MAX77705_CHG_FSW_1_5MHz: c_int = 2;
// MAX77705_CHG_REG_CNFG_09
pub const MAX77705_CHG_DISABLE: c_int = 0;
// MAX77705_CHG_REG_CNFG_12
// REG=4.5V, UVLO=4.7V
pub const MAX77705_VCHGIN_4_5: c_int = 0;
// REG=4.5V, UVLO=4.7V
pub const MAX77705_WCIN_4_5: c_int = 0;
pub const MAX77705_DISABLE_SKIP: c_int = 1;
pub const MAX77705_AUTO_SKIP: c_int = 0;
pub const AICL_WORK_DELAY_MS: c_int = 100;
// uA
pub const MAX77705_CURRENT_CHGIN_STEP: c_int = 25000;
pub const MAX77705_CURRENT_CHG_STEP: c_int = 50000;
pub const MAX77705_CURRENT_CHGIN_MIN: c_int = 100000;
pub const MAX77705_CURRENT_CHGIN_MAX: c_int = 3200000;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum max77705_field_idx {
    MAX77705_CHGPROT,
    MAX77705_CHG_EN,
    MAX77705_CHG_CC_LIM,
    MAX77705_CHG_CHGIN_LIM,
    MAX77705_CHG_CV_PRM,
    MAX77705_CHG_PQEN,
    MAX77705_CHG_RSTRT,
    MAX77705_CHG_WCIN,
    MAX77705_FCHGTIME,
    MAX77705_LX_SLOPE,
    MAX77705_MODE,
    MAX77705_OTG_ILIM,
    MAX77705_REG_B2SOVRC,
    MAX77705_REG_DISKIP,
    MAX77705_REG_FSW,
    MAX77705_SYS_TRACK,
    MAX77705_TO,
    MAX77705_TO_TIME,
    MAX77705_VBYPSET,
    MAX77705_VCHGIN,
    MAX77705_WCIN,
    MAX77705_N_REGMAP_FIELDS,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct max77705_charger_data {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub rfield: [*mut regmap_field; MAX77705_N_REGMAP_FIELDS],
    pub bat_info: *mut power_supply_battery_info,
    pub wqueue: *mut workqueue_struct,
    pub chgin_work: work_struct,
    pub psy_chg: *mut power_supply,
}
