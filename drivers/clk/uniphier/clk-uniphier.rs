//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/clk/uniphier/clk-uniphier.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright (C) 2016 Socionext Inc.
// Author: Masahiro Yamada <yamada.masahiro@socionext.com>
//
pub const UNIPHIER_CLK_CPUGEAR_MAX_PARENTS: c_int = 16;
pub const UNIPHIER_CLK_MUX_MAX_PARENTS: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum uniphier_clk_type {
    UNIPHIER_CLK_TYPE_CPUGEAR,
    UNIPHIER_CLK_TYPE_FIXED_FACTOR,
    UNIPHIER_CLK_TYPE_FIXED_RATE,
    UNIPHIER_CLK_TYPE_GATE,
    UNIPHIER_CLK_TYPE_MUX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uniphier_clk_cpugear_data {
    pub parent_names: [*const c_char; UNIPHIER_CLK_CPUGEAR_MAX_PARENTS],
    pub num_parents: c_uint,
    pub regbase: c_uint,
    pub mask: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uniphier_clk_fixed_factor_data {
    pub parent_name: *const c_char,
    pub mult: c_uint,
    pub div: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uniphier_clk_fixed_rate_data {
    pub fixed_rate: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uniphier_clk_gate_data {
    pub parent_name: *const c_char,
    pub reg: c_uint,
    pub bit: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uniphier_clk_mux_data {
    pub parent_names: [*const c_char; UNIPHIER_CLK_MUX_MAX_PARENTS],
    pub num_parents: c_uint,
    pub reg: c_uint,
    pub masks: [c_uint; UNIPHIER_CLK_MUX_MAX_PARENTS],
    pub vals: [c_uint; UNIPHIER_CLK_MUX_MAX_PARENTS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uniphier_clk_data {
    pub name: *const c_char,
    pub type: uniphier_clk_type,
    pub idx: c_int,
    pub cpugear: uniphier_clk_cpugear_data,
    pub factor: uniphier_clk_fixed_factor_data,
    pub rate: uniphier_clk_fixed_rate_data,
    pub gate: uniphier_clk_gate_data,
    pub mux: uniphier_clk_mux_data,
    pub data: },
}

