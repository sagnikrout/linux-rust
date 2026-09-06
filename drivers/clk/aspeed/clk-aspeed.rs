//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/clk/aspeed/clk-aspeed.h
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
// Structures used by ASPEED clock drivers
//
// Copyright 2019 IBM Corp.
//

//
// struct aspeed_gate_data - Aspeed gated clocks
// @clock_idx: bit used to gate this clock in the clock register
// @reset_idx: bit used to reset this IP in the reset register. -1 if no
// reset is required when enabling the clock
// @name: the clock name
// @parent_name: the name of the parent clock
// @flags: standard clock framework flags
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aspeed_gate_data {
    pub clock_idx: u8,
    pub reset_idx: i8,
    pub name: *const c_char,
    pub parent_name: *const c_char,
    pub flags: c_ulong,
}

//
// struct aspeed_clk_gate - Aspeed specific clk_gate structure
// @hw:		handle between common and hardware-specific interfaces
// @reg:	register controlling gate
// @clock_idx:	bit used to gate this clock in the clock register
// @reset_idx:	bit used to reset this IP in the reset register. -1 if no
// reset is required when enabling the clock
// @flags:	hardware-specific flags
// @lock:	register lock
//
// Some of the clocks in the Aspeed SoC must be put in reset before enabling.
// This modified version of clk_gate allows an optional reset bit to be
// specified.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aspeed_clk_gate {
    pub hw: clk_hw,
    pub map: *mut regmap,
    pub clock_idx: u8,
    pub reset_idx: i8,
    pub flags: u8,
    pub lock: *mut spinlock_t,
}

//
// struct aspeed_reset - Aspeed reset controller
// @map: regmap to access the containing system controller
// @rcdev: reset controller device
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aspeed_reset {
    pub map: *mut regmap,
    pub rcdev: reset_controller_dev,
}

//
// struct aspeed_clk_soc_data - Aspeed SoC specific divisor information
// @div_table: Common divider lookup table
// @eclk_div_table: Divider lookup table for ECLK
// @mac_div_table: Divider lookup table for MAC (Ethernet) clocks
// @calc_pll: Callback to maculate common PLL settings
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aspeed_clk_soc_data {
    pub div_table: *const clk_div_table,
    pub eclk_div_table: *const clk_div_table,
    pub mac_div_table: *const clk_div_table,
    pub val): *const *const *const *const clk_hw (calc_pll)(char name, u32,
}
