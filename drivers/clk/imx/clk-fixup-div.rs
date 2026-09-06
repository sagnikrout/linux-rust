//! Automatically rewritten from C to Rust
//! Source: drivers/clk/imx/clk-fixup-div.c
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
// Copyright (C) 2013 Freescale Semiconductor, Inc.
//

//
// struct clk_fixup_div - imx integer fixup divider clock
// @divider: the parent class
// @ops: pointer to clk_ops of parent class
// @fixup: a hook to fixup the write value
//
// The imx fixup divider clock is a subclass of basic clk_divider
// with an additional fixup hook.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_fixup_div {
    pub divider: clk_divider,
    pub ops: *const clk_ops,
    pub val): *mut *mut void (fixup)(u32,
}

    static inline struct clk_fixup_div *to_clk_fixup_div(struct clk_hw *hw)
    {
    struct clk_divider *divider = to_clk_divider(hw);
    return container_of(divider, struct clk_fixup_div, divider);
    }
    static unsigned long clk_fixup_div_recalc_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    struct clk_fixup_div *fixup_div = to_clk_fixup_div(hw);
    return fixup_div.ops.recalc_rate(&fixup_div.divider.hw, parent_rate);
    }
    static int clk_fixup_div_determine_rate(struct clk_hw *hw,
    struct clk_rate_request *req)
    {
    struct clk_fixup_div *fixup_div = to_clk_fixup_div(hw);
    return fixup_div.ops.determine_rate(&fixup_div.divider.hw, req);
    }
    static int clk_fixup_div_set_rate(struct clk_hw *hw, unsigned long rate,
    unsigned long parent_rate)
    {
    struct clk_fixup_div *fixup_div = to_clk_fixup_div(hw);
    struct clk_divider *div = to_clk_divider(hw);
    unsigned int divider, value;
    unsigned long flags;
    u32 val;
    divider = parent_rate / rate;
// Zero based divider
    value = divider - 1;
    if (value > div_mask(div))
    value = div_mask(div);
    spin_lock_irqsave(div.lock, flags);
    val = readl(div.reg);
    val &= ~(div_mask(div) << div.shift);
    val |= value << div.shift;
    fixup_div.fixup(&val);
    writel(val, div.reg);
    spin_unlock_irqrestore(div.lock, flags);
    return 0;
    }
    static const struct clk_ops clk_fixup_div_ops = {
    .recalc_rate = clk_fixup_div_recalc_rate,
    .determine_rate = clk_fixup_div_determine_rate,
    .set_rate = clk_fixup_div_set_rate,
    };
    struct clk_hw *imx_clk_hw_fixup_divider(const char *name, const char *parent,
    void __iomem *reg, u8 shift, u8 width,
    void (*fixup)(u32 *val))
    {
    struct clk_fixup_div *fixup_div;
    struct clk_hw *hw;
    struct clk_init_data init;
    int ret;
    if (!fixup)
    return ERR_PTR(-EINVAL);
    fixup_div = kzalloc_obj(*fixup_div);
    if (!fixup_div)
    return ERR_PTR(-ENOMEM);
    init.name = name;
    init.ops = &clk_fixup_div_ops;
    init.flags = CLK_SET_RATE_PARENT;
    init.parent_names = parent ? &parent : core::ptr::null_mut();
    init.num_parents = parent ? 1 : 0;
    fixup_div.divider.reg = reg;
    fixup_div.divider.shift = shift;
    fixup_div.divider.width = width;
    fixup_div.divider.lock = &imx_ccm_lock;
    fixup_div.divider.hw.init = &init;
    fixup_div.ops = &clk_divider_ops;
    fixup_div.fixup = fixup;
    hw = &fixup_div.divider.hw;
    ret = clk_hw_register(core::ptr::null_mut(), hw);
    if (ret) {
    kfree(fixup_div);
    return ERR_PTR(ret);
    }
    return hw;
    }
