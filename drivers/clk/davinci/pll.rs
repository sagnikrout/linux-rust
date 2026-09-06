//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/clk/davinci/pll.h
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
// Clock driver for TI Davinci PSC controllers
//
// Copyright (C) 2018 David Lechner <david@lechnology.com>
//

// davinci_pll_clk_info - controller-specific PLL info
// @name: The name of the PLL
// @unlock_reg: Option CFGCHIP register for unlocking PLL
// @unlock_mask: Bitmask used with @unlock_reg
// @pllm_mask: Bitmask for PLLM[PLLM] value
// @pllm_min: Minimum allowable value for PLLM[PLLM]
// @pllm_max: Maximum allowable value for PLLM[PLLM]
// @pllout_min_rate: Minimum allowable rate for PLLOUT
// @pllout_max_rate: Maximum allowable rate for PLLOUT
// @flags: Bitmap of PLL_* flags.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct davinci_pll_clk_info {
    pub name: *const c_char,
    pub unlock_reg: u32,
    pub unlock_mask: u32,
    pub pllm_mask: u32,
    pub pllm_min: u32,
    pub pllm_max: u32,
    pub pllout_min_rate: c_ulong,
    pub pllout_max_rate: c_ulong,
    pub flags: u32,
}

// davinci_pll_sysclk_info - SYSCLKn-specific info
// @name: The name of the clock
// @parent_name: The name of the parent clock
// @id: "n" in "SYSCLKn"
// @ratio_width: Width (in bits) of RATIO in PLLDIVn register
// @flags: Bitmap of SYSCLK_* flags.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct davinci_pll_sysclk_info {
    pub name: *const c_char,
    pub parent_name: *const c_char,
    pub id: u32,
    pub ratio_width: u32,
    pub flags: u32,
}

// davinci_pll_obsclk_info - OBSCLK-specific info
// @name: The name of the clock
// @parent_names: Array of names of the parent clocks
// @num_parents: Length of @parent_names
// @table: Array of values to write to OCSEL[OCSRC] corresponding to
// @parent_names
// @ocsrc_mask: Bitmask for OCSEL[OCSRC]
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct davinci_pll_obsclk_info {
    pub name: *const c_char,
    pub parent_names: *const *const c_char,
    pub num_parents: u8,
    pub table: *mut u32,
    pub ocsrc_mask: u32,
}

// Platform-specific callbacks
extern "C" {
    pub fn da850_pll1_init(dev: *mut device, base: *mut void __iomem, cfgchip: *mut regmap) -> c_int;
}
extern "C" {
    pub fn of_da850_pll0_init(node: *mut device_node);
}
extern "C" {
    pub fn of_da850_pll1_init(dev: *mut device, base: *mut void __iomem, cfgchip: *mut regmap) -> c_int;
}
