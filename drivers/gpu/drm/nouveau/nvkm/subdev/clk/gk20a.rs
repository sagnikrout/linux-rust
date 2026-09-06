//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/nvkm/subdev/clk/gk20a.h
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
// Copyright (c) 2016, NVIDIA CORPORATION. All rights reserved.
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
// THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.
//

pub const GK20A_CLK_GPC_MDIV: c_int = 1000;
pub const SYS_GPCPLL_CFG_BASE: c_uint = 0x00137000;

pub const GPCPLL_CFG2_SETUP2_SHIFT: c_int = 16;
pub const GPCPLL_CFG2_PLL_STEPA_SHIFT: c_int = 24;

pub const GPCPLL_CFG3_VCO_CTRL_SHIFT: c_int = 0;
pub const GPCPLL_CFG3_VCO_CTRL_WIDTH: c_int = 9;

pub const GPCPLL_CFG3_PLL_STEPB_SHIFT: c_int = 16;
pub const GPCPLL_CFG3_PLL_STEPB_WIDTH: c_int = 8;

pub const GPCPLL_COEFF_M_SHIFT: c_int = 0;
pub const GPCPLL_COEFF_M_WIDTH: c_int = 8;
pub const GPCPLL_COEFF_N_SHIFT: c_int = 8;
pub const GPCPLL_COEFF_N_WIDTH: c_int = 8;

pub const GPCPLL_COEFF_P_SHIFT: c_int = 16;
pub const GPCPLL_COEFF_P_WIDTH: c_int = 6;

pub const GPCPLL_NDIV_SLOWDOWN_NDIV_LO_SHIFT: c_int = 0;
pub const GPCPLL_NDIV_SLOWDOWN_NDIV_MID_SHIFT: c_int = 8;
pub const GPCPLL_NDIV_SLOWDOWN_STEP_SIZE_LO2MID_SHIFT: c_int = 16;
pub const GPCPLL_NDIV_SLOWDOWN_SLOWDOWN_USING_PLL_SHIFT: c_int = 22;
pub const GPCPLL_NDIV_SLOWDOWN_EN_DYNRAMP_SHIFT: c_int = 31;
pub const GPC_BCAST_GPCPLL_CFG_BASE: c_uint = 0x00132800;

pub const GPC_BCAST_NDIV_SLOWDOWN_DEBUG_PLL_DYNRAMP_DONE_SYNCED_SHIFT: c_int = 24;

pub const SEL_VCO_GPC2CLK_OUT_SHIFT: c_int = 0;

pub const GPC2CLK_OUT_SDIV14_INDIV4_WIDTH: c_int = 1;
pub const GPC2CLK_OUT_SDIV14_INDIV4_SHIFT: c_int = 31;
pub const GPC2CLK_OUT_SDIV14_INDIV4_MODE: c_int = 1;
pub const GPC2CLK_OUT_VCODIV_WIDTH: c_int = 6;
pub const GPC2CLK_OUT_VCODIV_SHIFT: c_int = 8;
pub const GPC2CLK_OUT_VCODIV1: c_int = 0;
pub const GPC2CLK_OUT_VCODIV2: c_int = 2;

pub const GPC2CLK_OUT_BYPDIV_WIDTH: c_int = 6;
pub const GPC2CLK_OUT_BYPDIV_SHIFT: c_int = 0;
pub const GPC2CLK_OUT_BYPDIV31: c_uint = 0x3c;

// All frequencies in Khz
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gk20a_clk_pllg_params {
    pub max_vco: u32 min_vco,,
    pub max_u: u32 min_u,,
    pub max_m: u32 min_m,,
    pub max_n: u32 min_n,,
    pub max_pl: u32 min_pl,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gk20a_pll {
    pub m: u32,
    pub n: u32,
    pub pl: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gk20a_clk {
    pub base: nvkm_clk,
    pub params: *const gk20a_clk_pllg_params,
    pub pll: gk20a_pll,
    pub parent_rate: u32,
    pub devfreq: *mut gk20a_devfreq,
    pub (*div_to_pl)(u32): *mut u32,
    pub (*pl_to_div)(u32): *mut u32,
}

extern "C" {
    pub fn gk20a_pllg_calc_rate(: *mut gk20a_clk, : *mut gk20a_pll) -> u32;
}
extern "C" {
    pub fn gk20a_pllg_calc_mnp(: *mut gk20a_clk, long: unsigned, : *mut gk20a_pll) -> c_int;
}
extern "C" {
    pub fn gk20a_pllg_read_mnp(: *mut gk20a_clk, : *mut gk20a_pll);
}
extern "C" {
    pub fn gk20a_pllg_write_mnp(: *mut gk20a_clk, : *const gk20a_pll);
}
extern "C" {
    pub fn gk20a_clk_fini(: *mut nvkm_clk);
}
extern "C" {
    pub fn gk20a_clk_read(: *mut nvkm_clk, nv_clk_src: enum) -> c_int;
}
extern "C" {
    pub fn gk20a_clk_calc(: *mut nvkm_clk, : *mut nvkm_cstate) -> c_int;
}
extern "C" {
    pub fn gk20a_clk_prog(: *mut nvkm_clk) -> c_int;
}
extern "C" {
    pub fn gk20a_clk_tidy(: *mut nvkm_clk);
}
extern "C" {
    pub fn gk20a_clk_setup_slide(: *mut gk20a_clk) -> c_int;
}
