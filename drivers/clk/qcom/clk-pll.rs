//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/clk/qcom/clk-pll.h
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
// Copyright (c) 2013, The Linux Foundation. All rights reserved.
//

//
// struct pll_freq_tbl - PLL frequency table
// @l: L value
// @m: M value
// @n: N value
// @ibits: internal values
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pll_freq_tbl {
    pub freq: c_ulong,
    pub l: u16,
    pub m: u16,
    pub n: u16,
    pub ibits: u32,
}

//
// struct clk_pll - phase locked loop (PLL)
// @l_reg: L register
// @m_reg: M register
// @n_reg: N register
// @config_reg: config register
// @mode_reg: mode register
// @status_reg: status register
// @status_bit: ANDed with @status_reg to determine if PLL is enabled
// @freq_tbl: PLL frequency table
// @hw: handle between common and hardware-specific interfaces
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_pll {
    pub l_reg: u32,
    pub m_reg: u32,
    pub n_reg: u32,
    pub config_reg: u32,
    pub mode_reg: u32,
    pub status_reg: u32,
    pub status_bit: u8,
    pub post_div_width: u8,
    pub post_div_shift: u8,
    pub freq_tbl: *const pll_freq_tbl,
    pub clkr: clk_regmap,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pll_config {
    pub l: u16,
    pub m: u32,
    pub n: u32,
    pub vco_val: u32,
    pub vco_mask: u32,
    pub pre_div_val: u32,
    pub pre_div_mask: u32,
    pub post_div_val: u32,
    pub post_div_mask: u32,
    pub mn_ena_mask: u32,
    pub main_output_mask: u32,
    pub aux_output_mask: u32,
}
