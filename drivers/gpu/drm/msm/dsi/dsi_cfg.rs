//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/msm/dsi/dsi_cfg.h
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

pub const MSM_DSI_VER_MAJOR_V2: c_uint = 0x02;
pub const MSM_DSI_VER_MAJOR_6G: c_uint = 0x03;
pub const MSM_DSI_6G_VER_MINOR_V1_0: c_uint = 0x10000000;
pub const MSM_DSI_6G_VER_MINOR_V1_0_2: c_uint = 0x10000002;
pub const MSM_DSI_6G_VER_MINOR_V1_1: c_uint = 0x10010000;
pub const MSM_DSI_6G_VER_MINOR_V1_1_1: c_uint = 0x10010001;
pub const MSM_DSI_6G_VER_MINOR_V1_2: c_uint = 0x10020000;
pub const MSM_DSI_6G_VER_MINOR_V1_3: c_uint = 0x10030000;
pub const MSM_DSI_6G_VER_MINOR_V1_3_1: c_uint = 0x10030001;
pub const MSM_DSI_6G_VER_MINOR_V1_4_1: c_uint = 0x10040001;
pub const MSM_DSI_6G_VER_MINOR_V1_4_2: c_uint = 0x10040002;
pub const MSM_DSI_6G_VER_MINOR_V2_0_0: c_uint = 0x20000000;
pub const MSM_DSI_6G_VER_MINOR_V2_1_0: c_uint = 0x20010000;
pub const MSM_DSI_6G_VER_MINOR_V2_2_1: c_uint = 0x20020001;
pub const MSM_DSI_6G_VER_MINOR_V2_3_0: c_uint = 0x20030000;
pub const MSM_DSI_6G_VER_MINOR_V2_3_1: c_uint = 0x20030001;
pub const MSM_DSI_6G_VER_MINOR_V2_4_0: c_uint = 0x20040000;
pub const MSM_DSI_6G_VER_MINOR_V2_4_1: c_uint = 0x20040001;
pub const MSM_DSI_6G_VER_MINOR_V2_5_0: c_uint = 0x20050000;
pub const MSM_DSI_6G_VER_MINOR_V2_5_1: c_uint = 0x20050001;
pub const MSM_DSI_6G_VER_MINOR_V2_6_0: c_uint = 0x20060000;
pub const MSM_DSI_6G_VER_MINOR_V2_7_0: c_uint = 0x20070000;
pub const MSM_DSI_6G_VER_MINOR_V2_8_0: c_uint = 0x20080000;
pub const MSM_DSI_6G_VER_MINOR_V2_9_0: c_uint = 0x20090000;
pub const MSM_DSI_6G_VER_MINOR_V2_10_0: c_uint = 0x200a0000;
pub const MSM_DSI_V2_VER_MINOR_8064: c_uint = 0x0;
pub const DSI_6G_REG_SHIFT: c_int = 4;
// Maximum number of configurations matched against the same hw revision
pub const VARIANTS_MAX: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msm_dsi_config {
    pub io_offset: u32,
    pub regulator_data: *const regulator_bulk_data,
    pub num_regulators: c_int,
    pub bus_clk_names: *const *const c_char,
    pub num_bus_clks: c_int,
    pub io_start: [resource_size_t; VARIANTS_MAX][DSI_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msm_dsi_host_cfg_ops {
    pub msm_host): *mut *mut int (link_clk_set_rate)(struct msm_dsi_host,
    pub msm_host): *mut *mut int (link_clk_enable)(struct msm_dsi_host,
    pub msm_host): *mut *mut void (link_clk_disable)(struct msm_dsi_host,
    pub msm_host): *mut *mut int (clk_init_ver)(struct msm_dsi_host,
    pub size): *mut *mut *mut int (tx_buf_alloc)(struct msm_dsi_host msm_host, int,
    pub msm_host): *mut *mut *mut void (tx_buf_get)(struct msm_dsi_host,
    pub msm_host): *mut *mut void (tx_buf_put)(struct msm_dsi_host,
    pub iova): *mut *mut *mut int (dma_base_get)(struct msm_dsi_host msm_host, uint64_t,
    pub is_bonded_dsi): *mut *mut *mut int (calc_clk_rate)(struct msm_dsi_host msm_host, bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msm_dsi_cfg_handler {
    pub major: u32,
    pub minor: u32,
    pub cfg: *const msm_dsi_config,
    pub ops: *const msm_dsi_host_cfg_ops,
}
