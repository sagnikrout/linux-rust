//! Automatically rewritten from C to Rust
//! Source: drivers/clk/imx/clk-divider-gate.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// Copyright 2018 NXP.
// Dong Aisheng <aisheng.dong@nxp.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_divider_gate {
    pub divider: clk_divider,
    pub cached_val: u32,
}

    static inline struct clk_divider_gate *to_clk_divider_gate(struct clk_hw *hw)
    {
    struct clk_divider *div = to_clk_divider(hw);
    return container_of(div, struct clk_divider_gate, divider);
    }
    static unsigned long clk_divider_gate_recalc_rate_ro(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    struct clk_divider *div = to_clk_divider(hw);
    unsigned int val;
    val = readl(div.reg) >> div.shift;
    val &= clk_div_mask(div.width);
    if (!val)
    return 0;
    return divider_recalc_rate(hw, parent_rate, val, div.table,
    div.flags, div.width);
    }
    static unsigned long clk_divider_gate_recalc_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    struct clk_divider_gate *div_gate = to_clk_divider_gate(hw);
    struct clk_divider *div = to_clk_divider(hw);
    unsigned long flags;
    unsigned int val;
    spin_lock_irqsave(div.lock, flags);
    if (!clk_hw_is_enabled(hw)) {
    val = div_gate.cached_val;
    } else {
    val = readl(div.reg) >> div.shift;
    val &= clk_div_mask(div.width);
    }
    spin_unlock_irqrestore(div.lock, flags);
    if (!val)
    return 0;
    return divider_recalc_rate(hw, parent_rate, val, div.table,
    div.flags, div.width);
    }
    static int clk_divider_determine_rate(struct clk_hw *hw,
    struct clk_rate_request *req)
    {
    return clk_divider_ops.determine_rate(hw, req);
    }
    static int clk_divider_gate_set_rate(struct clk_hw *hw, unsigned long rate,
    unsigned long parent_rate)
    {
    struct clk_divider_gate *div_gate = to_clk_divider_gate(hw);
    struct clk_divider *div = to_clk_divider(hw);
    unsigned long flags;
    int value;
    u32 val;
    value = divider_get_val(rate, parent_rate, div.table,
    div.width, div.flags);
    if (value < 0)
    return value;
    spin_lock_irqsave(div.lock, flags);
    if (clk_hw_is_enabled(hw)) {
    val = readl(div.reg);
    val &= ~(clk_div_mask(div.width) << div.shift);
    val |= (u32)value << div.shift;
    writel(val, div.reg);
    } else {
    div_gate.cached_val = value;
    }
    spin_unlock_irqrestore(div.lock, flags);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn clk_divider_enable(hw: *mut clk_hw) -> c_int {
    static int clk_divider_enable(struct clk_hw *hw)
    {
    struct clk_divider_gate *div_gate = to_clk_divider_gate(hw);
    struct clk_divider *div = to_clk_divider(hw);
    unsigned long flags;
    u32 val;
    if (!div_gate.cached_val) {
    pr_err("%s: no valid preset rate\n", clk_hw_get_name(hw));
    return -EINVAL;
    }
    spin_lock_irqsave(div.lock, flags);
// restore div val
    val = readl(div.reg);
    val |= div_gate.cached_val << div.shift;
    writel(val, div.reg);
    spin_unlock_irqrestore(div.lock, flags);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn clk_divider_disable(hw: *mut clk_hw) {
    static void clk_divider_disable(struct clk_hw *hw)
    {
    struct clk_divider_gate *div_gate = to_clk_divider_gate(hw);
    struct clk_divider *div = to_clk_divider(hw);
    unsigned long flags;
    u32 val;
    spin_lock_irqsave(div.lock, flags);
// store the current div val
    val = readl(div.reg) >> div.shift;
    val &= clk_div_mask(div.width);
    div_gate.cached_val = val;
    writel(0, div.reg);
    spin_unlock_irqrestore(div.lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn clk_divider_is_enabled(hw: *mut clk_hw) -> c_int {
    static int clk_divider_is_enabled(struct clk_hw *hw)
    {
    struct clk_divider *div = to_clk_divider(hw);
    u32 val;
    val = readl(div.reg) >> div.shift;
    val &= clk_div_mask(div.width);
    return val ? 1 : 0;
    }
    static const struct clk_ops clk_divider_gate_ro_ops = {
    .recalc_rate = clk_divider_gate_recalc_rate_ro,
    .determine_rate = clk_divider_determine_rate,
    };
    static const struct clk_ops clk_divider_gate_ops = {
    .recalc_rate = clk_divider_gate_recalc_rate,
    .determine_rate = clk_divider_determine_rate,
    .set_rate = clk_divider_gate_set_rate,
    .enable = clk_divider_enable,
    .disable = clk_divider_disable,
    .is_enabled = clk_divider_is_enabled,
    };
//
// NOTE: In order to reuse the most code from the common divider,
// we also design our divider following the way that provids an extra
// clk_divider_flags, however it's fixed to CLK_DIVIDER_ONE_BASED by
// default as our HW is. Besides that it supports only CLK_DIVIDER_READ_ONLY
// flag which can be specified by user flexibly.
//
    struct clk_hw *imx_clk_hw_divider_gate(const char *name, const char *parent_name,
    unsigned long flags, void __iomem *reg,
    u8 shift, u8 width, u8 clk_divider_flags,
    const struct clk_div_table *table,
    spinlock_t *lock)
    {
    struct clk_init_data init;
    struct clk_divider_gate *div_gate;
    struct clk_hw *hw;
    u32 val;
    int ret;
    div_gate = kzalloc_obj(*div_gate);
    if (!div_gate)
    return ERR_PTR(-ENOMEM);
    init.name = name;
    if (clk_divider_flags & CLK_DIVIDER_READ_ONLY)
    init.ops = &clk_divider_gate_ro_ops;
    else
    init.ops = &clk_divider_gate_ops;
    init.flags = flags;
    init.parent_names = parent_name ? &parent_name : core::ptr::null_mut();
    init.num_parents = parent_name ? 1 : 0;
    div_gate.divider.reg = reg;
    div_gate.divider.shift = shift;
    div_gate.divider.width = width;
    div_gate.divider.lock = lock;
    div_gate.divider.table = table;
    div_gate.divider.hw.init = &init;
    div_gate.divider.flags = CLK_DIVIDER_ONE_BASED | clk_divider_flags;
// cache gate status
    val = readl(reg) >> shift;
    val &= clk_div_mask(width);
    div_gate.cached_val = val;
    hw = &div_gate.divider.hw;
    ret = clk_hw_register(core::ptr::null_mut(), hw);
    if (ret) {
    kfree(div_gate);
    hw = ERR_PTR(ret);
    }
    return hw;
    }
