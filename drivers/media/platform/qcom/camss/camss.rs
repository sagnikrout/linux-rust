//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/qcom/camss/camss.h
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
// camss.h
//
// Qualcomm MSM Camera Subsystem - Core
//
// Copyright (c) 2015, The Linux Foundation. All rights reserved.
// Copyright (C) 2015-2018 Linaro Ltd.
//

pub const CAMSS_RES_MAX: c_int = 17;
pub const CAMSS_INIT_BUF_COUNT: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct camss_subdev_resources {
    pub regulators: [regulator_bulk_data; CAMSS_RES_MAX],
    pub clock: [*mut c_char; CAMSS_RES_MAX],
    pub clock_for_reset: [*mut c_char; CAMSS_RES_MAX],
    pub clock_rate: [u32; CAMSS_RES_MAX][CAMSS_RES_MAX],
    pub reg: [*mut c_char; CAMSS_RES_MAX],
    pub interrupt: [*mut c_char; CAMSS_RES_MAX],
    pub csiphy: csiphy_subdev_resources,
    pub tpg: tpg_subdev_resources,
    pub csid: csid_subdev_resources,
    pub vfe: vfe_subdev_resources,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icc_bw_tbl {
    pub avg: u32,
    pub peak: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct resources_icc {
    pub name: *mut c_char,
    pub icc_bw_tbl: icc_bw_tbl,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct resources_wrapper {
    pub reg: *mut c_char,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pm_domain {
    PM_DOMAIN_VFE0 = 0,
    PM_DOMAIN_VFE1 = 1,
    PM_DOMAIN_VFELITE = 2,		/* VFELITE / TOP GDSC */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum camss_version {
    CAMSS_660,
    CAMSS_2290,
    CAMSS_6150,
    CAMSS_6350,
    CAMSS_7280,
    CAMSS_8x16,
    CAMSS_8x39,
    CAMSS_8x53,
    CAMSS_8x96,
    CAMSS_8250,
    CAMSS_8280XP,
    CAMSS_8300,
    CAMSS_845,
    CAMSS_8550,
    CAMSS_8650,
    CAMSS_8775P,
    CAMSS_X1E80100,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum icc_count {
    ICC_DEFAULT_COUNT = 0,
    ICC_SM8250_COUNT = 4,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct camss_resources {
    pub version: camss_version,
    pub pd_name: *const c_char,
    pub csiphy_res: *const camss_subdev_resources,
    pub tpg_res: *const camss_subdev_resources,
    pub csid_res: *const camss_subdev_resources,
    pub ispif_res: *const camss_subdev_resources,
    pub vfe_res: *const camss_subdev_resources,
    pub csid_wrapper_res: *const resources_wrapper,
    pub icc_res: *const resources_icc,
    pub icc_path_num: c_uint,
    pub csiphy_num: c_uint,
    pub tpg_num: c_uint,
    pub csid_num: c_uint,
    pub vfe_num: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct camss {
    pub v4l2_dev: v4l2_device,
    pub notifier: v4l2_async_notifier,
    pub media_dev: media_device,
    pub dev: *mut device,
    pub csiphy: *mut csiphy_device,
    pub tpg: *mut tpg_device,
    pub csid: *mut csid_device,
    pub ispif: *mut ispif_device,
    pub vfe: *mut vfe_device,
    pub csid_wrapper_base: *mut void __iomem,
    pub ref_count: core::sync::atomic::AtomicI32,
    pub genpd_num: c_int,
    pub genpd: *mut device,
    pub genpd_link: *mut device_link,
    pub icc_path: [*mut icc_path; ICC_SM8250_COUNT],
    pub res: *const camss_resources,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct camss_camera_interface {
    pub csiphy_id: u8,
    pub csi2: csiphy_csi2_cfg,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct camss_async_subdev {
    pub /: *mut *mut v4l2_async_connection asd; / must be first,
    pub interface: camss_camera_interface,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct camss_clock {
    pub clk: *mut clk,
    pub name: *const c_char,
    pub freq: *mut u32,
    pub nfreqs: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct parent_dev_ops {
    pub id): *mut *mut *mut int (get)(struct camss camss, int,
    pub id): *mut *mut *mut int (put)(struct camss camss, int,
    pub id): *mut *mut *mut *mut void __iomem (get_base_address)(struct camss camss, int,
}

extern "C" {
    pub fn camss_add_clock_margin(rate: *mut u64);
}
extern "C" {
    pub fn camss_disable_clocks(nclocks: c_int, clock: *mut camss_clock);
}
extern "C" {
    pub fn camss_get_pixel_clock(entity: *mut media_entity, pixel_clock: *mut u64) -> c_int;
}
extern "C" {
    pub fn camss_pm_domain_on(camss: *mut camss, id: c_int) -> c_int;
}
extern "C" {
    pub fn camss_pm_domain_off(camss: *mut camss, id: c_int);
}
extern "C" {
    pub fn camss_vfe_get(camss: *mut camss, id: c_int) -> c_int;
}
extern "C" {
    pub fn camss_vfe_put(camss: *mut camss, id: c_int);
}
extern "C" {
    pub fn camss_delete(camss: *mut camss);
}
extern "C" {
    pub fn camss_buf_done(camss: *mut camss, hw_id: c_int, port_id: c_int);
}
