//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/clk/renesas/rzv2h-cpg.h
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
// Renesas RZ/V2H(P) Clock Pulse Generator
//
// Copyright (C) 2024 Renesas Electronics Corp.
//

//
// struct pll - Structure for PLL configuration
//
// @offset: STBY register offset
// @has_clkn: Flag to indicate if CLK1/2 are accessible or not
// @instance: PLL instance number
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pll {
    pub offset:9: c_uint,
    pub has_clkn:1: c_uint,
    pub instance:2: c_uint,
    pub limits: *const rzv2h_pll_limits,
}

//
// struct ddiv - Structure for dynamic switching divider
//
// @offset: register offset
// @shift: position of the divider bit
// @width: width of the divider
// @monbit: monitor bit in CPG_CLKSTATUS0 register
// @no_rmw: flag to indicate if the register is read-modify-write
// (1: no RMW, 0: RMW)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ddiv {
    pub offset:11: c_uint,
    pub shift:4: c_uint,
    pub width:4: c_uint,
    pub monbit:5: c_uint,
    pub no_rmw:1: c_uint,
}

//
// On RZ/V2H(P), the dynamic divider clock supports up to 19 monitor bits,
// while on RZ/G3E, it supports up to 16 monitor bits. Use the maximum value
// `0x1f` to indicate that monitor bits are not supported for static divider
// clocks.
//

//
// struct smuxed - Structure for static muxed clocks
//
// @offset: register offset
// @shift: position of the divider field
// @width: width of the divider field
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smuxed {
    pub offset:11: c_uint,
    pub shift:4: c_uint,
    pub width:4: c_uint,
}

//
// struct fixed_mod_conf - Structure for fixed module configuration
//
// @mon_index: monitor index
// @mon_bit: monitor bit
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fixed_mod_conf {
    pub mon_index: u8,
    pub mon_bit: u8,
}

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
    pub ddiv: ddiv,
    pub pll: pll,
    pub smux: smuxed,
    pub fixed_mod: fixed_mod_conf,
    pub cfg: },
    pub dtable: *const clk_div_table,
    pub parent_names: *const *const c_char,
    pub num_parents: c_uint,
    pub mux_flags: u8,
    pub flag: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum clk_types {
// Generic
    CLK_TYPE_IN,		/* External Clock Input */
    CLK_TYPE_FF,		/* Fixed Factor Clock */
    CLK_TYPE_FF_MOD_STATUS,	/* Fixed Factor Clock which can report the status of module clock */
    CLK_TYPE_PLL,
    CLK_TYPE_DDIV,		/* Dynamic Switching Divider */
    CLK_TYPE_SMUX,		/* Static Mux */
    CLK_TYPE_PLLDSI,	/* PLLDSI */
    CLK_TYPE_PLLDSI_DIV,	/* PLLDSI divider */
    CLK_TYPE_PLLDSI_SMUX,	/* PLLDSI Static Mux */
}

//
// struct rzv2h_mod_clk - Module Clocks definitions
//
// @name: handle between common and hardware-specific interfaces
// @mstop_data: packed data mstop register offset and mask
// @parent: id of parent clock
// @critical: flag to indicate the clock is critical
// @no_pm: flag to indicate PM is not supported
// @on_index: control register index
// @on_bit: ON bit
// @mon_index: monitor register index
// @mon_bit: monitor bit
// @ext_clk_mux_index: mux index for external clock source, or -1 if internal
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rzv2h_mod_clk {
    pub name: *const c_char,
    pub mstop_data: u32,
    pub parent: u16,
    pub critical: bool,
    pub no_pm: bool,
    pub on_index: u8,
    pub on_bit: u8,
    pub mon_index: i8,
    pub mon_bit: u8,
    pub ext_clk_mux_index: i8,
}

//
// struct rzv2h_reset - Reset definitions
//
// @reset_index: reset register index
// @reset_bit: reset bit
// @mon_index: monitor register index
// @mon_bit: monitor bit
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rzv2h_reset {
    pub reset_index: u8,
    pub reset_bit: u8,
    pub mon_index: u8,
    pub mon_bit: u8,
}

//
// struct rzv2h_cpg_info - SoC-specific CPG Description
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
// @num_mstop_bits: Maximum number of MSTOP bits supported, equivalent to the
// number of CPG_BUS_m_MSTOP registers multiplied by 16.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rzv2h_cpg_info {
// Core Clocks
    pub core_clks: *const cpg_core_clk,
    pub num_core_clks: c_uint,
    pub last_dt_core_clk: c_uint,
    pub num_total_core_clks: c_uint,
// Module Clocks
    pub mod_clks: *const rzv2h_mod_clk,
    pub num_mod_clks: c_uint,
    pub num_hw_mod_clks: c_uint,
// Resets
    pub resets: *const rzv2h_reset,
    pub num_resets: c_uint,
    pub num_mstop_bits: c_uint,
}
