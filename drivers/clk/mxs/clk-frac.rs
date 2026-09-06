//! Automatically rewritten from C to Rust
//! Source: drivers/clk/mxs/clk-frac.c
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
// struct clk_frac - mxs fractional divider clock
// @hw: clk_hw for the fractional divider clock
// @reg: register address
// @shift: the divider bit shift
// @width: the divider bit width
// @busy: busy bit shift
//
// The clock is an adjustable fractional divider with a busy bit to wait
// when the divider is adjusted.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_frac {
    pub hw: clk_hw,
    pub reg: *mut void __iomem,
    pub shift: u8,
    pub width: u8,
    pub busy: u8,
}

    static unsigned long clk_frac_recalc_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    struct clk_frac *frac = to_clk_frac(hw);
    u32 div;
    u64 tmp_rate;
    div = readl_relaxed(frac.reg) >> frac.shift;
    div &= (1 << frac.width) - 1;
    tmp_rate = (u64)parent_rate * div;
    return tmp_rate >> frac.width;
    }
    static int clk_frac_determine_rate(struct clk_hw *hw,
    struct clk_rate_request *req)
    {
    struct clk_frac *frac = to_clk_frac(hw);
    let mut parent_rate: c_ulong = req.best_parent_rate;
    u32 div;
    u64 tmp, tmp_rate, result;
    if (req.rate > parent_rate)
    return -EINVAL;
    tmp = req.rate;
    tmp <<= frac.width;
    do_div(tmp, parent_rate);
    div = tmp;
    if (!div)
    return -EINVAL;
    tmp_rate = (u64)parent_rate * div;
    result = tmp_rate >> frac.width;
    if ((result << frac.width) < tmp_rate)
    result += 1;
    req.rate = result;
    return 0;
    }
    static int clk_frac_set_rate(struct clk_hw *hw, unsigned long rate,
    unsigned long parent_rate)
    {
    struct clk_frac *frac = to_clk_frac(hw);
    unsigned long flags;
    u32 div, val;
    u64 tmp;
    if (rate > parent_rate)
    return -EINVAL;
    tmp = rate;
    tmp <<= frac.width;
    do_div(tmp, parent_rate);
    div = tmp;
    if (!div)
    return -EINVAL;
    spin_lock_irqsave(&mxs_lock, flags);
    val = readl_relaxed(frac.reg);
    val &= ~(((1 << frac.width) - 1) << frac.shift);
    val |= div << frac.shift;
    writel_relaxed(val, frac.reg);
    spin_unlock_irqrestore(&mxs_lock, flags);
    return mxs_clk_wait(frac.reg, frac.busy);
    }
    static const struct clk_ops clk_frac_ops = {
    .recalc_rate = clk_frac_recalc_rate,
    .determine_rate = clk_frac_determine_rate,
    .set_rate = clk_frac_set_rate,
    };
    struct clk *mxs_clk_frac(const char *name, const char *parent_name,
    void __iomem *reg, u8 shift, u8 width, u8 busy)
    {
    struct clk_frac *frac;
    struct clk *clk;
    struct clk_init_data init;
    frac = kzalloc_obj(*frac);
    if (!frac)
    return ERR_PTR(-ENOMEM);
    init.name = name;
    init.ops = &clk_frac_ops;
    init.flags = CLK_SET_RATE_PARENT;
    init.parent_names = (parent_name ? &parent_name: core::ptr::null_mut());
    init.num_parents = (parent_name ? 1 : 0);
    frac.reg = reg;
    frac.shift = shift;
    frac.width = width;
    frac.busy = busy;
    frac.hw.init = &init;
    clk = clk_register(core::ptr::null_mut(), &frac.hw);
    if (IS_ERR(clk))
    kfree(frac);
    return clk;
    }
