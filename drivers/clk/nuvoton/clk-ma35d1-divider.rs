//! Automatically rewritten from C to Rust
//! Source: drivers/clk/nuvoton/clk-ma35d1-divider.c
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
// Copyright (C) 2023 Nuvoton Technology Corp.
// Author: Chi-Fang Li <cfli0@nuvoton.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ma35d1_adc_clk_div {
    pub hw: clk_hw,
    pub reg: *mut void __iomem,
    pub shift: u8,
    pub width: u8,
    pub mask: u32,
    pub table: *const clk_div_table,
// protects concurrent access to clock divider registers
    pub lock: *mut spinlock_t,
}

    static inline struct ma35d1_adc_clk_div *to_ma35d1_adc_clk_div(struct clk_hw *_hw)
    {
    return container_of(_hw, struct ma35d1_adc_clk_div, hw);
    }
#[no_mangle]
unsafe extern "C" fn ma35d1_clkdiv_recalc_rate(hw: *mut clk_hw, parent_rate: c_ulong) -> c_ulong {
    static unsigned long ma35d1_clkdiv_recalc_rate(struct clk_hw *hw, unsigned long parent_rate)
    {
    unsigned int val;
    struct ma35d1_adc_clk_div *dclk = to_ma35d1_adc_clk_div(hw);
    val = readl_relaxed(dclk.reg) >> dclk.shift;
    val &= clk_div_mask(dclk.width);
    val += 1;
    return divider_recalc_rate(hw, parent_rate, val, dclk.table,
    CLK_DIVIDER_ROUND_CLOSEST, dclk.width);
    }
    static int ma35d1_clkdiv_determine_rate(struct clk_hw *hw,
    struct clk_rate_request *req)
    {
    struct ma35d1_adc_clk_div *dclk = to_ma35d1_adc_clk_div(hw);
    return divider_determine_rate(hw, req, dclk.table, dclk.width,
    CLK_DIVIDER_ROUND_CLOSEST);
    }
#[no_mangle]
unsafe extern "C" fn ma35d1_clkdiv_set_rate(hw: *mut clk_hw, rate: c_ulong, parent_rate: c_ulong) -> c_int {
    static int ma35d1_clkdiv_set_rate(struct clk_hw *hw, unsigned long rate, unsigned long parent_rate)
    {
    int value;
    let mut flags: c_ulong = 0;
    u32 data;
    struct ma35d1_adc_clk_div *dclk = to_ma35d1_adc_clk_div(hw);
    value = divider_get_val(rate, parent_rate, dclk.table,
    dclk.width, CLK_DIVIDER_ROUND_CLOSEST);
    spin_lock_irqsave(dclk.lock, flags);
    data = readl_relaxed(dclk.reg);
    data &= ~(clk_div_mask(dclk.width) << dclk.shift);
    data |= (value - 1) << dclk.shift;
    data |= dclk.mask;
    writel_relaxed(data, dclk.reg);
    spin_unlock_irqrestore(dclk.lock, flags);
    return 0;
    }
    static const struct clk_ops ma35d1_adc_clkdiv_ops = {
    .recalc_rate = ma35d1_clkdiv_recalc_rate,
    .determine_rate = ma35d1_clkdiv_determine_rate,
    .set_rate = ma35d1_clkdiv_set_rate,
    };
    struct clk_hw *ma35d1_reg_adc_clkdiv(struct device *dev, const char *name,
    struct clk_hw *parent_hw, spinlock_t *lock,
    unsigned long flags, void __iomem *reg,
    u8 shift, u8 width, u32 mask_bit)
    {
    struct ma35d1_adc_clk_div *div;
    struct clk_init_data init;
    struct clk_div_table *table;
    let mut pdata: clk_parent_data = { .index = 0 };
    u32 max_div, min_div;
    struct clk_hw *hw;
    int ret;
    int i;
    div = devm_kzalloc(dev, sizeof(*div), GFP_KERNEL);
    if (!div)
    return ERR_PTR(-ENOMEM);
    max_div = clk_div_mask(width) + 1;
    min_div = 1;
    table = devm_kcalloc(dev, max_div + 1, sizeof(*table), GFP_KERNEL);
    if (!table)
    return ERR_PTR(-ENOMEM);
    for (i = 0; i < max_div; i++) {
    table[i].val = min_div + i;
    table[i].div = 2 * table[i].val;
    }
    table[max_div].val = 0;
    table[max_div].div = 0;
    memset(&init, 0, sizeof(init));
    init.name = name;
    init.ops = &ma35d1_adc_clkdiv_ops;
    init.flags |= flags;
    pdata.hw = parent_hw;
    init.parent_data = &pdata;
    init.num_parents = 1;
    div.reg = reg;
    div.shift = shift;
    div.width = width;
    div.mask = mask_bit ? BIT(mask_bit) : 0;
    div.lock = lock;
    div.hw.init = &init;
    div.table = table;
    hw = &div.hw;
    ret = devm_clk_hw_register(dev, hw);
    if (ret)
    return ERR_PTR(ret);
    return hw;
    }
    EXPORT_SYMBOL_GPL(ma35d1_reg_adc_clkdiv);
