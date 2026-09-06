//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/sprd/sprd_dsi.h
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
// Copyright (C) 2020 Unisoc Inc.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dsi_work_mode {
    DSI_MODE_CMD = 0,
    DSI_MODE_VIDEO
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum video_burst_mode {
    VIDEO_NON_BURST_WITH_SYNC_PULSES = 0,
    VIDEO_NON_BURST_WITH_SYNC_EVENTS,
    VIDEO_BURST_WITH_SYNC_PULSES
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dsi_color_coding {
    COLOR_CODE_16BIT_CONFIG1 = 0,
    COLOR_CODE_16BIT_CONFIG2,
    COLOR_CODE_16BIT_CONFIG3,
    COLOR_CODE_18BIT_CONFIG1,
    COLOR_CODE_18BIT_CONFIG2,
    COLOR_CODE_24BIT,
    COLOR_CODE_20BIT_YCC422_LOOSELY,
    COLOR_CODE_24BIT_YCC422,
    COLOR_CODE_16BIT_YCC422,
    COLOR_CODE_30BIT,
    COLOR_CODE_36BIT,
    COLOR_CODE_12BIT_YCC420,
    COLOR_CODE_COMPRESSTION,
    COLOR_CODE_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pll_timing {
    NONE,
    REQUEST_TIME,
    PREPARE_TIME,
    SETTLE_TIME,
    ZERO_TIME,
    TRAIL_TIME,
    EXIT_TIME,
    CLKPOST_TIME,
    TA_GET,
    TA_GO,
    TA_SURE,
    TA_WAIT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dphy_pll {
    pub /: *mut *mut u8 refin; / Pre-divider control signal,
    pub /: *mut *mut u8 cp_s; / 00: SDM_EN=1, 10: SDM_EN=0,
    pub /: *mut *mut u8 fdk_s; / PLL mode control: integer or fraction,
    pub sdm_en: u8,
    pub div: u8,
    pub /: *mut *mut u8 int_n; / integer N PLL,
    pub /: *mut *mut u32 ref_clk; / dphy reference clock, unit: MHz,
    pub /: *mut *mut u32 freq; / panel config, unit: KHz,
    pub fvco: u32,
    pub potential_fvco: u32,
    pub /: *mut *mut u32 nint; / sigma delta modulator NINT control,
    pub /: *mut *mut u32 kint; / sigma delta modulator KINT control,
    pub /: *mut *mut u8 lpf_sel; / low pass filter control,
    pub /: *mut *mut u8 out_sel; / post divider control,
    pub /: *mut *mut u8 vco_band; / vco range,
    pub det_delay: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsi_context {
    pub base: *mut void __iomem,
    pub regmap: *mut regmap,
    pub pll: dphy_pll,
    pub vm: videomode,
    pub enabled: bool,
    pub work_mode: u8,
    pub burst_mode: u8,
    pub int0_mask: u32,
    pub int1_mask: u32,
// maximum time (ns) for data lanes from HS to LP
    pub data_hs2lp: u16,
// maximum time (ns) for data lanes from LP to HS
    pub data_lp2hs: u16,
// maximum time (ns) for clk lanes from HS to LP
    pub clk_hs2lp: u16,
// maximum time (ns) for clk lanes from LP to HS
    pub clk_lp2hs: u16,
// maximum time (ns) for BTA operation - REQUIRED
    pub max_rd_time: u16,
// enable receiving frame ack packets - for video mode
    pub frame_ack_en: bool,
// enable receiving tear effect ack packets - for cmd mode
    pub te_ack_en: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sprd_dsi {
    pub drm: *mut drm_device,
    pub host: mipi_dsi_host,
    pub slave: *mut mipi_dsi_device,
    pub encoder: drm_encoder,
    pub panel_bridge: *mut drm_bridge,
    pub ctx: dsi_context,
}

extern "C" {
    pub fn dphy_pll_config(ctx: *mut dsi_context) -> c_int;
}
extern "C" {
    pub fn dphy_timing_config(ctx: *mut dsi_context);
}
