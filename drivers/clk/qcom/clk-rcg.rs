//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/clk/qcom/clk-rcg.h
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
// Copyright (c) 2013, 2018, The Linux Foundation. All rights reserved.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct freq_tbl {
    pub freq: c_ulong,
    pub src: u8,
    pub pre_div: u8,
    pub m: u16,
    pub n: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct freq_conf {
    pub src: u8,
    pub pre_div: u8,
    pub m: u16,
    pub n: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct freq_multi_tbl {
    pub freq: c_ulong,
    pub num_confs: usize,
    pub confs: *const freq_conf,
}

//
// struct mn - M/N:D counter
// @mnctr_en_bit: bit to enable mn counter
// @mnctr_reset_bit: bit to assert mn counter reset
// @mnctr_mode_shift: lowest bit of mn counter mode field
// @n_val_shift: lowest bit of n value field
// @m_val_shift: lowest bit of m value field
// @width: number of bits in m/n/d values
// @reset_in_cc: true if the mnctr_reset_bit is in the CC register
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mn {
    pub mnctr_en_bit: u8,
    pub mnctr_reset_bit: u8,
    pub mnctr_mode_shift: u8,
pub const MNCTR_MODE_DUAL: c_uint = 0x2;
pub const MNCTR_MODE_MASK: c_uint = 0x3;
    pub n_val_shift: u8,
    pub m_val_shift: u8,
    pub width: u8,
    pub reset_in_cc: bool,
}

//
// struct pre_div - pre-divider
// @pre_div_shift: lowest bit of pre divider field
// @pre_div_width: number of bits in predivider
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pre_div {
    pub pre_div_shift: u8,
    pub pre_div_width: u8,
}

//
// struct src_sel - source selector
// @src_sel_shift: lowest bit of source selection field
// @parent_map: map from software's parent index to hardware's src_sel field
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct src_sel {
    pub src_sel_shift: u8,
pub const SRC_SEL_MASK: c_uint = 0x7;
    pub parent_map: *const parent_map,
}

//
// struct clk_rcg - root clock generator
//
// @ns_reg: NS register
// @md_reg: MD register
// @mn: mn counter
// @p: pre divider
// @s: source selector
// @freq_tbl: frequency table
// @clkr: regmap clock handle
// @lock: register lock
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_rcg {
    pub ns_reg: u32,
    pub md_reg: u32,
    pub mn: mn,
    pub p: pre_div,
    pub s: src_sel,
    pub freq_tbl: *const freq_tbl,
    pub clkr: clk_regmap,
}

//
// struct clk_dyn_rcg - root clock generator with glitch free mux
//
// @mux_sel_bit: bit to switch glitch free mux
// @ns_reg: NS0 and NS1 register
// @md_reg: MD0 and MD1 register
// @bank_reg: register to XOR @mux_sel_bit into to switch glitch free mux
// @mn: mn counter (banked)
// @s: source selector (banked)
// @freq_tbl: frequency table
// @clkr: regmap clock handle
// @lock: register lock
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_dyn_rcg {
    pub ns_reg: [u32; 2],
    pub md_reg: [u32; 2],
    pub bank_reg: u32,
    pub mux_sel_bit: u8,
    pub mn: [mn; 2],
    pub p: [pre_div; 2],
    pub s: [src_sel; 2],
    pub freq_tbl: *const freq_tbl,
    pub clkr: clk_regmap,
}

//
// struct clk_rcg2 - root clock generator
//
// @cmd_rcgr: corresponds to *_CMD_RCGR
// @mnd_width: number of bits in m/n/d values
// @hid_width: number of bits in half integer divider
// @safe_src_index: safe src index value
// @parent_map: map from software's parent index to hardware's src_sel field
// @freq_tbl: frequency table
// @freq_multi_tbl: frequency table for clocks reachable with multiple RCGs conf
// @clkr: regmap clock handle
// @cfg_off: defines the cfg register offset from the CMD_RCGR + CFG_REG
// @parked_cfg: cached value of the CFG register for parked RCGs
// @hw_clk_ctrl: whether to enable hardware clock control
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_rcg2 {
    pub cmd_rcgr: u32,
    pub mnd_width: u8,
    pub hid_width: u8,
    pub safe_src_index: u8,
    pub parent_map: *const parent_map,
    pub freq_tbl: *const freq_tbl,
    pub freq_multi_tbl: *const freq_multi_tbl,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_rcg2_gfx3d {
    pub div: u8,
    pub rcg: clk_rcg2,
    pub hws: *mut clk_hw,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_rcg_dfs_data {
    pub rcg: *mut clk_rcg2,
    pub init: *mut clk_init_data,
}

