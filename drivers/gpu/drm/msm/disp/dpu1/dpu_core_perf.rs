//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/msm/disp/dpu1/dpu_core_perf.h
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
// Copyright (c) 2016-2018, The Linux Foundation. All rights reserved.
//

//
// struct dpu_core_perf_params - definition of performance parameters
// @max_per_pipe_ib: maximum instantaneous bandwidth request
// @bw_ctl: arbitrated bandwidth request
// @core_clk_rate: core clock rate request
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpu_core_perf_params {
    pub max_per_pipe_ib: u32,
    pub bw_ctl: u64,
    pub core_clk_rate: u64,
}

//
// struct dpu_core_perf_tune - definition of performance tuning control
// @mode: performance mode
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpu_core_perf_tune {
    pub mode: u32,
}

//
// struct dpu_core_perf - definition of core performance context
// @perf_cfg: Platform-specific performance configuration
// @core_clk_rate: current core clock rate
// @max_core_clk_rate: maximum allowable core clock rate
// @perf_tune: debug control for performance tuning
// @enable_bw_release: debug control for bandwidth release
// @fix_core_clk_rate: fixed core clock request in Hz used in mode 2
// @fix_core_ib_vote: fixed core ib vote in KBps used in mode 2
// @fix_core_ab_vote: fixed core ab vote in KBps used in mode 2
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpu_core_perf {
    pub perf_cfg: *const dpu_perf_cfg,
    pub core_clk_rate: u64,
    pub max_core_clk_rate: u64,
    pub perf_tune: dpu_core_perf_tune,
    pub enable_bw_release: u32,
    pub fix_core_clk_rate: u64,
    pub fix_core_ib_vote: u32,
    pub fix_core_ab_vote: u32,
}

extern "C" {
    pub fn dpu_core_perf_crtc_release_bw(crtc: *mut drm_crtc);
}
extern "C" {
    pub fn dpu_core_perf_debugfs_init(dpu_kms: *mut dpu_kms, parent: *mut dentry) -> c_int;
}
