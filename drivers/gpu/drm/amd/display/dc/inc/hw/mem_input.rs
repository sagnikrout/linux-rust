//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/inc/hw/mem_input.h
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
// Copyright 2012-15 Advanced Micro Devices, Inc.
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
pub struct cstate_pstate_watermarks_st {
    pub cstate_exit_ns: u32,
    pub cstate_exit_z8_ns: u32,
    pub cstate_enter_plus_exit_z8_ns: u32,
    pub cstate_enter_plus_exit_ns: u32,
    pub pstate_change_ns: u32,
    pub fclk_pstate_change_ns: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcn_watermarks {
    pub pte_meta_urgent_ns: u32,
    pub urgent_ns: u32,
    pub frac_urg_bw_nom: u32,
    pub frac_urg_bw_flip: u32,
    pub urgent_latency_ns: u32,
    pub cstate_pstate: cstate_pstate_watermarks_st,
    pub usr_retraining_ns: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union dcn_watermark_set {
    pub a: dcn_watermarks,
    pub b: dcn_watermarks,
    pub c: dcn_watermarks,
    pub d: dcn_watermarks,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dce_watermarks {
    pub a_mark: c_int,
    pub b_mark: c_int,
    pub c_mark: c_int,
    pub d_mark: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stutter_modes {
    pub enhanced: bool,
    pub quad_dmif_buffer: bool,
    pub watermark_nb_pstate: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mem_input {
    pub funcs: *const mem_input_funcs,
    pub ctx: *mut dc_context,
    pub request_address: dc_plane_address,
    pub current_address: dc_plane_address,
    pub inst: c_int,
    pub stutter_mode: stutter_modes,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vm_system_aperture_param {
    pub sys_default: PHYSICAL_ADDRESS_LOC,
    pub sys_low: PHYSICAL_ADDRESS_LOC,
    pub sys_high: PHYSICAL_ADDRESS_LOC,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vm_context0_param {
    pub pte_base: PHYSICAL_ADDRESS_LOC,
    pub pte_start: PHYSICAL_ADDRESS_LOC,
    pub pte_end: PHYSICAL_ADDRESS_LOC,
    pub fault_default: PHYSICAL_ADDRESS_LOC,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mem_input_funcs {
    pub pipe_dest): *mut _vcs_dpi_display_pipe_dest_params_st,
    pub independent_64b_blks): bool,
    pub viewport_c): *const rect,
    pub total_dest_line_time_ns): u32,
    pub total_dest_line_time_ns): u32,
    pub total_streams_num): u32,
    pub paths_num): u32,
    pub flip_immediate): bool,
    pub rotation): dc_rotation_angle,
    pub apt): *mut vm_system_aperture_param,
    pub vm0): *const vm_context0_param,
    pub horizontal_mirror): bool,
    pub mem_input): *mut *mut bool (mem_input_is_flip_pending)(struct mem_input,
    pub dh_data): *mut dchub_init_data,
    pub blank): *mut *mut *mut void (set_blank)(struct mem_input mi, bool,
    pub blank): *mut *mut *mut void (set_hubp_blank_en)(struct mem_input mi, bool,
    pub attr): *const dc_cursor_attributes,
    pub param): *const dc_cursor_mi_param,
    pub mem_input): *mut mem_input,
}
