//! Automatically rewritten from C to Rust
//! Source: drivers/clk/clk-multiplier.c
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
// Copyright (C) 2015 Maxime Ripard <maxime.ripard@free-electrons.com>
//

#[no_mangle]
pub unsafe extern "C" fn clk_mult_readl(mult: *mut clk_multiplier) -> u32 {
    static inline u32 clk_mult_readl(struct clk_multiplier *mult)
    {
    if (mult.flags & CLK_MULTIPLIER_BIG_ENDIAN)
    return ioread32be(mult.reg);
    return readl(mult.reg);
    }
#[no_mangle]
pub unsafe extern "C" fn clk_mult_writel(mult: *mut clk_multiplier, val: u32) {
    static inline void clk_mult_writel(struct clk_multiplier *mult, u32 val)
    {
    if (mult.flags & CLK_MULTIPLIER_BIG_ENDIAN)
    iowrite32be(val, mult.reg);
    else
    writel(val, mult.reg);
    }
    static unsigned long __get_mult(struct clk_multiplier *mult,
    unsigned long rate,
    unsigned long parent_rate)
    {
    if (mult.flags & CLK_MULTIPLIER_ROUND_CLOSEST)
    return DIV_ROUND_CLOSEST(rate, parent_rate);
    return rate / parent_rate;
    }
    static unsigned long clk_multiplier_recalc_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    struct clk_multiplier *mult = to_clk_multiplier(hw);
    unsigned long val;
    val = clk_mult_readl(mult) >> mult.shift;
    val &= GENMASK(mult.width - 1, 0);
    if (!val && mult.flags & CLK_MULTIPLIER_ZERO_BYPASS)
    val = 1;
    return parent_rate * val;
    }
    static bool __is_best_rate(unsigned long rate, unsigned long new,
    unsigned long best, unsigned long flags)
    {
    if (flags & CLK_MULTIPLIER_ROUND_CLOSEST)
    return abs(rate - new) < abs(rate - best);
    return new >= rate && new < best;
    }
    static unsigned long __bestmult(struct clk_hw *hw, unsigned long rate,
    unsigned long *best_parent_rate,
    u8 width, unsigned long flags)
    {
    struct clk_multiplier *mult = to_clk_multiplier(hw);
    let mut orig_parent_rate: c_ulong = *best_parent_rate;
    unsigned long parent_rate, current_rate, best_rate = ~0;
    unsigned int i, bestmult = 0;
    let mut maxmult: c_uint = (1 << width) - 1;
    if (!(clk_hw_get_flags(hw) & CLK_SET_RATE_PARENT)) {
    bestmult = rate / orig_parent_rate;
// Make sure we don't end up with a 0 multiplier
    if ((bestmult == 0) &&
    !(mult.flags & CLK_MULTIPLIER_ZERO_BYPASS))
    bestmult = 1;
// Make sure we don't overflow the multiplier
    if (bestmult > maxmult)
    bestmult = maxmult;
    return bestmult;
    }
    for (i = 1; i < maxmult; i++) {
    if (rate == orig_parent_rate * i) {
//
// This is the best case for us if we have a
// perfect match without changing the parent
// rate.
//
// best_parent_rate = orig_parent_rate;
    return i;
    }
    parent_rate = clk_hw_round_rate(clk_hw_get_parent(hw),
    rate / i);
    current_rate = parent_rate * i;
    if (__is_best_rate(rate, current_rate, best_rate, flags)) {
    bestmult = i;
    best_rate = current_rate;
// best_parent_rate = parent_rate;
    }
    }
    return bestmult;
    }
    static int clk_multiplier_determine_rate(struct clk_hw *hw,
    struct clk_rate_request *req)
    {
    struct clk_multiplier *mult = to_clk_multiplier(hw);
    unsigned long factor = __bestmult(hw, req.rate, &req.best_parent_rate,
    mult.width, mult.flags);
    req.rate = req.best_parent_rate * factor;
    return 0;
    }
    static int clk_multiplier_set_rate(struct clk_hw *hw, unsigned long rate,
    unsigned long parent_rate)
    {
    struct clk_multiplier *mult = to_clk_multiplier(hw);
    let mut factor: c_ulong = __get_mult(mult, rate, parent_rate);
    let mut flags: c_ulong = 0;
    unsigned long val;
    if (mult.lock)
    spin_lock_irqsave(mult.lock, flags);
    else
    __acquire(mult.lock);
    val = clk_mult_readl(mult);
    val &= ~GENMASK(mult.width + mult.shift - 1, mult.shift);
    val |= factor << mult.shift;
    clk_mult_writel(mult, val);
    if (mult.lock)
    spin_unlock_irqrestore(mult.lock, flags);
    else
    __release(mult.lock);
    return 0;
    }
    const struct clk_ops clk_multiplier_ops = {
    .recalc_rate	= clk_multiplier_recalc_rate,
    .determine_rate = clk_multiplier_determine_rate,
    .set_rate	= clk_multiplier_set_rate,
    };
    EXPORT_SYMBOL_GPL(clk_multiplier_ops);
