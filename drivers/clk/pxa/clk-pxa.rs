//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/clk/pxa/clk-pxa.h
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
// Marvell PXA family clocks
//
// Copyright (C) 2014 Robert Jarzmik
//
// Common clock code for PXA clocks ("CKEN" type clocks + DT)
//
pub const CLKCFG_TURBO: c_uint = 0x1;
pub const CLKCFG_FCS: c_uint = 0x2;
pub const CLKCFG_HALFTURBO: c_uint = 0x4;
pub const CLKCFG_FASTBUS: c_uint = 0x8;

//
// CKEN clock type
// This clock takes it source from 2 possible parents :
// - a low power parent
// - a normal parent
//
// +------------+     +-----------+
// |  Low Power | --- | x mult_lp |
// |    Clock   |     | / div_lp  |\
// +------------+     +-----------+ \+-----+   +-----------+
// | Mux |---| CKEN gate |
// +------------+     +-----------+ /+-----+   +-----------+
// | High Power |     | x mult_hp |
// |    Clock   | --- | / div_hp  |
// +------------+     +-----------+
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct desc_clk_cken {
    pub hw: clk_hw,
    pub ckid: c_int,
    pub cken_reg: c_int,
    pub name: *const c_char,
    pub dev_id: *const c_char,
    pub con_id: *const c_char,
    pub parent_names: *const *const c_char,
    pub lp: clk_fixed_factor,
    pub hp: clk_fixed_factor,
    pub gate: clk_gate,
    pub (*is_in_low_power)(void): *mut bool,
    pub flags: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pxa2xx_freq {
    pub cpll: c_ulong,
    pub membus_khz: c_uint,
    pub cccr: c_uint,
    pub div2: c_uint,
    pub clkcfg: c_uint,
}

extern "C" {
    pub fn clk_pxa_dt_common_init(np: *mut device_node);
}
extern "C" {
    pub fn pxa2xx_core_turbo_switch(on: bool);
}
