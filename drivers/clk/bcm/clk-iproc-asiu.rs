//! Automatically rewritten from C to Rust
//! Source: drivers/clk/bcm/clk-iproc-asiu.c
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
// Copyright (C) 2014 Broadcom Corporation

    struct iproc_asiu;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iproc_asiu_clk {
    pub hw: clk_hw,
    pub name: *const c_char,
    pub asiu: *mut iproc_asiu,
    pub rate: c_ulong,
    pub div: iproc_asiu_div,
    pub gate: iproc_asiu_gate,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iproc_asiu {
    pub div_base: *mut void __iomem,
    pub gate_base: *mut void __iomem,
    pub clks: [iproc_asiu_clk; ],
}

#[no_mangle]
unsafe extern "C" fn iproc_asiu_clk_enable(hw: *mut clk_hw) -> c_int {
    static int iproc_asiu_clk_enable(struct clk_hw *hw)
    {
    struct iproc_asiu_clk *clk = to_asiu_clk(hw);
    struct iproc_asiu *asiu = clk.asiu;
    u32 val;
// some clocks at the ASIU level are always enabled
    if (clk.gate.offset == IPROC_CLK_INVALID_OFFSET)
    return 0;
    val = readl(asiu.gate_base + clk.gate.offset);
    val |= (1 << clk.gate.en_shift);
    writel(val, asiu.gate_base + clk.gate.offset);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn iproc_asiu_clk_disable(hw: *mut clk_hw) {
    static void iproc_asiu_clk_disable(struct clk_hw *hw)
    {
    struct iproc_asiu_clk *clk = to_asiu_clk(hw);
    struct iproc_asiu *asiu = clk.asiu;
    u32 val;
// some clocks at the ASIU level are always enabled
    if (clk.gate.offset == IPROC_CLK_INVALID_OFFSET)
    return;
    val = readl(asiu.gate_base + clk.gate.offset);
    val &= ~(1 << clk.gate.en_shift);
    writel(val, asiu.gate_base + clk.gate.offset);
    }
    static unsigned long iproc_asiu_clk_recalc_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    struct iproc_asiu_clk *clk = to_asiu_clk(hw);
    struct iproc_asiu *asiu = clk.asiu;
    u32 val;
    unsigned int div_h, div_l;
    if (parent_rate == 0) {
    clk.rate = 0;
    return 0;
    }
// if clock divisor is not enabled, simply return parent rate
    val = readl(asiu.div_base + clk.div.offset);
    if ((val & (1 << clk.div.en_shift)) == 0) {
    clk.rate = parent_rate;
    return parent_rate;
    }
// clock rate = parent rate / (high_div + 1) + (low_div + 1)
    div_h = (val >> clk.div.high_shift) & bit_mask(clk.div.high_width);
    div_h++;
    div_l = (val >> clk.div.low_shift) & bit_mask(clk.div.low_width);
    div_l++;
    clk.rate = parent_rate / (div_h + div_l);
    pr_debug("%s: rate: %lu. parent rate: %lu div_h: %u div_l: %u\n",
    __func__, clk.rate, parent_rate, div_h, div_l);
    return clk.rate;
    }
    static int iproc_asiu_clk_determine_rate(struct clk_hw *hw,
    struct clk_rate_request *req)
    {
    unsigned int div;
    if (req.rate == 0 || req.best_parent_rate == 0)
    return -EINVAL;
    if (req.rate == req.best_parent_rate)
    return 0;
    div = DIV_ROUND_CLOSEST(req.best_parent_rate, req.rate);
    if (div < 2) {
    req.rate = req.best_parent_rate;
    return 0;
    }
    req.rate = req.best_parent_rate / div;
    return 0;
    }
    static int iproc_asiu_clk_set_rate(struct clk_hw *hw, unsigned long rate,
    unsigned long parent_rate)
    {
    struct iproc_asiu_clk *clk = to_asiu_clk(hw);
    struct iproc_asiu *asiu = clk.asiu;
    unsigned int div, div_h, div_l;
    u32 val;
    if (rate == 0 || parent_rate == 0)
    return -EINVAL;
// simply disable the divisor if one wants the same rate as parent
    if (rate == parent_rate) {
    val = readl(asiu.div_base + clk.div.offset);
    val &= ~(1 << clk.div.en_shift);
    writel(val, asiu.div_base + clk.div.offset);
    return 0;
    }
    div = DIV_ROUND_CLOSEST(parent_rate, rate);
    if (div < 2)
    return -EINVAL;
    div_h = div_l = div >> 1;
    div_h--;
    div_l--;
    val = readl(asiu.div_base + clk.div.offset);
    val |= 1 << clk.div.en_shift;
    if (div_h) {
    val &= ~(bit_mask(clk.div.high_width)
    << clk.div.high_shift);
    val |= div_h << clk.div.high_shift;
    } else {
    val &= ~(bit_mask(clk.div.high_width)
    << clk.div.high_shift);
    }
    if (div_l) {
    val &= ~(bit_mask(clk.div.low_width) << clk.div.low_shift);
    val |= div_l << clk.div.low_shift;
    } else {
    val &= ~(bit_mask(clk.div.low_width) << clk.div.low_shift);
    }
    writel(val, asiu.div_base + clk.div.offset);
    return 0;
    }
    static const struct clk_ops iproc_asiu_ops = {
    .enable = iproc_asiu_clk_enable,
    .disable = iproc_asiu_clk_disable,
    .recalc_rate = iproc_asiu_clk_recalc_rate,
    .determine_rate = iproc_asiu_clk_determine_rate,
    .set_rate = iproc_asiu_clk_set_rate,
    };
    void __init iproc_asiu_setup(struct device_node *node,
    const struct iproc_asiu_div *div,
    const struct iproc_asiu_gate *gate,
    unsigned int num_clks)
    {
    int i, ret;
    struct iproc_asiu *asiu;
    struct clk_hw_onecell_data *clk_data;
    if (WARN_ON(!gate || !div))
    return;
    asiu = kzalloc_flex(*asiu, clks, num_clks);
    if (WARN_ON(!asiu))
    return;
    clk_data = kzalloc_flex(*clk_data, hws, num_clks);
    if (WARN_ON(!clk_data))
    goto err_clks;
    clk_data.num = num_clks;
    asiu.div_base = of_iomap(node, 0);
    if (WARN_ON(!asiu.div_base))
    goto err_iomap_div;
    asiu.gate_base = of_iomap(node, 1);
    if (WARN_ON(!asiu.gate_base))
    goto err_iomap_gate;
    for (i = 0; i < num_clks; i++) {
    struct clk_init_data init;
    const char *parent_name;
    struct iproc_asiu_clk *asiu_clk;
    const char *clk_name;
    ret = of_property_read_string_index(node, "clock-output-names",
    i, &clk_name);
    if (WARN_ON(ret))
    goto err_clk_register;
    asiu_clk = &asiu.clks[i];
    asiu_clk.name = clk_name;
    asiu_clk.asiu = asiu;
    asiu_clk.div = div[i];
    asiu_clk.gate = gate[i];
    init.name = clk_name;
    init.ops = &iproc_asiu_ops;
    init.flags = 0;
    parent_name = of_clk_get_parent_name(node, 0);
    init.parent_names = (parent_name ? &parent_name : core::ptr::null_mut());
    init.num_parents = (parent_name ? 1 : 0);
    asiu_clk.hw.init = &init;
    ret = clk_hw_register(core::ptr::null_mut(), &asiu_clk.hw);
    if (WARN_ON(ret))
    goto err_clk_register;
    clk_data.hws[i] = &asiu_clk.hw;
    }
    ret = of_clk_add_hw_provider(node, of_clk_hw_onecell_get,
    clk_data);
    if (WARN_ON(ret))
    goto err_clk_register;
    return;
    err_clk_register:
    while (--i >= 0)
    clk_hw_unregister(clk_data.hws[i]);
    iounmap(asiu.gate_base);
    err_iomap_gate:
    iounmap(asiu.div_base);
    err_iomap_div:
    kfree(clk_data);
    err_clks:
    kfree(asiu);
    }
