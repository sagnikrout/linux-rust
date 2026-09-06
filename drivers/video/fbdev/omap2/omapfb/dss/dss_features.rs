//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/video/fbdev/omap2/omapfb/dss/dss_features.h
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
// linux/drivers/video/omap2/dss/dss_features.h
//
// Copyright (C) 2010 Texas Instruments
// Author: Archit Taneja <archit@ti.com>
//
pub const MAX_DSS_MANAGERS: c_int = 4;
pub const MAX_DSS_OVERLAYS: c_int = 4;
pub const MAX_DSS_LCD_MANAGERS: c_int = 3;
pub const MAX_NUM_DSI: c_int = 2;
// DSS has feature id
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dss_feat_id {
    FEAT_LCDENABLEPOL,
    FEAT_LCDENABLESIGNAL,
    FEAT_PCKFREEENABLE,
    FEAT_FUNCGATED,
    FEAT_MGR_LCD2,
    FEAT_MGR_LCD3,
    FEAT_LINEBUFFERSPLIT,
    FEAT_ROWREPEATENABLE,
    FEAT_RESIZECONF,
// Independent core clk divider
    FEAT_CORE_CLK_DIV,
    FEAT_LCD_CLK_SRC,
// DSI-PLL power command 0x3 is not working
    FEAT_DSI_PLL_PWR_BUG,
    FEAT_DSI_DCS_CMD_CONFIG_VC,
    FEAT_DSI_VC_OCP_WIDTH,
    FEAT_DSI_REVERSE_TXCLKESC,
    FEAT_DSI_GNQ,
    FEAT_DPI_USES_VDDS_DSI,
    FEAT_HDMI_CTS_SWMODE,
    FEAT_HDMI_AUDIO_USE_MCLK,
    FEAT_HANDLE_UV_SEPARATE,
    FEAT_ATTR2,
    FEAT_VENC_REQUIRES_TV_DAC_CLK,
    FEAT_CPR,
    FEAT_PRELOAD,
    FEAT_FIR_COEF_V,
    FEAT_ALPHA_FIXED_ZORDER,
    FEAT_ALPHA_FREE_ZORDER,
    FEAT_FIFO_MERGE,
// An unknown HW bug causing the normal FIFO thresholds not to work
    FEAT_OMAP3_DSI_FIFO_BUG,
    FEAT_BURST_2D,
    FEAT_DSI_PHY_DCC,
    FEAT_MFLAG,
}

// DSS register field id
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dss_feat_reg_field {
    FEAT_REG_FIRHINC,
    FEAT_REG_FIRVINC,
    FEAT_REG_FIFOHIGHTHRESHOLD,
    FEAT_REG_FIFOLOWTHRESHOLD,
    FEAT_REG_FIFOSIZE,
    FEAT_REG_HORIZONTALACCU,
    FEAT_REG_VERTICALACCU,
    FEAT_REG_DISPC_CLK_SWITCH,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dss_range_param {
    FEAT_PARAM_DSS_FCK,
    FEAT_PARAM_DSS_PCD,
    FEAT_PARAM_DSIPLL_LPDIV,
    FEAT_PARAM_DSI_FCK,
    FEAT_PARAM_DOWNSCALE,
    FEAT_PARAM_LINEWIDTH,
}

// DSS Feature Functions
extern "C" {
    pub fn dss_feat_get_param_min(param: dss_range_param) -> c_ulong;
}
extern "C" {
    pub fn dss_feat_get_param_max(param: dss_range_param) -> c_ulong;
}
extern "C" {
    pub fn dss_feat_get_overlay_caps(plane: omap_plane) -> omap_overlay_caps;
}
extern "C" {
    pub fn dss_feat_rotation_type_supported(rot_type: omap_dss_rotation_type) -> bool;
}
extern "C" {
    pub fn dss_has_feature(id: dss_feat_id) -> bool;
}
extern "C" {
    pub fn dss_feat_get_reg_field(id: dss_feat_reg_field, start: *mut u8, end: *mut u8);
}
extern "C" {
    pub fn dss_features_init(version: omapdss_version);
}
extern "C" {
    pub fn dss_feat_get_supported_displays(channel: omap_channel) -> omap_display_type;
}
extern "C" {
    pub fn dss_feat_get_supported_outputs(channel: omap_channel) -> omap_dss_output_id;
}
