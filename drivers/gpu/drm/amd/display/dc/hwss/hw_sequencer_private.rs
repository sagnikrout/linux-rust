//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/hwss/hw_sequencer_private.h
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
// Copyright 2015-2026 Advanced Micro Devices, Inc.
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
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pipe_gating_control {
    PIPE_GATING_CONTROL_DISABLE = 0,
    PIPE_GATING_CONTROL_ENABLE,
    PIPE_GATING_CONTROL_INIT
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dce_hwseq_wa {
    pub blnd_crtc_trigger: bool,
    pub DEGVIDCN10_253: bool,
    pub false_optc_underflow: bool,
    pub DEGVIDCN10_254: bool,
    pub DEGVIDCN21: bool,
    pub disallow_self_refresh_during_multi_plane_transition: bool,
    pub dp_hpo_and_otg_sequence: bool,
    pub wait_hubpret_read_start_during_mpo_transition: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hwseq_wa_state {
    pub DEGVIDCN10_253_applied: bool,
    pub disallow_self_refresh_during_multi_plane_transition_applied: bool,
    pub disallow_self_refresh_during_multi_plane_transition_applied_on_frame: c_uint,
    pub skip_blank_stream: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hwseq_private_funcs {
    pub pipe_ctx): *mut *mut *mut void (disable_stream_gating)(struct dc dc, struct pipe_ctx,
    pub pipe_ctx): *mut *mut *mut void (enable_stream_gating)(struct dc dc, struct pipe_ctx,
    pub context): *mut *mut *mut void (init_pipes)(struct dc dc, struct dc_state,
    pub context): *mut *mut *mut void (reset_hw_ctx_wrap)(struct dc dc, struct dc_state,
    pub pipe_ctx): *mut pipe_ctx,
    pub seq_state): *mut block_sequence_state,
    pub pipe_ctx): *mut *mut *mut void (update_mpcc)(struct dc dc, struct pipe_ctx,
    pub seq_state): *mut block_sequence_state,
    pub plane_state): *const dc_plane_state,
    pub params): *mut *mut bool (set_output_transfer_func)(struct set_output_transfer_func_params,
    pub dc): *mut *mut void (power_down)(struct dc,
    pub clock_gating): bool,
    pub power_gating): pipe_gating_control,
    pub blank): bool,
    pub seq_state): *mut block_sequence_state,
    pub dc): *mut dc,
    pub enable): bool,
    pub pipe_ctx): *mut pipe_ctx,
    pub seq_state): *mut block_sequence_state,
    pub pipe_ctx): *mut *mut *mut bool (did_underflow_occur)(struct dc dc, struct pipe_ctx,
    pub tg): *mut *mut *mut void (init_blank)(struct dc dc, struct timing_generator,
    pub hws): *mut *mut void (disable_vga)(struct dce_hwseq,
    pub dc): *mut *mut void (bios_golden_init)(struct dc,
    pub hubp): *mut hubp,
    pub seq_state): *mut block_sequence_state,
    pub pipe_ctx): *mut *mut *mut void (plane_atomic_disable)(struct dc dc, struct pipe_ctx,
    pub enable): bool,
    pub clock_on): bool,
    pub clock_on): bool,
    pub clock_on): bool,
    pub clock_on): bool,
    pub power_on): bool,
    pub power_on): bool,
    pub power_on): bool,
    pub dsc_inst): c_uint,
    pub pipe_ctx): *mut pipe_ctx,
    pub seq_state): *mut *mut pipe_ctx pipe_ctx, block_sequence_state,
    pub context): *mut dc_state,
    pub seq_state): *mut block_sequence_state,
    pub dc): *mut *mut bool (s0i3_golden_init_wa)(struct dc,
    pub pipe_ctx): *mut *mut void (set_hdr_multiplier)(struct pipe_ctx,
    pub seq_state): *mut block_sequence_state,
    pub dc): *mut *mut void (verify_allow_pstate_change_high)(struct dc,
    pub seq_state): *mut block_sequence_state,
    pub context): *mut dc_state,
    pub seq_state): *mut block_sequence_state,
    pub opp): *mut *mut bool (wait_for_blank_complete)(struct output_pixel_processor,
    pub hws): *mut *mut void (dccg_init)(struct dce_hwseq,
    pub plane_state): *const dc_plane_state,
    pub plane_state): *const dc_plane_state,
    pub plane_state): *const dc_plane_state,
    pub pipe_ctx): *mut pipe_ctx,
    pub enable): *const *const *const void (setup_hpo_hw_control)(struct dce_hwseq hws, bool,
    pub context): *mut dc_state,
    pub context): *mut *mut *mut void (program_mall_pipe_config)(struct dc dc, struct dc_state,
    pub seq_state): *mut block_sequence_state,
    pub context): *mut *mut *mut void (update_force_pstate)(struct dc dc, struct dc_state,
    pub context): *mut *mut *mut void (update_mall_sel)(struct dc dc, struct dc_state,
    pub k2_div): *mut c_uint,
    pub current_pipe_idx): c_uint,
    pub dc): *mut dc,
    pub pipe_ctx): *mut *mut bool (is_dp_dig_pixel_rate_div_policy)(struct pipe_ctx,
    pub context): *mut dc_state,
    pub pipe_ctx): *mut *mut void (perform_3dlut_wa_unlock)(struct pipe_ctx,
    pub is_surface_update_only): *mut *mut *mut *mut void (wait_for_pipe_update_if_needed)(struct dc dc, struct pipe_ctx pipe_ctx, bool,
    pub pipe_ctx): *mut *mut *mut void (set_wait_for_update_needed_for_pipe)(struct dc dc, struct pipe_ctx,
    pub enable): *mut *mut *mut void (dc_ip_request_cntl)(struct dc dc, bool,
    pub plane_state): *const dc_plane_state,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dce_hwseq {
    pub ctx: *mut dc_context,
    pub regs: *const dce_hwseq_registers,
    pub shifts: *const dce_hwseq_shift,
    pub masks: *const dce_hwseq_mask,
    pub wa: dce_hwseq_wa,
    pub wa_state: hwseq_wa_state,
    pub funcs: hwseq_private_funcs,
    pub fb_base: PHYSICAL_ADDRESS_LOC,
    pub fb_top: PHYSICAL_ADDRESS_LOC,
    pub fb_offset: PHYSICAL_ADDRESS_LOC,
    pub uma_top: PHYSICAL_ADDRESS_LOC,
}
