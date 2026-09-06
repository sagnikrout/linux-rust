//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/msm/dsi/dsi.h
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

pub const DSI_0: c_int = 0;
pub const DSI_1: c_int = 1;
pub const DSI_MAX: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum msm_dsi_phy_usecase {
    MSM_DSI_PHY_STANDALONE,
    MSM_DSI_PHY_MASTER,
    MSM_DSI_PHY_SLAVE,
}

pub const DSI_BUS_CLK_MAX: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msm_dsi {
    pub dev: *mut drm_device,
    pub pdev: *mut platform_device,
    pub host: *mut mipi_dsi_host,
    pub phy: *mut msm_dsi_phy,
    pub te_source: *const c_char,
    pub next_bridge: *mut drm_bridge,
    pub phy_dev: *mut device,
    pub phy_enabled: bool,
    pub id: c_int,
}

// dsi manager
extern "C" {
    pub fn msm_dsi_manager_cmd_xfer(id: c_int, msg: *const mipi_dsi_msg) -> c_int;
}
extern "C" {
    pub fn msm_dsi_manager_cmd_xfer_trigger(id: c_int, dma_base: u32, len: u32) -> bool;
}
extern "C" {
    pub fn msm_dsi_manager_register(msm_dsi: *mut msm_dsi) -> c_int;
}
extern "C" {
    pub fn msm_dsi_manager_unregister(msm_dsi: *mut msm_dsi);
}
extern "C" {
    pub fn msm_dsi_manager_tpg_enable();
}
// dsi host
extern "C" {
    pub fn msm_dsi_host_enable(host: *mut mipi_dsi_host) -> c_int;
}
extern "C" {
    pub fn msm_dsi_host_disable(host: *mut mipi_dsi_host) -> c_int;
}
extern "C" {
    pub fn msm_dsi_host_enable_irq(host: *mut mipi_dsi_host);
}
extern "C" {
    pub fn msm_dsi_host_disable_irq(host: *mut mipi_dsi_host);
}
extern "C" {
    pub fn msm_dsi_host_power_off(host: *mut mipi_dsi_host) -> c_int;
}
extern "C" {
    pub fn msm_dsi_host_get_mode_flags(host: *mut mipi_dsi_host) -> c_ulong;
}
extern "C" {
    pub fn msm_dsi_host_register(host: *mut mipi_dsi_host) -> c_int;
}
extern "C" {
    pub fn msm_dsi_host_unregister(host: *mut mipi_dsi_host);
}
extern "C" {
    pub fn msm_dsi_host_reset_phy(host: *mut mipi_dsi_host);
}
extern "C" {
    pub fn msm_dsi_host_destroy(host: *mut mipi_dsi_host);
}
extern "C" {
    pub fn msm_dsi_host_init(msm_dsi: *mut msm_dsi) -> c_int;
}
extern "C" {
    pub fn msm_dsi_runtime_suspend(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn msm_dsi_runtime_resume(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn dsi_link_clk_set_rate_6g(msm_host: *mut msm_dsi_host) -> c_int;
}
extern "C" {
    pub fn dsi_link_clk_set_rate_6g_v2_9(msm_host: *mut msm_dsi_host) -> c_int;
}
extern "C" {
    pub fn dsi_link_clk_set_rate_v2(msm_host: *mut msm_dsi_host) -> c_int;
}
extern "C" {
    pub fn dsi_link_clk_enable_6g(msm_host: *mut msm_dsi_host) -> c_int;
}
extern "C" {
    pub fn dsi_link_clk_enable_v2(msm_host: *mut msm_dsi_host) -> c_int;
}
extern "C" {
    pub fn dsi_link_clk_disable_6g(msm_host: *mut msm_dsi_host);
}
extern "C" {
    pub fn dsi_link_clk_disable_v2(msm_host: *mut msm_dsi_host);
}
extern "C" {
    pub fn dsi_tx_buf_alloc_6g(msm_host: *mut msm_dsi_host, size: c_int) -> c_int;
}
extern "C" {
    pub fn dsi_tx_buf_alloc_v2(msm_host: *mut msm_dsi_host, size: c_int) -> c_int;
}
extern "C" {
    pub fn dsi_tx_buf_put_6g(msm_host: *mut msm_dsi_host);
}
extern "C" {
    pub fn msm_dsi_tx_buf_free(mipi_host: *mut mipi_dsi_host);
}
extern "C" {
    pub fn dsi_dma_base_get_6g(msm_host: *mut msm_dsi_host, iova: *mut u64) -> c_int;
}
extern "C" {
    pub fn dsi_dma_base_get_v2(msm_host: *mut msm_dsi_host, iova: *mut u64) -> c_int;
}
extern "C" {
    pub fn dsi_clk_init_v2(msm_host: *mut msm_dsi_host) -> c_int;
}
extern "C" {
    pub fn dsi_clk_init_6g_v2(msm_host: *mut msm_dsi_host) -> c_int;
}
extern "C" {
    pub fn dsi_clk_init_6g_v2_9(msm_host: *mut msm_dsi_host) -> c_int;
}
extern "C" {
    pub fn dsi_calc_clk_rate_v2(msm_host: *mut msm_dsi_host, is_bonded_dsi: bool) -> c_int;
}
extern "C" {
    pub fn dsi_calc_clk_rate_6g(msm_host: *mut msm_dsi_host, is_bonded_dsi: bool) -> c_int;
}
extern "C" {
    pub fn msm_dsi_host_snapshot(disp_state: *mut msm_disp_state, host: *mut mipi_dsi_host);
}
extern "C" {
    pub fn msm_dsi_host_test_pattern_en(host: *mut mipi_dsi_host);
}
extern "C" {
    pub fn msm_dsi_host_is_wide_bus_enabled(host: *mut mipi_dsi_host) -> bool;
}
// dsi phy
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msm_dsi_phy_shared_timings {
    pub clk_post: u32,
    pub clk_pre: u32,
    pub clk_pre_inc_by_2: bool,
    pub byte_intf_clk_div_2: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msm_dsi_phy_clk_request {
    pub bitclk_rate: c_ulong,
    pub escclk_rate: c_ulong,
}

extern "C" {
    pub fn msm_dsi_phy_driver_register();
}
extern "C" {
    pub fn msm_dsi_phy_driver_unregister();
}
extern "C" {
    pub fn msm_dsi_phy_disable(phy: *mut msm_dsi_phy);
}
extern "C" {
    pub fn msm_dsi_phy_pll_save_state(phy: *mut msm_dsi_phy);
}
extern "C" {
    pub fn msm_dsi_phy_pll_restore_state(phy: *mut msm_dsi_phy) -> c_int;
}
extern "C" {
    pub fn msm_dsi_phy_snapshot(disp_state: *mut msm_disp_state, phy: *mut msm_dsi_phy);
}
extern "C" {
    pub fn msm_dsi_phy_set_continuous_clock(phy: *mut msm_dsi_phy, enable: bool) -> bool;
}
