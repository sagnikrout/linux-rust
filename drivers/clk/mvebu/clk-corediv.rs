//! Automatically rewritten from C to Rust
//! Source: drivers/clk/mvebu/clk-corediv.c
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


// SPDX-License-Identifier: GPL-2.0
//
// MVEBU Core divider clock
//
// Copyright (C) 2013 Marvell
//
// Ezequiel Garcia <ezequiel.garcia@free-electrons.com>
//

pub const CORE_CLK_DIV_RATIO_MASK: c_uint = 0xff;
//
// This structure describes the hardware details (bit offset and mask)
// to configure one particular core divider clock. Those hardware
// details may differ from one SoC to another. This structure is
// therefore typically instantiated statically to describe the
// hardware details.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_corediv_desc {
    pub mask: c_uint,
    pub offset: c_uint,
    pub fieldbit: c_uint,
}

//
// This structure describes the hardware details to configure the core
// divider clocks on a given SoC. Amongst others, it points to the
// array of core divider clock descriptors for this SoC, as well as
// the corresponding operations to manipulate them.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_corediv_soc_desc {
    pub descs: *const clk_corediv_desc,
    pub ndescs: c_uint,
    pub ops: clk_ops,
    pub ratio_reload: u32,
    pub enable_bit_offset: u32,
    pub ratio_offset: u32,
}

//
// This structure represents one core divider clock for the clock
// framework, and is dynamically allocated for each core divider clock
// existing in the current SoC.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_corediv {
    pub hw: clk_hw,
    pub reg: *mut void __iomem,
    pub desc: *const clk_corediv_desc,
    pub soc_desc: *const clk_corediv_soc_desc,
    pub lock: spinlock_t,
}

    static struct clk_onecell_data clk_data;
//
// Description of the core divider clocks available. For now, we
// support only NAND, and it is available at the same register
// locations regardless of the SoC.
//
    static const struct clk_corediv_desc mvebu_corediv_desc[] = {
    { .mask = 0x3f, .offset = 8, .fieldbit = 1 }, /* NAND clock */
    };
    static const struct clk_corediv_desc mv98dx3236_corediv_desc[] = {
    { .mask = 0x0f, .offset = 6, .fieldbit = 27 }, /* NAND clock */
    };

#[no_mangle]
unsafe extern "C" fn clk_corediv_is_enabled(hwclk: *mut clk_hw) -> c_int {
    static int clk_corediv_is_enabled(struct clk_hw *hwclk)
    {
    struct clk_corediv *corediv = to_corediv_clk(hwclk);
    const struct clk_corediv_soc_desc *soc_desc = corediv.soc_desc;
    const struct clk_corediv_desc *desc = corediv.desc;
    let mut enable_mask: u32 = BIT(desc.fieldbit) << soc_desc.enable_bit_offset;
    return !!(readl(corediv.reg) & enable_mask);
    }
#[no_mangle]
unsafe extern "C" fn clk_corediv_enable(hwclk: *mut clk_hw) -> c_int {
    static int clk_corediv_enable(struct clk_hw *hwclk)
    {
    struct clk_corediv *corediv = to_corediv_clk(hwclk);
    const struct clk_corediv_soc_desc *soc_desc = corediv.soc_desc;
    const struct clk_corediv_desc *desc = corediv.desc;
    let mut flags: c_ulong = 0;
    u32 reg;
    spin_lock_irqsave(&corediv.lock, flags);
    reg = readl(corediv.reg);
    reg |= (BIT(desc.fieldbit) << soc_desc.enable_bit_offset);
    writel(reg, corediv.reg);
    spin_unlock_irqrestore(&corediv.lock, flags);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn clk_corediv_disable(hwclk: *mut clk_hw) {
    static void clk_corediv_disable(struct clk_hw *hwclk)
    {
    struct clk_corediv *corediv = to_corediv_clk(hwclk);
    const struct clk_corediv_soc_desc *soc_desc = corediv.soc_desc;
    const struct clk_corediv_desc *desc = corediv.desc;
    let mut flags: c_ulong = 0;
    u32 reg;
    spin_lock_irqsave(&corediv.lock, flags);
    reg = readl(corediv.reg);
    reg &= ~(BIT(desc.fieldbit) << soc_desc.enable_bit_offset);
    writel(reg, corediv.reg);
    spin_unlock_irqrestore(&corediv.lock, flags);
    }
    static unsigned long clk_corediv_recalc_rate(struct clk_hw *hwclk,
    unsigned long parent_rate)
    {
    struct clk_corediv *corediv = to_corediv_clk(hwclk);
    const struct clk_corediv_soc_desc *soc_desc = corediv.soc_desc;
    const struct clk_corediv_desc *desc = corediv.desc;
    u32 reg, div;
    reg = readl(corediv.reg + soc_desc.ratio_offset);
    div = (reg >> desc.offset) & desc.mask;
    return parent_rate / div;
    }
    static int clk_corediv_determine_rate(struct clk_hw *hw,
    struct clk_rate_request *req)
    {
// Valid ratio are 1:4, 1:5, 1:6 and 1:8
    u32 div;
    div = req.best_parent_rate / req.rate;
    if (div < 4)
    div = 4;
#[no_mangle]
pub unsafe extern "C" fn if(6: div >) -> else {
    else if (div > 6)
    div = 8;
    req.rate = req.best_parent_rate / div;
    return 0;
    }
    static int clk_corediv_set_rate(struct clk_hw *hwclk, unsigned long rate,
    unsigned long parent_rate)
    {
    struct clk_corediv *corediv = to_corediv_clk(hwclk);
    const struct clk_corediv_soc_desc *soc_desc = corediv.soc_desc;
    const struct clk_corediv_desc *desc = corediv.desc;
    let mut flags: c_ulong = 0;
    u32 reg, div;
    div = parent_rate / rate;
    spin_lock_irqsave(&corediv.lock, flags);
// Write new divider to the divider ratio register
    reg = readl(corediv.reg + soc_desc.ratio_offset);
    reg &= ~(desc.mask << desc.offset);
    reg |= (div & desc.mask) << desc.offset;
    writel(reg, corediv.reg + soc_desc.ratio_offset);
// Set reload-force for this clock
    reg = readl(corediv.reg) | BIT(desc.fieldbit);
    writel(reg, corediv.reg);
// Now trigger the clock update
    reg = readl(corediv.reg) | soc_desc.ratio_reload;
    writel(reg, corediv.reg);
//
// Wait for clocks to settle down, and then clear all the
// ratios request and the reload request.
//
    udelay(1000);
    reg &= ~(CORE_CLK_DIV_RATIO_MASK | soc_desc.ratio_reload);
    writel(reg, corediv.reg);
    udelay(1000);
    spin_unlock_irqrestore(&corediv.lock, flags);
    return 0;
    }
    static const struct clk_corediv_soc_desc armada370_corediv_soc = {
    .descs = mvebu_corediv_desc,
    .ndescs = ARRAY_SIZE(mvebu_corediv_desc),
    .ops = {
    .enable = clk_corediv_enable,
    .disable = clk_corediv_disable,
    .is_enabled = clk_corediv_is_enabled,
    .recalc_rate = clk_corediv_recalc_rate,
    .determine_rate = clk_corediv_determine_rate,
    .set_rate = clk_corediv_set_rate,
    },
    .ratio_reload = BIT(8),
    .enable_bit_offset = 24,
    .ratio_offset = 0x8,
    };
    static const struct clk_corediv_soc_desc armada380_corediv_soc = {
    .descs = mvebu_corediv_desc,
    .ndescs = ARRAY_SIZE(mvebu_corediv_desc),
    .ops = {
    .enable = clk_corediv_enable,
    .disable = clk_corediv_disable,
    .is_enabled = clk_corediv_is_enabled,
    .recalc_rate = clk_corediv_recalc_rate,
    .determine_rate = clk_corediv_determine_rate,
    .set_rate = clk_corediv_set_rate,
    },
    .ratio_reload = BIT(8),
    .enable_bit_offset = 16,
    .ratio_offset = 0x4,
    };
    static const struct clk_corediv_soc_desc armada375_corediv_soc = {
    .descs = mvebu_corediv_desc,
    .ndescs = ARRAY_SIZE(mvebu_corediv_desc),
    .ops = {
    .recalc_rate = clk_corediv_recalc_rate,
    .determine_rate = clk_corediv_determine_rate,
    .set_rate = clk_corediv_set_rate,
    },
    .ratio_reload = BIT(8),
    .ratio_offset = 0x4,
    };
    static const struct clk_corediv_soc_desc mv98dx3236_corediv_soc = {
    .descs = mv98dx3236_corediv_desc,
    .ndescs = ARRAY_SIZE(mv98dx3236_corediv_desc),
    .ops = {
    .recalc_rate = clk_corediv_recalc_rate,
    .determine_rate = clk_corediv_determine_rate,
    .set_rate = clk_corediv_set_rate,
    },
    .ratio_reload = BIT(10),
    .ratio_offset = 0x8,
    };
    static void __init
    mvebu_corediv_clk_init(struct device_node *node,
    const struct clk_corediv_soc_desc *soc_desc)
    {
    struct clk_init_data init;
    struct clk_corediv *corediv;
    struct clk **clks;
    void __iomem *base;
    const char *parent_name;
    const char *clk_name;
    int i;
    base = of_iomap(node, 0);
    if (WARN_ON(!base))
    return;
    parent_name = of_clk_get_parent_name(node, 0);
    clk_data.clk_num = soc_desc.ndescs;
// clks holds the clock array
    clks = kzalloc_objs(struct clk *, clk_data.clk_num);
    if (WARN_ON(!clks))
    goto err_unmap;
// corediv holds the clock specific array
    corediv = kzalloc_objs(struct clk_corediv, clk_data.clk_num);
    if (WARN_ON(!corediv))
    goto err_free_clks;
    spin_lock_init(&corediv.lock);
    for (i = 0; i < clk_data.clk_num; i++) {
    of_property_read_string_index(node, "clock-output-names",
    i, &clk_name);
    init.num_parents = 1;
    init.parent_names = &parent_name;
    init.name = clk_name;
    init.ops = &soc_desc.ops;
    init.flags = 0;
    corediv[i].soc_desc = soc_desc;
    corediv[i].desc = soc_desc.descs + i;
    corediv[i].reg = base;
    corediv[i].hw.init = &init;
    clks[i] = clk_register(core::ptr::null_mut(), &corediv[i].hw);
    WARN_ON(IS_ERR(clks[i]));
    }
    clk_data.clks = clks;
    of_clk_add_provider(node, of_clk_src_onecell_get, &clk_data);
    return;
    err_free_clks:
    kfree(clks);
    err_unmap:
    iounmap(base);
    }
#[no_mangle]
unsafe extern "C" fn armada370_corediv_clk_init(node: *mut device_node) -> void __init {
    static void __init armada370_corediv_clk_init(struct device_node *node)
    {
    return mvebu_corediv_clk_init(node, &armada370_corediv_soc);
    }
    CLK_OF_DECLARE(armada370_corediv_clk, "marvell,armada-370-corediv-clock",
    armada370_corediv_clk_init);
#[no_mangle]
unsafe extern "C" fn armada375_corediv_clk_init(node: *mut device_node) -> void __init {
    static void __init armada375_corediv_clk_init(struct device_node *node)
    {
    return mvebu_corediv_clk_init(node, &armada375_corediv_soc);
    }
    CLK_OF_DECLARE(armada375_corediv_clk, "marvell,armada-375-corediv-clock",
    armada375_corediv_clk_init);
#[no_mangle]
unsafe extern "C" fn armada380_corediv_clk_init(node: *mut device_node) -> void __init {
    static void __init armada380_corediv_clk_init(struct device_node *node)
    {
    return mvebu_corediv_clk_init(node, &armada380_corediv_soc);
    }
    CLK_OF_DECLARE(armada380_corediv_clk, "marvell,armada-380-corediv-clock",
    armada380_corediv_clk_init);
#[no_mangle]
unsafe extern "C" fn mv98dx3236_corediv_clk_init(node: *mut device_node) -> void __init {
    static void __init mv98dx3236_corediv_clk_init(struct device_node *node)
    {
    return mvebu_corediv_clk_init(node, &mv98dx3236_corediv_soc);
    }
    CLK_OF_DECLARE(mv98dx3236_corediv_clk, "marvell,mv98dx3236-corediv-clock",
    mv98dx3236_corediv_clk_init);
