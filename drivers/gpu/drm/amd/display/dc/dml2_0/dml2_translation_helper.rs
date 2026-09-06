//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/dml2_0/dml2_translation_helper.h
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
extern "C" {
    pub fn dml2_init_ip_params(dml2: *mut dml2_context, in_dc: *const dc, out: *mut ip_params_st);
}
extern "C" {
    pub fn dml2_init_socbb_params(dml2: *mut dml2_context, in_dc: *const dc, out: *mut soc_bounding_box_st);
}
extern "C" {
    pub fn dml2_translate_ip_params(in_dc: *const dc, out: *mut ip_params_st);
}
extern "C" {
    pub fn dml2_translate_socbb_params(in_dc: *const dc, out: *mut soc_bounding_box_st);
}
extern "C" {
    pub fn dml2_translate_soc_states(in_dc: *const dc, out: *mut soc_states_st, num_states: c_int);
}
extern "C" {
    pub fn map_dc_state_into_dml_display_cfg(dml2: *mut dml2_context, context: *mut dc_state, dml_dispcfg: *mut dml_display_cfg_st);
}
extern "C" {
    pub fn dml2_update_pipe_ctx_dchub_regs(rq_regs: *mut _vcs_dpi_dml_display_rq_regs_st, disp_dlg_regs: *mut _vcs_dpi_dml_display_dlg_regs_st, disp_ttu_regs: *mut _vcs_dpi_dml_display_ttu_regs_st, out: *mut pipe_ctx);
}
extern "C" {
    pub fn is_dp2p0_output_encoder(pipe: *const pipe_ctx) -> bool;
}
