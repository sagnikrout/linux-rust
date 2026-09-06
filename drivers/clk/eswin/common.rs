//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/clk/eswin/common.h
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
// Copyright 2026, Beijing ESWIN Computing Technology Co., Ltd..
// All rights reserved.
//
// Authors:
// Yifeng Huang <huangyifeng@eswincomputing.com>
// Xuyang Dong <dongxuyang@eswincomputing.com>
//
pub const APLL_HIGH_FREQ: c_int = 983040000;
pub const APLL_LOW_FREQ: c_int = 225792000;
pub const PLL_HIGH_FREQ: c_int = 1800000000;
pub const PLL_LOW_FREQ: c_int = 24000000;
//
// ESWIN_PRIV_DIV_MIN_2: If ESWIN_PRIV_DIV_MIN_2 is set, the minimum value of
// the register is 2, i.e. the minimum division ratio is 2.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum eswin_clk_type {
    CLK_FIXED_FACTOR,
    CLK_MUX,
    CLK_DIVIDER,
    CLK_GATE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct eswin_clock_data {
    pub base: *mut void __iomem,
    pub original_clk: *mut clk_hw,
    pub pll_nb: notifier_block,
    pub /: *mut *mut spinlock_t lock; / protect register read-modify-write cycle,
    pub clk_data: clk_hw_onecell_data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct eswin_divider_clock {
    pub hw: clk_hw,
    pub id: c_uint,
    pub name: *const c_char,
    pub parent_data: *const clk_parent_data,
    pub /: *mut *mut *mut void __iomem ctrl_reg; / register address of the divider clock,
    pub flags: c_ulong,
    pub /: *mut *mut unsigned long reg; / register offset,
    pub shift: u8,
    pub width: u8,
    pub div_flags: c_ulong,
    pub priv_flag: c_ulong,
    pub /: *mut *mut *mut spinlock_t lock; / protect register read-modify-write cycle,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct eswin_fixed_rate_clock {
    pub hw: clk_hw,
    pub id: c_uint,
    pub name: *const c_char,
    pub flags: c_ulong,
    pub rate: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct eswin_fixed_factor_clock {
    pub hw: clk_hw,
    pub id: c_uint,
    pub name: *const c_char,
    pub parent_data: *const clk_parent_data,
    pub mult: c_ulong,
    pub div: c_ulong,
    pub flags: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct eswin_gate_clock {
    pub hw: clk_hw,
    pub id: c_uint,
    pub name: *const c_char,
    pub parent_data: *const clk_parent_data,
    pub flags: c_ulong,
    pub reg: c_ulong,
    pub bit_idx: u8,
    pub gate_flags: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct eswin_mux_clock {
    pub hw: clk_hw,
    pub id: c_uint,
    pub name: *const c_char,
    pub parent_data: *const clk_parent_data,
    pub num_parents: u8,
    pub flags: c_ulong,
    pub reg: c_ulong,
    pub shift: u8,
    pub width: u8,
    pub mux_flags: u8,
    pub table: *mut u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct eswin_pll_clock {
    pub hw: clk_hw,
    pub id: u32,
    pub name: *const c_char,
    pub parent_data: *const clk_parent_data,
    pub ctrl_reg0: u32,
    pub fbdiv_shift: u8,
    pub ctrl_reg1: u32,
    pub frac_shift: u8,
    pub ctrl_reg2: u32,
    pub status_reg: u32,
    pub lock_shift: u8,
    pub lock_width: u8,
    pub max_rate: u64,
    pub min_rate: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct eswin_clk_pll {
    pub hw: clk_hw,
    pub id: u32,
    pub ctrl_reg0: *mut void __iomem,
    pub fbdiv_shift: u8,
    pub ctrl_reg1: *mut void __iomem,
    pub frac_shift: u8,
    pub ctrl_reg2: *mut void __iomem,
    pub status_reg: *mut void __iomem,
    pub lock_shift: u8,
    pub lock_width: u8,
    pub max_rate: u64,
    pub min_rate: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct eswin_clk_info {
    pub type: c_uint,
    pub pid: c_uint,
    pub id: c_uint,
    pub hw: clk_hw,
    pub div: eswin_divider_clock,
    pub factor: eswin_fixed_factor_clock,
    pub gate: eswin_gate_clock,
    pub mux: eswin_mux_clock,
    pub data: },
}

