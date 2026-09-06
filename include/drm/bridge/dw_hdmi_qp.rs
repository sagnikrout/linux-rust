//! Automatically rewritten from C Header to Rust Module
//! Source: include/drm/bridge/dw_hdmi_qp.h
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
// Copyright (c) 2021-2022 Rockchip Electronics Co., Ltd.
// Copyright (c) 2024 Collabora Ltd.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dw_hdmi_qp_phy_ops {
    pub data): *mut *mut *mut int (init)(struct dw_hdmi_qp hdmi, void,
    pub data): *mut *mut *mut void (disable)(struct dw_hdmi_qp hdmi, void,
    pub data): *mut *mut *mut drm_connector_status (read_hpd)(struct dw_hdmi_qp hdmi, void,
    pub data): *mut *mut *mut void (setup_hpd)(struct dw_hdmi_qp hdmi, void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dw_hdmi_qp_plat_data {
    pub phy_ops: *const dw_hdmi_qp_phy_ops,
    pub phy_data: *mut c_void,
    pub main_irq: c_int,
    pub cec_irq: c_int,
    pub ref_clk_rate: c_ulong,
// Supported output formats: bitmask of @drm_output_color_format
    pub supported_formats: c_uint,
// Maximum bits per color channel: 8, 10 or 12
    pub max_bpc: c_uint,
}

extern "C" {
    pub fn dw_hdmi_qp_suspend(dev: *mut device, hdmi: *mut dw_hdmi_qp);
}
extern "C" {
    pub fn dw_hdmi_qp_resume(dev: *mut device, hdmi: *mut dw_hdmi_qp);
}
