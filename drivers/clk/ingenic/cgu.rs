//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/clk/ingenic/cgu.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Ingenic SoC CGU driver
//
// Copyright (c) 2013-2015 Imagination Technologies
// Author: Paul Burton <paul.burton@mips.com>
//

//
// struct ingenic_cgu_pll_info - information about a PLL
// @reg: the offset of the PLL's control register within the CGU
// @rate_multiplier: the multiplier needed by pll rate calculation
// @m_shift: the number of bits to shift the multiplier value by (ie. the
// index of the lowest bit of the multiplier value in the PLL's
// control register)
// @m_bits: the size of the multiplier field in bits
// @m_offset: the multiplier value which encodes to 0 in the PLL's control
// register
// @n_shift: the number of bits to shift the divider value by (ie. the
// index of the lowest bit of the divider value in the PLL's
// control register)
// @n_bits: the size of the divider field in bits
// @n_offset: the divider value which encodes to 0 in the PLL's control
// register
// @od_shift: the number of bits to shift the post-VCO divider value by (ie.
// the index of the lowest bit of the post-VCO divider value in
// the PLL's control register)
// @od_bits: the size of the post-VCO divider field in bits, or 0 if no
// OD field exists (then the OD is fixed to 1)
// @od_max: the maximum post-VCO divider value
// @od_encoding: a pointer to an array mapping post-VCO divider values to
// their encoded values in the PLL control register, or -1 for
// unsupported values
// @bypass_reg: the offset of the bypass control register within the CGU
// @bypass_bit: the index of the bypass bit in the PLL control register, or
// -1 if there is no bypass bit
// @enable_bit: the index of the enable bit in the PLL control register, or
// -1 if there is no enable bit (ie, the PLL is always on)
// @stable_bit: the index of the stable bit in the PLL control register, or
// -1 if there is no stable bit
// @set_rate_hook: hook called immediately after updating the CGU register,
// before releasing the spinlock
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ingenic_cgu_pll_info {
    pub reg: unsigned,
    pub rate_multiplier: unsigned,
    pub od_encoding: *const i8,
    pub m_offset: u8 m_shift, m_bits,,
    pub n_offset: u8 n_shift, n_bits,,
    pub od_max: u8 od_shift, od_bits,,
    pub bypass_reg: unsigned,
    pub bypass_bit: i8,
    pub enable_bit: i8,
    pub stable_bit: i8,
    pub od): *mut *mut *mut unsigned int m, unsigned int n, unsigned int,
    pub parent_rate): unsigned long rate, unsigned long,
}

//
// struct ingenic_cgu_mux_info - information about a clock mux
// @reg: offset of the mux control register within the CGU
// @shift: number of bits to shift the mux value by (ie. the index of
// the lowest bit of the mux value within its control register)
// @bits: the size of the mux value in bits
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ingenic_cgu_mux_info {
    pub reg: unsigned,
    pub shift: u8,
    pub bits: u8,
}

//
// struct ingenic_cgu_div_info - information about a divider
// @reg: offset of the divider control register within the CGU
// @shift: number of bits to left shift the divide value by (ie. the index of
// the lowest bit of the divide value within its control register)
// @div: number to divide the divider value by (i.e. if the
// effective divider value is the value written to the register
// multiplied by some constant)
// @bits: the size of the divide value in bits
// @ce_bit: the index of the change enable bit within reg, or -1 if there
// isn't one
// @busy_bit: the index of the busy bit within reg, or -1 if there isn't one
// @stop_bit: the index of the stop bit within reg, or -1 if there isn't one
// @bypass_mask: mask of parent clocks for which the divider does not apply
// @div_table: optional table to map the value read from the register to the
// actual divider value
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ingenic_cgu_div_info {
    pub reg: unsigned,
    pub shift: u8,
    pub div: u8,
    pub bits: u8,
    pub ce_bit: i8,
    pub busy_bit: i8,
    pub stop_bit: i8,
    pub bypass_mask: u8,
    pub div_table: *const u8,
}

//
// struct ingenic_cgu_fixdiv_info - information about a fixed divider
// @div: the divider applied to the parent clock
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ingenic_cgu_fixdiv_info {
    pub div: unsigned,
}

//
// struct ingenic_cgu_gate_info - information about a clock gate
// @reg: offset of the gate control register within the CGU
// @bit: offset of the bit in the register that controls the gate
// @clear_to_gate: if set, the clock is gated when the bit is cleared
// @delay_us: delay in microseconds after which the clock is considered stable
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ingenic_cgu_gate_info {
    pub reg: unsigned,
    pub bit: u8,
    pub clear_to_gate: bool,
    pub delay_us: u16,
}

//
// struct ingenic_cgu_custom_info - information about a custom (SoC) clock
// @clk_ops: custom clock operation callbacks
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ingenic_cgu_custom_info {
    pub clk_ops: *const clk_ops,
}

//
// struct ingenic_cgu_clk_info - information about a clock
// @name: name of the clock
// @type: a bitmask formed from CGU_CLK_* values
// @flags: common clock flags to set on this clock
// @parents: an array of the indices of potential parents of this clock
// within the clock_info array of the CGU, or -1 in entries
// which correspond to no valid parent
// @pll: information valid if type includes CGU_CLK_PLL
// @gate: information valid if type includes CGU_CLK_GATE
// @mux: information valid if type includes CGU_CLK_MUX
// @div: information valid if type includes CGU_CLK_DIV
// @fixdiv: information valid if type includes CGU_CLK_FIXDIV
// @custom: information valid if type includes CGU_CLK_CUSTOM
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ingenic_cgu_clk_info {
    pub name: *const c_char,
    pub type: },
    pub flags: c_ulong,
    pub parents: [c_int; 4],
    pub pll: ingenic_cgu_pll_info,
    pub gate: ingenic_cgu_gate_info,
    pub mux: ingenic_cgu_mux_info,
    pub div: ingenic_cgu_div_info,
    pub fixdiv: ingenic_cgu_fixdiv_info,
}

//
// struct ingenic_cgu - data about the CGU
// @np: the device tree node that caused the CGU to be probed
// @base: the ioremap'ed base address of the CGU registers
// @clock_info: an array containing information about implemented clocks
// @clocks: used to provide clocks to DT, allows lookup of struct clk
// @lock: lock to be held whilst manipulating CGU registers
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ingenic_cgu {
    pub np: *mut device_node,
    pub base: *mut void __iomem,
    pub clock_info: *const ingenic_cgu_clk_info,
    pub clocks: clk_onecell_data,
    pub lock: spinlock_t,
}

//
// struct ingenic_clk - private data for a clock
// @hw: see Documentation/driver-api/clk.rst
// @cgu: a pointer to the CGU data
// @idx: the index of this clock in cgu->clock_info
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ingenic_clk {
    pub hw: clk_hw,
    pub cgu: *mut ingenic_cgu,
    pub idx: unsigned,
}

//
// ingenic_cgu_new() - create a new CGU instance
// @clock_info: an array of clock information structures describing the clocks
// which are implemented by the CGU
// @num_clocks: the number of entries in clock_info
// @np: the device tree node which causes this CGU to be probed
//
// Return: a pointer to the CGU instance if initialisation is successful,
// otherwise NULL.
//
// ingenic_cgu_register_clocks() - Registers the clocks
// @cgu: pointer to cgu data
//
// Register the clocks described by the CGU with the common clock framework.
//
// Return: 0 on success or -errno if unsuccessful.
//
extern "C" {
    pub fn ingenic_cgu_register_clocks(cgu: *mut ingenic_cgu) -> c_int;
}
