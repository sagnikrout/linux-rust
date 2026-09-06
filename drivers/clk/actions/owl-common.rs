//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/clk/actions/owl-common.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// OWL common clock driver
//
// Copyright (c) 2014 Actions Semi Inc.
// Author: David Liu <liuwei@actions-semi.com>
//
// Copyright (c) 2018 Linaro Ltd.
// Author: Manivannan Sadhasivam <manivannan.sadhasivam@linaro.org>

#[repr(C)]
#[derive(Copy, Clone)]
pub struct owl_clk_common {
    pub regmap: *mut regmap,
    pub hw: clk_hw,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct owl_clk_desc {
    pub clks: *mut owl_clk_common,
    pub num_clks: c_ulong,
    pub hw_clks: *mut clk_hw_onecell_data,
    pub resets: *const owl_reset_map,
    pub num_resets: c_ulong,
    pub regmap: *mut regmap,
}

extern "C" {
    pub fn container_of(_arg: hw, owl_clk_common: struct, _arg: hw) -> return;
}
extern "C" {
    pub fn owl_clk_probe(dev: *mut device, hw_clks: *mut clk_hw_onecell_data) -> c_int;
}
