//! Automatically rewritten from C to Rust
//! Source: drivers/clk/at91/clk-plldiv.c
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
// Copyright (C) 2013 Boris BREZILLON <b.brezillon@overkiz.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_plldiv {
    pub hw: clk_hw,
    pub regmap: *mut regmap,
}

    static unsigned long clk_plldiv_recalc_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    struct clk_plldiv *plldiv = to_clk_plldiv(hw);
    unsigned int mckr;
    regmap_read(plldiv.regmap, AT91_PMC_MCKR, &mckr);
    if (mckr & AT91_PMC_PLLADIV2)
    return parent_rate / 2;
    return parent_rate;
    }
    static int clk_plldiv_determine_rate(struct clk_hw *hw,
    struct clk_rate_request *req)
    {
    unsigned long div;
    if (req.rate > req.best_parent_rate) {
    req.rate = req.best_parent_rate;
    return 0;
    }
    div = req.best_parent_rate / 2;
    if (req.rate < div) {
    req.rate = div;
    return 0;
    }
    if (req.rate - div < req.best_parent_rate - req.rate) {
    req.rate = div;
    return 0;
    }
    req.rate = req.best_parent_rate;
    return 0;
    }
    static int clk_plldiv_set_rate(struct clk_hw *hw, unsigned long rate,
    unsigned long parent_rate)
    {
    struct clk_plldiv *plldiv = to_clk_plldiv(hw);
    if ((parent_rate != rate) && (parent_rate / 2 != rate))
    return -EINVAL;
    regmap_update_bits(plldiv.regmap, AT91_PMC_MCKR, AT91_PMC_PLLADIV2,
    parent_rate != rate ? AT91_PMC_PLLADIV2 : 0);
    return 0;
    }
    static const struct clk_ops plldiv_ops = {
    .recalc_rate = clk_plldiv_recalc_rate,
    .determine_rate = clk_plldiv_determine_rate,
    .set_rate = clk_plldiv_set_rate,
    };
    struct clk_hw * __init
    at91_clk_register_plldiv(struct regmap *regmap, const char *name,
    const char *parent_name)
    {
    struct clk_plldiv *plldiv;
    struct clk_hw *hw;
    struct clk_init_data init;
    int ret;
    plldiv = kzalloc_obj(*plldiv);
    if (!plldiv)
    return ERR_PTR(-ENOMEM);
    init.name = name;
    init.ops = &plldiv_ops;
    init.parent_names = parent_name ? &parent_name : core::ptr::null_mut();
    init.num_parents = parent_name ? 1 : 0;
    init.flags = CLK_SET_RATE_GATE;
    plldiv.hw.init = &init;
    plldiv.regmap = regmap;
    hw = &plldiv.hw;
    ret = clk_hw_register(core::ptr::null_mut(), &plldiv.hw);
    if (ret) {
    kfree(plldiv);
    hw = ERR_PTR(ret);
    }
    return hw;
    }
