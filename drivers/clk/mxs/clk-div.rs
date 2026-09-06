//! Automatically rewritten from C to Rust
//! Source: drivers/clk/mxs/clk-div.c
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
// struct clk_div - mxs integer divider clock
// @divider: the parent class
// @ops: pointer to clk_ops of parent class
// @reg: register address
// @busy: busy bit shift
//
// The mxs divider clock is a subclass of basic clk_divider with an
// additional busy bit.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_div {
    pub divider: clk_divider,
    pub ops: *const clk_ops,
    pub reg: *mut void __iomem,
    pub busy: u8,
}

    static inline struct clk_div *to_clk_div(struct clk_hw *hw)
    {
    struct clk_divider *divider = to_clk_divider(hw);
    return container_of(divider, struct clk_div, divider);
    }
    static unsigned long clk_div_recalc_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    struct clk_div *div = to_clk_div(hw);
    return div.ops.recalc_rate(&div.divider.hw, parent_rate);
    }
    static int clk_div_determine_rate(struct clk_hw *hw,
    struct clk_rate_request *req)
    {
    struct clk_div *div = to_clk_div(hw);
    return div.ops.determine_rate(&div.divider.hw, req);
    }
    static int clk_div_set_rate(struct clk_hw *hw, unsigned long rate,
    unsigned long parent_rate)
    {
    struct clk_div *div = to_clk_div(hw);
    int ret;
    ret = div.ops.set_rate(&div.divider.hw, rate, parent_rate);
    if (!ret)
    ret = mxs_clk_wait(div.reg, div.busy);
    return ret;
    }
    static const struct clk_ops clk_div_ops = {
    .recalc_rate = clk_div_recalc_rate,
    .determine_rate = clk_div_determine_rate,
    .set_rate = clk_div_set_rate,
    };
    struct clk *mxs_clk_div(const char *name, const char *parent_name,
    void __iomem *reg, u8 shift, u8 width, u8 busy)
    {
    struct clk_div *div;
    struct clk *clk;
    struct clk_init_data init;
    div = kzalloc_obj(*div);
    if (!div)
    return ERR_PTR(-ENOMEM);
    init.name = name;
    init.ops = &clk_div_ops;
    init.flags = CLK_SET_RATE_PARENT;
    init.parent_names = (parent_name ? &parent_name: core::ptr::null_mut());
    init.num_parents = (parent_name ? 1 : 0);
    div.reg = reg;
    div.busy = busy;
    div.divider.reg = reg;
    div.divider.shift = shift;
    div.divider.width = width;
    div.divider.flags = CLK_DIVIDER_ONE_BASED;
    div.divider.lock = &mxs_lock;
    div.divider.hw.init = &init;
    div.ops = &clk_divider_ops;
    clk = clk_register(core::ptr::null_mut(), &div.divider.hw);
    if (IS_ERR(clk))
    kfree(div);
    return clk;
    }
