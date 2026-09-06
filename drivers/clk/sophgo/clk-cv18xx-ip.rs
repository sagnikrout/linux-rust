//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/clk/sophgo/clk-cv18xx-ip.h
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
pub struct cv1800_clk_gate {
    pub common: cv1800_clk_common,
    pub gate: cv1800_clk_regbit,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cv1800_clk_div_data {
    pub reg: u32,
    pub mask: u32,
    pub width: u32,
    pub init: u32,
    pub flags: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cv1800_clk_div {
    pub common: cv1800_clk_common,
    pub gate: cv1800_clk_regbit,
    pub div: cv1800_clk_regfield,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cv1800_clk_bypass_div {
    pub div: cv1800_clk_div,
    pub bypass: cv1800_clk_regbit,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cv1800_clk_mux {
    pub common: cv1800_clk_common,
    pub gate: cv1800_clk_regbit,
    pub div: cv1800_clk_regfield,
    pub mux: cv1800_clk_regfield,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cv1800_clk_bypass_mux {
    pub mux: cv1800_clk_mux,
    pub bypass: cv1800_clk_regbit,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cv1800_clk_mmux {
    pub common: cv1800_clk_common,
    pub gate: cv1800_clk_regbit,
    pub div: [cv1800_clk_regfield; 2],
    pub mux: [cv1800_clk_regfield; 2],
    pub bypass: cv1800_clk_regbit,
    pub clk_sel: cv1800_clk_regbit,
    pub parent2sel: *const i8,
    pub sel2parent: [*const u8; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cv1800_clk_audio {
    pub common: cv1800_clk_common,
    pub src_en: cv1800_clk_regbit,
    pub output_en: cv1800_clk_regbit,
    pub div_en: cv1800_clk_regbit,
    pub div_up: cv1800_clk_regbit,
    pub m: cv1800_clk_regfield,
    pub n: cv1800_clk_regfield,
    pub target_rate: u32,
}

