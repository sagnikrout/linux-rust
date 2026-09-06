//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/msm/disp/dpu1/dpu_kms.h
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
// Copyright (c) 2015-2018, The Linux Foundation. All rights reserved.
// Copyright (C) 2013 Red Hat
// Author: Rob Clark <robdclark@gmail.com>
//

//
// DPU_DEBUG - macro for kms/plane/crtc/encoder/connector logs
// @fmt: Pointer to format string
//

//
// DPU_DEBUG_DRIVER - macro for hardware driver logging
// @fmt: Pointer to format string
//

//
// ktime_compare_safe - compare two ktime structures
// This macro is similar to the standard ktime_compare() function, but
// attempts to also handle ktime overflows.
// @A: First ktime value
// @B: Second ktime value
// Returns: -1 if A < B, 0 if A == B, 1 if A > B
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpu_kms {
    pub base: msm_kms,
    pub dev: *mut drm_device,
    pub catalog: *const dpu_mdss_cfg,
    pub mdss: *const qcom_ubwc_cfg_data,
// io/register spaces:
    pub vbif: *mut *mut void __iomem mmio,,
    pub vdd: *mut regulator,
    pub mmagic: *mut regulator,
    pub venus: *mut regulator,
    pub hw_intr: *mut dpu_hw_intr,
    pub perf: dpu_core_perf,
//
// Global private object state, Do not access directly, use
// dpu_kms_global_get_state()
//
    pub global_state: drm_private_obj,
    pub rm: dpu_rm,
    pub hw_vbif: *mut dpu_hw_vbif,
    pub hw_mdp: *mut dpu_hw_mdp,
    pub has_danger_ctrl: bool,
    pub pdev: *mut platform_device,
    pub rpm_enabled: bool,
    pub clocks: *mut clk_bulk_data,
    pub num_clocks: usize,
// reference count bandwidth requests, so we know when we can
// release bandwidth.  Each atomic update increments, and frame-
// done event decrements.  Additionally, for video mode, the
// reference is incremented when crtc is enabled, and decremented
// when disabled.
//
    pub bandwidth_ref: core::sync::atomic::AtomicI32,
    pub path: [*mut icc_path; 2],
    pub num_paths: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vsync_info {
    pub frame_count: u32,
    pub line_count: u32,
}

pub const DPU_ENC_WR_PTR_START_TIMEOUT_US: c_int = 20000;
pub const DPU_ENC_MAX_POLL_TIMEOUT_US: c_int = 2000;

// Global private object state for tracking resources that are shared across
// multiple kms objects (planes/crtcs/etc).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpu_global_state {
    pub base: drm_private_state,
    pub rm: *mut dpu_rm,
    pub PINGPONG_0]: uint32_t pingpong_to_crtc_id[PINGPONG_MAX -,
    pub LM_0]: uint32_t mixer_to_crtc_id[LM_MAX -,
    pub CTL_0]: uint32_t ctl_to_crtc_id[CTL_MAX -,
    pub DSPP_0]: uint32_t dspp_to_crtc_id[DSPP_MAX -,
    pub DSC_0]: uint32_t dsc_to_crtc_id[DSC_MAX -,
    pub cdm_to_crtc_id: u32,
    pub SSPP_NONE]: uint32_t sspp_to_crtc_id[SSPP_MAX -,
    pub CWB_0]: uint32_t cwb_to_crtc_id[CWB_MAX -,
}

// dpu_kms_get_existing_global_state(struct dpu_kms *dpu_kms);
// __must_check dpu_kms_get_global_state(struct drm_atomic_commit *s);
//
// Debugfs functions - extra helper functions for debugfs support
//
// Main debugfs documentation is located at,
//
// Documentation/filesystems/debugfs.rst
//
// @dpu_debugfs_create_regset32: Create 32-bit register dump file
//
// DPU info management functions
// These functions/definitions allow for building up a 'dpu_info' structure
// containing one or more "key=value\n" entries.
//
pub const DPU_KMS_INFO_MAX_SIZE: c_int = 4096;
//
// Vblank enable/disable functions
//
extern "C" {
    pub fn dpu_enable_vblank(kms: *mut msm_kms, crtc: *mut drm_crtc) -> c_int;
}
extern "C" {
    pub fn dpu_disable_vblank(kms: *mut msm_kms, crtc: *mut drm_crtc);
}
extern "C" {
    pub fn dpu_kms_get_clk_rate(dpu_kms: *mut dpu_kms, clock_name: *mut c_char) -> c_ulong;
}
