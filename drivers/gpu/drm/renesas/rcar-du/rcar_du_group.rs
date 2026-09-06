//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/renesas/rcar-du/rcar_du_group.h
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
// R-Car Display Unit Planes and CRTCs Group
//
// Copyright (C) 2013-2014 Renesas Electronics Corporation
//
// Contact: Laurent Pinchart (laurent.pinchart@ideasonboard.com)
//

//
// struct rcar_du_group - CRTCs and planes group
// @dev: the DU device
// @mmio_offset: registers offset in the device memory map
// @index: group index
// @channels_mask: bitmask of populated DU channels in this group
// @cmms_mask: bitmask of available CMMs in this group
// @num_crtcs: number of CRTCs in this group (1 or 2)
// @use_count: number of users of the group (rcar_du_group_(get|put))
// @used_crtcs: number of CRTCs currently in use
// @lock: protects the dptsr_planes field and the DPTSR register
// @dptsr_planes: bitmask of planes driven by dot-clock and timing generator 1
// @num_planes: number of planes in the group
// @planes: planes handled by the group
// @need_restart: the group needs to be restarted due to a configuration change
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rcar_du_group {
    pub dev: *mut rcar_du_device,
    pub mmio_offset: c_uint,
    pub index: c_uint,
    pub channels_mask: c_uint,
    pub cmms_mask: c_uint,
    pub num_crtcs: c_uint,
    pub use_count: c_uint,
    pub used_crtcs: c_uint,
    pub lock: mutex,
    pub dptsr_planes: c_uint,
    pub num_planes: c_uint,
    pub planes: [rcar_du_plane; RCAR_DU_NUM_KMS_PLANES],
    pub need_restart: bool,
}

extern "C" {
    pub fn rcar_du_group_read(rgrp: *mut rcar_du_group, reg: u32) -> u32;
}
extern "C" {
    pub fn rcar_du_group_write(rgrp: *mut rcar_du_group, reg: u32, data: u32);
}
extern "C" {
    pub fn rcar_du_group_get(rgrp: *mut rcar_du_group) -> c_int;
}
extern "C" {
    pub fn rcar_du_group_put(rgrp: *mut rcar_du_group);
}
extern "C" {
    pub fn rcar_du_group_start_stop(rgrp: *mut rcar_du_group, start: bool);
}
extern "C" {
    pub fn rcar_du_group_restart(rgrp: *mut rcar_du_group);
}
extern "C" {
    pub fn rcar_du_group_set_routing(rgrp: *mut rcar_du_group) -> c_int;
}
extern "C" {
    pub fn rcar_du_set_dpad0_vsp1_routing(rcdu: *mut rcar_du_device) -> c_int;
}
