//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/msm/disp/dpu1/dpu_vbif.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpu_vbif_set_ot_params {
    pub xin_id: u32,
    pub num: u32,
    pub width: u32,
    pub height: u32,
    pub frame_rate: u32,
    pub rd: bool,
    pub is_wfd: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpu_vbif_set_memtype_params {
    pub xin_id: u32,
    pub is_cacheable: bool,
}

//
// struct dpu_vbif_set_qos_params - QoS remapper parameter
// @xin_id: client interface identifier
// @num: pipe identifier (debug only)
// @is_rt: true if pipe is used in real-time use case
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpu_vbif_set_qos_params {
    pub xin_id: u32,
    pub num: u32,
    pub is_rt: bool,
}

extern "C" {
    pub fn dpu_vbif_clear_errors(dpu_kms: *mut dpu_kms);
}
extern "C" {
    pub fn dpu_vbif_init_memtypes(dpu_kms: *mut dpu_kms);
}
extern "C" {
    pub fn dpu_debugfs_vbif_init(dpu_kms: *mut dpu_kms, debugfs_root: *mut dentry);
}
