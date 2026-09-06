//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/hwss/dcn10/dcn10_hwseq.h
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
// Copyright 2016-2020 Advanced Micro Devices, Inc.
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
    pub fn dcn10_hw_sequencer_construct(dc: *mut dc);
}
extern "C" {
    pub fn dcn10_get_vupdate_offset_from_vsync(pipe_ctx: *mut pipe_ctx) -> c_int;
}
extern "C" {
    pub fn dcn10_setup_vupdate_interrupt(dc: *mut dc, pipe_ctx: *mut pipe_ctx);
}
extern "C" {
    pub fn dcn10_cursor_lock(dc: *mut dc, pipe: *mut pipe_ctx, lock: bool);
}
extern "C" {
    pub fn dcn10_set_output_transfer_func(params: *mut set_output_transfer_func_params) -> bool;
}
extern "C" {
    pub fn dcn10_update_plane_addr(dc: *const dc, pipe_ctx: *mut pipe_ctx);
}
extern "C" {
    pub fn dcn10_update_mpcc(dc: *mut dc, pipe_ctx: *mut pipe_ctx);
}
extern "C" {
    pub fn dcn10_disable_plane(dc: *mut dc, state: *mut dc_state, pipe_ctx: *mut pipe_ctx);
}
extern "C" {
    pub fn dcn10_plane_atomic_disable(dc: *mut dc, pipe_ctx: *mut pipe_ctx);
}
extern "C" {
    pub fn dcn10_program_gamut_remap(params: *mut program_gamut_remap_params);
}
extern "C" {
    pub fn dcn10_init_hw(dc: *mut dc);
}
extern "C" {
    pub fn dcn10_init_pipes(dc: *mut dc, context: *mut dc_state);
}
extern "C" {
    pub fn dcn10_power_down_on_boot(dc: *mut dc);
}
extern "C" {
    pub fn dcn10_update_dchub(hws: *mut dce_hwseq, dh_data: *mut dchub_init_data);
}
extern "C" {
    pub fn dcn10_update_pending_status(pipe_ctx: *mut pipe_ctx);
}
extern "C" {
    pub fn dce110_power_down(dc: *mut dc);
}
extern "C" {
    pub fn dce110_enable_accelerated_mode(dc: *mut dc, context: *mut dc_state);
}
extern "C" {
    pub fn dce110_update_info_frame(pipe_ctx: *mut pipe_ctx);
}
extern "C" {
    pub fn dce110_blank_stream(pipe_ctx: *mut pipe_ctx);
}
extern "C" {
    pub fn dce110_enable_audio_stream(pipe_ctx: *mut pipe_ctx);
}
extern "C" {
    pub fn dce110_disable_audio_stream(pipe_ctx: *mut pipe_ctx);
}
extern "C" {
    pub fn dcn10_setup_stereo(pipe_ctx: *mut pipe_ctx, dc: *mut dc);
}
extern "C" {
    pub fn dce110_set_avmute(pipe_ctx: *mut pipe_ctx, enable: bool);
}
extern "C" {
    pub fn dcn10_clear_status_bits(dc: *mut dc, mask: c_uint);
}
extern "C" {
    pub fn dcn10_set_cursor_position(pipe_ctx: *mut pipe_ctx);
}
extern "C" {
    pub fn dcn10_set_cursor_attribute(pipe_ctx: *mut pipe_ctx);
}
extern "C" {
    pub fn dcn10_set_cursor_sdr_white_level(pipe_ctx: *mut pipe_ctx);
}
extern "C" {
    pub fn dcn10_did_underflow_occur(dc: *mut dc, pipe_ctx: *mut pipe_ctx) -> bool;
}
extern "C" {
    pub fn dcn10_bios_golden_init(dc: *mut dc);
}
extern "C" {
    pub fn dcn10_set_hdr_multiplier(pipe_ctx: *mut pipe_ctx);
}
extern "C" {
    pub fn dcn10_verify_allow_pstate_change_high(dc: *mut dc);
}
extern "C" {
    pub fn dcn10_get_dcc_en_bits(dc: *mut dc, dcc_en_bits: *mut c_int);
}
