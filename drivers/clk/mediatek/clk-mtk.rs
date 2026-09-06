//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/clk/mediatek/clk-mtk.h
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
// Copyright (c) 2014 MediaTek Inc.
// Author: James Liao <jamesjj.liao@mediatek.com>
//

pub const MAX_MUX_GATE_BIT: c_int = 31;

pub const MTK_WAIT_HWV_DONE_US: c_int = 30;
//
// We need the clock IDs to start from zero but to maintain devicetree
// backwards compatibility we can't change bindings to start from zero.
// Only a few platforms are affected, so we solve issues given by the
// commonized MTK clocks probe function(s) by adding a dummy clock at
// the beginning where needed.
//
pub const CLK_DUMMY: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_fixed_clk {
    pub id: c_int,
    pub name: *const c_char,
    pub parent: *const c_char,
    pub rate: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_fixed_factor {
    pub id: c_int,
    pub name: *const c_char,
    pub parent_name: *const c_char,
    pub mult: c_int,
    pub div: c_int,
    pub flags: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_composite {
    pub id: c_int,
    pub name: *const c_char,
    pub parent_names: *const *const c_char,
    pub parent: *const c_char,
    pub flags: unsigned,
    pub mux_reg: u32,
    pub divider_reg: u32,
    pub gate_reg: u32,
    pub mux_shift: signed char,
    pub mux_width: signed char,
    pub gate_shift: signed char,
    pub divider_shift: signed char,
    pub divider_width: signed char,
    pub mux_flags: u8,
    pub num_parents: signed char,
}

//
// In case the rate change propagation to parent clocks is undesirable,
// this macro allows to specify the clock flags manually.
//

//
// Unless necessary, all MUX_GATE clocks propagate rate changes to their
// parent clock by default.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_clk_divider {
    pub id: c_int,
    pub name: *const c_char,
    pub parent_name: *const c_char,
    pub flags: c_ulong,
    pub div_reg: u32,
    pub div_shift: c_uchar,
    pub div_width: c_uchar,
    pub clk_divider_flags: c_uchar,
    pub clk_div_table: *const clk_div_table,
}

extern "C" {
    pub fn mtk_free_clk_data(clk_data: *mut clk_hw_onecell_data);
}
extern "C" {
    pub fn mtk_clk_unregister_ref2usb_tx(hw: *mut clk_hw);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_clk_desc {
    pub clks: *const mtk_gate,
    pub num_clks: usize,
    pub composite_clks: *const mtk_composite,
    pub num_composite_clks: usize,
    pub divider_clks: *const mtk_clk_divider,
    pub num_divider_clks: usize,
    pub fixed_clks: *const mtk_fixed_clk,
    pub num_fixed_clks: usize,
    pub factor_clks: *const mtk_fixed_factor,
    pub num_factor_clks: usize,
    pub mux_clks: *const mtk_mux,
    pub num_mux_clks: usize,
    pub rst_desc: *const mtk_clk_rst_desc,
    pub clk_lock: *mut spinlock_t,
    pub shared_io: bool,
    pub clk): *mut *mut *mut int (clk_notifier_func)(struct device dev, struct clk,
    pub mfg_clk_idx: c_uint,
    pub need_runtime_pm: bool,
}

extern "C" {
    pub fn mtk_clk_pdev_probe(pdev: *mut platform_device) -> c_int;
}
extern "C" {
    pub fn mtk_clk_pdev_remove(pdev: *mut platform_device);
}
extern "C" {
    pub fn mtk_clk_simple_probe(pdev: *mut platform_device) -> c_int;
}
extern "C" {
    pub fn mtk_clk_simple_remove(pdev: *mut platform_device);
}
