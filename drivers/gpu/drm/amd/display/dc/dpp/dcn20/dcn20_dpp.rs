//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/dpp/dcn20/dcn20_dpp.h
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

// Macro flag: #define TO_DCN20_DPP(dpp)\

// Macro flag: #define TF_REG_LIST_SH_MASK_DCN20_UPDATED(mask_sh)\
// Macro flag: #define TF_REG_LIST_SH_MASK_DCN20_COMMON(mask_sh)\
// Macro flag: #define TF_REG_LIST_SH_MASK_DCN20(mask_sh)\
// DPP CM debug status register:
//
// Status index including current ICSC, Gamut Remap Mode is 9
// ICSC Mode: [4..3]
// Gamut Remap Mode: [10..9]
//
pub const CM_TEST_DEBUG_DATA_STATUS_IDX: c_int = 9;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcn2_dpp_shift {
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcn2_dpp_mask {
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcn2_dpp_registers {
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcn20_dpp {
    pub base: dpp,
    pub tf_regs: *const dcn2_dpp_registers,
    pub tf_shift: *const dcn2_dpp_shift,
    pub tf_mask: *const dcn2_dpp_mask,
    pub filter_v: *const u16,
    pub filter_h: *const u16,
    pub filter_v_c: *const u16,
    pub filter_h_c: *const u16,
    pub lb_pixel_depth_supported: c_int,
    pub lb_memory_size: c_int,
    pub lb_bits_per_entry: c_int,
    pub is_write_to_ram_a_safe: bool,
    pub dispclk_r_gate_disable: bool,
    pub scl_data: scaler_data,
    pub pwl_data: pwl_params,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dcn20_input_csc_select {
    DCN2_ICSC_SELECT_BYPASS = 0,
    DCN2_ICSC_SELECT_ICSC_A = 1,
    DCN2_ICSC_SELECT_ICSC_B = 2
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dcn20_gamut_remap_select {
    DCN2_GAMUT_REMAP_BYPASS = 0,
    DCN2_GAMUT_REMAP_COEF_A = 1,
    DCN2_GAMUT_REMAP_COEF_B = 2
}
