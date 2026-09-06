//! Automatically rewritten from C to Rust
//! Source: drivers/clk/imx/clk-fracn-gppll.c
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
// Copyright 2021 NXP
//

pub const PLL_CTRL: c_uint = 0x0;

pub const PLL_ANA_PRG: c_uint = 0x10;
pub const PLL_SPREAD_SPECTRUM: c_uint = 0x30;
pub const PLL_NUMERATOR: c_uint = 0x40;

pub const PLL_DENOMINATOR: c_uint = 0x50;

pub const PLL_DIV: c_uint = 0x60;

pub const PLL_STATUS: c_uint = 0xF0;

pub const DFS_STATUS: c_uint = 0xF4;
pub const LOCK_TIMEOUT_US: c_int = 200;

    {							\
    .rate	=	(_rate),			\
    .mfi	=	(_mfi),				\
    .mfn	=	(_mfn),				\
    .mfd	=	(_mfd),				\
    .rdiv	=	(_rdiv),			\
    .odiv	=	(_odiv),			\
    }

    {							\
    .rate	=	(_rate),			\
    .mfi	=	(_mfi),				\
    .mfn	=	0,				\
    .mfd	=	0,				\
    .rdiv	=	(_rdiv),			\
    .odiv	=	(_odiv),			\
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_fracn_gppll {
    pub hw: clk_hw,
    pub base: *mut void __iomem,
    pub rate_table: *const imx_fracn_gppll_rate_table,
    pub rate_count: c_int,
    pub flags: u32,
}

//
// Fvco = (Fref / rdiv) * (MFI + MFN / MFD)
// Fout = Fvco / odiv
// The (Fref / rdiv) should be in range 20MHz to 40MHz
// The Fvco should be in range 2.5Ghz to 5Ghz
//
    static const struct imx_fracn_gppll_rate_table fracn_tbl[] = {
    PLL_FRACN_GP(1039500000U, 173, 25, 100, 1, 4),
    PLL_FRACN_GP(650000000U, 162, 50, 100, 0, 6),
    PLL_FRACN_GP(594000000U, 198, 0, 1, 0, 8),
    PLL_FRACN_GP(560000000U, 140, 0, 1, 0, 6),
    PLL_FRACN_GP(519750000U, 173, 25, 100, 1, 8),
    PLL_FRACN_GP(498000000U, 166, 0, 1, 0, 8),
    PLL_FRACN_GP(484000000U, 121, 0, 1, 0, 6),
    PLL_FRACN_GP(477400000U, 119, 35, 100, 0, 6),
    PLL_FRACN_GP(445333333U, 167, 0, 1, 0, 9),
    PLL_FRACN_GP(400000000U, 200, 0, 1, 0, 12),
    PLL_FRACN_GP(393216000U, 163, 84, 100, 0, 10),
    PLL_FRACN_GP(333333333U, 125, 0, 1, 1, 9),
    PLL_FRACN_GP(332600000U, 138, 584, 1000, 0, 10),
    PLL_FRACN_GP(300000000U, 150, 0, 1, 0, 12),
    PLL_FRACN_GP(241900000U, 201, 584, 1000, 0, 20),
    };
    struct imx_fracn_gppll_clk imx_fracn_gppll = {
    .rate_table = fracn_tbl,
    .rate_count = ARRAY_SIZE(fracn_tbl),
    };
    EXPORT_SYMBOL_GPL(imx_fracn_gppll);
//
// Fvco = (Fref / rdiv) * MFI
// Fout = Fvco / odiv
// The (Fref / rdiv) should be in range 20MHz to 40MHz
// The Fvco should be in range 2.5Ghz to 5Ghz
//
    static const struct imx_fracn_gppll_rate_table int_tbl[] = {
    PLL_FRACN_GP_INTEGER(1700000000U, 141, 1, 2),
    PLL_FRACN_GP_INTEGER(1400000000U, 175, 1, 3),
    PLL_FRACN_GP_INTEGER(900000000U, 150, 1, 4),
    PLL_FRACN_GP_INTEGER(800000000U, 200, 1, 6),
    };
    struct imx_fracn_gppll_clk imx_fracn_gppll_integer = {
    .rate_table = int_tbl,
    .rate_count = ARRAY_SIZE(int_tbl),
    };
    EXPORT_SYMBOL_GPL(imx_fracn_gppll_integer);
    static inline struct clk_fracn_gppll *to_clk_fracn_gppll(struct clk_hw *hw)
    {
    return container_of(hw, struct clk_fracn_gppll, hw);
    }
    static const struct imx_fracn_gppll_rate_table *
    imx_get_pll_settings(struct clk_fracn_gppll *pll, unsigned long rate)
    {
    const struct imx_fracn_gppll_rate_table *rate_table = pll.rate_table;
    int i;
    for (i = 0; i < pll.rate_count; i++)
    if (rate == rate_table[i].rate)
    return &rate_table[i];
    return core::ptr::null_mut();
    }
    static int clk_fracn_gppll_determine_rate(struct clk_hw *hw,
    struct clk_rate_request *req)
    {
    struct clk_fracn_gppll *pll = to_clk_fracn_gppll(hw);
    const struct imx_fracn_gppll_rate_table *rate_table = pll.rate_table;
    int i;
// Assuming rate_table is in descending order
    for (i = 0; i < pll.rate_count; i++)
    if (req.rate >= rate_table[i].rate) {
    req.rate = rate_table[i].rate;
    return 0;
    }
// return minimum supported value
    req.rate = rate_table[pll.rate_count - 1].rate;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn clk_fracn_gppll_recalc_rate(hw: *mut clk_hw, parent_rate: c_ulong) -> c_ulong {
    static unsigned long clk_fracn_gppll_recalc_rate(struct clk_hw *hw, unsigned long parent_rate)
    {
    struct clk_fracn_gppll *pll = to_clk_fracn_gppll(hw);
    const struct imx_fracn_gppll_rate_table *rate_table = pll.rate_table;
    u32 pll_numerator, pll_denominator, pll_div;
    u32 mfi, mfn, mfd, rdiv, odiv;
    let mut fvco: u64 = parent_rate;
    let mut rate: c_long = 0;
    int i;
    pll_numerator = readl_relaxed(pll.base + PLL_NUMERATOR);
    mfn = FIELD_GET(PLL_MFN_MASK, pll_numerator);
    pll_denominator = readl_relaxed(pll.base + PLL_DENOMINATOR);
    mfd = FIELD_GET(PLL_MFD_MASK, pll_denominator);
    pll_div = readl_relaxed(pll.base + PLL_DIV);
    mfi = FIELD_GET(PLL_MFI_MASK, pll_div);
    rdiv = FIELD_GET(PLL_RDIV_MASK, pll_div);
    odiv = FIELD_GET(PLL_ODIV_MASK, pll_div);
//
// Sometimes, the recalculated rate has deviation due to
// the frac part. So find the accurate pll rate from the table
// first, if no match rate in the table, use the rate calculated
// from the equation below.
//
    for (i = 0; i < pll.rate_count; i++) {
    if (rate_table[i].mfn == mfn && rate_table[i].mfi == mfi &&
    rate_table[i].mfd == mfd && rate_table[i].rdiv == rdiv &&
    rate_table[i].odiv == odiv)
    rate = rate_table[i].rate;
    }
    if (rate)
    return (unsigned long)rate;
    if (!rdiv)
    rdiv = rdiv + 1;
    switch (odiv) {
    case 0:
    odiv = 2;
    break;
    case 1:
    odiv = 3;
    break;
    default:
    break;
    }
    if (pll.flags & CLK_FRACN_GPPLL_INTEGER) {
// Fvco = (Fref / rdiv) * MFI
    fvco = fvco * mfi;
    do_div(fvco, rdiv * odiv);
    } else {
// Fvco = (Fref / rdiv) * (MFI + MFN / MFD)
    fvco = fvco * mfi * mfd + fvco * mfn;
    do_div(fvco, mfd * rdiv * odiv);
    }
    return (unsigned long)fvco;
    }
#[no_mangle]
unsafe extern "C" fn clk_fracn_gppll_wait_lock(pll: *mut clk_fracn_gppll) -> c_int {
    static int clk_fracn_gppll_wait_lock(struct clk_fracn_gppll *pll)
    {
    u32 val;
    return readl_poll_timeout(pll.base + PLL_STATUS, val,
    val & LOCK_STATUS, 0, LOCK_TIMEOUT_US);
    }
    static int clk_fracn_gppll_set_rate(struct clk_hw *hw, unsigned long drate,
    unsigned long prate)
    {
    struct clk_fracn_gppll *pll = to_clk_fracn_gppll(hw);
    const struct imx_fracn_gppll_rate_table *rate;
    u32 tmp, pll_div, ana_mfn;
    int ret;
    rate = imx_get_pll_settings(pll, drate);
// Hardware control select disable. PLL is control by register
    tmp = readl_relaxed(pll.base + PLL_CTRL);
    tmp &= ~HW_CTRL_SEL;
    writel_relaxed(tmp, pll.base + PLL_CTRL);
// Disable output
    tmp = readl_relaxed(pll.base + PLL_CTRL);
    tmp &= ~CLKMUX_EN;
    writel_relaxed(tmp, pll.base + PLL_CTRL);
// Power Down
    tmp &= ~POWERUP_MASK;
    writel_relaxed(tmp, pll.base + PLL_CTRL);
// Disable BYPASS
    tmp &= ~CLKMUX_BYPASS;
    writel_relaxed(tmp, pll.base + PLL_CTRL);
    pll_div = FIELD_PREP(PLL_RDIV_MASK, rate.rdiv) | rate.odiv |
    FIELD_PREP(PLL_MFI_MASK, rate.mfi);
    writel_relaxed(pll_div, pll.base + PLL_DIV);
    readl(pll.base + PLL_DIV);
    if (pll.flags & CLK_FRACN_GPPLL_FRACN) {
    writel_relaxed(rate.mfd, pll.base + PLL_DENOMINATOR);
    writel_relaxed(FIELD_PREP(PLL_MFN_MASK, rate.mfn), pll.base + PLL_NUMERATOR);
    readl(pll.base + PLL_NUMERATOR);
    }
// Wait for 5us according to fracn mode pll doc
    udelay(5);
// Enable Powerup
    tmp |= POWERUP_MASK;
    writel_relaxed(tmp, pll.base + PLL_CTRL);
    readl(pll.base + PLL_CTRL);
// Wait Lock
    ret = clk_fracn_gppll_wait_lock(pll);
    if (ret)
    return ret;
// Enable output
    tmp |= CLKMUX_EN;
    writel_relaxed(tmp, pll.base + PLL_CTRL);
    ana_mfn = readl_relaxed(pll.base + PLL_STATUS);
    ana_mfn = FIELD_GET(PLL_MFN_MASK, ana_mfn);
    WARN(ana_mfn != rate.mfn, "ana_mfn != rate.mfn\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn clk_fracn_gppll_prepare(hw: *mut clk_hw) -> c_int {
    static int clk_fracn_gppll_prepare(struct clk_hw *hw)
    {
    struct clk_fracn_gppll *pll = to_clk_fracn_gppll(hw);
    u32 val;
    int ret;
    val = readl_relaxed(pll.base + PLL_CTRL);
    if (val & POWERUP_MASK)
    return 0;
    if (pll.flags & CLK_FRACN_GPPLL_FRACN)
    writel_relaxed(readl_relaxed(pll.base + PLL_NUMERATOR),
    pll.base + PLL_NUMERATOR);
    val |= CLKMUX_BYPASS;
    writel_relaxed(val, pll.base + PLL_CTRL);
    val |= POWERUP_MASK;
    writel_relaxed(val, pll.base + PLL_CTRL);
    readl(pll.base + PLL_CTRL);
    ret = clk_fracn_gppll_wait_lock(pll);
    if (ret)
    return ret;
    val |= CLKMUX_EN;
    writel_relaxed(val, pll.base + PLL_CTRL);
    val &= ~CLKMUX_BYPASS;
    writel_relaxed(val, pll.base + PLL_CTRL);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn clk_fracn_gppll_is_prepared(hw: *mut clk_hw) -> c_int {
    static int clk_fracn_gppll_is_prepared(struct clk_hw *hw)
    {
    struct clk_fracn_gppll *pll = to_clk_fracn_gppll(hw);
    u32 val;
    val = readl_relaxed(pll.base + PLL_CTRL);
    return (val & POWERUP_MASK) ? 1 : 0;
    }
#[no_mangle]
unsafe extern "C" fn clk_fracn_gppll_unprepare(hw: *mut clk_hw) {
    static void clk_fracn_gppll_unprepare(struct clk_hw *hw)
    {
    struct clk_fracn_gppll *pll = to_clk_fracn_gppll(hw);
    u32 val;
    val = readl_relaxed(pll.base + PLL_CTRL);
    val &= ~POWERUP_MASK;
    writel_relaxed(val, pll.base + PLL_CTRL);
    }
    static const struct clk_ops clk_fracn_gppll_ops = {
    .prepare	= clk_fracn_gppll_prepare,
    .unprepare	= clk_fracn_gppll_unprepare,
    .is_prepared	= clk_fracn_gppll_is_prepared,
    .recalc_rate	= clk_fracn_gppll_recalc_rate,
    .determine_rate = clk_fracn_gppll_determine_rate,
    .set_rate	= clk_fracn_gppll_set_rate,
    };
    static struct clk_hw *_imx_clk_fracn_gppll(const char *name, const char *parent_name,
    void __iomem *base,
    const struct imx_fracn_gppll_clk *pll_clk,
    u32 pll_flags)
    {
    struct clk_fracn_gppll *pll;
    struct clk_hw *hw;
    struct clk_init_data init;
    int ret;
    pll = kzalloc_obj(*pll);
    if (!pll)
    return ERR_PTR(-ENOMEM);
    init.name = name;
    init.flags = pll_clk.flags;
    init.parent_names = &parent_name;
    init.num_parents = 1;
    init.ops = &clk_fracn_gppll_ops;
    pll.base = base;
    pll.hw.init = &init;
    pll.rate_table = pll_clk.rate_table;
    pll.rate_count = pll_clk.rate_count;
    pll.flags = pll_flags;
    hw = &pll.hw;
    ret = clk_hw_register(core::ptr::null_mut(), hw);
    if (ret) {
    pr_err("%s: failed to register pll %s %d\n", __func__, name, ret);
    kfree(pll);
    return ERR_PTR(ret);
    }
    return hw;
    }
    struct clk_hw *imx_clk_fracn_gppll(const char *name, const char *parent_name, void __iomem *base,
    const struct imx_fracn_gppll_clk *pll_clk)
    {
    return _imx_clk_fracn_gppll(name, parent_name, base, pll_clk, CLK_FRACN_GPPLL_FRACN);
    }
    EXPORT_SYMBOL_GPL(imx_clk_fracn_gppll);
    struct clk_hw *imx_clk_fracn_gppll_integer(const char *name, const char *parent_name,
    void __iomem *base,
    const struct imx_fracn_gppll_clk *pll_clk)
    {
    return _imx_clk_fracn_gppll(name, parent_name, base, pll_clk, CLK_FRACN_GPPLL_INTEGER);
    }
    EXPORT_SYMBOL_GPL(imx_clk_fracn_gppll_integer);
