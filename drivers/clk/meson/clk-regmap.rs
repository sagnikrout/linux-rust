//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/clk/meson/clk-regmap.h
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
// Copyright (c) 2018 BayLibre, SAS.
// Author: Jerome Brunet <jbrunet@baylibre.com>
//

//
// struct clk_regmap - regmap backed clock
//
// @hw:		handle between common and hardware-specific interfaces
// @map:	pointer to the regmap structure controlling the clock
// @data:	data specific to the clock type
//
// Clock which is controlled by regmap backed registers. The actual type of
// of the clock is controlled by the clock_ops and data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_regmap {
    pub hw: clk_hw,
    pub map: *mut regmap,
    pub data: *mut c_void,
}

extern "C" {
    pub fn container_of(_arg: hw, clk_regmap: struct, _arg: hw) -> return;
}
// clk_regmap init op to get and cache regmap from the controllers
extern "C" {
    pub fn clk_regmap_init(hw: *mut clk_hw) -> c_int;
}
//
// struct clk_regmap_gate_data - regmap backed gate specific data
//
// @offset:	offset of the register controlling gate
// @bit_idx:	single bit controlling gate
// @flags:	hardware-specific flags
//
// Flags:
// Same as clk_gate except CLK_GATE_HIWORD_MASK which is ignored
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_regmap_gate_data {
    pub offset: c_uint,
    pub bit_idx: u8,
    pub flags: u8,
}

//
// struct clk_regmap_div_data - regmap backed adjustable divider specific data
//
// @offset:	offset of the register controlling the divider
// @shift:	shift to the divider bit field
// @width:	width of the divider bit field
// @table:	array of value/divider pairs, last entry should have div = 0
//
// Flags:
// Same as clk_divider except CLK_DIVIDER_HIWORD_MASK which is ignored
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_regmap_div_data {
    pub offset: c_uint,
    pub shift: u8,
    pub width: u8,
    pub flags: u8,
    pub table: *const clk_div_table,
}

//
// struct clk_regmap_mux_data - regmap backed multiplexer clock specific data
//
// @hw:		handle between common and hardware-specific interfaces
// @offset:	offset of theregister controlling multiplexer
// @table:	array of parent indexed register values
// @shift:	shift to multiplexer bit field
// @mask:	mask of mutliplexer bit field
// @flags:	hardware-specific flags
//
// Flags:
// Same as clk_divider except CLK_MUX_HIWORD_MASK which is ignored
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_regmap_mux_data {
    pub offset: c_uint,
    pub table: *mut u32,
    pub mask: u32,
    pub shift: u8,
    pub flags: u8,
}
