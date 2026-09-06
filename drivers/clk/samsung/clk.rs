//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/clk/samsung/clk.h
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
// Copyright (c) 2013 Samsung Electronics Co., Ltd.
// Copyright (c) 2013 Linaro Ltd.
// Author: Thomas Abraham <thomas.ab@samsung.com>
//
// Common Clock Framework support for all Samsung platforms
//

//
// struct samsung_clk_provider - information about clock provider
// @reg_base: virtual address for the register base
// @dev: clock provider device needed for runtime PM
// @sysreg: syscon regmap for clock-provider sysreg controller
// @lock: maintains exclusion between callbacks for a given clock-provider
// @auto_clock_gate: enable auto clk mode for all clocks in clock-provider
// @gate_dbg_offset: gate debug reg offset. Used for all gates in auto clk mode
// @option_offset: option reg offset. Enables auto mode for clock-provider
// @drcg_offset: dynamic root clk gate enable register offset in sysreg
// @memclk_offset: memclk enable register offset in sysreg
// @clk_data: holds clock related data like clk_hw* and number of clocks
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct samsung_clk_provider {
    pub reg_base: *mut void __iomem,
    pub dev: *mut device,
    pub sysreg: *mut regmap,
    pub lock: spinlock_t,
    pub auto_clock_gate: bool,
    pub gate_dbg_offset: u32,
    pub option_offset: u32,
    pub drcg_offset: u32,
    pub memclk_offset: u32,
// clk_data must be the last entry due to variable length 'hws' array
    pub clk_data: clk_hw_onecell_data,
}

//
// struct samsung_clock_alias - information about mux clock
// @id: platform specific id of the clock
// @dev_name: name of the device to which this clock belongs
// @alias: optional clock alias name to be assigned to this clock
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct samsung_clock_alias {
    pub id: c_uint,
    pub dev_name: *const c_char,
    pub alias: *const c_char,
}

//
// struct samsung_fixed_rate_clock - information about fixed-rate clock
// @id: platform specific id of the clock
// @name: name of this fixed-rate clock
// @parent_name: optional parent clock name
// @flags: optional fixed-rate clock flags
// @fixed_rate: fixed clock rate of this clock
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct samsung_fixed_rate_clock {
    pub id: c_uint,
    pub name: *mut c_char,
    pub parent_name: *const c_char,
    pub flags: c_ulong,
    pub fixed_rate: c_ulong,
}

//
// struct samsung_fixed_factor_clock - information about fixed-factor clock
// @id: platform specific id of the clock
// @name: name of this fixed-factor clock
// @parent_name: parent clock name
// @mult: fixed multiplication factor
// @div: fixed division factor
// @flags: optional fixed-factor clock flags
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct samsung_fixed_factor_clock {
    pub id: c_uint,
    pub name: *mut c_char,
    pub parent_name: *const c_char,
    pub mult: c_ulong,
    pub div: c_ulong,
    pub flags: c_ulong,
}

//
// struct samsung_mux_clock - information about mux clock
// @id: platform specific id of the clock
// @name: name of this mux clock
// @parent_names: array of pointer to parent clock names
// @num_parents: number of parents listed in @parent_names
// @flags: optional flags for basic clock
// @offset: offset of the register for configuring the mux
// @shift: starting bit location of the mux control bit-field in @reg
// @width: width of the mux control bit-field in @reg
// @mux_flags: flags for mux-type clock
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct samsung_mux_clock {
    pub id: c_uint,
    pub name: *const c_char,
    pub parent_names: *const *const c_char,
    pub num_parents: u8,
    pub flags: c_ulong,
    pub offset: c_ulong,
    pub shift: u8,
    pub width: u8,
    pub mux_flags: u8,
}

// Used by MUX clocks where reparenting on clock rate change is allowed.

//
// struct samsung_div_clock - information about div clock
// @id: platform specific id of the clock
// @name: name of this div clock
// @parent_name: name of the parent clock
// @flags: optional flags for basic clock
// @offset: offset of the register for configuring the div
// @shift: starting bit location of the div control bit-field in @reg
// @width: width of the bitfield
// @div_flags: flags for div-type clock
// @table: array of divider/value pairs ending with a div set to 0
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct samsung_div_clock {
    pub id: c_uint,
    pub name: *const c_char,
    pub parent_name: *const c_char,
    pub flags: c_ulong,
    pub offset: c_ulong,
    pub shift: u8,
    pub width: u8,
    pub div_flags: u8,
    pub table: *mut clk_div_table,
}

//
// struct samsung_gate_clock - information about gate clock
// @id: platform specific id of the clock
// @name: name of this gate clock
// @parent_name: name of the parent clock
// @flags: optional flags for basic clock
// @offset: offset of the register for configuring the gate
// @bit_idx: bit index of the gate control bit-field in @reg
// @gate_flags: flags for gate-type clock
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct samsung_gate_clock {
    pub id: c_uint,
    pub name: *const c_char,
    pub parent_name: *const c_char,
    pub flags: c_ulong,
    pub offset: c_ulong,
    pub bit_idx: u8,
    pub gate_flags: u8,
}

//
// struct samsung_clk_reg_dump - register dump of clock controller registers
// @offset: clock register offset from the controller base address
// @value: the value to be register at offset
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct samsung_clk_reg_dump {
    pub offset: u32,
    pub value: u32,
}

//
// struct samsung_pll_clock - information about pll clock
// @id: platform specific id of the clock
// @name: name of this pll clock
// @parent_name: name of the parent clock
// @flags: optional flags for basic clock
// @con_offset: offset of the register for configuring the PLL
// @lock_offset: offset of the register for locking the PLL
// @type: type of PLL to be registered
// @rate_table: array of PLL settings for possible PLL rates
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct samsung_pll_clock {
    pub id: c_uint,
    pub name: *const c_char,
    pub parent_name: *const c_char,
    pub flags: c_ulong,
    pub con_offset: c_int,
    pub lock_offset: c_int,
    pub type: samsung_pll_type,
    pub rate_table: *const samsung_pll_rate_table,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct samsung_cpu_clock {
    pub id: c_uint,
    pub name: *const c_char,
    pub parent_id: c_uint,
    pub alt_parent_id: c_uint,
    pub flags: c_ulong,
    pub offset: c_int,
    pub reg_layout: exynos_cpuclk_layout,
    pub cfg: *const exynos_cpuclk_cfg_data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct samsung_clock_reg_cache {
    pub node: list_head,
    pub reg_base: *mut void __iomem,
    pub sysreg: *mut regmap,
    pub rsuspend: *const samsung_clk_reg_dump,
    pub rsuspend_num: c_uint,
    pub rd_num: c_uint,
    pub __counted_by(rd_num): samsung_clk_reg_dump rdump[],
}

//
// struct samsung_cmu_info - all clocks information needed for CMU registration
// @pll_clks: list of PLL clocks
// @nr_pll_clks: count of clocks in @pll_clks
// @mux_clks: list of mux clocks
// @nr_mux_clks: count of clocks in @mux_clks
// @div_clks: list of div clocks
// @nr_div_clks: count of clocks in @div_clks
// @gate_clks: list of gate clocks
// @nr_gate_clks: count of clocks in @gate_clks
// @fixed_clks: list of fixed clocks
// @nr_fixed_clks: count clocks in @fixed_clks
// @fixed_factor_clks: list of fixed factor clocks
// @nr_fixed_factor_clks: count of clocks in @fixed_factor_clks
// @nr_clk_ids: total number of clocks with IDs assigned
// @cpu_clks: list of CPU clocks
// @nr_cpu_clks: count of clocks in @cpu_clks
// @clk_regs: list of clock registers
// @nr_clk_regs: count of clock registers in @clk_regs
// @suspend_regs: list of clock registers to set before suspend
// @nr_suspend_regs: count of clock registers in @suspend_regs
// @clk_name: name of the parent clock needed for CMU register access
// @sysreg_clk_regs: list of sysreg clock registers
// @nr_sysreg_clk_regs: count of clock registers in @sysreg_clk_regs
// @manual_plls: Enable manual control for PLL clocks
// @auto_clock_gate: enable auto clock mode for all components in CMU
// @gate_dbg_offset: gate debug reg offset. Used by all gates in auto clk mode
// @option_offset: option reg offset. Enables auto clk mode for entire CMU
// @drcg_offset: dynamic root clk gate enable register offset in sysreg
// @memclk_offset: memclk enable register offset in sysreg
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct samsung_cmu_info {
    pub pll_clks: *const samsung_pll_clock,
    pub nr_pll_clks: c_uint,
    pub mux_clks: *const samsung_mux_clock,
    pub nr_mux_clks: c_uint,
    pub div_clks: *const samsung_div_clock,
    pub nr_div_clks: c_uint,
    pub gate_clks: *const samsung_gate_clock,
    pub nr_gate_clks: c_uint,
    pub fixed_clks: *const samsung_fixed_rate_clock,
    pub nr_fixed_clks: c_uint,
    pub fixed_factor_clks: *const samsung_fixed_factor_clock,
    pub nr_fixed_factor_clks: c_uint,
    pub nr_clk_ids: c_uint,
    pub cpu_clks: *const samsung_cpu_clock,
    pub nr_cpu_clks: c_uint,
    pub clk_regs: *const c_ulong,
    pub nr_clk_regs: c_uint,
    pub suspend_regs: *const samsung_clk_reg_dump,
    pub nr_suspend_regs: c_uint,
    pub clk_name: *const c_char,
    pub sysreg_clk_regs: *const c_ulong,
    pub nr_sysreg_clk_regs: c_uint,
// ARM64 Exynos CMUs
    pub manual_plls: bool,
    pub auto_clock_gate: bool,
    pub gate_dbg_offset: u32,
    pub option_offset: u32,
    pub drcg_offset: u32,
    pub memclk_offset: u32,
}

extern "C" {
    pub fn samsung_is_auto_capable(np: *mut device_node) -> bool;
}
