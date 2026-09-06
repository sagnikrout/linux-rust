//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/clk/mediatek/clk-pll.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_pll_div_table {
    pub div: u32,
    pub freq: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_pll_data {
    pub id: c_int,
    pub name: *const c_char,
    pub reg: u32,
    pub pwr_reg: u32,
    pub en_mask: u32,
    pub fenc_sta_ofs: u32,
    pub pd_reg: u32,
    pub tuner_reg: u32,
    pub tuner_en_reg: u32,
    pub tuner_en_bit: u8,
    pub pd_shift: c_int,
    pub flags: c_uint,
    pub ops: *const clk_ops,
    pub rst_bar_mask: u32,
    pub fmin: c_ulong,
    pub fmax: c_ulong,
    pub pcwbits: c_int,
    pub pcwibits: c_int,
    pub pcw_reg: u32,
    pub pcw_shift: c_int,
    pub pcw_chg_reg: u32,
    pub div_table: *const mtk_pll_div_table,
    pub parent_name: *const c_char,
    pub en_reg: u32,
    pub en_set_reg: u32,
    pub en_clr_reg: u32,
    pub /: *mut *mut u8 pll_en_bit; / Assume 0, indicates BIT(0) by default,
    pub pcw_chg_bit: u8,
    pub fenc_sta_bit: u8,
}

//
// MediaTek PLLs are configured through their pcw value. The pcw value describes
// a divider in the PLL feedback loop which consists of 7 bits for the integer
// part and the remaining bits (if present) for the fractional part. Also they
// have a 3 bit power-of-two post divider.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_clk_pll {
    pub dev: *mut device,
    pub hw: clk_hw,
    pub base_addr: *mut void __iomem,
    pub pd_addr: *mut void __iomem,
    pub pwr_addr: *mut void __iomem,
    pub tuner_addr: *mut void __iomem,
    pub tuner_en_addr: *mut void __iomem,
    pub pcw_addr: *mut void __iomem,
    pub pcw_chg_addr: *mut void __iomem,
    pub en_addr: *mut void __iomem,
    pub en_set_addr: *mut void __iomem,
    pub en_clr_addr: *mut void __iomem,
    pub fenc_addr: *mut void __iomem,
    pub data: *const mtk_pll_data,
}

extern "C" {
    pub fn container_of(_arg: hw, mtk_clk_pll: struct, _arg: hw) -> return;
}
extern "C" {
    pub fn mtk_pll_is_prepared(hw: *mut clk_hw) -> c_int;
}
extern "C" {
    pub fn mtk_pll_prepare(hw: *mut clk_hw) -> c_int;
}
extern "C" {
    pub fn mtk_pll_unprepare(hw: *mut clk_hw);
}
extern "C" {
    pub fn mtk_pll_recalc_rate(hw: *mut clk_hw, parent_rate: c_ulong) -> c_ulong;
}
extern "C" {
    pub fn mtk_pll_determine_rate(hw: *mut clk_hw, req: *mut clk_rate_request) -> c_int;
}
extern "C" {
    pub fn mtk_clk_unregister_pll(hw: *mut clk_hw);
}
