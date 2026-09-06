//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/phy/samsung/phy-samsung-ufs.h
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
// UFS PHY driver for Samsung EXYNOS SoC
//
// Copyright (C) 2020 Samsung Electronics Co., Ltd.
// Author: Seungwon Jeon <essuuj@gmail.com>
// Author: Alim Akhtar <alim.akhtar@samsung.com>
//

pub const PHY_COMN_BLK: c_int = 1;
pub const PHY_TRSV_BLK: c_int = 2;

pub const PHY_TRSV_CH_OFFSET: c_uint = 0x30;

// UFS PHY registers
pub const PHY_PLL_LOCK_STATUS: c_uint = 0x1e;

// description for PHY calibration
// applicable to any
// mode
// series
// gear
// field mask

// PHY calibration point/state
#[repr(C)]
#[derive(Copy, Clone)]
pub struct samsung_ufs_phy_cfg {
    pub off_0: u32,
    pub off_1: u32,
    pub val: u32,
    pub desc: u8,
    pub id: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct samsung_ufs_phy_pmu_isol {
    pub offset: u32,
    pub mask: u32,
    pub en: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct samsung_ufs_phy_drvdata {
    pub cfgs: *const samsung_ufs_phy_cfg,
    pub cfgs_hibern8: *const samsung_ufs_phy_cfg,
    pub isol: samsung_ufs_phy_pmu_isol,
    pub clk_list: *const *const c_char,
    pub num_clks: c_int,
    pub cdr_lock_status_offset: u32,
// SoC's specific operations
    pub lane): *mut *mut *mut int (wait_for_cal)(struct phy phy, u8,
    pub lane): *mut *mut *mut int (wait_for_cdr)(struct phy phy, u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct samsung_ufs_phy {
    pub dev: *mut device,
    pub reg_pma: *mut void __iomem,
    pub reg_pmu: *mut regmap,
    pub clks: *mut clk_bulk_data,
    pub drvdata: *const samsung_ufs_phy_drvdata,
    pub cfgs: *const *const samsung_ufs_phy_cfg,
    pub cfgs_hibern8: *const *const samsung_ufs_phy_cfg,
    pub isol: samsung_ufs_phy_pmu_isol,
    pub lane_cnt: u8,
    pub ufs_phy_state: c_int,
    pub mode: phy_mode,
}

extern "C" {
    pub fn samsung_ufs_phy_wait_for_lock_acq(phy: *mut phy, lane: u8) -> c_int;
}
extern "C" {
    pub fn exynosautov920_ufs_phy_wait_cdr_lock(phy: *mut phy, lane: u8) -> c_int;
}
