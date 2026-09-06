//! Automatically rewritten from C to Rust
//! Source: drivers/clk/rockchip/clk-half-divider.c
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
// Copyright (c) 2018 Fuzhou Rockchip Electronics Co., Ltd
//

    static bool _is_best_half_div(unsigned long rate, unsigned long now,
    unsigned long best, unsigned long flags)
    {
    if (flags & CLK_DIVIDER_ROUND_CLOSEST)
    return abs(rate - now) < abs(rate - best);
    return now <= rate && now > best;
    }
    static unsigned long clk_half_divider_recalc_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    struct clk_divider *divider = to_clk_divider(hw);
    unsigned int val;
    val = readl(divider.reg) >> divider.shift;
    val &= div_mask(divider.width);
    val = val * 2 + 3;
    return DIV_ROUND_UP_ULL(((u64)parent_rate * 2), val);
    }
    static int clk_half_divider_bestdiv(struct clk_hw *hw, unsigned long rate,
    unsigned long *best_parent_rate, u8 width,
    unsigned long flags)
    {
    unsigned int i, bestdiv = 0;
    unsigned long parent_rate, best = 0, now, maxdiv;
    let mut parent_rate_saved: c_ulong = *best_parent_rate;
    if (!rate)
    rate = 1;
    maxdiv = div_mask(width);
    if (!(clk_hw_get_flags(hw) & CLK_SET_RATE_PARENT)) {
    parent_rate = *best_parent_rate;
    bestdiv = DIV_ROUND_UP_ULL(((u64)parent_rate * 2), rate);
    if (bestdiv < 3)
    bestdiv = 0;
    else
    bestdiv = (bestdiv - 3) / 2;
    bestdiv = bestdiv > maxdiv ? maxdiv : bestdiv;
    return bestdiv;
    }
//
// The maximum divider we can use without overflowing
// unsigned long in rate * i below
//
    maxdiv = min(ULONG_MAX / rate, maxdiv);
    for (i = 0; i <= maxdiv; i++) {
    if (((u64)rate * (i * 2 + 3)) == ((u64)parent_rate_saved * 2)) {
//
// It's the most ideal case if the requested rate can be
// divided from parent clock without needing to change
// parent rate, so return the divider immediately.
//
// best_parent_rate = parent_rate_saved;
    return i;
    }
    parent_rate = clk_hw_round_rate(clk_hw_get_parent(hw),
    ((u64)rate * (i * 2 + 3)) / 2);
    now = DIV_ROUND_UP_ULL(((u64)parent_rate * 2),
    (i * 2 + 3));
    if (_is_best_half_div(rate, now, best, flags)) {
    bestdiv = i;
    best = now;
// best_parent_rate = parent_rate;
    }
    }
    if (!bestdiv) {
    bestdiv = div_mask(width);
// best_parent_rate = clk_hw_round_rate(clk_hw_get_parent(hw), 1);
    }
    return bestdiv;
    }
    static int clk_half_divider_determine_rate(struct clk_hw *hw,
    struct clk_rate_request *req)
    {
    struct clk_divider *divider = to_clk_divider(hw);
    int div;
    div = clk_half_divider_bestdiv(hw, req.rate, &req.best_parent_rate,
    divider.width,
    divider.flags);
    req.rate = DIV_ROUND_UP_ULL(((u64)req.best_parent_rate * 2), div * 2 + 3);
    return 0;
    }
    static int clk_half_divider_set_rate(struct clk_hw *hw, unsigned long rate,
    unsigned long parent_rate)
    {
    struct clk_divider *divider = to_clk_divider(hw);
    unsigned int value;
    let mut flags: c_ulong = 0;
    u32 val;
    value = DIV_ROUND_UP_ULL(((u64)parent_rate * 2), rate);
    value = (value - 3) / 2;
    value =  min_t(unsigned int, value, div_mask(divider.width));
    if (divider.lock)
    spin_lock_irqsave(divider.lock, flags);
    else
    __acquire(divider.lock);
    if (divider.flags & CLK_DIVIDER_HIWORD_MASK) {
    val = div_mask(divider.width) << (divider.shift + 16);
    } else {
    val = readl(divider.reg);
    val &= ~(div_mask(divider.width) << divider.shift);
    }
    val |= value << divider.shift;
    writel(val, divider.reg);
    if (divider.lock)
    spin_unlock_irqrestore(divider.lock, flags);
    else
    __release(divider.lock);
    return 0;
    }
    static const struct clk_ops clk_half_divider_ops = {
    .recalc_rate = clk_half_divider_recalc_rate,
    .determine_rate = clk_half_divider_determine_rate,
    .set_rate = clk_half_divider_set_rate,
    };
//
// Register a clock branch.
// Most clock branches have a form like
//
// src1 --|--\
// |M |--[GATE]-[DIV]-
// src2 --|--
//
// sometimes without one of those components.
//
    struct clk *rockchip_clk_register_halfdiv(const char *name,
    const char *const *parent_names,
    u8 num_parents, void __iomem *base,
    int muxdiv_offset, u8 mux_shift,
    u8 mux_width, u8 mux_flags,
    u8 div_shift, u8 div_width,
    u8 div_flags, int gate_offset,
    u8 gate_shift, u8 gate_flags,
    unsigned long flags,
    spinlock_t *lock)
    {
    struct clk_hw *hw = ERR_PTR(-ENOMEM);
    struct clk_mux *mux = core::ptr::null_mut();
    struct clk_gate *gate = core::ptr::null_mut();
    struct clk_divider *div = core::ptr::null_mut();
    const struct clk_ops *mux_ops = core::ptr::null_mut(), *div_ops = core::ptr::null_mut(),
// gate_ops = NULL;
    if (num_parents > 1) {
    mux = kzalloc_obj(*mux);
    if (!mux)
    return ERR_PTR(-ENOMEM);
    mux.reg = base + muxdiv_offset;
    mux.shift = mux_shift;
    mux.mask = BIT(mux_width) - 1;
    mux.flags = mux_flags;
    mux.lock = lock;
    mux_ops = (mux_flags & CLK_MUX_READ_ONLY) ? &clk_mux_ro_ops
    : &clk_mux_ops;
    }
    if (gate_offset >= 0) {
    gate = kzalloc_obj(*gate);
    if (!gate)
    goto err_gate;
    gate.flags = gate_flags;
    gate.reg = base + gate_offset;
    gate.bit_idx = gate_shift;
    gate.lock = lock;
    gate_ops = &clk_gate_ops;
    }
    if (div_width > 0) {
    div = kzalloc_obj(*div);
    if (!div)
    goto err_div;
    div.flags = div_flags;
    div.reg = base + muxdiv_offset;
    div.shift = div_shift;
    div.width = div_width;
    div.lock = lock;
    div_ops = &clk_half_divider_ops;
    }
    hw = clk_hw_register_composite(core::ptr::null_mut(), name, parent_names, num_parents,
    mux ? &mux.hw : core::ptr::null_mut(), mux_ops,
    div ? &div.hw : core::ptr::null_mut(), div_ops,
    gate ? &gate.hw : core::ptr::null_mut(), gate_ops,
    flags);
    if (IS_ERR(hw))
    goto err_div;
    return hw.clk;
    err_div:
    kfree(gate);
    err_gate:
    kfree(mux);
    return ERR_CAST(hw);
    }
