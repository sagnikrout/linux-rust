//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/hwss/dcn32/dcn32_hwseq.h
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
// Copyright 2016 Advanced Micro Devices, Inc.
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
    pub fn dcn32_hubp_pg_control(hws: *mut dce_hwseq, hubp_inst: c_uint, power_on: bool);
}
extern "C" {
    pub fn dcn32_apply_idle_power_optimizations(dc: *mut dc, enable: bool) -> bool;
}
extern "C" {
    pub fn dcn32_cab_for_ss_control(dc: *mut dc, enable: bool);
}
extern "C" {
    pub fn dcn32_commit_subvp_config(dc: *mut dc, context: *mut dc_state);
}
extern "C" {
    pub fn dcn32_set_output_transfer_func(params: *mut set_output_transfer_func_params) -> bool;
}
extern "C" {
    pub fn dcn32_init_hw(dc: *mut dc);
}
extern "C" {
    pub fn dcn32_program_mall_pipe_config(dc: *mut dc, context: *mut dc_state);
}
extern "C" {
    pub fn dcn32_update_mall_sel(dc: *mut dc, context: *mut dc_state);
}
extern "C" {
    pub fn dcn32_update_force_pstate(dc: *mut dc, context: *mut dc_state);
}
extern "C" {
    pub fn dcn32_update_odm(dc: *mut dc, context: *mut dc_state, pipe_ctx: *mut pipe_ctx);
}
extern "C" {
    pub fn dcn32_update_dsc_on_stream(pipe_ctx: *mut pipe_ctx, enable: bool);
}
extern "C" {
    pub fn dcn32_calculate_dccg_k1_k2_values(pipe_ctx: *mut pipe_ctx, k1_div: *mut c_uint, k2_div: *mut c_uint) -> c_uint;
}
extern "C" {
    pub fn dcn32_resync_fifo_dccg_dio(hws: *mut dce_hwseq, dc: *mut dc, context: *mut dc_state, current_pipe_idx: c_uint);
}
extern "C" {
    pub fn dcn32_subvp_pipe_control_lock_fast(params: *mut block_sequence_params);
}
extern "C" {
    pub fn dcn32_is_dp_dig_pixel_rate_div_policy(pipe_ctx: *mut pipe_ctx) -> bool;
}
extern "C" {
    pub fn dcn32_calculate_pix_rate_divider(dc: *mut dc, context: *mut dc_state, stream: *const dc_stream_state);
}
extern "C" {
    pub fn dcn32_apply_update_flags_for_phantom(phantom_pipe: *mut pipe_ctx);
}
extern "C" {
    pub fn dcn32_enable_phantom_streams(dc: *mut dc, context: *mut dc_state);
}
extern "C" {
    pub fn dcn32_disable_phantom_streams(dc: *mut dc, context: *mut dc_state);
}
