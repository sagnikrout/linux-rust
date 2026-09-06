//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/rockchip/cdn-dp-core.h
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
// Copyright (C) 2016 Chris Zhong <zyw@rock-chips.com>
// Copyright (C) Rockchip Electronics Co., Ltd.
//

pub const MAX_PHY: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum audio_format {
    AFMT_I2S = 0,
    AFMT_SPDIF = 1,
    AFMT_UNUSED,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct audio_info {
    pub format: audio_format,
    pub sample_rate: c_int,
    pub channels: c_int,
    pub sample_width: c_int,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vic_pxl_encoding_format {
    PXL_RGB = 0x1,
    YCBCR_4_4_4 = 0x2,
    YCBCR_4_2_2 = 0x4,
    YCBCR_4_2_0 = 0x8,
    Y_ONLY = 0x10,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct video_info {
    pub h_sync_polarity: bool,
    pub v_sync_polarity: bool,
    pub interlaced: bool,
    pub color_depth: c_int,
    pub color_fmt: vic_pxl_encoding_format,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdn_firmware_header {
    pub /: *mut *mut u32 size_bytes; / size of the entire header+image(s) in bytes,
    pub /: *mut *mut u32 header_size; / size of just the header in bytes,
    pub /: *mut *mut u32 iram_size; / size of iram,
    pub /: *mut *mut u32 dram_size; / size of dram,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdn_dp_port {
    pub dp: *mut cdn_dp_device,
    pub event_nb: notifier_block,
    pub extcon: *mut extcon_dev,
    pub phy: *mut phy,
    pub lanes: u8,
    pub phy_enabled: bool,
    pub id: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdn_dp_device {
    pub dev: *mut device,
    pub drm_dev: *mut drm_device,
    pub bridge: drm_bridge,
    pub encoder: rockchip_encoder,
    pub mode: drm_display_mode,
    pub audio_pdev: *mut platform_device,
    pub event_work: work_struct,
    pub lock: mutex,
    pub connected: bool,
    pub active: bool,
    pub suspended: bool,
    pub /: *const *const *const firmware fw; / cdn dp firmware,
    pub /: *mut *mut unsigned int fw_version; / cdn fw version,
    pub fw_loaded: bool,
    pub regs: *mut void __iomem,
    pub grf: *mut regmap,
    pub core_clk: *mut clk,
    pub pclk: *mut clk,
    pub spdif_clk: *mut clk,
    pub grf_clk: *mut clk,
    pub spdif_rst: *mut reset_control,
    pub dptx_rst: *mut reset_control,
    pub apb_rst: *mut reset_control,
    pub core_rst: *mut reset_control,
    pub audio_info: audio_info,
    pub video_info: video_info,
    pub port: [*mut cdn_dp_port; MAX_PHY],
    pub ports: u8,
    pub max_lanes: u8,
    pub max_rate: c_uint,
    pub lanes: u8,
    pub active_port: c_int,
    pub dpcd: [u8; DP_RECEIVER_CAP_SIZE],
}
