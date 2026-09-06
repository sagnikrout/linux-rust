//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/dc_dsc.h
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


//
// Copyright 2019 Advanced Micro Devices, Inc.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
// THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
// OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
// ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
// OTHER DEALINGS IN THE SOFTWARE.
//
// Author: AMD
//
// put it here temporarily until linux has the new addresses official defined
// DP Extended DSC Capabilities
pub const DP_DSC_BRANCH_OVERALL_THROUGHPUT_0: c_uint = 0x0a0   /* DP 1.4a SCR */;
pub const DP_DSC_BRANCH_OVERALL_THROUGHPUT_1: c_uint = 0x0a1;
pub const DP_DSC_BRANCH_MAX_LINE_WIDTH: c_uint = 0x0a2;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_dsc_bw_range {
    pub /: *mut *mut uint32_t min_kbps; / Bandwidth if min_target_bpp_x16 is used,
    pub min_target_bpp_x16: u32,
    pub /: *mut *mut uint32_t max_kbps; / Bandwidth if max_target_bpp_x16 is used,
    pub max_target_bpp_x16: u32,
    pub /: *mut *mut uint32_t stream_kbps; / Uncompressed stream bandwidth,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct display_stream_compressor {
    pub funcs: *const dsc_funcs,
    pub ctx: *mut dc_context,
    pub inst: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_dsc_policy {
    pub use_min_slices_h: bool,
    pub 0: int max_slices_h; // Maximum available if,
    pub 8: int min_slice_height; // Must not be less than,
    pub max_target_bpp: u32,
    pub min_target_bpp: u32,
    pub enable_dsc_when_not_needed: bool,
    pub ycbcr422_simple: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_dsc_config_options {
    pub dsc_min_slice_height_override: u32,
    pub max_target_bpp_limit_override_x16: u32,
    pub slice_height_granularity: u32,
    pub dsc_force_odm_hslice_override: u32,
    pub force_dsc_when_not_needed: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_dsc_primary_bpp {
    pub vic: u32,
    pub target_bpp: u32,
}

// TODO - Hardware/specs limitation should be owned by dc dsc and returned to DM,
// and DM can choose to OVERRIDE the limitation on CASE BY CASE basis.
// Hardware/specs limitation should not be writable by DM.
// It should be decoupled from DM specific policy and named differently.
//
extern "C" {
    pub fn dc_dsc_policy_set_max_target_bpp_limit(limit: u32);
}
extern "C" {
    pub fn dc_dsc_policy_set_enable_dsc_when_not_needed(enable: bool);
}
extern "C" {
    pub fn dc_dsc_policy_set_disable_dsc_stream_overhead(disable: bool);
}
extern "C" {
    pub fn dc_dsc_get_default_config_option(dc: *const dc, options: *mut dc_dsc_config_options);
}
