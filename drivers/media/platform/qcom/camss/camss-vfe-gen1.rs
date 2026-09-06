//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/qcom/camss/camss-vfe-gen1.h
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
// camss-vfe.h
//
// Qualcomm MSM Camera Subsystem - VFE (Video Front End) Module
//
// Copyright (c) 2013-2015, The Linux Foundation. All rights reserved.
// Copyright (C) 2015-2018 Linaro Ltd.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vfe_hw_ops_gen1 {
    pub id): *mut *mut *mut void (bus_connect_wm_to_rdi)(struct vfe_device vfe, u8 wm, enum vfe_line_id,
    pub id): *mut *mut *mut void (bus_disconnect_wm_from_rdi)(struct vfe_device vfe, u8 wm, enum vfe_line_id,
    pub enable): *mut *mut *mut void (bus_enable_wr_if)(struct vfe_device vfe, u8,
    pub wm): *mut *mut *mut void (bus_reload_wm)(struct vfe_device vfe, u8,
    pub dev): *mut *mut *mut int (camif_wait_for_stop)(struct vfe_device vfe, struct device,
    pub vfe): *mut *mut void (enable_irq_common)(struct vfe_device,
    pub enable): u8,
    pub enable): u8,
    pub vfe_id): *mut *mut u16 (get_ub_size)(u8,
    pub vfe): *mut *mut void (halt_clear)(struct vfe_device,
    pub vfe): *mut *mut void (halt_request)(struct vfe_device,
    pub line): *mut *mut *mut void (set_camif_cfg)(struct vfe_device vfe, struct vfe_line,
    pub enable): *mut *mut *mut void (set_camif_cmd)(struct vfe_device vfe, u8,
    pub enable): *mut *mut *mut void (set_cgc_override)(struct vfe_device vfe, u8 wm, u8,
    pub vfe): *mut *mut void (set_clamp_cfg)(struct vfe_device,
    pub line): *mut *mut *mut void (set_crop_cfg)(struct vfe_device vfe, struct vfe_line,
    pub line): *mut *mut *mut void (set_demux_cfg)(struct vfe_device vfe, struct vfe_line,
    pub vfe): *mut *mut void (set_ds)(struct vfe_device,
    pub enable): *mut *mut *mut void (set_module_cfg)(struct vfe_device vfe, u8,
    pub line): *mut *mut *mut void (set_scale_cfg)(struct vfe_device vfe, struct vfe_line,
    pub cid): *mut *mut *mut void (set_rdi_cid)(struct vfe_device vfe, enum vfe_line_id id, u8,
    pub enable): *mut *mut *mut *mut void (set_realign_cfg)(struct vfe_device vfe, struct vfe_line line, u8,
    pub vfe): *mut *mut void (set_qos)(struct vfe_device,
    pub enable): *mut *mut *mut *mut void (set_xbar_cfg)(struct vfe_device vfe, struct vfe_output output, u8,
    pub enable): *mut *mut *mut void (wm_frame_based)(struct vfe_device vfe, u8 wm, u8,
    pub enable): u8 plane, u32,
    pub depth): *mut *mut *mut void (wm_set_ub_cfg)(struct vfe_device vfe, u8 wm, u16 offset, u16,
    pub wm): *mut *mut *mut void (wm_set_subsample)(struct vfe_device vfe, u8,
    pub per): *mut *mut *mut void (wm_set_framedrop_period)(struct vfe_device vfe, u8 wm, u8,
    pub pattern): *mut *mut *mut void (wm_set_framedrop_pattern)(struct vfe_device vfe, u8 wm, u32,
    pub addr): *mut *mut *mut void (wm_set_ping_addr)(struct vfe_device vfe, u8 wm, u32,
    pub addr): *mut *mut *mut void (wm_set_pong_addr)(struct vfe_device vfe, u8 wm, u32,
    pub wm): *mut *mut *mut int (wm_get_ping_pong_status)(struct vfe_device vfe, u8,
    pub enable): *mut *mut *mut void (wm_enable)(struct vfe_device vfe, u8 wm, u8,
}

//
// vfe_calc_interp_reso - Calculate interpolation mode
// @input: Input resolution
// @output: Output resolution
//
// Return interpolation mode
//
// vfe_gen1_disable - Disable streaming on VFE line
// @line: VFE line
//
// Return 0 on success or a negative error code otherwise
//
extern "C" {
    pub fn vfe_gen1_disable(line: *mut vfe_line) -> c_int;
}
//
// vfe_gen1_enable - Enable VFE module
// @line: VFE line
//
// Return 0 on success
//
extern "C" {
    pub fn vfe_gen1_enable(line: *mut vfe_line) -> c_int;
}
//
// vfe_gen1_enable - Halt VFE module
// @vfe: VFE device
//
// Return 0 on success
//
extern "C" {
    pub fn vfe_gen1_halt(vfe: *mut vfe_device) -> c_int;
}
//
// vfe_word_per_line - Calculate number of words per frame width
// @format: V4L2 format
// @width: Frame width
//
// Return number of words per frame width
//
extern "C" {
    pub fn vfe_word_per_line(format: u32, width: u32) -> c_int;
}
