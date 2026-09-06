//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/dml2_0/dml2_mall_phantom.h
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
//
// Copyright 2023 Advanced Micro Devices, Inc.
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
// Authors: AMD
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dml2_svp_helper_select_best_svp_candidate_params {
    pub dml_config: *const dml_display_cfg_st,
    pub mode_support_info: *const dml_mode_support_info_st,
    pub blacklist: c_uint,
    pub candidate_index: *mut c_uint,
}

extern "C" {
    pub fn dml2_helper_calculate_num_ways_for_subvp(ctx: *mut dml2_context, context: *mut dc_state) -> c_uint;
}
extern "C" {
    pub fn dml2_svp_add_phantom_pipe_to_dc_state(ctx: *mut dml2_context, state: *mut dc_state, mode_support_info: *mut dml_mode_support_info_st) -> bool;
}
extern "C" {
    pub fn dml2_svp_remove_all_phantom_pipes(ctx: *mut dml2_context, state: *mut dc_state) -> bool;
}
extern "C" {
    pub fn dml2_svp_validate_static_schedulability(ctx: *mut dml2_context, context: *mut dc_state, pstate_change_type: dml_dram_clock_change_support) -> bool;
}
extern "C" {
    pub fn dml2_svp_drr_schedulable(ctx: *mut dml2_context, context: *mut dc_state, drr_timing: *mut dc_crtc_timing) -> bool;
}
