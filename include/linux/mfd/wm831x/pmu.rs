//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/wm831x/pmu.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// include/linux/mfd/wm831x/pmu.h -- PMU for WM831x
//
// Copyright 2009 Wolfson Microelectronics PLC.
//
// Author: Mark Brown <broonie@opensource.wolfsonmicro.com>
//
// R16387 (0x4003) - Power State
//
pub const WM831X_CHIP_ON: c_uint = 0x8000  /* CHIP_ON */;
pub const WM831X_CHIP_ON_MASK: c_uint = 0x8000  /* CHIP_ON */;

pub const WM831X_CHIP_SLP: c_uint = 0x4000  /* CHIP_SLP */;
pub const WM831X_CHIP_SLP_MASK: c_uint = 0x4000  /* CHIP_SLP */;

pub const WM831X_REF_LP: c_uint = 0x1000  /* REF_LP */;
pub const WM831X_REF_LP_MASK: c_uint = 0x1000  /* REF_LP */;

pub const WM831X_PWRSTATE_DLY_MASK: c_uint = 0x0C00  /* PWRSTATE_DLY - [11:10] */;

pub const WM831X_SWRST_DLY: c_uint = 0x0200  /* SWRST_DLY */;
pub const WM831X_SWRST_DLY_MASK: c_uint = 0x0200  /* SWRST_DLY */;

pub const WM831X_USB100MA_STARTUP_MASK: c_uint = 0x0030  /* USB100MA_STARTUP - [5:4] */;

pub const WM831X_USB_CURR_STS: c_uint = 0x0008  /* USB_CURR_STS */;
pub const WM831X_USB_CURR_STS_MASK: c_uint = 0x0008  /* USB_CURR_STS */;

pub const WM831X_USB_ILIM_MASK: c_uint = 0x0007  /* USB_ILIM - [2:0] */;

//
// R16397 (0x400D) - System Status
//
pub const WM831X_THW_STS: c_uint = 0x8000  /* THW_STS */;
pub const WM831X_THW_STS_MASK: c_uint = 0x8000  /* THW_STS */;

pub const WM831X_PWR_SRC_BATT: c_uint = 0x0400  /* PWR_SRC_BATT */;
pub const WM831X_PWR_SRC_BATT_MASK: c_uint = 0x0400  /* PWR_SRC_BATT */;

pub const WM831X_PWR_WALL: c_uint = 0x0200  /* PWR_WALL */;
pub const WM831X_PWR_WALL_MASK: c_uint = 0x0200  /* PWR_WALL */;

pub const WM831X_PWR_USB: c_uint = 0x0100  /* PWR_USB */;
pub const WM831X_PWR_USB_MASK: c_uint = 0x0100  /* PWR_USB */;

pub const WM831X_MAIN_STATE_MASK: c_uint = 0x001F  /* MAIN_STATE - [4:0] */;

//
// R16456 (0x4048) - Charger Control 1
//
pub const WM831X_CHG_ENA: c_uint = 0x8000  /* CHG_ENA */;
pub const WM831X_CHG_ENA_MASK: c_uint = 0x8000  /* CHG_ENA */;

pub const WM831X_CHG_FRC: c_uint = 0x4000  /* CHG_FRC */;
pub const WM831X_CHG_FRC_MASK: c_uint = 0x4000  /* CHG_FRC */;

pub const WM831X_CHG_ITERM_MASK: c_uint = 0x1C00  /* CHG_ITERM - [12:10] */;

pub const WM831X_CHG_FAST: c_uint = 0x0020  /* CHG_FAST */;
pub const WM831X_CHG_FAST_MASK: c_uint = 0x0020  /* CHG_FAST */;

pub const WM831X_CHG_IMON_ENA: c_uint = 0x0002  /* CHG_IMON_ENA */;
pub const WM831X_CHG_IMON_ENA_MASK: c_uint = 0x0002  /* CHG_IMON_ENA */;

pub const WM831X_CHG_CHIP_TEMP_MON: c_uint = 0x0001  /* CHG_CHIP_TEMP_MON */;
pub const WM831X_CHG_CHIP_TEMP_MON_MASK: c_uint = 0x0001  /* CHG_CHIP_TEMP_MON */;

//
// R16457 (0x4049) - Charger Control 2
//
pub const WM831X_CHG_OFF_MSK: c_uint = 0x4000  /* CHG_OFF_MSK */;
pub const WM831X_CHG_OFF_MSK_MASK: c_uint = 0x4000  /* CHG_OFF_MSK */;

pub const WM831X_CHG_TIME_MASK: c_uint = 0x0F00  /* CHG_TIME - [11:8] */;

pub const WM831X_CHG_TRKL_ILIM_MASK: c_uint = 0x00C0  /* CHG_TRKL_ILIM - [7:6] */;

pub const WM831X_CHG_VSEL_MASK: c_uint = 0x0030  /* CHG_VSEL - [5:4] */;

pub const WM831X_CHG_FAST_ILIM_MASK: c_uint = 0x000F  /* CHG_FAST_ILIM - [3:0] */;

//
// R16458 (0x404A) - Charger Status
//
pub const WM831X_BATT_OV_STS: c_uint = 0x8000  /* BATT_OV_STS */;
pub const WM831X_BATT_OV_STS_MASK: c_uint = 0x8000  /* BATT_OV_STS */;

pub const WM831X_CHG_STATE_MASK: c_uint = 0x7000  /* CHG_STATE - [14:12] */;

pub const WM831X_BATT_HOT_STS: c_uint = 0x0800  /* BATT_HOT_STS */;
pub const WM831X_BATT_HOT_STS_MASK: c_uint = 0x0800  /* BATT_HOT_STS */;

pub const WM831X_BATT_COLD_STS: c_uint = 0x0400  /* BATT_COLD_STS */;
pub const WM831X_BATT_COLD_STS_MASK: c_uint = 0x0400  /* BATT_COLD_STS */;

pub const WM831X_CHG_TOPOFF: c_uint = 0x0200  /* CHG_TOPOFF */;
pub const WM831X_CHG_TOPOFF_MASK: c_uint = 0x0200  /* CHG_TOPOFF */;

pub const WM831X_CHG_ACTIVE: c_uint = 0x0100  /* CHG_ACTIVE */;
pub const WM831X_CHG_ACTIVE_MASK: c_uint = 0x0100  /* CHG_ACTIVE */;

pub const WM831X_CHG_TIME_ELAPSED_MASK: c_uint = 0x00FF  /* CHG_TIME_ELAPSED - [7:0] */;

//
// R16459 (0x404B) - Backup Charger Control
//
pub const WM831X_BKUP_CHG_ENA: c_uint = 0x8000  /* BKUP_CHG_ENA */;
pub const WM831X_BKUP_CHG_ENA_MASK: c_uint = 0x8000  /* BKUP_CHG_ENA */;

pub const WM831X_BKUP_CHG_STS: c_uint = 0x4000  /* BKUP_CHG_STS */;
pub const WM831X_BKUP_CHG_STS_MASK: c_uint = 0x4000  /* BKUP_CHG_STS */;

pub const WM831X_BKUP_CHG_MODE: c_uint = 0x1000  /* BKUP_CHG_MODE */;
pub const WM831X_BKUP_CHG_MODE_MASK: c_uint = 0x1000  /* BKUP_CHG_MODE */;

pub const WM831X_BKUP_BATT_DET_ENA: c_uint = 0x0800  /* BKUP_BATT_DET_ENA */;
pub const WM831X_BKUP_BATT_DET_ENA_MASK: c_uint = 0x0800  /* BKUP_BATT_DET_ENA */;

pub const WM831X_BKUP_BATT_STS: c_uint = 0x0400  /* BKUP_BATT_STS */;
pub const WM831X_BKUP_BATT_STS_MASK: c_uint = 0x0400  /* BKUP_BATT_STS */;

pub const WM831X_BKUP_CHG_VLIM: c_uint = 0x0010  /* BKUP_CHG_VLIM */;
pub const WM831X_BKUP_CHG_VLIM_MASK: c_uint = 0x0010  /* BKUP_CHG_VLIM */;

pub const WM831X_BKUP_CHG_ILIM_MASK: c_uint = 0x0003  /* BKUP_CHG_ILIM - [1:0] */;

