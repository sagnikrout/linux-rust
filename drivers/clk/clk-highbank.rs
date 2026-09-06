//! Automatically rewritten from C to Rust
//! Source: drivers/clk/clk-highbank.c
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
// Copyright 2011-2012 Calxeda, Inc.
//

pub const HB_PLL_LOCK_500: c_uint = 0x20000000;
pub const HB_PLL_LOCK: c_uint = 0x10000000;
pub const HB_PLL_DIVF_SHIFT: c_int = 20;
pub const HB_PLL_DIVF_MASK: c_uint = 0x0ff00000;
pub const HB_PLL_DIVQ_SHIFT: c_int = 16;
pub const HB_PLL_DIVQ_MASK: c_uint = 0x00070000;
pub const HB_PLL_DIVR_SHIFT: c_int = 8;
pub const HB_PLL_DIVR_MASK: c_uint = 0x00001f00;
pub const HB_PLL_RANGE_SHIFT: c_int = 4;
pub const HB_PLL_RANGE_MASK: c_uint = 0x00000070;
pub const HB_PLL_BYPASS: c_uint = 0x00000008;
pub const HB_PLL_RESET: c_uint = 0x00000004;
pub const HB_PLL_EXT_BYPASS: c_uint = 0x00000002;
pub const HB_PLL_EXT_ENA: c_uint = 0x00000001;
pub const HB_PLL_VCO_MIN_FREQ: c_int = 2133000000;

pub const HB_A9_BCLK_DIV_MASK: c_uint = 0x00000006;
pub const HB_A9_BCLK_DIV_SHIFT: c_int = 1;
pub const HB_A9_PCLK_DIV: c_uint = 0x00000001;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hb_clk {
    pub hw: clk_hw,
    pub reg: *mut void __iomem,
}

#[no_mangle]
unsafe extern "C" fn clk_pll_prepare(hwclk: *mut clk_hw) -> c_int {
    static int clk_pll_prepare(struct clk_hw *hwclk)
    {
    struct hb_clk *hbclk = to_hb_clk(hwclk);
    u32 reg;
    reg = readl(hbclk.reg);
    reg &= ~HB_PLL_RESET;
    writel(reg, hbclk.reg);
    while ((readl(hbclk.reg) & HB_PLL_LOCK) == 0)
    ;
    while ((readl(hbclk.reg) & HB_PLL_LOCK_500) == 0)
    ;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn clk_pll_unprepare(hwclk: *mut clk_hw) {
    static void clk_pll_unprepare(struct clk_hw *hwclk)
    {
    struct hb_clk *hbclk = to_hb_clk(hwclk);
    u32 reg;
    reg = readl(hbclk.reg);
    reg |= HB_PLL_RESET;
    writel(reg, hbclk.reg);
    }
#[no_mangle]
unsafe extern "C" fn clk_pll_enable(hwclk: *mut clk_hw) -> c_int {
    static int clk_pll_enable(struct clk_hw *hwclk)
    {
    struct hb_clk *hbclk = to_hb_clk(hwclk);
    u32 reg;
    reg = readl(hbclk.reg);
    reg |= HB_PLL_EXT_ENA;
    writel(reg, hbclk.reg);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn clk_pll_disable(hwclk: *mut clk_hw) {
    static void clk_pll_disable(struct clk_hw *hwclk)
    {
    struct hb_clk *hbclk = to_hb_clk(hwclk);
    u32 reg;
    reg = readl(hbclk.reg);
    reg &= ~HB_PLL_EXT_ENA;
    writel(reg, hbclk.reg);
    }
    static unsigned long clk_pll_recalc_rate(struct clk_hw *hwclk,
    unsigned long parent_rate)
    {
    struct hb_clk *hbclk = to_hb_clk(hwclk);
    unsigned long divf, divq, vco_freq, reg;
    reg = readl(hbclk.reg);
    if (reg & HB_PLL_EXT_BYPASS)
    return parent_rate;
    divf = (reg & HB_PLL_DIVF_MASK) >> HB_PLL_DIVF_SHIFT;
    divq = (reg & HB_PLL_DIVQ_MASK) >> HB_PLL_DIVQ_SHIFT;
    vco_freq = parent_rate * (divf + 1);
    return vco_freq / (1 << divq);
    }
    static void clk_pll_calc(unsigned long rate, unsigned long ref_freq,
    u32 *pdivq, u32 *pdivf)
    {
    u32 divq, divf;
    unsigned long vco_freq;
    if (rate < HB_PLL_MIN_FREQ)
    rate = HB_PLL_MIN_FREQ;
    if (rate > HB_PLL_MAX_FREQ)
    rate = HB_PLL_MAX_FREQ;
    for (divq = 1; divq <= 6; divq++) {
    if ((rate * (1 << divq)) >= HB_PLL_VCO_MIN_FREQ)
    break;
    }
    vco_freq = rate * (1 << divq);
    divf = (vco_freq + (ref_freq / 2)) / ref_freq;
    divf--;
// pdivq = divq;
// pdivf = divf;
    }
    static int clk_pll_determine_rate(struct clk_hw *hw,
    struct clk_rate_request *req)
    {
    u32 divq, divf;
    let mut ref_freq: c_ulong = req.best_parent_rate;
    clk_pll_calc(req.rate, ref_freq, &divq, &divf);
    req.rate = (ref_freq * (divf + 1)) / (1 << divq);
    return 0;
    }
    static int clk_pll_set_rate(struct clk_hw *hwclk, unsigned long rate,
    unsigned long parent_rate)
    {
    struct hb_clk *hbclk = to_hb_clk(hwclk);
    u32 divq, divf;
    u32 reg;
    clk_pll_calc(rate, parent_rate, &divq, &divf);
    reg = readl(hbclk.reg);
    if (divf != ((reg & HB_PLL_DIVF_MASK) >> HB_PLL_DIVF_SHIFT)) {
// Need to re-lock PLL, so put it into bypass mode
    reg |= HB_PLL_EXT_BYPASS;
    writel(reg | HB_PLL_EXT_BYPASS, hbclk.reg);
    writel(reg | HB_PLL_RESET, hbclk.reg);
    reg &= ~(HB_PLL_DIVF_MASK | HB_PLL_DIVQ_MASK);
    reg |= (divf << HB_PLL_DIVF_SHIFT) | (divq << HB_PLL_DIVQ_SHIFT);
    writel(reg | HB_PLL_RESET, hbclk.reg);
    writel(reg, hbclk.reg);
    while ((readl(hbclk.reg) & HB_PLL_LOCK) == 0)
    ;
    while ((readl(hbclk.reg) & HB_PLL_LOCK_500) == 0)
    ;
    reg |= HB_PLL_EXT_ENA;
    reg &= ~HB_PLL_EXT_BYPASS;
    } else {
    writel(reg | HB_PLL_EXT_BYPASS, hbclk.reg);
    reg &= ~HB_PLL_DIVQ_MASK;
    reg |= divq << HB_PLL_DIVQ_SHIFT;
    writel(reg | HB_PLL_EXT_BYPASS, hbclk.reg);
    }
    writel(reg, hbclk.reg);
    return 0;
    }
    static const struct clk_ops clk_pll_ops = {
    .prepare = clk_pll_prepare,
    .unprepare = clk_pll_unprepare,
    .enable = clk_pll_enable,
    .disable = clk_pll_disable,
    .recalc_rate = clk_pll_recalc_rate,
    .determine_rate = clk_pll_determine_rate,
    .set_rate = clk_pll_set_rate,
    };
    static unsigned long clk_cpu_periphclk_recalc_rate(struct clk_hw *hwclk,
    unsigned long parent_rate)
    {
    struct hb_clk *hbclk = to_hb_clk(hwclk);
    let mut div: u32 = (readl(hbclk.reg) & HB_A9_PCLK_DIV) ? 8 : 4;
    return parent_rate / div;
    }
    static const struct clk_ops a9periphclk_ops = {
    .recalc_rate = clk_cpu_periphclk_recalc_rate,
    };
    static unsigned long clk_cpu_a9bclk_recalc_rate(struct clk_hw *hwclk,
    unsigned long parent_rate)
    {
    struct hb_clk *hbclk = to_hb_clk(hwclk);
    let mut div: u32 = (readl(hbclk.reg) & HB_A9_BCLK_DIV_MASK) >> HB_A9_BCLK_DIV_SHIFT;
    return parent_rate / (div + 2);
    }
    static const struct clk_ops a9bclk_ops = {
    .recalc_rate = clk_cpu_a9bclk_recalc_rate,
    };
    static unsigned long clk_periclk_recalc_rate(struct clk_hw *hwclk,
    unsigned long parent_rate)
    {
    struct hb_clk *hbclk = to_hb_clk(hwclk);
    u32 div;
    div = readl(hbclk.reg) & 0x1f;
    div++;
    div *= 2;
    return parent_rate / div;
    }
    static int clk_periclk_determine_rate(struct clk_hw *hw,
    struct clk_rate_request *req)
    {
    u32 div;
    div = req.best_parent_rate / req.rate;
    div++;
    div &= ~0x1;
    req.rate = req.best_parent_rate / div;
    return 0;
    }
    static int clk_periclk_set_rate(struct clk_hw *hwclk, unsigned long rate,
    unsigned long parent_rate)
    {
    struct hb_clk *hbclk = to_hb_clk(hwclk);
    u32 div;
    div = parent_rate / rate;
    if (div & 0x1)
    return -EINVAL;
    writel(div >> 1, hbclk.reg);
    return 0;
    }
    static const struct clk_ops periclk_ops = {
    .recalc_rate = clk_periclk_recalc_rate,
    .determine_rate = clk_periclk_determine_rate,
    .set_rate = clk_periclk_set_rate,
    };
#[no_mangle]
unsafe extern "C" fn hb_clk_init(node: *mut device_node, ops: *const clk_ops, clkflags: c_ulong) -> void __init {
    static void __init hb_clk_init(struct device_node *node, const struct clk_ops *ops, unsigned long clkflags)
    {
    u32 reg;
    struct hb_clk *hb_clk;
    const char *clk_name = node.name;
    const char *parent_name;
    struct clk_init_data init;
    struct device_node *srnp;
    int rc;
    rc = of_property_read_u32(node, "reg", &reg);
    if (WARN_ON(rc))
    return;
    hb_clk = kzalloc_obj(*hb_clk);
    if (WARN_ON(!hb_clk))
    return;
// Map system registers
    srnp = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), "calxeda,hb-sregs");
    hb_clk.reg = of_iomap(srnp, 0);
    of_node_put(srnp);
    BUG_ON(!hb_clk.reg);
    hb_clk.reg += reg;
    of_property_read_string(node, "clock-output-names", &clk_name);
    init.name = clk_name;
    init.ops = ops;
    init.flags = clkflags;
    parent_name = of_clk_get_parent_name(node, 0);
    init.parent_names = &parent_name;
    init.num_parents = 1;
    hb_clk.hw.init = &init;
    rc = clk_hw_register(core::ptr::null_mut(), &hb_clk.hw);
    if (WARN_ON(rc)) {
    kfree(hb_clk);
    return;
    }
    of_clk_add_hw_provider(node, of_clk_hw_simple_get, &hb_clk.hw);
    }
#[no_mangle]
unsafe extern "C" fn hb_pll_init(node: *mut device_node) -> void __init {
    static void __init hb_pll_init(struct device_node *node)
    {
    hb_clk_init(node, &clk_pll_ops, 0);
    }
    CLK_OF_DECLARE(hb_pll, "calxeda,hb-pll-clock", hb_pll_init);
#[no_mangle]
unsafe extern "C" fn hb_a9periph_init(node: *mut device_node) -> void __init {
    static void __init hb_a9periph_init(struct device_node *node)
    {
    hb_clk_init(node, &a9periphclk_ops, 0);
    }
    CLK_OF_DECLARE(hb_a9periph, "calxeda,hb-a9periph-clock", hb_a9periph_init);
#[no_mangle]
unsafe extern "C" fn hb_a9bus_init(node: *mut device_node) -> void __init {
    static void __init hb_a9bus_init(struct device_node *node)
    {
    hb_clk_init(node, &a9bclk_ops, CLK_IS_CRITICAL);
    }
    CLK_OF_DECLARE(hb_a9bus, "calxeda,hb-a9bus-clock", hb_a9bus_init);
#[no_mangle]
unsafe extern "C" fn hb_emmc_init(node: *mut device_node) -> void __init {
    static void __init hb_emmc_init(struct device_node *node)
    {
    hb_clk_init(node, &periclk_ops, 0);
    }
    CLK_OF_DECLARE(hb_emmc, "calxeda,hb-emmc-clock", hb_emmc_init);
