//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/rl6231.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// rl6231.h - RL6231 class device shared support
//
// Copyright 2014 Realtek Semiconductor Corp.
//
// Author: Oder Chiou <oder_chiou@realtek.com>
//
pub const RL6231_PLL_INP_MAX: c_int = 50000000;
pub const RL6231_PLL_INP_MIN: c_int = 256000;
pub const RL6231_PLL_N_MAX: c_uint = 0x1ff;
pub const RL6231_PLL_K_MAX: c_uint = 0x1f;
pub const RL6231_PLL_M_MAX: c_uint = 0xf;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rl6231_pll_code {
    pub /: *mut *mut bool m_bp; / Indicates bypass m code or not.,
    pub /: *mut *mut bool k_bp; / Indicates bypass k code or not.,
    pub m_code: c_int,
    pub n_code: c_int,
    pub k_code: c_int,
}

extern "C" {
    pub fn rl6231_calc_dmic_clk(rate: c_int) -> c_int;
}
extern "C" {
    pub fn rl6231_get_clk_info(sclk: c_int, rate: c_int) -> c_int;
}
extern "C" {
    pub fn rl6231_get_pre_div(map: *mut regmap, reg: c_uint, sft: c_int) -> c_int;
}
