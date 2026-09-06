//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/clk/pistachio/clk.h
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
// Copyright (C) 2014 Google, Inc.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pistachio_gate {
    pub id: c_uint,
    pub reg: c_ulong,
    pub shift: c_uint,
    pub name: *const c_char,
    pub parent: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pistachio_mux {
    pub id: c_uint,
    pub reg: c_ulong,
    pub shift: c_uint,
    pub num_parents: c_uint,
    pub name: *const c_char,
    pub parents: *const *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pistachio_div {
    pub id: c_uint,
    pub reg: c_ulong,
    pub width: c_uint,
    pub div_flags: c_uint,
    pub name: *const c_char,
    pub parent: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pistachio_fixed_factor {
    pub id: c_uint,
    pub div: c_uint,
    pub name: *const c_char,
    pub parent: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pistachio_pll_rate_table {
    pub fref: c_ulonglong,
    pub fout: c_ulonglong,
    pub refdiv: c_ulonglong,
    pub fbdiv: c_ulonglong,
    pub postdiv1: c_ulonglong,
    pub postdiv2: c_ulonglong,
    pub frac: c_ulonglong,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pistachio_pll_type {
    PLL_GF40LP_LAINT,
    PLL_GF40LP_FRAC,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pistachio_pll {
    pub id: c_uint,
    pub reg_base: c_ulong,
    pub type: pistachio_pll_type,
    pub rates: *mut pistachio_pll_rate_table,
    pub nr_rates: c_uint,
    pub name: *const c_char,
    pub parent: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pistachio_clk_provider {
    pub node: *mut device_node,
    pub base: *mut void __iomem,
    pub clk_data: clk_onecell_data,
}

extern "C" {
    pub fn pistachio_clk_register_provider(p: *mut pistachio_clk_provider);
}
