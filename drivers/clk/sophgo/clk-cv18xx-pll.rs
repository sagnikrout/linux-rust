//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/clk/sophgo/clk-cv18xx-pll.h
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
// Copyright (C) 2023 Inochi Amaoto <inochiama@outlook.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cv1800_clk_pll_limit {
    pub min: u8,
    pub max: u8,
    pub mode: } pre_div, div, post_div, ictrl,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cv1800_clk_pll_synthesizer {
    pub en: cv1800_clk_regbit,
    pub clk_half: cv1800_clk_regbit,
    pub ctrl: u32,
    pub set: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cv1800_clk_pll {
    pub common: cv1800_clk_common,
    pub pll_reg: u32,
    pub pll_pwd: cv1800_clk_regbit,
    pub pll_status: cv1800_clk_regbit,
    pub pll_limit: *const cv1800_clk_pll_limit,
    pub pll_syn: *mut cv1800_clk_pll_synthesizer,
}

