//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/resource/dcn32/dcn32_resource.h
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
// Copyright 2020 Advanced Micro Devices, Inc.
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

pub const DCN3_2_DEFAULT_DET_SIZE: c_int = 256;
pub const DCN3_2_MAX_DET_SIZE: c_int = 1152;
pub const DCN3_2_MIN_DET_SIZE: c_int = 128;
pub const DCN3_2_MIN_COMPBUF_SIZE_KB: c_int = 128;
pub const DCN3_2_DET_SEG_SIZE: c_int = 64;

pub const DCN3_2_MBLK_WIDTH: c_int = 128;
pub const DCN3_2_MBLK_HEIGHT_4BPE: c_int = 128;
pub const DCN3_2_MBLK_HEIGHT_8BPE: c_int = 64;

pub const SUBVP_HIGH_REFRESH_LIST_LEN: c_int = 4;
pub const SUBVP_ACTIVE_MARGIN_LIST_LEN: c_int = 2;
pub const DCN3_2_MAX_SUBVP_PIXEL_RATE_MHZ: c_int = 1800;
pub const DCN3_2_VMIN_DISPCLK_HZ: c_int = 717000000;
pub const MIN_SUBVP_DCFCLK_KHZ: c_int = 400000;
// Macro flag: #define TO_DCN32_RES_POOL(pool)\
#[repr(C)]
#[derive(Copy, Clone)]
pub struct subvp_high_refresh_list {
    pub min_refresh: c_int,
    pub max_refresh: c_int,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct resolution {
    pub width: c_int,
    pub height: c_int,
    pub res: [}; SUBVP_HIGH_REFRESH_LIST_LEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct subvp_active_margin_list {
    pub min_refresh: c_int,
    pub max_refresh: c_int,
    pub width: c_int,
    pub height: c_int,
    pub res: [}; SUBVP_ACTIVE_MARGIN_LIST_LEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcn32_resource_pool {
    pub base: resource_pool,
}

extern "C" {
    pub fn dcn32_mpo_in_use(context: *mut dc_state) -> bool;
}
extern "C" {
    pub fn dcn32_any_surfaces_rotated(dc: *mut dc, context: *mut dc_state) -> bool;
}
extern "C" {
    pub fn dcn32_is_center_timing(pipe: *mut pipe_ctx) -> bool;
}
extern "C" {
    pub fn dcn32_is_psr_capable(pipe: *mut pipe_ctx) -> bool;
}
extern "C" {
    pub fn dcn32_allow_subvp_with_active_margin(pipe: *mut pipe_ctx) -> bool;
}
extern "C" {
    pub fn dcn32_allow_subvp_high_refresh_rate(dc: *mut dc, context: *mut dc_state, pipe: *mut pipe_ctx) -> bool;
}
extern "C" {
    pub fn dcn32_calc_num_avail_chans_for_mall(dc: *mut dc, num_chans: c_int) -> c_uint;
}
extern "C" {
    pub fn dcn32_determine_max_vratio_prefetch(dc: *mut dc, context: *mut dc_state) -> double;
}
extern "C" {
    pub fn dcn32_check_native_scaling_for_res(pipe: *mut pipe_ctx, width: c_uint, height: c_uint) -> bool;
}
extern "C" {
    pub fn dcn32_subvp_drr_admissable(dc: *mut dc, context: *mut dc_state) -> bool;
}
extern "C" {
    pub fn dcn32_subvp_vblank_admissable(dc: *mut dc, context: *mut dc_state, vlevel: c_int) -> bool;
}
extern "C" {
    pub fn dcn32_update_dml_pipes_odm_policy_based_on_context(dc: *mut dc, context: *mut dc_state, pipes: *mut display_e2e_pipe_params_st);
}
extern "C" {
    pub fn dcn32_override_min_req_dcfclk(dc: *mut dc, context: *mut dc_state);
}
extern "C" {
    pub fn dcn32_calculate_mall_ways_from_bytes(dc: *const dc, total_size_in_mall_bytes: c_uint) -> c_uint;
}
// definitions for run time init of reg offsets
// CLK SRC

// ABM

// Audio

// VPG

// AFMT

// APG

// Stream encoder

// Aux regs

// HDP

// Link encoder

// HPO DP stream encoder

// HPO DP link encoder regs

// DPP

// OPP

// Aux engine regs

// DWBC

// MCIF

// DSC

// MPC

// OPTC

// HUBP

// HUBBUB

// DCCG

// VMID

// I2C HW

