//! Automatically rewritten from C to Rust
//! Source: drivers/clk/hisilicon/clkdivider-hi6220.c
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
// Hisilicon hi6220 SoC divider clock driver
//
// Copyright (c) 2015 Hisilicon Limited.
//
// Author: Bintian Wang <bintian.wang@huawei.com>
//

//
// struct hi6220_clk_divider - divider clock for hi6220
//
// @hw:		handle between common and hardware-specific interfaces
// @reg:	register containing divider
// @shift:	shift to the divider bit field
// @width:	width of the divider bit field
// @mask:	mask for setting divider rate
// @lock:	register lock
// @table:	the div table that the divider supports
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hi6220_clk_divider {
    pub hw: clk_hw,
    pub reg: *mut void __iomem,
    pub shift: u8,
    pub width: u8,
    pub mask: u32,
    pub lock: *mut spinlock_t,
    pub table: [clk_div_table; ],
}

    container_of(_hw, struct hi6220_clk_divider, hw)
    static unsigned long hi6220_clkdiv_recalc_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    unsigned int val;
    struct hi6220_clk_divider *dclk = to_hi6220_clk_divider(hw);
    val = readl_relaxed(dclk.reg) >> dclk.shift;
    val &= div_mask(dclk.width);
    return divider_recalc_rate(hw, parent_rate, val, dclk.table,
    CLK_DIVIDER_ROUND_CLOSEST, dclk.width);
    }
    static int hi6220_clkdiv_determine_rate(struct clk_hw *hw,
    struct clk_rate_request *req)
    {
    struct hi6220_clk_divider *dclk = to_hi6220_clk_divider(hw);
    return divider_determine_rate(hw, req, dclk.table, dclk.width,
    CLK_DIVIDER_ROUND_CLOSEST);
    }
    static int hi6220_clkdiv_set_rate(struct clk_hw *hw, unsigned long rate,
    unsigned long parent_rate)
    {
    int value;
    let mut flags: c_ulong = 0;
    u32 data;
    struct hi6220_clk_divider *dclk = to_hi6220_clk_divider(hw);
    value = divider_get_val(rate, parent_rate, dclk.table,
    dclk.width, CLK_DIVIDER_ROUND_CLOSEST);
    if (dclk.lock)
    spin_lock_irqsave(dclk.lock, flags);
    data = readl_relaxed(dclk.reg);
    data &= ~(div_mask(dclk.width) << dclk.shift);
    data |= value << dclk.shift;
    data |= dclk.mask;
    writel_relaxed(data, dclk.reg);
    if (dclk.lock)
    spin_unlock_irqrestore(dclk.lock, flags);
    return 0;
    }
    static const struct clk_ops hi6220_clkdiv_ops = {
    .recalc_rate = hi6220_clkdiv_recalc_rate,
    .determine_rate = hi6220_clkdiv_determine_rate,
    .set_rate = hi6220_clkdiv_set_rate,
    };
    struct clk *hi6220_register_clkdiv(struct device *dev, const char *name,
    const char *parent_name, unsigned long flags, void __iomem *reg,
    u8 shift, u8 width, u32 mask_bit, spinlock_t *lock)
    {
    struct hi6220_clk_divider *div;
    struct clk *clk;
    struct clk_init_data init;
    struct clk_div_table *table;
    u32 max_div, min_div;
    int i;
// Init the divider table
    max_div = div_mask(width) + 1;
    min_div = 1;
// allocate the divider
    div = kzalloc_flex(*div, table, max_div + 1);
    if (!div)
    return ERR_PTR(-ENOMEM);
    for (i = 0; i < max_div; i++) {
    table = &div.table[i];
    table.div = min_div + i;
    table.val = table.div - 1;
    }
    init.name = name;
    init.ops = &hi6220_clkdiv_ops;
    init.flags = flags;
    init.parent_names = parent_name ? &parent_name : core::ptr::null_mut();
    init.num_parents = parent_name ? 1 : 0;
// struct hi6220_clk_divider assignments
    div.reg = reg;
    div.shift = shift;
    div.width = width;
    div.mask = mask_bit ? BIT(mask_bit) : 0;
    div.lock = lock;
    div.hw.init = &init;
// register the clock
    clk = clk_register(dev, &div.hw);
    if (IS_ERR(clk))
    kfree(div);
    return clk;
    }
