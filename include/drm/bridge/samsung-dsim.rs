//! Automatically rewritten from C Header to Rust Module
//! Source: include/drm/bridge/samsung-dsim.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// Copyright (C) 2022 Amarula Solutions(India)
// Author: Jagan Teki <jagan@amarulasolutions.com>
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum samsung_dsim_type {
    DSIM_TYPE_EXYNOS3250,
    DSIM_TYPE_EXYNOS4210,
    DSIM_TYPE_EXYNOS5410,
    DSIM_TYPE_EXYNOS5422,
    DSIM_TYPE_EXYNOS5433,
    DSIM_TYPE_EXYNOS7870,
    DSIM_TYPE_IMX8MM,
    DSIM_TYPE_IMX8MP,
    DSIM_TYPE_COUNT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct samsung_dsim_transfer {
    pub list: list_head,
    pub completed: completion,
    pub result: c_int,
    pub packet: mipi_dsi_packet,
    pub flags: u16,
    pub tx_done: u16,
    pub rx_payload: *mut u8,
    pub rx_len: u16,
    pub rx_done: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct samsung_dsim_driver_data {
    pub reg_ofs: *const c_uint,
    pub plltmr_reg: c_uint,
    pub has_legacy_status_reg:1: c_uint,
    pub has_freqband:1: c_uint,
    pub has_clklane_stop:1: c_uint,
    pub has_broken_fifoctrl_emptyhdr:1: c_uint,
    pub has_sfrctrl:1: c_uint,
    pub clk_data: *mut clk_bulk_data,
    pub num_clks: c_uint,
    pub min_freq: c_uint,
    pub max_freq: c_uint,
    pub wait_for_hdr_fifo: c_uint,
    pub wait_for_reset: c_uint,
    pub num_bits_resol: c_uint,
    pub video_mode_bit: c_uint,
    pub pll_stable_bit: c_uint,
    pub esc_clken_bit: c_uint,
    pub byte_clken_bit: c_uint,
    pub tx_req_hsclk_bit: c_uint,
    pub lane_esc_clk_bit: c_uint,
    pub lane_esc_data_offset: c_uint,
    pub pll_p_offset: c_uint,
    pub pll_m_offset: c_uint,
    pub pll_s_offset: c_uint,
    pub main_vsa_offset: c_uint,
    pub reg_values: *const c_uint,
    pub pll_fin_min: c_uint,
    pub pll_fin_max: c_uint,
    pub m_min: u16,
    pub m_max: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct samsung_dsim_host_ops {
    pub dsim): *mut *mut int (register_host)(struct samsung_dsim,
    pub dsim): *mut *mut void (unregister_host)(struct samsung_dsim,
    pub device): *mut *mut *mut int (attach)(struct samsung_dsim dsim, struct mipi_dsi_device,
    pub device): *mut *mut *mut void (detach)(struct samsung_dsim dsim, struct mipi_dsi_device,
    pub dsim): *mut *mut irqreturn_t (te_irq_handler)(struct samsung_dsim,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct samsung_dsim_plat_data {
    pub hw_type: samsung_dsim_type,
    pub host_ops: *const samsung_dsim_host_ops,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct samsung_dsim {
    pub dsi_host: mipi_dsi_host,
    pub bridge: drm_bridge,
    pub dev: *mut device,
    pub mode: drm_display_mode,
    pub reg_base: *mut void __iomem,
    pub phy: *mut phy,
    pub pll_clk: *mut clk,
    pub supplies: [regulator_bulk_data; 2],
    pub irq: c_int,
    pub te_gpio: *mut gpio_desc,
    pub pll_clk_rate: u32,
    pub burst_clk_rate: u32,
    pub hs_clock: u32,
    pub esc_clk_rate: u32,
    pub lanes: u32,
    pub mode_flags: u32,
    pub format: u32,
    pub swap_dn_dp_clk: bool,
    pub swap_dn_dp_data: bool,
    pub state: c_int,
    pub brightness: *mut drm_property,
    pub completed: completion,
    pub /: *mut *mut spinlock_t transfer_lock; / protects transfer_list,
    pub transfer_list: list_head,
    pub driver_data: *const samsung_dsim_driver_data,
    pub plat_data: *const samsung_dsim_plat_data,
    pub priv: *mut c_void,
}

extern "C" {
    pub fn samsung_dsim_probe(pdev: *mut platform_device) -> c_int;
}
extern "C" {
    pub fn samsung_dsim_remove(pdev: *mut platform_device);
}
