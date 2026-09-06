//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/clk/meson/clk-pll.h
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
//
// Copyright (c) 2019 BayLibre, SAS.
// Author: Jerome Brunet <jbrunet@baylibre.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pll_params_table {
    pub m: c_uint,
    pub n: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pll_mult_range {
    pub min: c_uint,
    pub max: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct meson_clk_pll_data {
    pub en: parm,
    pub m: parm,
    pub n: parm,
    pub frac: parm,
    pub l: parm,
    pub rst: parm,
    pub current_en: parm,
    pub l_detect: parm,
    pub init_regs: *const reg_sequence,
    pub init_count: c_uint,
    pub table: *const pll_params_table,
    pub range: *const pll_mult_range,
    pub frac_max: c_uint,
    pub flags: u8,
}
