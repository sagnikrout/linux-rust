//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/msm/dsi/phy/dsi_phy.h
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
// Copyright (c) 2015, The Linux Foundation. All rights reserved.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msm_dsi_phy_ops {
    pub phy): *mut *mut int (pll_init)(struct msm_dsi_phy,
    pub clk_req): *mut msm_dsi_phy_clk_request,
    pub phy): *mut *mut void (disable)(struct msm_dsi_phy,
    pub phy): *mut *mut void (save_pll_state)(struct msm_dsi_phy,
    pub phy): *mut *mut int (restore_pll_state)(struct msm_dsi_phy,
    pub enable): *mut *mut *mut bool (set_continuous_clock)(struct msm_dsi_phy phy, bool,
    pub phy): *mut *mut int (parse_dt_properties)(struct msm_dsi_phy,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msm_dsi_phy_cfg {
    pub regulator_data: *const regulator_bulk_data,
    pub num_regulators: c_int,
    pub ops: msm_dsi_phy_ops,
    pub min_pll_rate: c_ulong,
    pub max_pll_rate: c_ulong,
    pub io_start: [resource_size_t; DSI_MAX],
    pub num_dsi_phy: c_int,
    pub quirks: c_int,
    pub has_phy_regulator: bool,
    pub has_phy_lane: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msm_dsi_dphy_timing {
    pub clk_zero: u32,
    pub clk_trail: u32,
    pub clk_prepare: u32,
    pub hs_exit: u32,
    pub hs_zero: u32,
    pub hs_prepare: u32,
    pub hs_trail: u32,
    pub hs_rqst: u32,
    pub ta_go: u32,
    pub ta_sure: u32,
    pub ta_get: u32,
    pub shared_timings: msm_dsi_phy_shared_timings,
// For PHY v2 only
    pub hs_rqst_ckln: u32,
    pub hs_prep_dly: u32,
    pub hs_prep_dly_ckln: u32,
    pub hs_halfbyte_en: u8,
    pub hs_halfbyte_en_ckln: u8,
}

pub const DSI_LANE_MAX: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msm_dsi_phy {
    pub pdev: *mut platform_device,
    pub base: *mut void __iomem,
    pub pll_base: *mut void __iomem,
    pub reg_base: *mut void __iomem,
    pub lane_base: *mut void __iomem,
    pub base_size: phys_addr_t,
    pub pll_size: phys_addr_t,
    pub reg_size: phys_addr_t,
    pub lane_size: phys_addr_t,
    pub id: c_int,
    pub supplies: *mut regulator_bulk_data,
    pub timing: msm_dsi_dphy_timing,
    pub cfg: *const msm_dsi_phy_cfg,
    pub tuning_cfg: *mut c_void,
    pub pll_data: *mut c_void,
    pub usecase: msm_dsi_phy_usecase,
    pub regulator_ldo_mode: bool,
    pub cphy_mode: bool,
    pub vco_hw: *mut clk_hw,
    pub pll_on: bool,
    pub provided_clocks: *mut clk_hw_onecell_data,
    pub state_saved: bool,
}

//
// PHY internal functions
//
