//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xlnx/zynqmp_dpsub.h
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
// ZynqMP DPSUB Subsystem Driver
//
// Copyright (C) 2017 - 2020 Xilinx, Inc.
//
// Authors:
// - Hyun Woo Kwon <hyun.kwon@xilinx.com>
// - Laurent Pinchart <laurent.pinchart@ideasonboard.com>
//

pub const ZYNQMP_DPSUB_NUM_LAYERS: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum zynqmp_dpsub_port {
    ZYNQMP_DPSUB_PORT_LIVE_VIDEO,
    ZYNQMP_DPSUB_PORT_LIVE_GFX,
    ZYNQMP_DPSUB_PORT_LIVE_AUDIO,
    ZYNQMP_DPSUB_PORT_OUT_VIDEO,
    ZYNQMP_DPSUB_PORT_OUT_AUDIO,
    ZYNQMP_DPSUB_PORT_OUT_DP,
    ZYNQMP_DPSUB_NUM_PORTS,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum zynqmp_dpsub_format {
    ZYNQMP_DPSUB_FORMAT_RGB,
    ZYNQMP_DPSUB_FORMAT_YCRCB444,
    ZYNQMP_DPSUB_FORMAT_YCRCB422,
    ZYNQMP_DPSUB_FORMAT_YONLY,
}

//
// struct zynqmp_dpsub - ZynqMP DisplayPort Subsystem
// @dev: The physical device
// @apb_clk: The APB clock
// @vid_clk: Video clock
// @vid_clk_from_ps: True of the video clock comes from PS, false from PL
// @aud_clk: Audio clock
// @aud_clk_from_ps: True of the audio clock comes from PS, false from PL
// @connected_ports: Bitmask of connected ports in the device tree
// @dma_enabled: True if the DMA interface is enabled, false if the DPSUB is
// driven by the live input
// @drm: The DRM/KMS device data
// @bridge: The DP encoder bridge
// @disp: The display controller
// @layers: Video and graphics layers
// @dp: The DisplayPort controller
// @dma_align: DMA alignment constraint (must be a power of 2)
// @audio: DP audio data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zynqmp_dpsub {
    pub dev: *mut device,
    pub apb_clk: *mut clk,
    pub vid_clk: *mut clk,
    pub vid_clk_from_ps: bool,
    pub aud_clk: *mut clk,
    pub aud_clk_from_ps: bool,
    pub connected_ports: c_uint,
    pub dma_enabled: bool,
    pub drm: *mut zynqmp_dpsub_drm,
    pub bridge: *mut drm_bridge,
    pub disp: *mut zynqmp_disp,
    pub layers: [*mut zynqmp_disp_layer; ZYNQMP_DPSUB_NUM_LAYERS],
    pub dp: *mut zynqmp_dp,
    pub dma_align: c_uint,
    pub audio: *mut zynqmp_dpsub_audio,
}

extern "C" {
    pub fn zynqmp_audio_init(dpsub: *mut zynqmp_dpsub) -> c_int;
}
extern "C" {
    pub fn zynqmp_audio_uninit(dpsub: *mut zynqmp_dpsub);
}

extern "C" {
    pub fn zynqmp_dpsub_release(dpsub: *mut zynqmp_dpsub);
}
