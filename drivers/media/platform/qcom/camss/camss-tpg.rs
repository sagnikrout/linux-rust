//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/qcom/camss/camss-tpg.h
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
// camss-tpg.h
//
// Qualcomm MSM Camera Subsystem - TPG Module
//
// Copyright (c) 2024 Qualcomm Innovation Center, Inc. All rights reserved.
//

pub const ENCODE_FORMAT_UNCOMPRESSED_8_BIT: c_uint = 0x1;
pub const ENCODE_FORMAT_UNCOMPRESSED_10_BIT: c_uint = 0x2;
pub const ENCODE_FORMAT_UNCOMPRESSED_12_BIT: c_uint = 0x3;
pub const ENCODE_FORMAT_UNCOMPRESSED_14_BIT: c_uint = 0x4;
pub const ENCODE_FORMAT_UNCOMPRESSED_16_BIT: c_uint = 0x5;
pub const ENCODE_FORMAT_UNCOMPRESSED_20_BIT: c_uint = 0x6;
pub const ENCODE_FORMAT_UNCOMPRESSED_24_BIT: c_uint = 0x7;
pub const MSM_TPG_PAD_SRC: c_int = 0;
pub const MSM_TPG_ACTIVE_VC: c_int = 0;
pub const MSM_TPG_ACTIVE_DT: c_int = 0;
pub const TPG_MIN_WIDTH: c_int = 1;
pub const TPG_MIN_HEIGHT: c_int = 1;
pub const TPG_MAX_WIDTH: c_int = 8191;
pub const TPG_MAX_HEIGHT: c_int = 8191;
pub const TPG_GRP_ID: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tpg_testgen_mode {
    TPG_PAYLOAD_MODE_DISABLED = 0,
    TPG_PAYLOAD_MODE_INCREMENTING = 1,
    TPG_PAYLOAD_MODE_ALTERNATING_55_AA = 2,
    TPG_PAYLOAD_MODE_RANDOM = 5,
    TPG_PAYLOAD_MODE_USER_SPECIFIED = 6,
    TPG_PAYLOAD_MODE_COLOR_BARS = 9,
    TPG_PAYLOAD_MODE_NUM_SUPPORTED_GEN1 = 9,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tpg_testgen_config {
    pub mode: tpg_testgen_mode,
    pub const*modes: *const *const c_char,
    pub nmodes: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tpg_format_info {
    pub code: u32,
    pub data_type: u8,
    pub encode_format: u8,
    pub bpp: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tpg_formats {
    pub nformats: c_uint,
    pub formats: *const tpg_format_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tpg_hw_ops {
    pub enable): *mut *mut *mut int (configure_stream)(struct tpg_device tpg, u8,
    pub val): *mut *mut *mut int (configure_testgen_pattern)(struct tpg_device tpg, s32,
    pub tpg): *mut *mut u32 (hw_version)(struct tpg_device,
    pub tpg): *mut *mut int (reset)(struct tpg_device,
    pub tpg): *mut *mut void (subdev_init)(struct tpg_device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tpg_subdev_resources {
    pub lane_cnt: u8,
    pub formats: *const tpg_formats,
    pub hw_ops: *const tpg_hw_ops,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tpg_device {
    pub camss: *mut camss,
    pub id: u8,
    pub subdev: v4l2_subdev,
    pub pad: media_pad,
    pub base: *mut void __iomem,
    pub clock: *mut camss_clock,
    pub nclocks: c_int,
    pub testgen: tpg_testgen_config,
    pub fmt: v4l2_mbus_framefmt,
    pub ctrls: v4l2_ctrl_handler,
    pub testgen_mode: *mut v4l2_ctrl,
    pub res: *const tpg_subdev_resources,
    pub hw_version: u32,
}

extern "C" {
    pub fn msm_tpg_unregister_entity(tpg: *mut tpg_device);
}
