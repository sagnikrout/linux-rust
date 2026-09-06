//! Automatically rewritten from C Header to Rust Module
//! Source: include/drm/bridge/dw_hdmi.h
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
// Copyright (C) 2011 Freescale Semiconductor, Inc.
//

//
// DOC: Supported input formats and encodings
//
// Depending on the Hardware configuration of the Controller IP, it supports
// a subset of the following input formats and encodings on its internal
// 48bit bus.
//
// +----------------------+----------------------------------+------------------------------+
// | Format Name          | Format Code                      | Encodings                    |
// +----------------------+----------------------------------+------------------------------+
// | RGB 4:4:4 8bit       | ``MEDIA_BUS_FMT_RGB888_1X24``    | ``V4L2_YCBCR_ENC_DEFAULT``   |
// +----------------------+----------------------------------+------------------------------+
// | RGB 4:4:4 10bits     | ``MEDIA_BUS_FMT_RGB101010_1X30`` | ``V4L2_YCBCR_ENC_DEFAULT``   |
// +----------------------+----------------------------------+------------------------------+
// | RGB 4:4:4 12bits     | ``MEDIA_BUS_FMT_RGB121212_1X36`` | ``V4L2_YCBCR_ENC_DEFAULT``   |
// +----------------------+----------------------------------+------------------------------+
// | RGB 4:4:4 16bits     | ``MEDIA_BUS_FMT_RGB161616_1X48`` | ``V4L2_YCBCR_ENC_DEFAULT``   |
// +----------------------+----------------------------------+------------------------------+
// | YCbCr 4:4:4 8bit     | ``MEDIA_BUS_FMT_YUV8_1X24``      | ``V4L2_YCBCR_ENC_601``       |
// |                      |                                  | or ``V4L2_YCBCR_ENC_709``    |
// |                      |                                  | or ``V4L2_YCBCR_ENC_XV601``  |
// |                      |                                  | or ``V4L2_YCBCR_ENC_XV709``  |
// +----------------------+----------------------------------+------------------------------+
// | YCbCr 4:4:4 10bits   | ``MEDIA_BUS_FMT_YUV10_1X30``     | ``V4L2_YCBCR_ENC_601``       |
// |                      |                                  | or ``V4L2_YCBCR_ENC_709``    |
// |                      |                                  | or ``V4L2_YCBCR_ENC_XV601``  |
// |                      |                                  | or ``V4L2_YCBCR_ENC_XV709``  |
// +----------------------+----------------------------------+------------------------------+
// | YCbCr 4:4:4 12bits   | ``MEDIA_BUS_FMT_YUV12_1X36``     | ``V4L2_YCBCR_ENC_601``       |
// |                      |                                  | or ``V4L2_YCBCR_ENC_709``    |
// |                      |                                  | or ``V4L2_YCBCR_ENC_XV601``  |
// |                      |                                  | or ``V4L2_YCBCR_ENC_XV709``  |
// +----------------------+----------------------------------+------------------------------+
// | YCbCr 4:4:4 16bits   | ``MEDIA_BUS_FMT_YUV16_1X48``     | ``V4L2_YCBCR_ENC_601``       |
// |                      |                                  | or ``V4L2_YCBCR_ENC_709``    |
// |                      |                                  | or ``V4L2_YCBCR_ENC_XV601``  |
// |                      |                                  | or ``V4L2_YCBCR_ENC_XV709``  |
// +----------------------+----------------------------------+------------------------------+
// | YCbCr 4:2:2 8bit     | ``MEDIA_BUS_FMT_UYVY8_1X16``     | ``V4L2_YCBCR_ENC_601``       |
// |                      |                                  | or ``V4L2_YCBCR_ENC_709``    |
// +----------------------+----------------------------------+------------------------------+
// | YCbCr 4:2:2 10bits   | ``MEDIA_BUS_FMT_UYVY10_1X20``    | ``V4L2_YCBCR_ENC_601``       |
// |                      |                                  | or ``V4L2_YCBCR_ENC_709``    |
// +----------------------+----------------------------------+------------------------------+
// | YCbCr 4:2:2 12bits   | ``MEDIA_BUS_FMT_UYVY12_1X24``    | ``V4L2_YCBCR_ENC_601``       |
// |                      |                                  | or ``V4L2_YCBCR_ENC_709``    |
// +----------------------+----------------------------------+------------------------------+
// | YCbCr 4:2:0 8bit     | ``MEDIA_BUS_FMT_UYYVYY8_0_5X24`` | ``V4L2_YCBCR_ENC_601``       |
// |                      |                                  | or ``V4L2_YCBCR_ENC_709``    |
// +----------------------+----------------------------------+------------------------------+
// | YCbCr 4:2:0 10bits   | ``MEDIA_BUS_FMT_UYYVYY10_0_5X30``| ``V4L2_YCBCR_ENC_601``       |
// |                      |                                  | or ``V4L2_YCBCR_ENC_709``    |
// +----------------------+----------------------------------+------------------------------+
// | YCbCr 4:2:0 12bits   | ``MEDIA_BUS_FMT_UYYVYY12_0_5X36``| ``V4L2_YCBCR_ENC_601``       |
// |                      |                                  | or ``V4L2_YCBCR_ENC_709``    |
// +----------------------+----------------------------------+------------------------------+
// | YCbCr 4:2:0 16bits   | ``MEDIA_BUS_FMT_UYYVYY16_0_5X48``| ``V4L2_YCBCR_ENC_601``       |
// |                      |                                  | or ``V4L2_YCBCR_ENC_709``    |
// +----------------------+----------------------------------+------------------------------+
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dw_hdmi_phy_type {
    DW_HDMI_PHY_DWC_HDMI_TX_PHY = 0x00,
    DW_HDMI_PHY_DWC_MHL_PHY_HEAC = 0xb2,
    DW_HDMI_PHY_DWC_MHL_PHY = 0xc2,
    DW_HDMI_PHY_DWC_HDMI_3D_TX_PHY_HEAC = 0xe2,
    DW_HDMI_PHY_DWC_HDMI_3D_TX_PHY = 0xf2,
    DW_HDMI_PHY_DWC_HDMI20_TX_PHY = 0xf3,
    DW_HDMI_PHY_VENDOR_PHY = 0xfe,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dw_hdmi_mpll_config {
    pub mpixelclock: c_ulong,
    pub cpce: u16,
    pub gmp: u16,
    pub res: [}; DW_HDMI_RES_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dw_hdmi_curr_ctrl {
    pub mpixelclock: c_ulong,
    pub curr: [u16; DW_HDMI_RES_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dw_hdmi_phy_config {
    pub mpixelclock: c_ulong,
    pub control*/: *mut *mut u16 sym_ctr; /clock symbol and transmitter,
    pub value*/: *mut *mut u16 term; /transmission termination,
    pub /: *mut *mut u16 vlev_ctr; / voltage level control,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dw_hdmi_phy_ops {
    pub mode): *const drm_display_mode,
    pub data): *mut *mut *mut void (disable)(struct dw_hdmi hdmi, void,
    pub data): *mut *mut *mut drm_connector_status (read_hpd)(struct dw_hdmi hdmi, void,
    pub rxsense): bool force, bool disabled, bool,
    pub data): *mut *mut *mut void (setup_hpd)(struct dw_hdmi hdmi, void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dw_hdmi_plat_data {
    pub regm: *mut regmap,
//
// The HDMI output port number must be 1 if the port is described
// in the device tree. 0 if the device tree does not describe the
// next component (legacy mode, i.e. without
// DRM_BRIDGE_ATTACH_NO_CONNECTOR flag when attaching bridge).
//
    pub output_port: c_uint,
    pub input_bus_encoding: c_ulong,
    pub use_drm_infoframe: bool,
    pub ycbcr_420_allowed: bool,
//
// Private data passed to all the .mode_valid() and .configure_phy()
// callback functions.
//
    pub priv_data: *mut c_void,
// Platform-specific mode validation (optional).
    pub mode): *const drm_display_mode,
//
// priv_audio is specially used for additional audio device to get
// driver data through this dw_hdmi_plat_data.
//
    pub priv_audio: *mut c_void,
// Platform-specific audio enable/disable (optional)
    pub iec958): int width, int rate, int non_pcm, int,
    pub hdmi): *mut *mut void (disable_audio)(struct dw_hdmi,
// Vendor PHY support
    pub phy_ops: *const dw_hdmi_phy_ops,
    pub phy_name: *const c_char,
    pub phy_data: *mut c_void,
    pub phy_force_vendor: c_uint,
// Synopsys PHY support
    pub mpll_cfg: *const dw_hdmi_mpll_config,
    pub cur_ctr: *const dw_hdmi_curr_ctrl,
    pub phy_config: *const dw_hdmi_phy_config,
    pub mpixelclock): c_ulong,
    pub 1: unsigned int disable_cec :,
}

extern "C" {
    pub fn dw_hdmi_remove(hdmi: *mut dw_hdmi);
}
extern "C" {
    pub fn dw_hdmi_unbind(hdmi: *mut dw_hdmi);
}
extern "C" {
    pub fn dw_hdmi_resume(hdmi: *mut dw_hdmi);
}
extern "C" {
    pub fn dw_hdmi_setup_rx_sense(hdmi: *mut dw_hdmi, hpd: bool, rx_sense: bool);
}
extern "C" {
    pub fn dw_hdmi_set_sample_non_pcm(hdmi: *mut dw_hdmi, non_pcm: c_uint);
}
extern "C" {
    pub fn dw_hdmi_set_sample_iec958(hdmi: *mut dw_hdmi, iec958: c_uint);
}
extern "C" {
    pub fn dw_hdmi_set_sample_width(hdmi: *mut dw_hdmi, width: c_uint);
}
extern "C" {
    pub fn dw_hdmi_set_sample_rate(hdmi: *mut dw_hdmi, rate: c_uint);
}
extern "C" {
    pub fn dw_hdmi_set_channel_count(hdmi: *mut dw_hdmi, cnt: c_uint);
}
extern "C" {
    pub fn dw_hdmi_set_channel_status(hdmi: *mut dw_hdmi, channel_status: *mut u8);
}
extern "C" {
    pub fn dw_hdmi_set_channel_allocation(hdmi: *mut dw_hdmi, ca: c_uint);
}
extern "C" {
    pub fn dw_hdmi_audio_enable(hdmi: *mut dw_hdmi);
}
extern "C" {
    pub fn dw_hdmi_audio_disable(hdmi: *mut dw_hdmi);
}
// PHY configuration
extern "C" {
    pub fn dw_hdmi_phy_i2c_set_addr(hdmi: *mut dw_hdmi, address: u8);
}
extern "C" {
    pub fn dw_hdmi_phy_gen1_reset(hdmi: *mut dw_hdmi);
}
extern "C" {
    pub fn dw_hdmi_phy_gen2_pddq(hdmi: *mut dw_hdmi, enable: u8);
}
extern "C" {
    pub fn dw_hdmi_phy_gen2_txpwron(hdmi: *mut dw_hdmi, enable: u8);
}
extern "C" {
    pub fn dw_hdmi_phy_gen2_reset(hdmi: *mut dw_hdmi);
}
extern "C" {
    pub fn dw_hdmi_phy_setup_hpd(hdmi: *mut dw_hdmi, data: *mut c_void);
}
extern "C" {
    pub fn dw_hdmi_bus_fmt_is_420(hdmi: *mut dw_hdmi) -> bool;
}
