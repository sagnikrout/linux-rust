//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/qcom/venus/hfi_plat_bufs.h
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
// Copyright (c) 2020, The Linux Foundation. All rights reserved.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_plat_buffers_params {
    pub width: u32,
    pub height: u32,
    pub out_width: u32,
    pub out_height: u32,
    pub codec: u32,
    pub hfi_color_fmt: u32,
    pub hfi_dpb_color_fmt: u32,
    pub version: hfi_version,
    pub num_vpp_pipes: u32,
    pub max_mbs_per_frame: u32,
    pub buffer_size_limit: u32,
    pub is_secondary_output: bool,
    pub is_interlaced: bool,
    pub dec: },
    pub work_mode: u32,
    pub rc_type: u32,
    pub num_b_frames: u32,
    pub is_tenbit: bool,
    pub enc: },
}
