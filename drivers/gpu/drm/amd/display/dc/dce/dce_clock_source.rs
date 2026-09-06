//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/dce/dce_clock_source.h
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

// Macro flag: #define TO_DCE110_CLK_SRC(clk_src)\

// Macro flag: #define CS_COMMON_MASK_SH_LIST_DCE_COMMON_BASE(mask_sh)\
// Macro flag: #define CS_COMMON_MASK_SH_LIST_DCE_112(mask_sh)\

// Macro flag: #define CS_COMMON_MASK_SH_LIST_DCN2_0(mask_sh)\
// Macro flag: #define CS_COMMON_MASK_SH_LIST_DCN3_1_4(mask_sh)\
// Macro flag: #define CS_COMMON_MASK_SH_LIST_DCN3_2(mask_sh)\
// Macro flag: #define CS_COMMON_MASK_SH_LIST_DCN4_0_1(mask_sh)\

// Macro flag: #define CS_COMMON_MASK_SH_LIST_DCN1_0(mask_sh)\

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dce110_clk_src_shift {
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dce110_clk_src_mask {
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dce110_clk_src_regs {
    pub RESYNC_CNTL: u32,
    pub PIXCLK_RESYNC_CNTL: u32,
    pub PLL_CNTL: u32,
    pub OTG_PIXEL_RATE_DIV: u32,
// below are for DTO.
// todo: should probably use different struct to not waste space
//
    pub PHASE: [u32; MAX_PIPES],
    pub MODULO: [u32; MAX_PIPES],
    pub PIXEL_RATE_CNTL: [u32; MAX_PIPES],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dce110_clk_src {
    pub base: clock_source,
    pub regs: *const dce110_clk_src_regs,
    pub cs_mask: *const dce110_clk_src_mask,
    pub cs_shift: *const dce110_clk_src_shift,
    pub bios: *mut dc_bios,
    pub dp_ss_params: *mut spread_spectrum_data,
    pub dp_ss_params_cnt: u32,
    pub hdmi_ss_params: *mut spread_spectrum_data,
    pub hdmi_ss_params_cnt: u32,
    pub dvi_ss_params: *mut spread_spectrum_data,
    pub dvi_ss_params_cnt: u32,
    pub lvds_ss_params: *mut spread_spectrum_data,
    pub lvds_ss_params_cnt: u32,
    pub ext_clk_khz: u32,
    pub ref_freq_khz: u32,
    pub calc_pll: calc_pll_clock_source,
    pub calc_pll_hdmi: calc_pll_clock_source,
}

// this table is use to find *1.001 and /1.001 pixel rates from non-precise pixel rate
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pixel_rate_range_table_entry {
    pub range_min_khz: c_uint,
    pub range_max_khz: c_uint,
    pub target_pixel_rate_khz: c_uint,
    pub mult_factor: c_ushort,
    pub div_factor: c_ushort,
}
