//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/clk/qcom/clk-regmap-mux-div.h
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
// Copyright (c) 2017, Linaro Limited
// Author: Georgi Djakov <georgi.djakov@linaro.org>
//

//
// struct mux_div_clk - combined mux/divider clock
// @reg_offset: offset of the mux/divider register
// @hid_width:	number of bits in half integer divider
// @hid_shift:	lowest bit of hid value field
// @src_width:	number of bits in source select
// @src_shift:	lowest bit of source select field
// @div:	the divider raw configuration value
// @src:	the mux index which will be used if the clock is enabled
// @parent_map: map from parent_names index to src_sel field
// @clkr:	handle between common and hardware-specific interfaces
// @pclk:	the input PLL clock
// @clk_nb:	clock notifier for rate changes of the input PLL
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_regmap_mux_div {
    pub reg_offset: u32,
    pub hid_width: u32,
    pub hid_shift: u32,
    pub src_width: u32,
    pub src_shift: u32,
    pub div: u32,
    pub src: u32,
    pub parent_map: *const u32,
    pub clkr: clk_regmap,
    pub pclk: *mut clk,
    pub clk_nb: notifier_block,
}

extern "C" {
    pub fn mux_div_set_src_div(md: *mut clk_regmap_mux_div, src: u32, div: u32) -> c_int;
}
