//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/renesas/vsp1/vsp1.h
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
// vsp1.h  --  R-Car VSP1 Driver
//
// Copyright (C) 2013-2014 Renesas Electronics Corporation
//
// Contact: Laurent Pinchart (laurent.pinchart@ideasonboard.com)
//

pub const VSP1_MAX_LIF: c_int = 2;
pub const VSP1_MAX_RPF: c_int = 5;
pub const VSP1_MAX_UDS: c_int = 3;
pub const VSP1_MAX_UIF: c_int = 2;
pub const VSP1_MAX_WPF: c_int = 4;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vsp1_device_info {
    pub version: u32,
    pub model: *const c_char,
    pub gen: c_uint,
    pub features: c_uint,
    pub lif_count: c_uint,
    pub rpf_count: c_uint,
    pub uds_count: c_uint,
    pub uif_count: c_uint,
    pub wpf_count: c_uint,
    pub num_bru_inputs: c_uint,
    pub soc: u8,
    pub uapi: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vsp1_device {
    pub dev: *mut device,
    pub info: *const vsp1_device_info,
    pub version: u32,
    pub mmio: *mut void __iomem,
    pub fcp: *mut rcar_fcp_device,
    pub bus_master: *mut device,
    pub rstc: *mut reset_control,
    pub brs: *mut vsp1_brx,
    pub bru: *mut vsp1_brx,
    pub clu: *mut vsp1_clu,
    pub hgo: *mut vsp1_hgo,
    pub hgt: *mut vsp1_hgt,
    pub hsi: *mut vsp1_hsit,
    pub hst: *mut vsp1_hsit,
    pub iif: *mut vsp1_iif,
    pub lif: [*mut vsp1_lif; VSP1_MAX_LIF],
    pub lut: *mut vsp1_lut,
    pub rpf: [*mut vsp1_rwpf; VSP1_MAX_RPF],
    pub sru: *mut vsp1_sru,
    pub uds: [*mut vsp1_uds; VSP1_MAX_UDS],
    pub uif: [*mut vsp1_uif; VSP1_MAX_UIF],
    pub wpf: [*mut vsp1_rwpf; VSP1_MAX_WPF],
    pub entities: list_head,
    pub videos: list_head,
    pub v4l2_dev: v4l2_device,
    pub media_dev: media_device,
    pub media_ops: media_entity_operations,
    pub drm: *mut vsp1_drm,
    pub vspx: *mut vsp1_vspx,
}

extern "C" {
    pub fn vsp1_device_get(vsp1: *mut vsp1_device) -> c_int;
}
extern "C" {
    pub fn vsp1_device_put(vsp1: *mut vsp1_device);
}
extern "C" {
    pub fn vsp1_reset_wpf(vsp1: *mut vsp1_device, index: c_uint) -> c_int;
}
extern "C" {
    pub fn ioread32(reg: vsp1->mmio +) -> return;
}
