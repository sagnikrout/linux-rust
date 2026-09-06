//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/clk/socfpga/stratix10-clk.h
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


// SPDX-License-Identifier:    GPL-2.0
//
// Copyright (C) 2017, Intel Corporation
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stratix10_clock_data {
    pub base: *mut void __iomem,
// Must be last
    pub clk_data: clk_hw_onecell_data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stratix10_pll_clock {
    pub id: c_uint,
    pub name: *const c_char,
    pub parent_data: *const clk_parent_data,
    pub num_parents: u8,
    pub flags: c_ulong,
    pub offset: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stratix10_perip_c_clock {
    pub id: c_uint,
    pub name: *const c_char,
    pub parent_name: *const c_char,
    pub parent_data: *const clk_parent_data,
    pub num_parents: u8,
    pub flags: c_ulong,
    pub offset: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct n5x_perip_c_clock {
    pub id: c_uint,
    pub name: *const c_char,
    pub parent_name: *const c_char,
    pub parent_names: *const *const c_char,
    pub num_parents: u8,
    pub flags: c_ulong,
    pub offset: c_ulong,
    pub shift: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stratix10_perip_cnt_clock {
    pub id: c_uint,
    pub name: *const c_char,
    pub parent_name: *const c_char,
    pub parent_data: *const clk_parent_data,
    pub num_parents: u8,
    pub flags: c_ulong,
    pub offset: c_ulong,
    pub fixed_divider: u8,
    pub bypass_reg: c_ulong,
    pub bypass_shift: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stratix10_gate_clock {
    pub id: c_uint,
    pub name: *const c_char,
    pub parent_name: *const c_char,
    pub parent_data: *const clk_parent_data,
    pub num_parents: u8,
    pub flags: c_ulong,
    pub gate_reg: c_ulong,
    pub gate_idx: u8,
    pub div_reg: c_ulong,
    pub div_offset: u8,
    pub div_width: u8,
    pub bypass_reg: c_ulong,
    pub bypass_shift: u8,
    pub fixed_div: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct agilex5_pll_clock {
    pub id: c_uint,
    pub name: *const c_char,
    pub parent_names: *const *const c_char,
    pub num_parents: u8,
    pub flags: c_ulong,
    pub offset: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct agilex5_perip_cnt_clock {
    pub id: c_uint,
    pub name: *const c_char,
    pub parent_names: *const *const c_char,
    pub num_parents: u8,
    pub flags: c_ulong,
    pub offset: c_ulong,
    pub fixed_divider: u8,
    pub bypass_reg: c_ulong,
    pub bypass_shift: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct agilex5_gate_clock {
    pub id: c_uint,
    pub name: *const c_char,
    pub parent_names: *const *const c_char,
    pub num_parents: u8,
    pub flags: c_ulong,
    pub gate_reg: c_ulong,
    pub gate_idx: u8,
    pub div_reg: c_ulong,
    pub div_offset: u8,
    pub div_width: u8,
    pub bypass_reg: c_ulong,
    pub bypass_shift: u8,
    pub fixed_div: u8,
}
