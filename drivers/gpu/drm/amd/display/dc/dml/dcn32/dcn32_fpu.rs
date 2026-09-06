//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/dml/dcn32/dcn32_fpu.h
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
// Copyright 2022 Advanced Micro Devices, Inc.
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

extern "C" {
    pub fn dcn32_build_wm_range_table_fpu(clk_mgr: *mut clk_mgr_internal);
}
extern "C" {
    pub fn dcn32_update_bw_bounding_box_fpu(dc: *mut dc, bw_params: *mut clk_bw_params);
}
extern "C" {
    pub fn dcn32_patch_dpm_table(bw_params: *mut clk_bw_params);
}
extern "C" {
    pub fn dcn32_assign_fpo_vactive_candidate(dc: *mut dc, context: *const dc_state, fpo_candidate_stream: *mut dc_stream_state);
}
extern "C" {
    pub fn dcn32_find_vactive_pipe(dc: *mut dc, context: *const dc_state, fpo_candidate_stream: *mut dc_stream_state, vactive_margin_req: u32) -> bool;
}
extern "C" {
    pub fn dcn32_override_min_req_memclk(dc: *mut dc, context: *mut dc_state);
}
extern "C" {
    pub fn dcn32_set_clock_limits(soc_bb: *const _vcs_dpi_soc_bounding_box_st);
}
extern "C" {
    pub fn dcn32_get_max_dispclk_mhz(dc: *mut dc, context: *mut dc_state) -> c_uint;
}
