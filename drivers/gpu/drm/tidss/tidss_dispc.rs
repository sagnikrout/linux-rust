//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/tidss/tidss_dispc.h
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
// Copyright (C) 2018 Texas Instruments Incorporated - https://www.ti.com
// Author: Tomi Valkeinen <tomi.valkeinen@ti.com>
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tidss_gamma_type {

    struct tidss_vp_feat {
    struct tidss_vp_color_feat {
    u32 gamma_size;
    enum tidss_gamma_type gamma_type;
    bool has_ctm;
    } color;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tidss_plane_feat {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tidss_plane_color_feat {
    pub encodings: u32,
    pub ranges: u32,
    pub default_encoding: drm_color_encoding,
    pub default_range: drm_color_range,
    pub color: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tidss_plane_blend_feat {
    pub global_alpha: bool,
    pub blend: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dispc_features_scaling {
    pub in_width_max_5tap_rgb: u32,
    pub in_width_max_3tap_rgb: u32,
    pub in_width_max_5tap_yuv: u32,
    pub in_width_max_3tap_yuv: u32,
    pub upscale_limit: u32,
    pub downscale_limit_5tap: u32,
    pub downscale_limit_3tap: u32,
    pub xinc_max: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dispc_vid_info {
    pub /: *const *const *const char name; / Should match dt reg names,
    pub hw_id: u32,
    pub is_lite: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dispc_errata {
    pub /: *mut *mut bool i2000; / DSS Does Not Support YUV Pixel Data Formats,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dispc_vp_bus_type {
    DISPC_VP_DPI,		/* DPI output */
    DISPC_VP_OLDI_AM65X,	/* OLDI (LVDS) output for AM65x DSS */
    DISPC_VP_INTERNAL,	/* SoC internal routing */
    DISPC_VP_TIED_OFF,	/* Tied off / Unavailable */
    DISPC_VP_MAX_BUS_TYPE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dispc_dss_subrevision {
    DISPC_K2G,
    DISPC_AM625,
    DISPC_AM62L,
    DISPC_AM62A7,
    DISPC_AM65X,
    DISPC_J721E,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dispc_features {
    pub scaling: dispc_features_scaling,
    pub subrev: dispc_dss_subrevision,
    pub common: *const c_char,
    pub common_regs: *const u16,
    pub num_vps: u32,
    pub /: *const *const *const char vp_name[TIDSS_MAX_PORTS]; / Should match dt reg names,
    pub /: *const *const *const char ovr_name[TIDSS_MAX_PORTS]; / Should match dt reg names,
    pub /: *const *const *const char vpclk_name[TIDSS_MAX_PORTS]; / Should match dt clk names,
    pub vp_bus_type: [dispc_vp_bus_type; TIDSS_MAX_PORTS],
    pub vp_feat: tidss_vp_feat,
    pub num_vids: u32,
    pub vid_info: [dispc_vid_info; TIDSS_MAX_PLANES],
    pub vid_order: [u32; TIDSS_MAX_PLANES],
}

extern "C" {
    pub fn tidss_disable_oldi(tidss: *mut tidss_device, hw_videoport: u32);
}
extern "C" {
    pub fn dispc_pclk_diff(rate: c_ulong, real_rate: c_ulong) -> c_uint;
}
extern "C" {
    pub fn dispc_set_irqenable(dispc: *mut dispc_device, mask: dispc_irq_t);
}
extern "C" {
    pub fn dispc_read_and_clear_irqstatus(dispc: *mut dispc_device) -> dispc_irq_t;
}
extern "C" {
    pub fn dispc_vp_enable(dispc: *mut dispc_device, hw_videoport: u32);
}
extern "C" {
    pub fn dispc_vp_disable(dispc: *mut dispc_device, hw_videoport: u32);
}
extern "C" {
    pub fn dispc_vp_unprepare(dispc: *mut dispc_device, hw_videoport: u32);
}
extern "C" {
    pub fn dispc_vp_go_busy(dispc: *mut dispc_device, hw_videoport: u32) -> bool;
}
extern "C" {
    pub fn dispc_vp_go(dispc: *mut dispc_device, hw_videoport: u32);
}
extern "C" {
    pub fn dispc_vp_enable_clk(dispc: *mut dispc_device, hw_videoport: u32) -> c_int;
}
extern "C" {
    pub fn dispc_vp_disable_clk(dispc: *mut dispc_device, hw_videoport: u32);
}
extern "C" {
    pub fn dispc_runtime_suspend(dispc: *mut dispc_device) -> c_int;
}
extern "C" {
    pub fn dispc_runtime_resume(dispc: *mut dispc_device) -> c_int;
}
extern "C" {
    pub fn dispc_plane_enable(dispc: *mut dispc_device, hw_plane: u32, enable: bool);
}
extern "C" {
    pub fn dispc_init(tidss: *mut tidss_device) -> c_int;
}
extern "C" {
    pub fn dispc_remove(tidss: *mut tidss_device);
}
