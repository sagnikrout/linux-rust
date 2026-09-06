//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/clk/ti/clock.h
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
// TI Clock driver internal definitions
//
// Copyright (C) 2014 Texas Instruments, Inc
// Tero Kristo (t-kristo@ti.com)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_omap_divider {
    pub hw: clk_hw,
    pub reg: clk_omap_reg,
    pub shift: u8,
    pub flags: u8,
    pub latch: i8,
    pub min: u16,
    pub max: u16,
    pub mask: u16,
    pub table: *const clk_div_table,
    pub context: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_omap_mux {
    pub hw: clk_hw,
    pub reg: clk_omap_reg,
    pub table: *mut u32,
    pub mask: u32,
    pub shift: u8,
    pub latch: i8,
    pub flags: u8,
    pub saved_parent: u8,
}

// Global flags

// Gate flags

// DPLL flags

// CLKCTRL flags

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ti_clk {
    pub name: *const c_char,
    pub clkdm_name: *const c_char,
    pub type: c_int,
    pub data: *mut c_void,
    pub patch: *mut ti_clk,
    pub clk: *mut clk,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ti_clk_mux {
    pub bit_shift: u8,
    pub num_parents: c_int,
    pub reg: u16,
    pub module: u8,
    pub parents: *const *const c_char,
    pub flags: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ti_clk_divider {
    pub parent: *const c_char,
    pub bit_shift: u8,
    pub max_div: u16,
    pub reg: u16,
    pub module: u8,
    pub dividers: *mut c_int,
    pub num_dividers: c_int,
    pub flags: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ti_clk_gate {
    pub parent: *const c_char,
    pub bit_shift: u8,
    pub reg: u16,
    pub module: u8,
    pub flags: u16,
}

// Composite clock component types
//
// struct ti_dt_clk - OMAP DT clock alias declarations
// @lk: clock lookup definition
// @node_name: clock DT node to map to
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ti_dt_clk {
    pub lk: clk_lookup,
    pub node_name: *mut c_char,
}

// CLKCTRL type definitions
#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap_clkctrl_div_data {
    pub dividers: *const c_int,
    pub max_div: c_int,
    pub flags: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap_clkctrl_bit_data {
    pub bit: u8,
    pub type: u8,
    pub parents: *const *const c_char,
    pub data: *const c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap_clkctrl_reg_data {
    pub offset: u16,
    pub bit_data: *const omap_clkctrl_bit_data,
    pub flags: u16,
    pub parent: *const c_char,
    pub clkdm_name: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap_clkctrl_data {
    pub addr: u32,
    pub regs: *const omap_clkctrl_reg_data,
}

extern "C" {
    pub fn void(: *mut *mut ti_of_clk_init_cb_t)(void, : *mut device_node) -> typedef;
}
extern "C" {
    pub fn ti_clk_add_alias(clk: *mut clk, con: *const c_char) -> c_int;
}
extern "C" {
    pub fn ti_clk_add_aliases();
}
extern "C" {
    pub fn ti_clk_latch(reg: *mut clk_omap_reg, shift: i8);
}
extern "C" {
    pub fn ti_clk_get_legacy_bit_shift(node: *mut device_node) -> c_int;
}
extern "C" {
    pub fn ti_dt_clocks_register(oclks: *mut ti_dt_clk);
}
extern "C" {
    pub fn ti_clk_add_component(node: *mut device_node, hw: *mut clk_hw, type: c_int) -> c_int;
}
extern "C" {
    pub fn of_ti_clk_autoidle_setup(node: *mut device_node) -> c_int;
}
extern "C" {
    pub fn omap2_clk_enable_init_clocks(clk_names: *const c_char, num_clocks: u8);
}
extern "C" {
    pub fn omap2_init_clk_clkdm(hw: *mut clk_hw) -> c_int;
}
extern "C" {
    pub fn omap2_clkops_enable_clkdm(hw: *mut clk_hw) -> c_int;
}
extern "C" {
    pub fn omap2_clkops_disable_clkdm(hw: *mut clk_hw);
}
extern "C" {
    pub fn omap2_dflt_clk_enable(hw: *mut clk_hw) -> c_int;
}
extern "C" {
    pub fn omap2_dflt_clk_disable(hw: *mut clk_hw);
}
extern "C" {
    pub fn omap2_dflt_clk_is_enabled(hw: *mut clk_hw) -> c_int;
}
extern "C" {
    pub fn omap2_clkt_iclk_allow_idle(clk: *mut clk_hw_omap);
}
extern "C" {
    pub fn omap2_clkt_iclk_deny_idle(clk: *mut clk_hw_omap);
}
extern "C" {
    pub fn omap2_init_dpll_parent(hw: *mut clk_hw) -> u8;
}
extern "C" {
    pub fn omap3_noncore_dpll_enable(hw: *mut clk_hw) -> c_int;
}
extern "C" {
    pub fn omap3_noncore_dpll_disable(hw: *mut clk_hw);
}
extern "C" {
    pub fn omap3_noncore_dpll_set_parent(hw: *mut clk_hw, index: u8) -> c_int;
}
extern "C" {
    pub fn omap2_dpll_determine_rate(hw: *mut clk_hw, req: *mut clk_rate_request) -> c_int;
}
//
// OMAP3_DPLL5_FREQ_FOR_USBHOST: USBHOST and USBTLL are the only clocks
// that are sourced by DPLL5, and both of these require this clock
// to be at 120 MHz for proper operation.
//
pub const OMAP3_DPLL5_FREQ_FOR_USBHOST: c_int = 120000000;
extern "C" {
    pub fn omap3_dpll_recalc(hw: *mut clk_hw, parent_rate: c_ulong) -> c_ulong;
}
extern "C" {
    pub fn omap3_clk_lock_dpll5();
}
extern "C" {
    pub fn omap2_clk_for_each(hw): *mut *mut int (fn)(struct clk_hw_omap) -> c_int;
}
