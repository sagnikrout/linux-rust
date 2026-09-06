//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/clk/qcom/common.h
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
pub const PLL_LOCK_COUNT_SHIFT: c_int = 8;
pub const PLL_LOCK_COUNT_MASK: c_uint = 0x3f;
pub const PLL_BIAS_COUNT_SHIFT: c_int = 14;
pub const PLL_BIAS_COUNT_MASK: c_uint = 0x3f;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qcom_icc_hws_data {
    pub master_id: c_int,
    pub slave_id: c_int,
    pub clk_id: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qcom_cc_driver_data {
    pub alpha_plls: *mut clk_alpha_pll,
    pub num_alpha_plls: usize,
    pub clk_cbcrs: *const u32,
    pub num_clk_cbcrs: usize,
    pub dfs_rcgs: *const clk_rcg_dfs_data,
    pub num_dfs_rcgs: usize,
    pub regmap): *mut *mut *mut void (clk_regs_configure)(struct device dev, struct regmap,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qcom_cc_desc {
    pub config: *const regmap_config,
    pub clks: *mut clk_regmap,
    pub num_clks: usize,
    pub resets: *const qcom_reset_map,
    pub num_resets: usize,
    pub gdscs: *mut gdsc,
    pub num_gdscs: usize,
    pub clk_hws: *mut clk_hw,
    pub num_clk_hws: usize,
    pub icc_hws: *const qcom_icc_hws_data,
    pub num_icc_hws: usize,
    pub icc_first_node_id: c_uint,
    pub use_rpm: bool,
    pub driver_data: *const qcom_cc_driver_data,
}

//
// struct parent_map - map table for source select configuration values
// @src: source
// @cfg: configuration value
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct parent_map {
    pub src: u8,
    pub cfg: u8,
}

extern "C" {
    pub fn qcom_cc_register_sleep_clk(dev: *mut device) -> c_int;
}
