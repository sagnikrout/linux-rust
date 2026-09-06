//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/renesas/rz-du/rzg2l_du_drv.h
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
// RZ/G2L Display Unit DRM driver
//
// Copyright (C) 2023 Renesas Electronics Corporation
//
// Based on rcar_du_drv.h
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rzg2l_du_output {
    RZG2L_DU_OUTPUT_DSI0,
    RZG2L_DU_OUTPUT_DPAD0,
    RZG2L_DU_OUTPUT_MAX,
}

//
// struct rzg2l_du_output_routing - Output routing specification
// @possible_outputs: bitmask of possible outputs
// @port: device tree port number corresponding to this output route
//
// The DU has 2 possible outputs (DPAD0, DSI0). Output routing data
// specify the valid SoC outputs, which CRTC can drive the output, and the type
// of in-SoC encoder for the output.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rzg2l_du_output_routing {
    pub possible_outputs: c_uint,
    pub port: c_uint,
}

//
// struct rzg2l_du_device_info - DU model-specific information
// @channels_mask: bit mask of available DU channels
// @routes: array of CRTC to output routes, indexed by output (RZG2L_DU_OUTPUT_*)
// @mode_clock_min: minimum pixel clock in kHz
// @mode_clock_max: maximum pixel clock in kHz
// @features: device features (RZG2L_DU_FEATURE_*)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rzg2l_du_device_info {
    pub channels_mask: c_uint,
    pub routes: [rzg2l_du_output_routing; RZG2L_DU_OUTPUT_MAX],
    pub mode_clock_min: u32,
    pub mode_clock_max: u32,
    pub features: c_uint,
}

pub const RZG2L_DU_MAX_CRTCS: c_int = 1;
pub const RZG2L_DU_MAX_VSPS: c_int = 1;
pub const RZG2L_DU_MAX_DSI: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rzg2l_du_device {
    pub dev: *mut device,
    pub info: *const rzg2l_du_device_info,
    pub mmio: *mut void __iomem,
    pub ddev: drm_device,
    pub crtcs: [rzg2l_du_crtc; RZG2L_DU_MAX_CRTCS],
    pub num_crtcs: c_uint,
    pub vsps: [rzg2l_du_vsp; RZG2L_DU_MAX_VSPS],
}

extern "C" {
    pub fn container_of(_arg: dev, rzg2l_du_device: struct, _arg: ddev) -> return;
}
