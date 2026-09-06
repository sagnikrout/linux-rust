//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/clk/qcom/clk-alpha-pll.h
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
// Copyright (c) 2015, 2018, 2021 The Linux Foundation. All rights reserved.
// Copyright (c) 2023 Qualcomm Innovation Center, Inc. All rights reserved.
//

// Alpha PLL types
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pll_vco {
    pub min_freq: c_ulong,
    pub max_freq: c_ulong,
    pub val: u32,
}

//
// struct clk_alpha_pll - phase locked loop (PLL)
// @offset: base address of registers
// @regs: alpha pll register map (see @clk_alpha_pll_regs)
// @config: array of pll settings
// @vco_table: array of VCO settings
// @num_vco: number of VCO settings in @vco_table
// @flags: bitmask to indicate features supported by the hardware
// @clkr: regmap clock handle
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_alpha_pll {
    pub offset: u32,
    pub regs: *const u8,
    pub config: *const alpha_pll_config,
    pub vco_table: *const pll_vco,
    pub num_vco: usize,

    pub flags: u8,
    pub clkr: clk_regmap,
}

//
// struct clk_alpha_pll_postdiv - phase locked loop (PLL) post-divider
// @offset: base address of registers
// @regs: alpha pll register map (see @clk_alpha_pll_regs)
// @width: width of post-divider
// @post_div_shift: shift to differentiate between odd & even post-divider
// @post_div_table: table with PLL odd and even post-divider settings
// @num_post_div: Number of PLL post-divider settings
//
// @clkr: regmap clock handle
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_alpha_pll_postdiv {
    pub offset: u32,
    pub width: u8,
    pub regs: *const u8,
    pub clkr: clk_regmap,
    pub post_div_shift: c_int,
    pub post_div_table: *const clk_div_table,
    pub num_post_div: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct alpha_pll_config {
    pub l: u32,
    pub cal_l: u32,
    pub alpha: u32,
    pub alpha_hi: u32,
    pub config_ctl_val: u32,
    pub config_ctl_hi_val: u32,
    pub config_ctl_hi1_val: u32,
    pub config_ctl_hi2_val: u32,
    pub user_ctl_val: u32,
    pub user_ctl_hi_val: u32,
    pub user_ctl_hi1_val: u32,
    pub test_ctl_val: u32,
    pub test_ctl_mask: u32,
    pub test_ctl_hi_val: u32,
    pub test_ctl_hi_mask: u32,
    pub test_ctl_hi1_val: u32,
    pub test_ctl_hi2_val: u32,
    pub test_ctl_hi3_val: u32,
    pub main_output_mask: u32,
    pub aux_output_mask: u32,
    pub aux2_output_mask: u32,
    pub early_output_mask: u32,
    pub alpha_en_mask: u32,
    pub alpha_mode_mask: u32,
    pub pre_div_val: u32,
    pub pre_div_mask: u32,
    pub post_div_val: u32,
    pub post_div_mask: u32,
    pub vco_val: u32,
    pub vco_mask: u32,
    pub status_val: u32,
    pub status_mask: u32,
    pub lock_det: u32,
}

extern "C" {
    pub fn qcom_clk_alpha_pll_configure(pll: *mut clk_alpha_pll, regmap: *mut regmap);
}
