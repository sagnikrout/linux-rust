//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/clk/meson/vclk.h
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
// Copyright (c) 2024 Neil Armstrong <neil.armstrong@linaro.org>
//

//
// struct meson_vclk_gate_data - vclk_gate regmap backed specific data
//
// @enable:	vclk enable field
// @reset:	vclk reset field
// @flags:	hardware-specific flags
//
// Flags:
// Same as clk_gate except CLK_GATE_HIWORD_MASK which is ignored
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct meson_vclk_gate_data {
    pub enable: parm,
    pub reset: parm,
    pub flags: u8,
}

//
// struct meson_vclk_div_data - vclk_div regmap back specific data
//
// @div:	divider field
// @enable:	vclk divider enable field
// @reset:	vclk divider reset field
// @table:	array of value/divider pairs, last entry should have div = 0
//
// Flags:
// Same as clk_divider except CLK_DIVIDER_HIWORD_MASK which is ignored
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct meson_vclk_div_data {
    pub div: parm,
    pub enable: parm,
    pub reset: parm,
    pub table: *const clk_div_table,
    pub flags: u8,
}
