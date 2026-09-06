//! Automatically rewritten from C to Rust
//! Source: drivers/clk/x86/clk-cgu-pll.c
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
// Copyright (C) 2020-2022 MaxLinear, Inc.
// Copyright (C) 2020 Intel Corporation.
// Zhu Yixin <yzhu@maxlinear.com>
// Rahul Tanwar <rtanwar@maxlinear.com>
//

//
// Calculate formula:
// rate = (prate * mult + (prate * frac) / frac_div) / div
//
    static unsigned long
    lgm_pll_calc_rate(unsigned long prate, unsigned int mult,
    unsigned int div, unsigned int frac, unsigned int frac_div)
    {
    u64 crate, frate, rate64;
    rate64 = prate;
    crate = rate64 * mult;
    frate = rate64 * frac;
    do_div(frate, frac_div);
    crate += frate;
    do_div(crate, div);
    return crate;
    }
#[no_mangle]
unsafe extern "C" fn lgm_pll_recalc_rate(hw: *mut clk_hw, prate: c_ulong) -> c_ulong {
    static unsigned long lgm_pll_recalc_rate(struct clk_hw *hw, unsigned long prate)
    {
    struct lgm_clk_pll *pll = to_lgm_clk_pll(hw);
    unsigned int div, mult, frac;
    mult = lgm_get_clk_val(pll.membase, PLL_REF_DIV(pll.reg), 0, 12);
    div = lgm_get_clk_val(pll.membase, PLL_REF_DIV(pll.reg), 18, 6);
    frac = lgm_get_clk_val(pll.membase, pll.reg, 2, 24);
    if (pll.type == TYPE_LJPLL)
    div *= 4;
    return lgm_pll_calc_rate(prate, mult, div, frac, BIT(24));
    }
#[no_mangle]
unsafe extern "C" fn lgm_pll_is_enabled(hw: *mut clk_hw) -> c_int {
    static int lgm_pll_is_enabled(struct clk_hw *hw)
    {
    struct lgm_clk_pll *pll = to_lgm_clk_pll(hw);
    unsigned int ret;
    ret = lgm_get_clk_val(pll.membase, pll.reg, 0, 1);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn lgm_pll_enable(hw: *mut clk_hw) -> c_int {
    static int lgm_pll_enable(struct clk_hw *hw)
    {
    struct lgm_clk_pll *pll = to_lgm_clk_pll(hw);
    u32 val;
    int ret;
    lgm_set_clk_val(pll.membase, pll.reg, 0, 1, 1);
    ret = regmap_read_poll_timeout_atomic(pll.membase, pll.reg,
    val, (val & 0x1), 1, 100);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn lgm_pll_disable(hw: *mut clk_hw) {
    static void lgm_pll_disable(struct clk_hw *hw)
    {
    struct lgm_clk_pll *pll = to_lgm_clk_pll(hw);
    lgm_set_clk_val(pll.membase, pll.reg, 0, 1, 0);
    }
    static const struct clk_ops lgm_pll_ops = {
    .recalc_rate = lgm_pll_recalc_rate,
    .is_enabled = lgm_pll_is_enabled,
    .enable = lgm_pll_enable,
    .disable = lgm_pll_disable,
    };
    static struct clk_hw *
    lgm_clk_register_pll(struct lgm_clk_provider *ctx,
    const struct lgm_pll_clk_data *list)
    {
    let mut init: clk_init_data = {};
    struct lgm_clk_pll *pll;
    struct device *dev = ctx.dev;
    struct clk_hw *hw;
    int ret;
    init.ops = &lgm_pll_ops;
    init.name = list.name;
    init.flags = list.flags;
    init.parent_data = list.parent_data;
    init.num_parents = list.num_parents;
    pll = devm_kzalloc(dev, sizeof(*pll), GFP_KERNEL);
    if (!pll)
    return ERR_PTR(-ENOMEM);
    pll.membase = ctx.membase;
    pll.reg = list.reg;
    pll.flags = list.flags;
    pll.type = list.type;
    pll.hw.init = &init;
    hw = &pll.hw;
    ret = devm_clk_hw_register(dev, hw);
    if (ret)
    return ERR_PTR(ret);
    return hw;
    }
    int lgm_clk_register_plls(struct lgm_clk_provider *ctx,
    const struct lgm_pll_clk_data *list,
    unsigned int nr_clk)
    {
    struct clk_hw *hw;
    int i;
    for (i = 0; i < nr_clk; i++, list++) {
    hw = lgm_clk_register_pll(ctx, list);
    if (IS_ERR(hw)) {
    dev_err(ctx.dev, "failed to register pll: %s\n",
    list.name);
    return PTR_ERR(hw);
    }
    ctx.clk_data.hws[list.id] = hw;
    }
    return 0;
    }
