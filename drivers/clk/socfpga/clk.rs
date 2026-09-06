//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/clk/socfpga/clk.h
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
// Copyright (c) 2013, Steffen Trumtrar <s.trumtrar@pengutronix.de>
//
// based on drivers/clk/tegra/clk.h
//

// Clock Manager offsets
pub const CLKMGR_CTRL: c_uint = 0x0;
pub const CLKMGR_BYPASS: c_uint = 0x4;
pub const CLKMGR_DBCTRL: c_uint = 0x10;
pub const CLKMGR_L4SRC: c_uint = 0x70;
pub const CLKMGR_PERPLL_SRC: c_uint = 0xAC;
pub const SOCFPGA_MAX_PARENTS: c_int = 5;

extern "C" {
    pub fn socfpga_pll_init(node: *mut device_node) -> void __init;
}
extern "C" {
    pub fn socfpga_periph_init(node: *mut device_node) -> void __init;
}
extern "C" {
    pub fn socfpga_gate_init(node: *mut device_node) -> void __init;
}
extern "C" {
    pub fn socfpga_a10_pll_init(node: *mut device_node);
}
extern "C" {
    pub fn socfpga_a10_periph_init(node: *mut device_node);
}
extern "C" {
    pub fn socfpga_a10_gate_init(node: *mut device_node);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct socfpga_pll {
    pub hw: clk_gate,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct socfpga_gate_clk {
    pub hw: clk_gate,
    pub parent_name: *mut c_char,
    pub fixed_div: u32,
    pub div_reg: *mut void __iomem,
    pub bypass_reg: *mut void __iomem,
    pub sys_mgr_base_addr: *mut regmap,
    pub /: *mut *mut u32 width; / only valid if div_reg != 0,
    pub /: *mut *mut u32 shift; / only valid if div_reg != 0,
    pub /: *mut *mut u32 bypass_shift; / only valid if bypass_reg != 0,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct socfpga_periph_clk {
    pub hw: clk_gate,
    pub parent_name: *mut c_char,
    pub fixed_div: u32,
    pub div_reg: *mut void __iomem,
    pub bypass_reg: *mut void __iomem,
    pub /: *mut *mut u32 width; / only valid if div_reg != 0,
    pub /: *mut *mut u32 shift; / only valid if div_reg != 0,
    pub /: *mut *mut u32 bypass_shift; / only valid if bypass_reg != 0,
}
