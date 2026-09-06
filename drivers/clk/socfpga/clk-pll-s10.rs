//! Automatically rewritten from C to Rust
//! Source: drivers/clk/socfpga/clk-pll-s10.c
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
// Copyright (C) 2017, Intel Corporation
//

// Clock Manager offsets
pub const CLK_MGR_PLL_CLK_SRC_SHIFT: c_int = 16;
pub const CLK_MGR_PLL_CLK_SRC_MASK: c_uint = 0x3;
// PLL Clock enable bits
pub const SOCFPGA_PLL_POWER: c_int = 0;
pub const SOCFPGA_PLL_RESET_MASK: c_uint = 0x2;
pub const SOCFPGA_PLL_REFDIV_MASK: c_uint = 0x00003F00;
pub const SOCFPGA_PLL_REFDIV_SHIFT: c_int = 8;
pub const SOCFPGA_PLL_AREFDIV_MASK: c_uint = 0x00000F00;
pub const SOCFPGA_PLL_DREFDIV_MASK: c_uint = 0x00003000;
pub const SOCFPGA_PLL_DREFDIV_SHIFT: c_int = 12;
pub const SOCFPGA_PLL_MDIV_MASK: c_uint = 0xFF000000;
pub const SOCFPGA_PLL_MDIV_SHIFT: c_int = 24;
pub const SOCFPGA_AGILEX_PLL_MDIV_MASK: c_uint = 0x000003FF;
pub const SWCTRLBTCLKSEL_MASK: c_uint = 0x200;
pub const SWCTRLBTCLKSEL_SHIFT: c_int = 9;

pub const SOCFPGA_N5X_PLLDIV_FDIV_SHIFT: c_int = 8;

pub const SOCFPGA_N5X_PLLDIV_QDIV_SHIFT: c_int = 24;

    static unsigned long n5x_clk_pll_recalc_rate(struct clk_hw *hwclk,
    unsigned long parent_rate)
    {
    struct socfpga_pll *socfpgaclk = to_socfpga_clk(hwclk);
    unsigned long fdiv, reg, rdiv, qdiv;
    let mut power: u32 = 1;
// read VCO1 reg for numerator and denominator
    reg = readl(socfpgaclk.hw.reg + 0x8);
    fdiv = (reg & SOCFPGA_N5X_PLLDIV_FDIV_MASK) >> SOCFPGA_N5X_PLLDIV_FDIV_SHIFT;
    rdiv = (reg & SOCFPGA_N5X_PLLDIV_RDIV_MASK);
    qdiv = (reg & SOCFPGA_N5X_PLLDIV_QDIV_MASK) >> SOCFPGA_N5X_PLLDIV_QDIV_SHIFT;
    while (qdiv) {
    power *= 2;
    qdiv--;
    }
    return ((parent_rate * 2 * (fdiv + 1)) / ((rdiv + 1) * power));
    }
    static unsigned long agilex_clk_pll_recalc_rate(struct clk_hw *hwclk,
    unsigned long parent_rate)
    {
    struct socfpga_pll *socfpgaclk = to_socfpga_clk(hwclk);
    unsigned long arefdiv, reg, mdiv;
    unsigned long long vco_freq;
// read VCO1 reg for numerator and denominator
    reg = readl(socfpgaclk.hw.reg);
    arefdiv = (reg & SOCFPGA_PLL_AREFDIV_MASK) >> SOCFPGA_PLL_REFDIV_SHIFT;
    vco_freq = (unsigned long long)parent_rate / arefdiv;
// Read mdiv and fdiv from the fdbck register
    reg = readl(socfpgaclk.hw.reg + 0x24);
    mdiv = reg & SOCFPGA_AGILEX_PLL_MDIV_MASK;
    vco_freq = (unsigned long long)vco_freq * mdiv;
    return (unsigned long)vco_freq;
    }
    static unsigned long clk_pll_recalc_rate(struct clk_hw *hwclk,
    unsigned long parent_rate)
    {
    struct socfpga_pll *socfpgaclk = to_socfpga_clk(hwclk);
    u32 mdiv;
    u32 refdiv;
    u32 reg;
    unsigned long long vco_freq;
// read VCO1 reg for numerator and denominator
    reg = readl(socfpgaclk.hw.reg);
    refdiv = (reg & SOCFPGA_PLL_REFDIV_MASK) >> SOCFPGA_PLL_REFDIV_SHIFT;
    vco_freq = parent_rate;
    do_div(vco_freq, refdiv);
// Read mdiv and fdiv from the fdbck register
    reg = readl(socfpgaclk.hw.reg + 0x4);
    mdiv = (reg & SOCFPGA_PLL_MDIV_MASK) >> SOCFPGA_PLL_MDIV_SHIFT;
    vco_freq = (unsigned long long)vco_freq * (mdiv + 6);
    return (unsigned long)vco_freq;
    }
    static unsigned long clk_boot_clk_recalc_rate(struct clk_hw *hwclk,
    unsigned long parent_rate)
    {
    struct socfpga_pll *socfpgaclk = to_socfpga_clk(hwclk);
    u32 div;
    div = ((readl(socfpgaclk.hw.reg) &
    SWCTRLBTCLKSEL_MASK) >>
    SWCTRLBTCLKSEL_SHIFT);
    div += 1;
    return parent_rate / div;
    }
#[no_mangle]
unsafe extern "C" fn clk_pll_get_parent(hwclk: *mut clk_hw) -> u8 {
    static u8 clk_pll_get_parent(struct clk_hw *hwclk)
    {
    struct socfpga_pll *socfpgaclk = to_socfpga_clk(hwclk);
    u32 pll_src;
    pll_src = readl(socfpgaclk.hw.reg);
    return (pll_src >> CLK_MGR_PLL_CLK_SRC_SHIFT) &
    CLK_MGR_PLL_CLK_SRC_MASK;
    }
#[no_mangle]
unsafe extern "C" fn clk_boot_get_parent(hwclk: *mut clk_hw) -> u8 {
    static u8 clk_boot_get_parent(struct clk_hw *hwclk)
    {
    struct socfpga_pll *socfpgaclk = to_socfpga_clk(hwclk);
    u32 pll_src;
    pll_src = readl(socfpgaclk.hw.reg);
    return (pll_src >> SWCTRLBTCLKSEL_SHIFT) &
    SWCTRLBTCLKSEL_MASK;
    }
#[no_mangle]
unsafe extern "C" fn clk_pll_prepare(hwclk: *mut clk_hw) -> c_int {
    static int clk_pll_prepare(struct clk_hw *hwclk)
    {
    struct socfpga_pll *socfpgaclk = to_socfpga_clk(hwclk);
    u32 reg;
// Bring PLL out of reset
    reg = readl(socfpgaclk.hw.reg);
    reg |= SOCFPGA_PLL_RESET_MASK;
    writel(reg, socfpgaclk.hw.reg);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn n5x_clk_pll_prepare(hwclk: *mut clk_hw) -> c_int {
    static int n5x_clk_pll_prepare(struct clk_hw *hwclk)
    {
    struct socfpga_pll *socfpgaclk = to_socfpga_clk(hwclk);
    u32 reg;
// Bring PLL out of reset
    reg = readl(socfpgaclk.hw.reg + 0x4);
    reg |= SOCFPGA_PLL_RESET_MASK;
    writel(reg, socfpgaclk.hw.reg + 0x4);
    return 0;
    }
    static const struct clk_ops n5x_clk_pll_ops = {
    .recalc_rate = n5x_clk_pll_recalc_rate,
    .get_parent = clk_pll_get_parent,
    .prepare = n5x_clk_pll_prepare,
    };
    static const struct clk_ops agilex_clk_pll_ops = {
    .recalc_rate = agilex_clk_pll_recalc_rate,
    .get_parent = clk_pll_get_parent,
    .prepare = clk_pll_prepare,
    };
    static const struct clk_ops clk_pll_ops = {
    .recalc_rate = clk_pll_recalc_rate,
    .get_parent = clk_pll_get_parent,
    .prepare = clk_pll_prepare,
    };
    static const struct clk_ops clk_boot_ops = {
    .recalc_rate = clk_boot_clk_recalc_rate,
    .get_parent = clk_boot_get_parent,
    .prepare = clk_pll_prepare,
    };
    struct clk_hw *s10_register_pll(const struct stratix10_pll_clock *clks,
    void __iomem *reg)
    {
    struct clk_hw *hw_clk;
    struct socfpga_pll *pll_clk;
    struct clk_init_data init;
    const char *name = clks.name;
    int ret;
    pll_clk = kzalloc_obj(*pll_clk);
    if (WARN_ON(!pll_clk))
    return core::ptr::null_mut();
    pll_clk.hw.reg = reg + clks.offset;
    if (streq(name, SOCFPGA_BOOT_CLK))
    init.ops = &clk_boot_ops;
    else
    init.ops = &clk_pll_ops;
    init.name = name;
    init.flags = clks.flags;
    init.num_parents = clks.num_parents;
    init.parent_names = core::ptr::null_mut();
    init.parent_data = clks.parent_data;
    pll_clk.hw.hw.init = &init;
    pll_clk.hw.bit_idx = SOCFPGA_PLL_POWER;
    hw_clk = &pll_clk.hw.hw;
    ret = clk_hw_register(core::ptr::null_mut(), hw_clk);
    if (ret) {
    kfree(pll_clk);
    return ERR_PTR(ret);
    }
    return hw_clk;
    }
    struct clk_hw *agilex_register_pll(const struct stratix10_pll_clock *clks,
    void __iomem *reg)
    {
    struct clk_hw *hw_clk;
    struct socfpga_pll *pll_clk;
    struct clk_init_data init;
    const char *name = clks.name;
    int ret;
    pll_clk = kzalloc_obj(*pll_clk);
    if (WARN_ON(!pll_clk))
    return core::ptr::null_mut();
    pll_clk.hw.reg = reg + clks.offset;
    if (streq(name, SOCFPGA_BOOT_CLK))
    init.ops = &clk_boot_ops;
    else
    init.ops = &agilex_clk_pll_ops;
    init.name = name;
    init.flags = clks.flags;
    init.num_parents = clks.num_parents;
    init.parent_names = core::ptr::null_mut();
    init.parent_data = clks.parent_data;
    pll_clk.hw.hw.init = &init;
    pll_clk.hw.bit_idx = SOCFPGA_PLL_POWER;
    hw_clk = &pll_clk.hw.hw;
    ret = clk_hw_register(core::ptr::null_mut(), hw_clk);
    if (ret) {
    kfree(pll_clk);
    return ERR_PTR(ret);
    }
    return hw_clk;
    }
    struct clk_hw *n5x_register_pll(const struct stratix10_pll_clock *clks,
    void __iomem *reg)
    {
    struct clk_hw *hw_clk;
    struct socfpga_pll *pll_clk;
    struct clk_init_data init;
    const char *name = clks.name;
    int ret;
    pll_clk = kzalloc_obj(*pll_clk);
    if (WARN_ON(!pll_clk))
    return core::ptr::null_mut();
    pll_clk.hw.reg = reg + clks.offset;
    if (streq(name, SOCFPGA_BOOT_CLK))
    init.ops = &clk_boot_ops;
    else
    init.ops = &n5x_clk_pll_ops;
    init.name = name;
    init.flags = clks.flags;
    init.num_parents = clks.num_parents;
    init.parent_names = core::ptr::null_mut();
    init.parent_data = clks.parent_data;
    pll_clk.hw.hw.init = &init;
    pll_clk.hw.bit_idx = SOCFPGA_PLL_POWER;
    hw_clk = &pll_clk.hw.hw;
    ret = clk_hw_register(core::ptr::null_mut(), hw_clk);
    if (ret) {
    kfree(pll_clk);
    return ERR_PTR(ret);
    }
    return hw_clk;
    }
    struct clk_hw *agilex5_register_pll(const struct agilex5_pll_clock *clks,
    void __iomem *reg)
    {
    struct clk_hw *hw_clk;
    struct socfpga_pll *pll_clk;
    struct clk_init_data init;
    const char *name = clks.name;
    int ret;
    pll_clk = kzalloc_obj(*pll_clk);
    if (WARN_ON(!pll_clk))
    return core::ptr::null_mut();
    pll_clk.hw.reg = reg + clks.offset;
    if (streq(name, SOCFPGA_BOOT_CLK))
    init.ops = &clk_boot_ops;
    else
    init.ops = &agilex_clk_pll_ops;
    init.name = name;
    init.flags = clks.flags;
    init.num_parents = clks.num_parents;
    init.parent_names = clks.parent_names;
    pll_clk.hw.hw.init = &init;
    pll_clk.hw.bit_idx = SOCFPGA_PLL_POWER;
    hw_clk = &pll_clk.hw.hw;
    ret = clk_hw_register(core::ptr::null_mut(), hw_clk);
    if (ret) {
    kfree(pll_clk);
    return ERR_PTR(ret);
    }
    return hw_clk;
    }
