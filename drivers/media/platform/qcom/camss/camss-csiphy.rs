//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/qcom/camss/camss-csiphy.h
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
// camss-csiphy.h
//
// Qualcomm MSM Camera Subsystem - CSIPHY Module
//
// Copyright (c) 2011-2015, The Linux Foundation. All rights reserved.
// Copyright (C) 2016-2018 Linaro Ltd.
//

pub const MSM_CSIPHY_PAD_SINK: c_int = 0;
pub const MSM_CSIPHY_PAD_SRC: c_int = 1;
pub const MSM_CSIPHY_PADS_NUM: c_int = 2;
pub const CSIPHY_GRP_ID: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct csiphy_lane {
    pub pos: u8,
    pub pol: u8,
}

//
// struct csiphy_lanes_cfg - CSIPHY lanes configuration
// @num_data: number of data lanes
// @data:     data lanes configuration
// @clk:      clock lane configuration (only for D-PHY)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct csiphy_lanes_cfg {
    pub num_data: c_int,
    pub data: *mut csiphy_lane,
    pub clk: csiphy_lane,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct csiphy_csi2_cfg {
    pub lane_cfg: csiphy_lanes_cfg,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct csiphy_config {
    pub combo_mode: u8,
    pub csid_id: u8,
    pub csi2: *mut csiphy_csi2_cfg,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct csiphy_format_info {
    pub code: u32,
    pub bpp: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct csiphy_formats {
    pub nformats: c_uint,
    pub formats: *const csiphy_format_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct csiphy_hw_ops {
//
// csiphy_get_lane_mask - Calculate CSI2 lane mask configuration parameter
// @lane_cfg - CSI2 lane configuration
//
// Return lane mask
//
    pub lane_cfg): *mut *mut u8 (get_lane_mask)(struct csiphy_lanes_cfg,
    pub dev): *mut device,
    pub csiphy): *mut *mut void (reset)(struct csiphy_device,
    pub lane_mask): s64 link_freq, u8,
    pub cfg): *mut csiphy_config,
    pub dev): *mut *mut irqreturn_t (isr)(int irq, void,
    pub csiphy): *mut *mut int (init)(struct csiphy_device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct csiphy_subdev_resources {
    pub id: u8,
    pub hw_ops: *const csiphy_hw_ops,
    pub formats: *const csiphy_formats,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct csiphy_device_regs {
    pub lane_regs: *const csiphy_lane_regs,
    pub lane_array_size: c_int,
    pub offset: u32,
    pub common_status_offset: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct csiphy_device {
    pub camss: *mut camss,
    pub id: u8,
    pub subdev: v4l2_subdev,
    pub pads: [media_pad; MSM_CSIPHY_PADS_NUM],
    pub base: *mut void __iomem,
    pub base_clk_mux: *mut void __iomem,
    pub irq: u32,
    pub irq_name: [c_char; 30],
    pub clock: *mut camss_clock,
    pub rate_set: *mut bool,
    pub nclocks: c_int,
    pub timer_clk_rate: u32,
    pub supplies: *mut regulator_bulk_data,
    pub num_supplies: c_int,
    pub cfg: csiphy_config,
    pub fmt: [v4l2_mbus_framefmt; MSM_CSIPHY_PADS_NUM],
    pub res: *const csiphy_subdev_resources,
    pub regs: *mut csiphy_device_regs,
}

extern "C" {
    pub fn msm_csiphy_unregister_entity(csiphy: *mut csiphy_device);
}
