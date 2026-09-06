//! Automatically rewritten from C to Rust
//! Source: drivers/clk/mxs/clk-pll.c
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
// Copyright 2012 Freescale Semiconductor, Inc.
//

//
// struct clk_pll - mxs pll clock
// @hw: clk_hw for the pll
// @base: base address of the pll
// @power: the shift of power bit
// @rate: the clock rate of the pll
//
// The mxs pll is a fixed rate clock with power and gate control,
// and the shift of gate bit is always 31.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_pll {
    pub hw: clk_hw,
    pub base: *mut void __iomem,
    pub power: u8,
    pub rate: c_ulong,
}

#[no_mangle]
unsafe extern "C" fn clk_pll_prepare(hw: *mut clk_hw) -> c_int {
    static int clk_pll_prepare(struct clk_hw *hw)
    {
    struct clk_pll *pll = to_clk_pll(hw);
    writel_relaxed(1 << pll.power, pll.base + SET);
    udelay(10);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn clk_pll_unprepare(hw: *mut clk_hw) {
    static void clk_pll_unprepare(struct clk_hw *hw)
    {
    struct clk_pll *pll = to_clk_pll(hw);
    writel_relaxed(1 << pll.power, pll.base + CLR);
    }
#[no_mangle]
unsafe extern "C" fn clk_pll_enable(hw: *mut clk_hw) -> c_int {
    static int clk_pll_enable(struct clk_hw *hw)
    {
    struct clk_pll *pll = to_clk_pll(hw);
    writel_relaxed(1 << 31, pll.base + CLR);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn clk_pll_disable(hw: *mut clk_hw) {
    static void clk_pll_disable(struct clk_hw *hw)
    {
    struct clk_pll *pll = to_clk_pll(hw);
    writel_relaxed(1 << 31, pll.base + SET);
    }
    static unsigned long clk_pll_recalc_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    struct clk_pll *pll = to_clk_pll(hw);
    return pll.rate;
    }
    static const struct clk_ops clk_pll_ops = {
    .prepare = clk_pll_prepare,
    .unprepare = clk_pll_unprepare,
    .enable = clk_pll_enable,
    .disable = clk_pll_disable,
    .recalc_rate = clk_pll_recalc_rate,
    };
    struct clk *mxs_clk_pll(const char *name, const char *parent_name,
    void __iomem *base, u8 power, unsigned long rate)
    {
    struct clk_pll *pll;
    struct clk *clk;
    struct clk_init_data init;
    pll = kzalloc_obj(*pll);
    if (!pll)
    return ERR_PTR(-ENOMEM);
    init.name = name;
    init.ops = &clk_pll_ops;
    init.flags = 0;
    init.parent_names = (parent_name ? &parent_name: core::ptr::null_mut());
    init.num_parents = (parent_name ? 1 : 0);
    pll.base = base;
    pll.rate = rate;
    pll.power = power;
    pll.hw.init = &init;
    clk = clk_register(core::ptr::null_mut(), &pll.hw);
    if (IS_ERR(clk))
    kfree(pll);
    return clk;
    }
