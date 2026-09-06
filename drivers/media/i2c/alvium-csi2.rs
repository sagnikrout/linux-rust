//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/i2c/alvium-csi2.h
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
// Allied Vision Technologies GmbH Alvium camera driver
//
// Copyright (C) 2023 Tommaso Merciai
// Copyright (C) 2023 Martin Hecht
// Copyright (C) 2023 Avnet EMG GmbH
//

// Basic Control Register Map register offsets (BCRM)

// Streaming Control Registers

// Acquisition Control Registers

// Heartbeat reg

// GenCP Registers

// defines
pub const REG_BCRM_HANDSHAKE_STATUS_MASK: c_uint = 0x01;
pub const REG_BCRM_HANDSHAKE_AVAILABLE_MASK: c_uint = 0x80;

pub const ALVIUM_DEFAULT_FR_HZ: c_int = 10;
pub const ALVIUM_DEFAULT_PIXEL_RATE_MHZ: c_int = 148000000;
pub const ALVIUM_LP2HS_DELAY_MS: c_int = 100;

pub const BCRM_DEVICE_FW_MAJOR_SHIFT: c_int = 8;

pub const BCRM_DEVICE_FW_MINOR_SHIFT: c_int = 16;

pub const BCRM_DEVICE_FW_PATCH_SHIFT: c_int = 32;

pub const BCRM_DEVICE_FW_SPEC_SHIFT: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum alvium_bcrm_mode {
    ALVIUM_BCM_MODE,
    ALVIUM_GENCP_MODE,
    ALVIUM_NUM_MODE
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum alvium_mipi_fmt {
    ALVIUM_FMT_UYVY8_2X8 = 0,
    ALVIUM_FMT_UYVY8_1X16,
    ALVIUM_FMT_YUYV8_1X16,
    ALVIUM_FMT_YUYV8_2X8,
    ALVIUM_FMT_YUYV10_1X20,
    ALVIUM_FMT_RGB888_1X24,
    ALVIUM_FMT_RBG888_1X24,
    ALVIUM_FMT_BGR888_1X24,
    ALVIUM_FMT_RGB888_3X8,
    ALVIUM_FMT_Y8_1X8,
    ALVIUM_FMT_SGRBG8_1X8,
    ALVIUM_FMT_SRGGB8_1X8,
    ALVIUM_FMT_SGBRG8_1X8,
    ALVIUM_FMT_SBGGR8_1X8,
    ALVIUM_FMT_Y10_1X10,
    ALVIUM_FMT_SGRBG10_1X10,
    ALVIUM_FMT_SRGGB10_1X10,
    ALVIUM_FMT_SGBRG10_1X10,
    ALVIUM_FMT_SBGGR10_1X10,
    ALVIUM_FMT_Y12_1X12,
    ALVIUM_FMT_SGRBG12_1X12,
    ALVIUM_FMT_SRGGB12_1X12,
    ALVIUM_FMT_SGBRG12_1X12,
    ALVIUM_FMT_SBGGR12_1X12,
    ALVIUM_FMT_SBGGR14_1X14,
    ALVIUM_FMT_SGBRG14_1X14,
    ALVIUM_FMT_SRGGB14_1X14,
    ALVIUM_FMT_SGRBG14_1X14,
    ALVIUM_NUM_SUPP_MIPI_DATA_FMT
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum alvium_av_bayer_bit {
    ALVIUM_BIT_BAY_NONE = -1,
    ALVIUM_BIT_BAY_MONO = 0,
    ALVIUM_BIT_BAY_GR,
    ALVIUM_BIT_BAY_RG,
    ALVIUM_BIT_BAY_GB,
    ALVIUM_BIT_BAY_BG,
    ALVIUM_NUM_BAY_AV_BIT
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum alvium_av_mipi_bit {
    ALVIUM_BIT_YUV420_8_LEG = 0,
    ALVIUM_BIT_YUV420_8,
    ALVIUM_BIT_YUV420_10,
    ALVIUM_BIT_YUV420_8_CSPS,
    ALVIUM_BIT_YUV420_10_CSPS,
    ALVIUM_BIT_YUV422_8,
    ALVIUM_BIT_YUV422_10,
    ALVIUM_BIT_RGB888,
    ALVIUM_BIT_RGB666,
    ALVIUM_BIT_RGB565,
    ALVIUM_BIT_RGB555,
    ALVIUM_BIT_RGB444,
    ALVIUM_BIT_RAW6,
    ALVIUM_BIT_RAW7,
    ALVIUM_BIT_RAW8,
    ALVIUM_BIT_RAW10,
    ALVIUM_BIT_RAW12,
    ALVIUM_BIT_RAW14,
    ALVIUM_BIT_JPEG,
    ALVIUM_NUM_SUPP_MIPI_DATA_BIT
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct alvium_avail_feat {
    pub rev_x:1: u64,
    pub rev_y:1: u64,
    pub int_autop:1: u64,
    pub black_lvl:1: u64,
    pub gain:1: u64,
    pub gamma:1: u64,
    pub contrast:1: u64,
    pub sat:1: u64,
    pub hue:1: u64,
    pub whiteb:1: u64,
    pub sharp:1: u64,
    pub auto_exp:1: u64,
    pub auto_gain:1: u64,
    pub auto_whiteb:1: u64,
    pub dev_temp:1: u64,
    pub acq_abort:1: u64,
    pub acq_fr:1: u64,
    pub fr_trigger:1: u64,
    pub exp_acq_line:1: u64,
    pub reserved:45: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct alvium_avail_mipi_fmt {
    pub yuv420_8_leg:1: u64,
    pub yuv420_8:1: u64,
    pub yuv420_10:1: u64,
    pub yuv420_8_csps:1: u64,
    pub yuv420_10_csps:1: u64,
    pub yuv422_8:1: u64,
    pub yuv422_10:1: u64,
    pub rgb888:1: u64,
    pub rgb666:1: u64,
    pub rgb565:1: u64,
    pub rgb555:1: u64,
    pub rgb444:1: u64,
    pub raw6:1: u64,
    pub raw7:1: u64,
    pub raw8:1: u64,
    pub raw10:1: u64,
    pub raw12:1: u64,
    pub raw14:1: u64,
    pub jpeg:1: u64,
    pub reserved:45: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct alvium_avail_bayer {
    pub mono:1: u8,
    pub gr:1: u8,
    pub rg:1: u8,
    pub gb:1: u8,
    pub bg:1: u8,
    pub reserved:3: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct alvium_mode {
    pub crop: v4l2_rect,
    pub fmt: v4l2_mbus_framefmt,
    pub width: u32,
    pub height: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct alvium_pixfmt {
    pub code: u32,
    pub colorspace: u32,
    pub mipi_fmt_regval: u64,
    pub bay_fmt_regval: u64,
    pub id: u8,
    pub is_raw: u8,
    pub fmt_av_bit: u8,
    pub bay_av_bit: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct alvium_ctrls {
    pub handler: v4l2_ctrl_handler,
    pub pixel_rate: *mut v4l2_ctrl,
    pub link_freq: *mut v4l2_ctrl,
    pub auto_exp: *mut v4l2_ctrl,
    pub exposure: *mut v4l2_ctrl,
    pub auto_wb: *mut v4l2_ctrl,
    pub blue_balance: *mut v4l2_ctrl,
    pub red_balance: *mut v4l2_ctrl,
    pub auto_gain: *mut v4l2_ctrl,
    pub gain: *mut v4l2_ctrl,
    pub saturation: *mut v4l2_ctrl,
    pub hue: *mut v4l2_ctrl,
    pub contrast: *mut v4l2_ctrl,
    pub gamma: *mut v4l2_ctrl,
    pub sharpness: *mut v4l2_ctrl,
    pub hflip: *mut v4l2_ctrl,
    pub vflip: *mut v4l2_ctrl,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct alvium_dev {
    pub i2c_client: *mut i2c_client,
    pub sd: v4l2_subdev,
    pub ep: v4l2_fwnode_endpoint,
    pub pad: media_pad,
    pub regmap: *mut regmap,
    pub reg_vcc: *mut regulator,
    pub bcrm_addr: u16,
    pub avail_ft: alvium_avail_feat,
    pub is_mipi_fmt_avail: [u8; ALVIUM_NUM_SUPP_MIPI_DATA_BIT],
    pub is_bay_avail: [u8; ALVIUM_NUM_BAY_AV_BIT],
    pub min_csi_clk: u32,
    pub max_csi_clk: u32,
    pub dft_img_width: u32,
    pub img_min_width: u32,
    pub img_max_width: u32,
    pub img_inc_width: u32,
    pub dft_img_height: u32,
    pub img_min_height: u32,
    pub img_max_height: u32,
    pub img_inc_height: u32,
    pub min_offx: u32,
    pub max_offx: u32,
    pub inc_offx: u32,
    pub min_offy: u32,
    pub max_offy: u32,
    pub inc_offy: u32,
    pub dft_gain: u64,
    pub min_gain: u64,
    pub max_gain: u64,
    pub inc_gain: u64,
    pub dft_exp: u64,
    pub min_exp: u64,
    pub max_exp: u64,
    pub inc_exp: u64,
    pub dft_rbalance: u64,
    pub min_rbalance: u64,
    pub max_rbalance: u64,
    pub inc_rbalance: u64,
    pub dft_bbalance: u64,
    pub min_bbalance: u64,
    pub max_bbalance: u64,
    pub inc_bbalance: u64,
    pub dft_hue: i32,
    pub min_hue: i32,
    pub max_hue: i32,
    pub inc_hue: i32,
    pub dft_contrast: u32,
    pub min_contrast: u32,
    pub max_contrast: u32,
    pub inc_contrast: u32,
    pub dft_sat: u32,
    pub min_sat: u32,
    pub max_sat: u32,
    pub inc_sat: u32,
    pub dft_black_lvl: i32,
    pub min_black_lvl: i32,
    pub max_black_lvl: i32,
    pub inc_black_lvl: i32,
    pub dft_gamma: u64,
    pub min_gamma: u64,
    pub max_gamma: u64,
    pub inc_gamma: u64,
    pub dft_sharp: i32,
    pub min_sharp: i32,
    pub max_sharp: i32,
    pub inc_sharp: i32,
    pub mode: alvium_mode,
    pub h_sup_csi_lanes: u8,
    pub link_freq: u64,
    pub ctrls: alvium_ctrls,
    pub bcrm_mode: u8,
    pub alvium_csi2_fmt: *mut alvium_pixfmt,
    pub alvium_csi2_fmt_n: u8,
    pub streaming: u8,
    pub apply_fiv: u8,
}

extern "C" {
    pub fn container_of_const(_arg: sd, alvium_dev: struct, _arg: sd) -> return;
}
