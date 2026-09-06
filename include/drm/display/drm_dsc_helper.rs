//! Automatically rewritten from C Header to Rust Module
//! Source: include/drm/display/drm_dsc_helper.h
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


// SPDX-License-Identifier: MIT
// Copyright (C) 2018 Intel Corp.
//
// Authors:
// Manasi Navare <manasi.d.navare@intel.com>
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drm_dsc_params_type {
    DRM_DSC_1_2_444,
    DRM_DSC_1_1_PRE_SCR, /* legacy params from DSC 1.1 */
    DRM_DSC_1_2_422,
    DRM_DSC_1_2_420,
}

extern "C" {
    pub fn drm_dsc_dp_pps_header_init(pps_header: *mut dp_sdp_header);
}
extern "C" {
    pub fn drm_dsc_dp_rc_buffer_size(rc_buffer_block_size: u8, rc_buffer_size: u8) -> c_int;
}
extern "C" {
    pub fn drm_dsc_set_const_params(vdsc_cfg: *mut drm_dsc_config);
}
extern "C" {
    pub fn drm_dsc_set_rc_buf_thresh(vdsc_cfg: *mut drm_dsc_config);
}
extern "C" {
    pub fn drm_dsc_setup_rc_params(vdsc_cfg: *mut drm_dsc_config, type: drm_dsc_params_type) -> c_int;
}
extern "C" {
    pub fn drm_dsc_compute_rc_parameters(vdsc_cfg: *mut drm_dsc_config) -> c_int;
}
extern "C" {
    pub fn drm_dsc_initial_scale_value(dsc: *const drm_dsc_config) -> u8;
}
extern "C" {
    pub fn drm_dsc_flatness_det_thresh(dsc: *const drm_dsc_config) -> u32;
}
extern "C" {
    pub fn drm_dsc_get_bpp_int(vdsc_cfg: *const drm_dsc_config) -> u32;
}
extern "C" {
    pub fn drm_dsc_dump_config(p: *mut drm_printer, indent: c_int, cfg: *const drm_dsc_config);
}
