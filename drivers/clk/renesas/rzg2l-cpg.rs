//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/clk/renesas/rzg2l-cpg.h
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
// RZ/G2L Clock Pulse Generator
//
// Copyright (C) 2021 Renesas Electronics Corp.
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
    pub name: *const c_char,
    pub id: c_uint,
    pub parent: c_uint,
    pub div: c_uint,
    pub mult: c_uint,
    pub type: c_uint,
    pub conf: c_uint,
    pub sconf: c_uint,
    pub dtable: *const clk_div_table,
    pub mtable: *const u32,
    pub invalid_rate: c_ulong,
    pub max_rate: c_ulong,
    pub default_rate: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum clk_types {
// Generic
    CLK_TYPE_IN,		/* External Clock Input */
    CLK_TYPE_FF,		/* Fixed Factor Clock */
    CLK_TYPE_SAM_PLL,
    CLK_TYPE_G3L_PLL,
    CLK_TYPE_G3S_PLL,

// Clock with divider
    CLK_TYPE_DIV,
    CLK_TYPE_G3S_DIV,

// Clock with clock source selector
    CLK_TYPE_MUX,

// Clock with SD clock source selector
    CLK_TYPE_SD_MUX,

// Clock for SIPLL5
    CLK_TYPE_SIPLL5,

// Clock for PLL5_4 clock source selector
    CLK_TYPE_PLL5_4_MUX,

// Clock for DSI divider
    CLK_TYPE_DSI_DIV,

}

//
// struct rzg2l_mod_clk - Module Clocks definitions
//
// @name: handle between common and hardware-specific interfaces
// @id: clock index in array containing all Core and Module Clocks
// @parent: id of parent clock
// @mstop_conf: MSTOP configuration
// @off: register offset
// @bit: ON/MON bit
// @is_coupled: flag to indicate coupled clock
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rzg2l_mod_clk {
    pub name: *const c_char,
    pub id: c_uint,
    pub parent: c_uint,
    pub mstop_conf: u32,
    pub off: u16,
    pub bit: u8,
    pub is_coupled: bool,
}

//
// struct rzg2l_reset - Reset definitions
//
// @off: register offset
// @bit: reset bit
// @monbit: monitor bit in CPG_RST_MON register, -1 if none
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rzg2l_reset {
    pub off: u16,
    pub bit: u8,
    pub monbit: i8,
}

//
// struct rzg2l_cpg_info - SoC-specific CPG Description
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
// @resets: Array of Module Reset definitions
// @num_resets: Number of entries in resets[]
//
// @crit_mod_clks: Array with Module Clock IDs of critical clocks that
// should not be disabled without a knowledgeable driver
// @num_crit_mod_clks: Number of entries in crit_mod_clks[]
// @crit_resets: Array with Reset IDs of critical resets that should not be
// asserted without a knowledgeable driver
// @num_crit_resets: Number of entries in crit_resets[]
// @has_clk_mon_regs: Flag indicating whether the SoC has CLK_MON registers
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rzg2l_cpg_info {
// Core Clocks
    pub core_clks: *const cpg_core_clk,
    pub num_core_clks: c_uint,
    pub last_dt_core_clk: c_uint,
    pub num_total_core_clks: c_uint,
// Module Clocks
    pub mod_clks: *const rzg2l_mod_clk,
    pub num_mod_clks: c_uint,
    pub num_hw_mod_clks: c_uint,
// No PM Module Clocks
    pub no_pm_mod_clks: *const c_uint,
    pub num_no_pm_mod_clks: c_uint,
// Resets
    pub resets: *const rzg2l_reset,
    pub num_resets: c_uint,
// Critical Module Clocks that should not be disabled
    pub crit_mod_clks: *const c_uint,
    pub num_crit_mod_clks: c_uint,
// Critical Resets that should not be asserted
    pub crit_resets: *const c_uint,
    pub num_crit_resets: c_uint,
    pub has_clk_mon_regs: bool,
}

extern "C" {
    pub fn rzg2l_cpg_sd_clk_mux_notifier(nb: *mut notifier_block, event: c_ulong, data: *mut c_void) -> c_int;
}
extern "C" {
    pub fn rzg3s_cpg_div_clk_notifier(nb: *mut notifier_block, event: c_ulong, data: *mut c_void) -> c_int;
}
