//! Automatically rewritten from C to Rust
//! Source: drivers/clk/mxs/clk-ref.c
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
// struct clk_ref - mxs reference clock
// @hw: clk_hw for the reference clock
// @reg: register address
// @idx: the index of the reference clock within the same register
//
// The mxs reference clock sources from pll.  Every 4 reference clocks share
// one register space, and @idx is used to identify them.  Each reference
// clock has a gate control and a fractional * divider.  The rate is calculated
// as pll rate  * (18 / FRAC), where FRAC = 18 ~ 35.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_ref {
    pub hw: clk_hw,
    pub reg: *mut void __iomem,
    pub idx: u8,
}

#[no_mangle]
unsafe extern "C" fn clk_ref_enable(hw: *mut clk_hw) -> c_int {
    static int clk_ref_enable(struct clk_hw *hw)
    {
    struct clk_ref *ref = to_clk_ref(hw);
    writel_relaxed(1 << ((ref.idx + 1) * 8 - 1), ref.reg + CLR);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn clk_ref_disable(hw: *mut clk_hw) {
    static void clk_ref_disable(struct clk_hw *hw)
    {
    struct clk_ref *ref = to_clk_ref(hw);
    writel_relaxed(1 << ((ref.idx + 1) * 8 - 1), ref.reg + SET);
    }
    static unsigned long clk_ref_recalc_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    struct clk_ref *ref = to_clk_ref(hw);
    let mut tmp: u64 = parent_rate;
    let mut frac: u8 = (readl_relaxed(ref.reg) >> (ref.idx * 8)) & 0x3f;
    tmp *= 18;
    do_div(tmp, frac);
    return tmp;
    }
    static int clk_ref_determine_rate(struct clk_hw *hw,
    struct clk_rate_request *req)
    {
    let mut parent_rate: c_ulong = req.best_parent_rate;
    let mut tmp: u64 = parent_rate;
    u8 frac;
    tmp = tmp * 18 + req.rate / 2;
    do_div(tmp, req.rate);
    frac = clamp(tmp, 18, 35);
    tmp = parent_rate;
    tmp *= 18;
    do_div(tmp, frac);
    req.rate = tmp;
    return 0;
    }
    static int clk_ref_set_rate(struct clk_hw *hw, unsigned long rate,
    unsigned long parent_rate)
    {
    struct clk_ref *ref = to_clk_ref(hw);
    unsigned long flags;
    let mut tmp: u64 = parent_rate;
    u32 val;
    u8 frac, shift = ref.idx * 8;
    tmp = tmp * 18 + rate / 2;
    do_div(tmp, rate);
    frac = clamp(tmp, 18, 35);
    spin_lock_irqsave(&mxs_lock, flags);
    val = readl_relaxed(ref.reg);
    val &= ~(0x3f << shift);
    val |= frac << shift;
    writel_relaxed(val, ref.reg);
    spin_unlock_irqrestore(&mxs_lock, flags);
    return 0;
    }
    static const struct clk_ops clk_ref_ops = {
    .enable		= clk_ref_enable,
    .disable	= clk_ref_disable,
    .recalc_rate	= clk_ref_recalc_rate,
    .determine_rate = clk_ref_determine_rate,
    .set_rate	= clk_ref_set_rate,
    };
    struct clk *mxs_clk_ref(const char *name, const char *parent_name,
    void __iomem *reg, u8 idx)
    {
    struct clk_ref *ref;
    struct clk *clk;
    struct clk_init_data init;
    ref = kzalloc_obj(*ref);
    if (!ref)
    return ERR_PTR(-ENOMEM);
    init.name = name;
    init.ops = &clk_ref_ops;
    init.flags = 0;
    init.parent_names = (parent_name ? &parent_name: core::ptr::null_mut());
    init.num_parents = (parent_name ? 1 : 0);
    ref.reg = reg;
    ref.idx = idx;
    ref.hw.init = &init;
    clk = clk_register(core::ptr::null_mut(), &ref.hw);
    if (IS_ERR(clk))
    kfree(ref);
    return clk;
    }
