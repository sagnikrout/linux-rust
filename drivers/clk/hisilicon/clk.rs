//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/clk/hisilicon/clk.h
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
// Hisilicon Hi3620 clock gate driver
//
// Copyright (c) 2012-2013 Hisilicon Limited.
// Copyright (c) 2012-2013 Linaro Limited.
//
// Author: Haojian Zhuang <haojian.zhuang@linaro.org>
// Xin Li <li.xin@linaro.org>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hisi_clock_data {
    pub clk_data: clk_onecell_data,
    pub base: *mut void __iomem,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hisi_fixed_rate_clock {
    pub id: c_uint,
    pub name: *mut c_char,
    pub parent_name: *const c_char,
    pub flags: c_ulong,
    pub fixed_rate: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hisi_fixed_factor_clock {
    pub id: c_uint,
    pub name: *mut c_char,
    pub parent_name: *const c_char,
    pub mult: c_ulong,
    pub div: c_ulong,
    pub flags: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hisi_mux_clock {
    pub id: c_uint,
    pub name: *const c_char,
    pub parent_names: *const *const c_char,
    pub num_parents: u8,
    pub flags: c_ulong,
    pub offset: c_ulong,
    pub shift: u8,
    pub width: u8,
    pub mux_flags: u8,
    pub table: *const u32,
    pub alias: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hisi_phase_clock {
    pub id: c_uint,
    pub name: *const c_char,
    pub parent_names: *const c_char,
    pub flags: c_ulong,
    pub offset: c_ulong,
    pub shift: u8,
    pub width: u8,
    pub phase_degrees: *mut u32,
    pub phase_regvals: *mut u32,
    pub phase_num: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hisi_divider_clock {
    pub id: c_uint,
    pub name: *const c_char,
    pub parent_name: *const c_char,
    pub flags: c_ulong,
    pub offset: c_ulong,
    pub shift: u8,
    pub width: u8,
    pub div_flags: u8,
    pub table: *mut clk_div_table,
    pub alias: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hi6220_divider_clock {
    pub id: c_uint,
    pub name: *const c_char,
    pub parent_name: *const c_char,
    pub flags: c_ulong,
    pub offset: c_ulong,
    pub shift: u8,
    pub width: u8,
    pub mask_bit: u32,
    pub alias: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hisi_gate_clock {
    pub id: c_uint,
    pub name: *const c_char,
    pub parent_name: *const c_char,
    pub flags: c_ulong,
    pub offset: c_ulong,
    pub bit_idx: u8,
    pub gate_flags: u8,
    pub alias: *const c_char,
}

