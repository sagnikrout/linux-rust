//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/clk/qcom/clk-regmap.h
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
// Copyright (c) 2014, The Linux Foundation. All rights reserved.

//
// struct clk_regmap - regmap supporting clock
// @hw:		handle between common and hardware-specific interfaces
// @regmap:	regmap to use for regmap helpers and/or by providers
// @enable_reg: register when using regmap enable/disable ops
// @enable_mask: mask when using regmap enable/disable ops
// @enable_is_inverted: flag to indicate set enable_mask bits to disable
// when using clock_enable_regmap and friends APIs.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_regmap {
    pub hw: clk_hw,
    pub regmap: *mut regmap,
    pub enable_reg: c_uint,
    pub enable_mask: c_uint,
    pub enable_is_inverted: bool,
}

extern "C" {
    pub fn container_of(_arg: hw, clk_regmap: struct, _arg: hw) -> return;
}
extern "C" {
    pub fn clk_is_enabled_regmap(hw: *mut clk_hw) -> c_int;
}
extern "C" {
    pub fn clk_enable_regmap(hw: *mut clk_hw) -> c_int;
}
extern "C" {
    pub fn clk_disable_regmap(hw: *mut clk_hw);
}
extern "C" {
    pub fn devm_clk_register_regmap(dev: *mut device, rclk: *mut clk_regmap) -> c_int;
}
