//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/renesas/rcar-du/rcar_du_drv.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// R-Car Display Unit DRM driver
//
// Copyright (C) 2013-2015 Renesas Electronics Corporation
//
// Contact: Laurent Pinchart (laurent.pinchart@ideasonboard.com)
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rcar_du_output {
    RCAR_DU_OUTPUT_DPAD0,
    RCAR_DU_OUTPUT_DPAD1,
    RCAR_DU_OUTPUT_DSI0,
    RCAR_DU_OUTPUT_DSI1,
    RCAR_DU_OUTPUT_HDMI0,
    RCAR_DU_OUTPUT_HDMI1,
    RCAR_DU_OUTPUT_LVDS0,
    RCAR_DU_OUTPUT_LVDS1,
    RCAR_DU_OUTPUT_TCON,
    RCAR_DU_OUTPUT_MAX,
}

//
// struct rcar_du_output_routing - Output routing specification
// @possible_crtcs: bitmask of possible CRTCs for the output
// @port: device tree port number corresponding to this output route
//
// The DU has 5 possible outputs (DPAD0/1, LVDS0/1, TCON). Output routing data
// specify the valid SoC outputs, which CRTCs can drive the output, and the type
// of in-SoC encoder for the output.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rcar_du_output_routing {
    pub possible_crtcs: c_uint,
    pub port: c_uint,
}

//
// struct rcar_du_device_info - DU model-specific information
// @gen: device generation (2 or 3)
// @features: device features (RCAR_DU_FEATURE_*)
// @quirks: device quirks (RCAR_DU_QUIRK_*)
// @channels_mask: bit mask of available DU channels
// @routes: array of CRTC to output routes, indexed by output (RCAR_DU_OUTPUT_*)
// @num_lvds: number of internal LVDS encoders
// @num_rpf: number of RPFs in VSP
// @dpll_mask: bit mask of DU channels equipped with a DPLL
// @dsi_clk_mask: bitmask of channels that can use the DSI clock as dot clock
// @lvds_clk_mask: bitmask of channels that can use the LVDS clock as dot clock
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rcar_du_device_info {
    pub gen: c_uint,
    pub features: c_uint,
    pub quirks: c_uint,
    pub channels_mask: c_uint,
    pub routes: [rcar_du_output_routing; RCAR_DU_OUTPUT_MAX],
    pub num_lvds: c_uint,
    pub num_rpf: c_uint,
    pub dpll_mask: c_uint,
    pub dsi_clk_mask: c_uint,
    pub lvds_clk_mask: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rcar_du_cmm {
    pub dev: *mut device,
    pub link: *mut device_link,
}

pub const RCAR_DU_MAX_CRTCS: c_int = 4;

pub const RCAR_DU_MAX_VSPS: c_int = 4;
pub const RCAR_DU_MAX_LVDS: c_int = 2;
pub const RCAR_DU_MAX_DSI: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rcar_du_device {
    pub dev: *mut device,
    pub info: *const rcar_du_device_info,
    pub mmio: *mut void __iomem,
    pub ddev: drm_device,
    pub crtcs: [rcar_du_crtc; RCAR_DU_MAX_CRTCS],
    pub num_crtcs: c_uint,
    pub groups: [rcar_du_group; RCAR_DU_MAX_GROUPS],
    pub cmms: [rcar_du_cmm; RCAR_DU_MAX_CRTCS],
    pub vsps: [rcar_du_vsp; RCAR_DU_MAX_VSPS],
    pub lvds: [*mut drm_bridge; RCAR_DU_MAX_LVDS],
    pub dsi: [*mut drm_bridge; RCAR_DU_MAX_DSI],
    pub colorkey: *mut drm_property,
    pub props: },
    pub dpad0_source: c_uint,
    pub dpad1_source: c_uint,
    pub vspd1_sink: c_uint,
}

extern "C" {
    pub fn container_of(_arg: dev, rcar_du_device: struct, _arg: ddev) -> return;
}
extern "C" {
    pub fn ioread32(reg: rcdu->mmio +) -> return;
}
