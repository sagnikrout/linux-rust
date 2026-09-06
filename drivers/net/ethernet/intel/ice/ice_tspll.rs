//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/ice/ice_tspll.h
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


// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2025, Intel Corporation.
//
// struct ice_tspll_params_e82x - E82X TSPLL parameters
// @refclk_pre_div: Reference clock pre-divisor
// @post_pll_div: Post PLL divisor
// @feedback_div: Feedback divisor
// @frac_n_div: Fractional divisor
//
// Clock Generation Unit parameters used to program the PLL based on the
// selected TIME_REF/TCXO frequency.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_tspll_params_e82x {
    pub refclk_pre_div: u8,
    pub post_pll_div: u8,
    pub feedback_div: u8,
    pub frac_n_div: u32,
}

pub const ICE_CGU_NET_REF_CLK0: c_uint = 0x0;
pub const ICE_CGU_REF_CLK_BYP0: c_uint = 0x5;
pub const ICE_CGU_REF_CLK_BYP0_DIV: c_uint = 0x0;
pub const ICE_CGU_REF_CLK_BYP1: c_uint = 0x4;
pub const ICE_CGU_REF_CLK_BYP1_DIV: c_uint = 0x1;
pub const ICE_TSPLL_CK_REFCLKFREQ_E825: c_uint = 0x1F;
pub const ICE_TSPLL_NDIVRATIO_E825: c_int = 5;
pub const ICE_TSPLL_FBDIV_INTGR_E825: c_int = 256;
extern "C" {
    pub fn ice_tspll_cfg_pps_out_e825c(hw: *mut ice_hw, enable: bool) -> c_int;
}
extern "C" {
    pub fn ice_tspll_init(hw: *mut ice_hw) -> c_int;
}
