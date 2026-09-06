//! Automatically rewritten from C to Rust
//! Source: drivers/clk/at91/clk-h32mx.c
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
// clk-h32mx.c
//
// Copyright (C) 2014 Atmel
//
// Alexandre Belloni <alexandre.belloni@free-electrons.com>
//

pub const H32MX_MAX_FREQ: c_int = 90000000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_sama5d4_h32mx {
    pub hw: clk_hw,
    pub regmap: *mut regmap,
}

    static unsigned long clk_sama5d4_h32mx_recalc_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    struct clk_sama5d4_h32mx *h32mxclk = to_clk_sama5d4_h32mx(hw);
    unsigned int mckr;
    regmap_read(h32mxclk.regmap, AT91_PMC_MCKR, &mckr);
    if (mckr & AT91_PMC_H32MXDIV)
    return parent_rate / 2;
    if (parent_rate > H32MX_MAX_FREQ)
    pr_warn("H32MX clock is too fast\n");
    return parent_rate;
    }
    static int clk_sama5d4_h32mx_determine_rate(struct clk_hw *hw,
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
    static int clk_sama5d4_h32mx_set_rate(struct clk_hw *hw, unsigned long rate,
    unsigned long parent_rate)
    {
    struct clk_sama5d4_h32mx *h32mxclk = to_clk_sama5d4_h32mx(hw);
    let mut mckr: u32 = 0;
    if (parent_rate != rate && (parent_rate / 2) != rate)
    return -EINVAL;
    if ((parent_rate / 2) == rate)
    mckr = AT91_PMC_H32MXDIV;
    regmap_update_bits(h32mxclk.regmap, AT91_PMC_MCKR,
    AT91_PMC_H32MXDIV, mckr);
    return 0;
    }
    static const struct clk_ops h32mx_ops = {
    .recalc_rate = clk_sama5d4_h32mx_recalc_rate,
    .determine_rate = clk_sama5d4_h32mx_determine_rate,
    .set_rate = clk_sama5d4_h32mx_set_rate,
    };
    struct clk_hw * __init
    at91_clk_register_h32mx(struct regmap *regmap, const char *name,
    const char *parent_name)
    {
    struct clk_sama5d4_h32mx *h32mxclk;
    struct clk_init_data init;
    int ret;
    h32mxclk = kzalloc_obj(*h32mxclk);
    if (!h32mxclk)
    return ERR_PTR(-ENOMEM);
    init.name = name;
    init.ops = &h32mx_ops;
    init.parent_names = parent_name ? &parent_name : core::ptr::null_mut();
    init.num_parents = parent_name ? 1 : 0;
    init.flags = CLK_SET_RATE_GATE;
    h32mxclk.hw.init = &init;
    h32mxclk.regmap = regmap;
    ret = clk_hw_register(core::ptr::null_mut(), &h32mxclk.hw);
    if (ret) {
    kfree(h32mxclk);
    return ERR_PTR(ret);
    }
    return &h32mxclk.hw;
    }
