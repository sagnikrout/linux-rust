//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xlnx/zynqmp_disp.h
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
// ZynqMP Display Driver
//
// Copyright (C) 2017 - 2020 Xilinx, Inc.
//
// Authors:
// - Hyun Woo Kwon <hyun.kwon@xilinx.com>
// - Laurent Pinchart <laurent.pinchart@ideasonboard.com>
//

//
// 3840x2160 is advertised as the maximum resolution, but almost any
// resolutions under a 300Mhz pixel rate would work. Pick 4096x4096.
//
pub const ZYNQMP_DISP_MAX_WIDTH: c_int = 4096;
pub const ZYNQMP_DISP_MAX_HEIGHT: c_int = 4096;
// The DPDMA is limited to 44 bit addressing.
pub const ZYNQMP_DISP_MAX_DMA_BIT: c_int = 44;
//
// enum zynqmp_dpsub_layer_id - Layer identifier
// @ZYNQMP_DPSUB_LAYER_VID: Video layer
// @ZYNQMP_DPSUB_LAYER_GFX: Graphics layer
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum zynqmp_dpsub_layer_id {
    ZYNQMP_DPSUB_LAYER_VID,
    ZYNQMP_DPSUB_LAYER_GFX,
}

extern "C" {
    pub fn zynqmp_disp_enable(disp: *mut zynqmp_disp);
}
extern "C" {
    pub fn zynqmp_disp_disable(disp: *mut zynqmp_disp);
}
extern "C" {
    pub fn zynqmp_disp_layer_enable(layer: *mut zynqmp_disp_layer);
}
extern "C" {
    pub fn zynqmp_disp_layer_disable(layer: *mut zynqmp_disp_layer);
}
extern "C" {
    pub fn zynqmp_disp_probe(dpsub: *mut zynqmp_dpsub) -> c_int;
}
extern "C" {
    pub fn zynqmp_disp_remove(dpsub: *mut zynqmp_dpsub);
}
