//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/clk/renesas/renesas-cpg-mssr.h
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
// Renesas Clock Pulse Generator / Module Standby and Software Reset
//
// Copyright (C) 2015 Glider bvba
//

//
// Definitions of CPG Core Clocks
//
// These include:
// - Clock outputs exported to DT
// - External input clocks
// - Internal CPG clocks
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpg_core_clk {
// Common
    pub name: *const c_char,
    pub id: c_uint,
    pub type: c_uint,
// Depending on type
    pub /: *mut *mut unsigned int parent; / Core Clocks only,
    pub div: c_uint,
    pub mult: c_uint,
    pub offset: c_uint,
    pub parent_names: *const *const c_char,
    pub dtable: *const clk_div_table,
}

//
// struct cpg_mssr_pub - data shared with device-specific clk registration code
//
// @base0: CPG/MSSR register block base0 address
// @base1: CPG/MSSR register block base1 address
// @notifiers: Notifier chain to save/restore clock state for system resume
// @rmw_lock: protects RMW register accesses
// @clks: pointer to clocks
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpg_mssr_pub {
    pub base0: *mut void __iomem,
    pub base1: *mut void __iomem,
    pub notifiers: raw_notifier_head,
    pub rmw_lock: spinlock_t,
    pub clks: *mut clk,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum clk_types {
// Generic
    CLK_TYPE_IN,		/* External Clock Input */
    CLK_TYPE_FF,		/* Fixed Factor Clock */
    CLK_TYPE_DIV6P1,	/* DIV6 Clock with 1 parent clock */
    CLK_TYPE_DIV6_RO,	/* DIV6 Clock read only with extra divisor */
    CLK_TYPE_FR,		/* Fixed Rate Clock */

// Custom definitions start here
    CLK_TYPE_CUSTOM,
}

//
// Definitions of Module Clocks
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mssr_mod_clk {
    pub name: *const c_char,
    pub id: c_uint,
    pub /: *mut *mut unsigned int parent; / Add MOD_CLK_BASE for Module Clocks,
}

// Convert from sparse base-100 to packed index space

// Convert from sparse base-10 to packed index space

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum clk_reg_layout {
    CLK_REG_LAYOUT_RCAR_GEN2_AND_GEN3 = 0,
    CLK_REG_LAYOUT_RZ_A,
    CLK_REG_LAYOUT_RCAR_GEN4,
    CLK_REG_LAYOUT_RZ_T2H,
}

//
// SoC-specific CPG/MSSR Description
//
// @early_core_clks: Array of Early Core Clock definitions
// @num_early_core_clks: Number of entries in early_core_clks[]
// @early_mod_clks: Array of Early Module Clock definitions
// @num_early_mod_clks: Number of entries in early_mod_clks[]
//
// @core_clks: Array of Core Clock definitions
// @num_core_clks: Number of entries in core_clks[]
// @last_dt_core_clk: ID of the last Core Clock exported to DT
// @num_total_core_clks: Total number of Core Clocks (exported + internal)
//
// @mod_clks: Array of Module Clock definitions
// @num_mod_clks: Number of entries in mod_clks[]
// @num_hw_mod_clks: Number of Module Clocks supported by the hardware
//
// @crit_mod_clks: Array with Module Clock IDs of critical clocks that
// should not be disabled without a knowledgeable driver
// @num_crit_mod_clks: Number of entries in crit_mod_clks[]
// @reg_layout: CPG/MSSR register layout from enum clk_reg_layout
//
// @core_pm_clks: Array with IDs of Core Clocks that are suitable for Power
// Management, in addition to Module Clocks
// @num_core_pm_clks: Number of entries in core_pm_clks[]
//
// @init: Optional callback to perform SoC-specific initialization
// @cpg_clk_register: Optional callback to handle special Core Clock types
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpg_mssr_info {
// Early Clocks
    pub early_core_clks: *const cpg_core_clk,
    pub num_early_core_clks: c_uint,
    pub early_mod_clks: *const mssr_mod_clk,
    pub num_early_mod_clks: c_uint,
// Core Clocks
    pub core_clks: *const cpg_core_clk,
    pub num_core_clks: c_uint,
    pub last_dt_core_clk: c_uint,
    pub num_total_core_clks: c_uint,
    pub reg_layout: clk_reg_layout,
// Module Clocks
    pub mod_clks: *const mssr_mod_clk,
    pub num_mod_clks: c_uint,
    pub num_hw_mod_clks: c_uint,
// Critical Module Clocks that should not be disabled
    pub crit_mod_clks: *const c_uint,
    pub num_crit_mod_clks: c_uint,
// Core Clocks suitable for PM, in addition to the Module Clocks
    pub core_pm_clks: *const c_uint,
    pub num_core_pm_clks: c_uint,
// Callbacks
    pub dev): *mut *mut int (init)(struct device,
    pub pub): *mut cpg_mssr_pub,
}

//
// Helpers for fixing up clock tables depending on SoC revision
//
