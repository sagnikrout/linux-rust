//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/clk/x86/clk-cgu.h
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
// Copyright (C) 2020-2022 MaxLinear, Inc.
// Copyright (C) 2020 Intel Corporation.
// Zhu Yixin <yzhu@maxlinear.com>
// Rahul Tanwar <rtanwar@maxlinear.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lgm_clk_mux {
    pub hw: clk_hw,
    pub membase: *mut regmap,
    pub reg: c_uint,
    pub shift: u8,
    pub width: u8,
    pub flags: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lgm_clk_divider {
    pub hw: clk_hw,
    pub membase: *mut regmap,
    pub reg: c_uint,
    pub shift: u8,
    pub width: u8,
    pub shift_gate: u8,
    pub width_gate: u8,
    pub flags: c_ulong,
    pub table: *const clk_div_table,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lgm_clk_ddiv {
    pub hw: clk_hw,
    pub membase: *mut regmap,
    pub reg: c_uint,
    pub shift0: u8,
    pub width0: u8,
    pub shift1: u8,
    pub width1: u8,
    pub shift2: u8,
    pub width2: u8,
    pub shift_gate: u8,
    pub width_gate: u8,
    pub mult: c_uint,
    pub div: c_uint,
    pub flags: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lgm_clk_gate {
    pub hw: clk_hw,
    pub membase: *mut regmap,
    pub reg: c_uint,
    pub shift: u8,
    pub flags: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lgm_clk_type {
    CLK_TYPE_FIXED,
    CLK_TYPE_MUX,
    CLK_TYPE_DIVIDER,
    CLK_TYPE_FIXED_FACTOR,
    CLK_TYPE_GATE,
    CLK_TYPE_NONE,
}

//
// struct lgm_clk_provider
// @membase: IO mem base address for CGU.
// @np: device node
// @dev: device
// @clk_data: array of hw clocks and clk number.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lgm_clk_provider {
    pub membase: *mut regmap,
    pub np: *mut device_node,
    pub dev: *mut device,
    pub clk_data: clk_hw_onecell_data,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pll_type {
    TYPE_ROPLL,
    TYPE_LJPLL,
    TYPE_NONE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lgm_clk_pll {
    pub hw: clk_hw,
    pub membase: *mut regmap,
    pub reg: c_uint,
    pub flags: c_ulong,
    pub type: pll_type,
}

//
// struct lgm_pll_clk_data
// @id: platform specific id of the clock.
// @name: name of this pll clock.
// @parent_data: parent clock data.
// @num_parents: number of parents.
// @flags: optional flags for basic clock.
// @type: platform type of pll.
// @reg: offset of the register.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lgm_pll_clk_data {
    pub id: c_uint,
    pub name: *const c_char,
    pub parent_data: *const clk_parent_data,
    pub num_parents: u8,
    pub flags: c_ulong,
    pub type: pll_type,
    pub reg: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lgm_clk_ddiv_data {
    pub id: c_uint,
    pub name: *const c_char,
    pub parent_data: *const clk_parent_data,
    pub flags: u8,
    pub div_flags: c_ulong,
    pub reg: c_uint,
    pub shift0: u8,
    pub width0: u8,
    pub shift1: u8,
    pub width1: u8,
    pub shift_gate: u8,
    pub width_gate: u8,
    pub ex_shift: u8,
    pub ex_width: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lgm_clk_branch {
    pub id: c_uint,
    pub type: lgm_clk_type,
    pub name: *const c_char,
    pub parent_data: *const clk_parent_data,
    pub num_parents: u8,
    pub flags: c_ulong,
    pub mux_off: c_uint,
    pub mux_shift: u8,
    pub mux_width: u8,
    pub mux_flags: c_ulong,
    pub mux_val: c_uint,
    pub div_off: c_uint,
    pub div_shift: u8,
    pub div_width: u8,
    pub div_shift_gate: u8,
    pub div_width_gate: u8,
    pub div_flags: c_ulong,
    pub div_val: c_uint,
    pub div_table: *const clk_div_table,
    pub gate_off: c_uint,
    pub gate_shift: u8,
    pub gate_flags: c_ulong,
    pub gate_val: c_uint,
    pub mult: c_uint,
    pub div: c_uint,
}

// clock flags definition

