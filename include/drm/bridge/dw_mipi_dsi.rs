//! Automatically rewritten from C Header to Rust Module
//! Source: include/drm/bridge/dw_mipi_dsi.h
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
// Copyright (C) STMicroelectronics SA 2017
//
// Authors: Philippe Cornu <philippe.cornu@st.com>
// Yannick Fertre <yannick.fertre@st.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dw_mipi_dsi_dphy_timing {
    pub data_hs2lp: u16,
    pub data_lp2hs: u16,
    pub clk_hs2lp: u16,
    pub clk_lp2hs: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dw_mipi_dsi_phy_ops {
    pub priv_data): *mut *mut int (init)(void,
    pub priv_data): *mut *mut void (power_on)(void,
    pub priv_data): *mut *mut void (power_off)(void,
    pub lane_mbps): *mut c_uint,
    pub timing): *mut dw_mipi_dsi_dphy_timing,
    pub esc_clk_rate): *mut *mut *mut int (get_esc_clk_rate)(void priv_data, unsigned int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dw_mipi_dsi_host_ops {
    pub dsi): *mut mipi_dsi_device,
    pub dsi): *mut mipi_dsi_device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dw_mipi_dsi_plat_data {
    pub base: *mut void __iomem,
    pub max_data_lanes: c_uint,
    pub format): u32 lanes, u32,
    pub adjusted_mode): *mut drm_display_mode,
    pub num_input_fmts): *mut c_uint,
    pub phy_ops: *const dw_mipi_dsi_phy_ops,
    pub host_ops: *const dw_mipi_dsi_host_ops,
    pub priv_data: *mut c_void,
}

// plat_data);
extern "C" {
    pub fn dw_mipi_dsi_remove(dsi: *mut dw_mipi_dsi);
}
extern "C" {
    pub fn dw_mipi_dsi_bind(dsi: *mut dw_mipi_dsi, encoder: *mut drm_encoder) -> c_int;
}
extern "C" {
    pub fn dw_mipi_dsi_unbind(dsi: *mut dw_mipi_dsi);
}
extern "C" {
    pub fn dw_mipi_dsi_set_slave(dsi: *mut dw_mipi_dsi, slave: *mut dw_mipi_dsi);
}
