//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/qcom/camss/camss-ispif.h
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
// camss-ispif.h
//
// Qualcomm MSM Camera Subsystem - ISPIF (ISP Interface) Module
//
// Copyright (c) 2013-2014, The Linux Foundation. All rights reserved.
// Copyright (C) 2015-2018 Linaro Ltd.
//

pub const MSM_ISPIF_PAD_SINK: c_int = 0;
pub const MSM_ISPIF_PAD_SRC: c_int = 1;
pub const MSM_ISPIF_PADS_NUM: c_int = 2;
pub const MSM_ISPIF_VFE_NUM: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ispif_intf {
    PIX0,
    RDI0,
    PIX1,
    RDI1,
    RDI2
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ispif_intf_cmd_reg {
    pub cmd_0: u32,
    pub cmd_1: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ispif_line {
    pub ispif: *mut ispif_device,
    pub id: u8,
    pub csid_id: u8,
    pub vfe_id: u8,
    pub interface: ispif_intf,
    pub subdev: v4l2_subdev,
    pub pads: [media_pad; MSM_ISPIF_PADS_NUM],
    pub fmt: [v4l2_mbus_framefmt; MSM_ISPIF_PADS_NUM],
    pub formats: *const u32,
    pub nformats: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ispif_device {
    pub base: *mut void __iomem,
    pub base_clk_mux: *mut void __iomem,
    pub irq: u32,
    pub irq_name: [c_char; 30],
    pub clock: *mut camss_clock,
    pub nclocks: c_int,
    pub clock_for_reset: *mut camss_clock,
    pub nclocks_for_reset: c_int,
    pub reset_complete: [completion; MSM_ISPIF_VFE_NUM],
    pub power_count: c_int,
    pub power_lock: mutex,
    pub intf_cmd: [ispif_intf_cmd_reg; MSM_ISPIF_VFE_NUM],
    pub config_lock: mutex,
    pub line_num: c_uint,
    pub line: *mut ispif_line,
    pub camss: *mut camss,
}

extern "C" {
    pub fn msm_ispif_unregister_entities(ispif: *mut ispif_device);
}
