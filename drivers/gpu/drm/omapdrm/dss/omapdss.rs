//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/omapdrm/dss/omapdss.h
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
// Copyright (C) 2016 Texas Instruments Incorporated - https://www.ti.com
// Author: Tomi Valkeinen <tomi.valkeinen@ti.com>
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum omap_display_type {
    OMAP_DISPLAY_TYPE_NONE		= 0,
    OMAP_DISPLAY_TYPE_DPI		= 1 << 0,
    OMAP_DISPLAY_TYPE_DBI		= 1 << 1,
    OMAP_DISPLAY_TYPE_SDI		= 1 << 2,
    OMAP_DISPLAY_TYPE_DSI		= 1 << 3,
    OMAP_DISPLAY_TYPE_VENC		= 1 << 4,
    OMAP_DISPLAY_TYPE_HDMI		= 1 << 5,
    OMAP_DISPLAY_TYPE_DVI		= 1 << 6,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum omap_plane_id {
    OMAP_DSS_GFX	= 0,
    OMAP_DSS_VIDEO1	= 1,
    OMAP_DSS_VIDEO2	= 2,
    OMAP_DSS_VIDEO3	= 3,
    OMAP_DSS_WB	= 4,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum omap_channel {
    OMAP_DSS_CHANNEL_LCD	= 0,
    OMAP_DSS_CHANNEL_DIGIT	= 1,
    OMAP_DSS_CHANNEL_LCD2	= 2,
    OMAP_DSS_CHANNEL_LCD3	= 3,
    OMAP_DSS_CHANNEL_WB	= 4,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum omap_color_mode {
    _UNUSED_,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum omap_dss_load_mode {
    OMAP_DSS_LOAD_CLUT_AND_FRAME	= 0,
    OMAP_DSS_LOAD_CLUT_ONLY		= 1,
    OMAP_DSS_LOAD_FRAME_ONLY	= 2,
    OMAP_DSS_LOAD_CLUT_ONCE_FRAME	= 3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum omap_dss_trans_key_type {
    OMAP_DSS_COLOR_KEY_GFX_DST = 0,
    OMAP_DSS_COLOR_KEY_VID_SRC = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum omap_dss_signal_level {
    OMAPDSS_SIG_ACTIVE_LOW,
    OMAPDSS_SIG_ACTIVE_HIGH,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum omap_dss_signal_edge {
    OMAPDSS_DRIVE_SIG_FALLING_EDGE,
    OMAPDSS_DRIVE_SIG_RISING_EDGE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum omap_dss_venc_type {
    OMAP_DSS_VENC_TYPE_COMPOSITE,
    OMAP_DSS_VENC_TYPE_SVIDEO,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum omap_dss_rotation_type {
    OMAP_DSS_ROT_NONE	= 0,
    OMAP_DSS_ROT_TILER	= 1 << 0,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum omap_overlay_caps {
    OMAP_DSS_OVL_CAP_SCALE = 1 << 0,
    OMAP_DSS_OVL_CAP_GLOBAL_ALPHA = 1 << 1,
    OMAP_DSS_OVL_CAP_PRE_MULT_ALPHA = 1 << 2,
    OMAP_DSS_OVL_CAP_ZORDER = 1 << 3,
    OMAP_DSS_OVL_CAP_POS = 1 << 4,
    OMAP_DSS_OVL_CAP_REPLICATION = 1 << 5,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum omap_dss_output_id {
    OMAP_DSS_OUTPUT_DPI	= 1 << 0,
    OMAP_DSS_OUTPUT_DBI	= 1 << 1,
    OMAP_DSS_OUTPUT_SDI	= 1 << 2,
    OMAP_DSS_OUTPUT_DSI1	= 1 << 3,
    OMAP_DSS_OUTPUT_DSI2	= 1 << 4,
    OMAP_DSS_OUTPUT_VENC	= 1 << 5,
    OMAP_DSS_OUTPUT_HDMI	= 1 << 6,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap_dss_cpr_coefs {
    pub rb: s16 rr, rg,,
    pub gb: s16 gr, gg,,
    pub bb: s16 br, bg,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap_overlay_info {
    pub paddr: dma_addr_t,
    pub /: *mut *mut dma_addr_t p_uv_addr; / for NV12 format,
    pub screen_width: u16,
    pub width: u16,
    pub height: u16,
    pub fourcc: u32,
    pub rotation: u8,
    pub rotation_type: omap_dss_rotation_type,
    pub pos_x: u16,
    pub pos_y: u16,
    pub /: *mut *mut u16 out_width; / if 0, out_width == width,
    pub /: *mut *mut u16 out_height; / if 0, out_height == height,
    pub global_alpha: u8,
    pub pre_mult_alpha: u8,
    pub zorder: u8,
    pub color_encoding: drm_color_encoding,
    pub color_range: drm_color_range,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap_overlay_manager_info {
    pub default_color: u32,
    pub trans_key_type: omap_dss_trans_key_type,
    pub trans_key: u32,
    pub trans_enabled: bool,
    pub partial_alpha_enabled: bool,
    pub cpr_enable: bool,
    pub cpr_coefs: omap_dss_cpr_coefs,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap_dss_writeback_info {
    pub paddr: u32,
    pub p_uv_addr: u32,
    pub buf_width: u16,
    pub width: u16,
    pub height: u16,
    pub fourcc: u32,
    pub rotation: u8,
    pub rotation_type: omap_dss_rotation_type,
    pub pre_mult_alpha: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct omapdss_dsi_ops {
    pub dssdev): *mut *mut int (update)(struct omap_dss_device,
    pub dssdev): *mut *mut bool (is_video_mode)(struct omap_dss_device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap_dss_device {
    pub dev: *mut device,
    pub dss: *mut dss_device,
    pub bridge: *mut drm_bridge,
    pub next_bridge: *mut drm_bridge,
    pub panel: *mut drm_panel,
    pub list: list_head,
//
// DSS type that this device generates (for DSS internal devices) or
// requires (for external encoders, connectors and panels). Must be a
// non-zero (different than OMAP_DISPLAY_TYPE_NONE) value.
//
    pub type: omap_display_type,
    pub name: *const c_char,
    pub dsi_ops: *const omapdss_dsi_ops,
    pub bus_flags: u32,
// OMAP DSS output specific fields
// DISPC channel for this output
    pub dispc_channel: omap_channel,
// output instance
    pub id: omap_dss_output_id,
// port number in DT
    pub of_port: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dss_pdata {
    pub dss: *mut dss_device,
}

extern "C" {
    pub fn omapdss_device_register(dssdev: *mut omap_dss_device);
}
extern "C" {
    pub fn omapdss_device_unregister(dssdev: *mut omap_dss_device);
}
extern "C" {
    pub fn omapdss_device_put(dssdev: *mut omap_dss_device);
}
extern "C" {
    pub fn omap_dss_get_num_overlay_managers() -> c_int;
}
extern "C" {
    pub fn omap_dss_get_num_overlays() -> c_int;
}

extern "C" {
    pub fn omapdss_device_cleanup_output(out: *mut omap_dss_device);
}
extern "C" {
    pub fn void(arg: *mut *mut omap_dispc_isr_t) (void, mask: u32) -> typedef;
}
extern "C" {
    pub fn omap_dispc_register_isr(isr: omap_dispc_isr_t, arg: *mut c_void, mask: u32) -> c_int;
}
extern "C" {
    pub fn omap_dispc_unregister_isr(isr: omap_dispc_isr_t, arg: *mut c_void, mask: u32) -> c_int;
}
extern "C" {
    pub fn omapdss_compat_init() -> c_int;
}
extern "C" {
    pub fn omapdss_compat_uninit();
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dss_writeback_channel {
    DSS_WB_LCD1_MGR =	0,
    DSS_WB_LCD2_MGR =	1,
    DSS_WB_TV_MGR =		2,
    DSS_WB_OVL0 =		3,
    DSS_WB_OVL1 =		4,
    DSS_WB_OVL2 =		5,
    DSS_WB_OVL3 =		6,
    DSS_WB_LCD3_MGR =	7,
}

extern "C" {
    pub fn omap_crtc_set_enabled(crtc: *mut drm_crtc, enable: bool);
}
extern "C" {
    pub fn omap_crtc_dss_enable(priv: *mut omap_drm_private, channel: omap_channel) -> c_int;
}
extern "C" {
    pub fn omap_crtc_dss_disable(priv: *mut omap_drm_private, channel: omap_channel);
}
extern "C" {
    pub fn dss_mgr_enable(dssdev: *mut omap_dss_device) -> c_int;
}
extern "C" {
    pub fn dss_mgr_disable(dssdev: *mut omap_dss_device);
}
extern "C" {
    pub fn dss_mgr_start_update(dssdev: *mut omap_dss_device);
}
extern "C" {
    pub fn omapdss_stack_is_ready() -> bool;
}
extern "C" {
    pub fn omapdss_gather_components(dev: *mut device);
}
extern "C" {
    pub fn omap_dss_init() -> c_int;
}
extern "C" {
    pub fn omap_dss_exit();
}
