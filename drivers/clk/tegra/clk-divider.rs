//! Automatically rewritten from C to Rust
//! Source: drivers/clk/tegra/clk-divider.c
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
// Copyright (c) 2012, NVIDIA CORPORATION.  All rights reserved.
//

    static int get_div(struct tegra_clk_frac_div *divider, unsigned long rate,
    unsigned long parent_rate)
    {
    int div;
    div = div_frac_get(rate, parent_rate, divider.width,
    divider.frac_width, divider.flags);
    if (div < 0)
    return 0;
    return div;
    }
    static unsigned long clk_frac_div_recalc_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    struct tegra_clk_frac_div *divider = to_clk_frac_div(hw);
    u32 reg;
    int div, mul;
    let mut rate: u64 = parent_rate;
    reg = readl_relaxed(divider.reg);
    if ((divider.flags & TEGRA_DIVIDER_UART) &&
    !(reg & PERIPH_CLK_UART_DIV_ENB))
    return rate;
    div = (reg >> divider.shift) & div_mask(divider);
    mul = get_mul(divider);
    div += mul;
    rate *= mul;
    rate += div - 1;
    do_div(rate, div);
    return rate;
    }
    static int clk_frac_div_determine_rate(struct clk_hw *hw,
    struct clk_rate_request *req)
    {
    struct tegra_clk_frac_div *divider = to_clk_frac_div(hw);
    int div, mul;
    let mut output_rate: c_ulong = req.best_parent_rate;
    if (!req.rate) {
    req.rate = output_rate;
    return 0;
    }
    div = get_div(divider, req.rate, output_rate);
    if (div < 0) {
    req.rate = req.best_parent_rate;
    return 0;
    }
    mul = get_mul(divider);
    req.rate = DIV_ROUND_UP(output_rate * mul, div + mul);
    return 0;
    }
    static int clk_frac_div_set_rate(struct clk_hw *hw, unsigned long rate,
    unsigned long parent_rate)
    {
    struct tegra_clk_frac_div *divider = to_clk_frac_div(hw);
    int div;
    let mut flags: c_ulong = 0;
    u32 val;
    div = get_div(divider, rate, parent_rate);
    if (div < 0)
    return div;
    if (divider.lock)
    spin_lock_irqsave(divider.lock, flags);
    val = readl_relaxed(divider.reg);
    val &= ~(div_mask(divider) << divider.shift);
    val |= div << divider.shift;
    if (divider.flags & TEGRA_DIVIDER_UART) {
    if (div)
    val |= PERIPH_CLK_UART_DIV_ENB;
    else
    val &= ~PERIPH_CLK_UART_DIV_ENB;
    }
    if (divider.flags & TEGRA_DIVIDER_FIXED)
    val |= pll_out_override(divider);
    writel_relaxed(val, divider.reg);
    if (divider.lock)
    spin_unlock_irqrestore(divider.lock, flags);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn clk_divider_restore_context(hw: *mut clk_hw) {
    static void clk_divider_restore_context(struct clk_hw *hw)
    {
    struct clk_hw *parent = clk_hw_get_parent(hw);
    let mut parent_rate: c_ulong = clk_hw_get_rate(parent);
    let mut rate: c_ulong = clk_hw_get_rate(hw);
    if (clk_frac_div_set_rate(hw, rate, parent_rate) < 0)
    WARN_ON(1);
    }
    const struct clk_ops tegra_clk_frac_div_ops = {
    .recalc_rate = clk_frac_div_recalc_rate,
    .set_rate = clk_frac_div_set_rate,
    .determine_rate = clk_frac_div_determine_rate,
    .restore_context = clk_divider_restore_context,
    };
    struct clk *tegra_clk_register_divider(const char *name,
    const char *parent_name, void __iomem *reg,
    unsigned long flags, u8 clk_divider_flags, u8 shift, u8 width,
    u8 frac_width, spinlock_t *lock)
    {
    struct tegra_clk_frac_div *divider;
    struct clk *clk;
    struct clk_init_data init;
    divider = kzalloc_obj(*divider);
    if (!divider) {
    pr_err("%s: could not allocate fractional divider clk\n",
    __func__);
    return ERR_PTR(-ENOMEM);
    }
    init.name = name;
    init.ops = &tegra_clk_frac_div_ops;
    init.flags = flags;
    init.parent_names = parent_name ? &parent_name : core::ptr::null_mut();
    init.num_parents = parent_name ? 1 : 0;
    divider.reg = reg;
    divider.shift = shift;
    divider.width = width;
    divider.frac_width = frac_width;
    divider.lock = lock;
    divider.flags = clk_divider_flags;
// Data in .init is copied by clk_register(), so stack variable OK
    divider.hw.init = &init;
    clk = clk_register(core::ptr::null_mut(), &divider.hw);
    if (IS_ERR(clk))
    kfree(divider);
    return clk;
    }
    static const struct clk_div_table mc_div_table[] = {
    { .val = 0, .div = 2 },
    { .val = 1, .div = 1 },
    { .val = 0, .div = 0 },
    };
    struct clk *tegra_clk_register_mc(const char *name, const char *parent_name,
    void __iomem *reg, spinlock_t *lock)
    {
    return clk_register_divider_table(core::ptr::null_mut(), name, parent_name,
    CLK_IS_CRITICAL,
    reg, 16, 1, CLK_DIVIDER_READ_ONLY,
    mc_div_table, lock);
    }
