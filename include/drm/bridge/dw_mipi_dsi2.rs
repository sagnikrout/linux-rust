//! Automatically rewritten from C Header to Rust Module
//! Source: include/drm/bridge/dw_mipi_dsi2.h
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
// Copyright (c) 2024, Fuzhou Rockchip Electronics Co., Ltd
//
// Authors: Guochun Huang <hero.huang@rock-chips.com>
// Heiko Stuebner <heiko.stuebner@cherry.de>
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dw_mipi_dsi2_phy_type {
    DW_MIPI_DSI2_DPHY,
    DW_MIPI_DSI2_CPHY,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dw_mipi_dsi2_phy_iface {
    pub ppi_width: c_int,
    pub phy_type: dw_mipi_dsi2_phy_type,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dw_mipi_dsi2_phy_timing {
    pub data_hs2lp: u32,
    pub data_lp2hs: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dw_mipi_dsi2_phy_ops {
    pub priv_data): *mut *mut int (init)(void,
    pub priv_data): *mut *mut void (power_on)(void,
    pub priv_data): *mut *mut void (power_off)(void,
    pub iface): *mut *mut *mut void (get_interface)(void priv_data, struct dw_mipi_dsi2_phy_iface,
    pub lane_mbps): *mut c_uint,
    pub timing): *mut dw_mipi_dsi2_phy_timing,
    pub esc_clk_rate): *mut *mut *mut int (get_esc_clk_rate)(void priv_data, unsigned int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dw_mipi_dsi2_host_ops {
    pub dsi): *mut mipi_dsi_device,
    pub dsi): *mut mipi_dsi_device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dw_mipi_dsi2_plat_data {
    pub regmap: *mut regmap,
    pub max_data_lanes: c_uint,
    pub format): u32 lanes, u32,
    pub adjusted_mode): *mut drm_display_mode,
    pub num_input_fmts): *mut c_uint,
    pub phy_ops: *const dw_mipi_dsi2_phy_ops,
    pub host_ops: *const dw_mipi_dsi2_host_ops,
    pub priv_data: *mut c_void,
}

extern "C" {
    pub fn dw_mipi_dsi2_remove(dsi2: *mut dw_mipi_dsi2);
}
extern "C" {
    pub fn dw_mipi_dsi2_bind(dsi2: *mut dw_mipi_dsi2, encoder: *mut drm_encoder) -> c_int;
}
extern "C" {
    pub fn dw_mipi_dsi2_unbind(dsi2: *mut dw_mipi_dsi2);
}
