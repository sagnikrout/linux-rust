//! Automatically rewritten from C to Rust
//! Source: drivers/clk/sunxi/clk-sun4i-tcon-ch1.c
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
// Copyright 2015 Maxime Ripard
//
// Maxime Ripard <maxime.ripard@free-electrons.com>
//

pub const TCON_CH1_SCLK2_PARENTS: c_int = 4;

pub const TCON_CH1_SCLK2_MUX_MASK: c_int = 3;
pub const TCON_CH1_SCLK2_MUX_SHIFT: c_int = 24;
pub const TCON_CH1_SCLK2_DIV_MASK: c_uint = 0xf;
pub const TCON_CH1_SCLK2_DIV_SHIFT: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcon_ch1_clk {
    pub hw: clk_hw,
    pub lock: spinlock_t,
    pub reg: *mut void __iomem,
}

#[no_mangle]
unsafe extern "C" fn tcon_ch1_disable(hw: *mut clk_hw) {
    static void tcon_ch1_disable(struct clk_hw *hw)
    {
    struct tcon_ch1_clk *tclk = hw_to_tclk(hw);
    unsigned long flags;
    u32 reg;
    spin_lock_irqsave(&tclk.lock, flags);
    reg = readl(tclk.reg);
    reg &= ~(TCON_CH1_SCLK2_GATE_BIT | TCON_CH1_SCLK1_GATE_BIT);
    writel(reg, tclk.reg);
    spin_unlock_irqrestore(&tclk.lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn tcon_ch1_enable(hw: *mut clk_hw) -> c_int {
    static int tcon_ch1_enable(struct clk_hw *hw)
    {
    struct tcon_ch1_clk *tclk = hw_to_tclk(hw);
    unsigned long flags;
    u32 reg;
    spin_lock_irqsave(&tclk.lock, flags);
    reg = readl(tclk.reg);
    reg |= TCON_CH1_SCLK2_GATE_BIT | TCON_CH1_SCLK1_GATE_BIT;
    writel(reg, tclk.reg);
    spin_unlock_irqrestore(&tclk.lock, flags);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tcon_ch1_is_enabled(hw: *mut clk_hw) -> c_int {
    static int tcon_ch1_is_enabled(struct clk_hw *hw)
    {
    struct tcon_ch1_clk *tclk = hw_to_tclk(hw);
    u32 reg;
    reg = readl(tclk.reg);
    return reg & (TCON_CH1_SCLK2_GATE_BIT | TCON_CH1_SCLK1_GATE_BIT);
    }
#[no_mangle]
unsafe extern "C" fn tcon_ch1_get_parent(hw: *mut clk_hw) -> u8 {
    static u8 tcon_ch1_get_parent(struct clk_hw *hw)
    {
    struct tcon_ch1_clk *tclk = hw_to_tclk(hw);
    u32 reg;
    reg = readl(tclk.reg) >> TCON_CH1_SCLK2_MUX_SHIFT;
    reg &= reg >> TCON_CH1_SCLK2_MUX_MASK;
    return reg;
    }
#[no_mangle]
unsafe extern "C" fn tcon_ch1_set_parent(hw: *mut clk_hw, index: u8) -> c_int {
    static int tcon_ch1_set_parent(struct clk_hw *hw, u8 index)
    {
    struct tcon_ch1_clk *tclk = hw_to_tclk(hw);
    unsigned long flags;
    u32 reg;
    spin_lock_irqsave(&tclk.lock, flags);
    reg = readl(tclk.reg);
    reg &= ~(TCON_CH1_SCLK2_MUX_MASK << TCON_CH1_SCLK2_MUX_SHIFT);
    reg |= index << TCON_CH1_SCLK2_MUX_SHIFT;
    writel(reg, tclk.reg);
    spin_unlock_irqrestore(&tclk.lock, flags);
    return 0;
    };
    static unsigned long tcon_ch1_calc_divider(unsigned long rate,
    unsigned long parent_rate,
    u8 *div,
    bool *half)
    {
    let mut best_rate: c_ulong = 0;
    let mut best_m: u8 = 0, m;
    bool is_double;
    for (m = 1; m < 16; m++) {
    u8 d;
    for (d = 1; d < 3; d++) {
    unsigned long tmp_rate;
    tmp_rate = parent_rate / m / d;
    if (tmp_rate > rate)
    continue;
    if (!best_rate ||
    (rate - tmp_rate) < (rate - best_rate)) {
    best_rate = tmp_rate;
    best_m = m;
    is_double = d;
    }
    }
    }
    if (div && half) {
// div = best_m;
// half = is_double;
    }
    return best_rate;
    }
    static int tcon_ch1_determine_rate(struct clk_hw *hw,
    struct clk_rate_request *req)
    {
    let mut best_rate: c_long = -EINVAL;
    int i;
    for (i = 0; i < clk_hw_get_num_parents(hw); i++) {
    unsigned long parent_rate;
    unsigned long tmp_rate;
    struct clk_hw *parent;
    parent = clk_hw_get_parent_by_index(hw, i);
    if (!parent)
    continue;
    parent_rate = clk_hw_get_rate(parent);
    tmp_rate = tcon_ch1_calc_divider(req.rate, parent_rate,
    core::ptr::null_mut(), core::ptr::null_mut());
    if (best_rate < 0 ||
    (req.rate - tmp_rate) < (req.rate - best_rate)) {
    best_rate = tmp_rate;
    req.best_parent_rate = parent_rate;
    req.best_parent_hw = parent;
    }
    }
    if (best_rate < 0)
    return best_rate;
    req.rate = best_rate;
    return 0;
    }
    static unsigned long tcon_ch1_recalc_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    struct tcon_ch1_clk *tclk = hw_to_tclk(hw);
    u32 reg;
    reg = readl(tclk.reg);
    parent_rate /= (reg & TCON_CH1_SCLK2_DIV_MASK) + 1;
    if (reg & TCON_CH1_SCLK1_HALF_BIT)
    parent_rate /= 2;
    return parent_rate;
    }
    static int tcon_ch1_set_rate(struct clk_hw *hw, unsigned long rate,
    unsigned long parent_rate)
    {
    struct tcon_ch1_clk *tclk = hw_to_tclk(hw);
    unsigned long flags;
    bool half;
    u8 div_m;
    u32 reg;
    tcon_ch1_calc_divider(rate, parent_rate, &div_m, &half);
    spin_lock_irqsave(&tclk.lock, flags);
    reg = readl(tclk.reg);
    reg &= ~(TCON_CH1_SCLK2_DIV_MASK | TCON_CH1_SCLK1_HALF_BIT);
    reg |= (div_m - 1) & TCON_CH1_SCLK2_DIV_MASK;
    if (half)
    reg |= TCON_CH1_SCLK1_HALF_BIT;
    writel(reg, tclk.reg);
    spin_unlock_irqrestore(&tclk.lock, flags);
    return 0;
    }
    static const struct clk_ops tcon_ch1_ops = {
    .disable	= tcon_ch1_disable,
    .enable		= tcon_ch1_enable,
    .is_enabled	= tcon_ch1_is_enabled,
    .get_parent	= tcon_ch1_get_parent,
    .set_parent	= tcon_ch1_set_parent,
    .determine_rate	= tcon_ch1_determine_rate,
    .recalc_rate	= tcon_ch1_recalc_rate,
    .set_rate	= tcon_ch1_set_rate,
    };
#[no_mangle]
unsafe extern "C" fn tcon_ch1_setup(node: *mut device_node) -> void __init {
    static void __init tcon_ch1_setup(struct device_node *node)
    {
    const char *parents[TCON_CH1_SCLK2_PARENTS];
    const char *clk_name = node.name;
    struct clk_init_data init;
    struct tcon_ch1_clk *tclk;
    struct resource res;
    struct clk *clk;
    void __iomem *reg;
    int ret;
    of_property_read_string(node, "clock-output-names", &clk_name);
    reg = of_io_request_and_map(node, 0, of_node_full_name(node));
    if (IS_ERR(reg)) {
    pr_err("%s: Could not map the clock registers\n", clk_name);
    return;
    }
    ret = of_clk_parent_fill(node, parents, TCON_CH1_SCLK2_PARENTS);
    if (ret != TCON_CH1_SCLK2_PARENTS) {
    pr_err("%s Could not retrieve the parents\n", clk_name);
    goto err_unmap;
    }
    tclk = kzalloc_obj(*tclk);
    if (!tclk)
    goto err_unmap;
    init.name = clk_name;
    init.ops = &tcon_ch1_ops;
    init.parent_names = parents;
    init.num_parents = TCON_CH1_SCLK2_PARENTS;
    init.flags = CLK_SET_RATE_PARENT;
    tclk.reg = reg;
    tclk.hw.init = &init;
    spin_lock_init(&tclk.lock);
    clk = clk_register(core::ptr::null_mut(), &tclk.hw);
    if (IS_ERR(clk)) {
    pr_err("%s: Couldn't register the clock\n", clk_name);
    goto err_free_data;
    }
    ret = of_clk_add_provider(node, of_clk_src_simple_get, clk);
    if (ret) {
    pr_err("%s: Couldn't register our clock provider\n", clk_name);
    goto err_unregister_clk;
    }
    return;
    err_unregister_clk:
    clk_unregister(clk);
    err_free_data:
    kfree(tclk);
    err_unmap:
    iounmap(reg);
    of_address_to_resource(node, 0, &res);
    release_mem_region(res.start, resource_size(&res));
    }
    CLK_OF_DECLARE(tcon_ch1, "allwinner,sun4i-a10-tcon-ch1-clk",
    tcon_ch1_setup);
