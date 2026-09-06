//! Automatically rewritten from C to Rust
//! Source: drivers/clk/nuvoton/clk-ma35d1-pll.c
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

// PLL frequency limits

pub const PLL_SS_RATE: c_uint = 0x77;
pub const PLL_SLOPE: c_uint = 0x58CFA;
pub const REG_PLL_CTL0_OFFSET: c_uint = 0x0;
pub const REG_PLL_CTL1_OFFSET: c_uint = 0x4;
pub const REG_PLL_CTL2_OFFSET: c_uint = 0x8;
// bit fields for REG_CLK_PLL0CTL0, which is SMIC PLL design

// bit fields for REG_CLK_PLLxCTL0 ~ REG_CLK_PLLxCTL2, where x = 2 ~ 5

pub const INDIV_MIN: c_int = 1;
pub const INDIV_MAX: c_int = 63;
pub const FBDIV_MIN: c_int = 16;
pub const FBDIV_MAX: c_int = 2047;
pub const FBDIV_FRAC_MIN: c_int = 1600;
pub const FBDIV_FRAC_MAX: c_int = 204700;
pub const OUTDIV_MIN: c_int = 1;
pub const OUTDIV_MAX: c_int = 7;
pub const PLL_MODE_INT: c_int = 0;
pub const PLL_MODE_FRAC: c_int = 1;
pub const PLL_MODE_SS: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ma35d1_clk_pll {
    pub hw: clk_hw,
    pub id: u32,
    pub mode: u8,
    pub ctl0_base: *mut void __iomem,
    pub ctl1_base: *mut void __iomem,
    pub ctl2_base: *mut void __iomem,
}

    static inline struct ma35d1_clk_pll *to_ma35d1_clk_pll(struct clk_hw *_hw)
    {
    return container_of(_hw, struct ma35d1_clk_pll, hw);
    }
    static unsigned long ma35d1_calc_smic_pll_freq(u32 pll0_ctl0,
    unsigned long parent_rate)
    {
    u32 m, n, p, outdiv;
    u64 pll_freq;
    if (pll0_ctl0 & SPLL0_CTL0_BP)
    return parent_rate;
    n = FIELD_GET(SPLL0_CTL0_FBDIV, pll0_ctl0);
    m = FIELD_GET(SPLL0_CTL0_INDIV, pll0_ctl0);
    p = FIELD_GET(SPLL0_CTL0_OUTDIV, pll0_ctl0);
    outdiv = 1 << p;
    pll_freq = (u64)parent_rate * n;
    pll_freq = div_u64(pll_freq, m * outdiv);
    return pll_freq;
    }
#[no_mangle]
unsafe extern "C" fn ma35d1_calc_pll_freq(mode: u8, reg_ctl: *mut u32, parent_rate: c_ulong) -> c_ulong {
    static unsigned long ma35d1_calc_pll_freq(u8 mode, u32 *reg_ctl, unsigned long parent_rate)
    {
    unsigned long pll_freq, x;
    u32 m, n, p;
    if (reg_ctl[1] & PLL_CTL1_BP)
    return parent_rate;
    n = FIELD_GET(PLL_CTL0_FBDIV, reg_ctl[0]);
    m = FIELD_GET(PLL_CTL0_INDIV, reg_ctl[0]);
    p = FIELD_GET(PLL_CTL1_OUTDIV, reg_ctl[1]);
    if (mode == PLL_MODE_INT) {
    pll_freq = (u64)parent_rate * n;
    pll_freq = div_u64(pll_freq, m * p);
    } else {
    x = FIELD_GET(PLL_CTL1_FRAC, reg_ctl[1]);
// convert 24-bit fraction to 3 decimal digits, rounding to closest
    n = n * 1000 + DIV_ROUND_CLOSEST_ULL((u64)x * 1000, 1ULL << 24);
    pll_freq = div_u64((u64)parent_rate * n, 1000 * m * p);
    }
    return pll_freq;
    }
    static int ma35d1_pll_find_closest(struct ma35d1_clk_pll *pll, unsigned long rate,
    unsigned long parent_rate, u32 *reg_ctl,
    unsigned long *freq)
    {
    let mut min_diff: c_ulong = ULONG_MAX;
    int fbdiv_min, fbdiv_max;
    int p, m, n;
// freq = 0;
    if (rate < PLL_FCLKO_MIN_FREQ || rate > PLL_FCLKO_MAX_FREQ)
    return -EINVAL;
    if (pll.mode == PLL_MODE_INT) {
    fbdiv_min = FBDIV_MIN;
    fbdiv_max = FBDIV_MAX;
    } else {
    fbdiv_min = FBDIV_FRAC_MIN;
    fbdiv_max = FBDIV_FRAC_MAX;
    }
    for (m = INDIV_MIN; m <= INDIV_MAX; m++) {
    for (n = fbdiv_min; n <= fbdiv_max; n++) {
    for (p = OUTDIV_MIN; p <= OUTDIV_MAX; p++) {
    unsigned long tmp, fout, fclk, diff;
    tmp = div_u64(parent_rate, m);
    if (tmp < PLL_FREF_M_MIN_FREQ ||
    tmp > PLL_FREF_M_MAX_FREQ)
    continue; /* constrain */
    fclk = div_u64(parent_rate * n, m);
// for 2 decimal places
    if (pll.mode != PLL_MODE_INT)
    fclk = div_u64(fclk, 100);
    if (fclk < PLL_FCLK_MIN_FREQ ||
    fclk > PLL_FCLK_MAX_FREQ)
    continue; /* constrain */
    fout = div_u64(fclk, p);
    if (fout < PLL_FCLKO_MIN_FREQ ||
    fout > PLL_FCLKO_MAX_FREQ)
    continue; /* constrain */
    diff = abs(rate - fout);
    if (diff < min_diff) {
    reg_ctl[0] = FIELD_PREP(PLL_CTL0_INDIV, m) |
    FIELD_PREP(PLL_CTL0_FBDIV, n);
    reg_ctl[1] = FIELD_PREP(PLL_CTL1_OUTDIV, p);
// freq = fout;
    min_diff = diff;
    if (min_diff == 0)
    break;
    }
    }
    }
    }
    if (*freq == 0)
    return -EINVAL; /* cannot find even one valid setting */
    return 0;
    }
    static int ma35d1_clk_pll_set_rate(struct clk_hw *hw, unsigned long rate,
    unsigned long parent_rate)
    {
    struct ma35d1_clk_pll *pll = to_ma35d1_clk_pll(hw);
    u32 reg_ctl[3] = { 0 };
    unsigned long pll_freq;
    int ret;
    if (parent_rate < PLL_FREF_MIN_FREQ || parent_rate > PLL_FREF_MAX_FREQ)
    return -EINVAL;
    ret = ma35d1_pll_find_closest(pll, rate, parent_rate, reg_ctl, &pll_freq);
    if (ret != 0)
    return ret;
    switch (pll.mode) {
    case PLL_MODE_INT:
    reg_ctl[0] |= FIELD_PREP(PLL_CTL0_MODE, PLL_MODE_INT);
    break;
    case PLL_MODE_FRAC:
    reg_ctl[0] |= FIELD_PREP(PLL_CTL0_MODE, PLL_MODE_FRAC);
    break;
    case PLL_MODE_SS:
    reg_ctl[0] |= FIELD_PREP(PLL_CTL0_MODE, PLL_MODE_SS) |
    FIELD_PREP(PLL_CTL0_SSRATE, PLL_SS_RATE);
    reg_ctl[2] = FIELD_PREP(PLL_CTL2_SLOPE, PLL_SLOPE);
    break;
    }
    reg_ctl[1] |= PLL_CTL1_PD;
    writel_relaxed(reg_ctl[0], pll.ctl0_base);
    writel_relaxed(reg_ctl[1], pll.ctl1_base);
    writel_relaxed(reg_ctl[2], pll.ctl2_base);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ma35d1_clk_pll_recalc_rate(hw: *mut clk_hw, parent_rate: c_ulong) -> c_ulong {
    static unsigned long ma35d1_clk_pll_recalc_rate(struct clk_hw *hw, unsigned long parent_rate)
    {
    struct ma35d1_clk_pll *pll = to_ma35d1_clk_pll(hw);
    u32 reg_ctl[3];
    unsigned long pll_freq;
    if (parent_rate < PLL_FREF_MIN_FREQ || parent_rate > PLL_FREF_MAX_FREQ)
    return 0;
    switch (pll.id) {
    case CAPLL:
    reg_ctl[0] = readl_relaxed(pll.ctl0_base);
    pll_freq = ma35d1_calc_smic_pll_freq(reg_ctl[0], parent_rate);
    return pll_freq;
    case DDRPLL:
    case APLL:
    case EPLL:
    case VPLL:
    reg_ctl[0] = readl_relaxed(pll.ctl0_base);
    reg_ctl[1] = readl_relaxed(pll.ctl1_base);
    pll_freq = ma35d1_calc_pll_freq(pll.mode, reg_ctl, parent_rate);
    return pll_freq;
    }
    return 0;
    }
    static int ma35d1_clk_pll_determine_rate(struct clk_hw *hw,
    struct clk_rate_request *req)
    {
    struct ma35d1_clk_pll *pll = to_ma35d1_clk_pll(hw);
    u32 reg_ctl[3] = { 0 };
    unsigned long pll_freq;
    long ret;
    if (req.best_parent_rate < PLL_FREF_MIN_FREQ || req.best_parent_rate > PLL_FREF_MAX_FREQ)
    return -EINVAL;
    switch (pll.id) {
    case CAPLL:
    case DDRPLL:
// Read-only PLLs: return current rate
    reg_ctl[0] = readl_relaxed(pll.ctl0_base);
    if (pll.id == CAPLL) {
    pll_freq = ma35d1_calc_smic_pll_freq(reg_ctl[0], req.best_parent_rate);
    } else {
    reg_ctl[1] = readl_relaxed(pll.ctl1_base);
    pll_freq = ma35d1_calc_pll_freq(pll.mode, reg_ctl, req.best_parent_rate);
    }
    req.rate = pll_freq;
    return 0;
    case APLL:
    case EPLL:
    case VPLL:
// Configurable PLLs: find closest achievable rate
    ret = ma35d1_pll_find_closest(pll, req.rate, req.best_parent_rate,
    reg_ctl, &pll_freq);
    if (ret < 0)
    return ret;
    req.rate = pll_freq;
    return 0;
    }
    req.rate = 0;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ma35d1_clk_pll_is_prepared(hw: *mut clk_hw) -> c_int {
    static int ma35d1_clk_pll_is_prepared(struct clk_hw *hw)
    {
    struct ma35d1_clk_pll *pll = to_ma35d1_clk_pll(hw);
    let mut val: u32 = readl_relaxed(pll.ctl1_base);
    return !(val & PLL_CTL1_PD);
    }
#[no_mangle]
unsafe extern "C" fn ma35d1_clk_pll_prepare(hw: *mut clk_hw) -> c_int {
    static int ma35d1_clk_pll_prepare(struct clk_hw *hw)
    {
    struct ma35d1_clk_pll *pll = to_ma35d1_clk_pll(hw);
    u32 val;
    val = readl_relaxed(pll.ctl1_base);
    val &= ~PLL_CTL1_PD;
    writel_relaxed(val, pll.ctl1_base);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ma35d1_clk_pll_unprepare(hw: *mut clk_hw) {
    static void ma35d1_clk_pll_unprepare(struct clk_hw *hw)
    {
    struct ma35d1_clk_pll *pll = to_ma35d1_clk_pll(hw);
    u32 val;
    val = readl_relaxed(pll.ctl1_base);
    val |= PLL_CTL1_PD;
    writel_relaxed(val, pll.ctl1_base);
    }
    static const struct clk_ops ma35d1_clk_pll_ops = {
    .is_prepared = ma35d1_clk_pll_is_prepared,
    .prepare = ma35d1_clk_pll_prepare,
    .unprepare = ma35d1_clk_pll_unprepare,
    .set_rate = ma35d1_clk_pll_set_rate,
    .recalc_rate = ma35d1_clk_pll_recalc_rate,
    .determine_rate = ma35d1_clk_pll_determine_rate,
    };
    static const struct clk_ops ma35d1_clk_fixed_pll_ops = {
    .recalc_rate = ma35d1_clk_pll_recalc_rate,
    .determine_rate = ma35d1_clk_pll_determine_rate,
    };
    struct clk_hw *ma35d1_reg_clk_pll(struct device *dev, u32 id, u8 u8mode, const char *name,
    struct clk_hw *parent_hw, void __iomem *base)
    {
    let mut pdata: clk_parent_data = { .index = 0 };
    let mut init: clk_init_data = {};
    struct ma35d1_clk_pll *pll;
    struct clk_hw *hw;
    int ret;
    pll = devm_kzalloc(dev, sizeof(*pll), GFP_KERNEL);
    if (!pll)
    return ERR_PTR(-ENOMEM);
    pll.id = id;
    pll.mode = u8mode;
    pll.ctl0_base = base + REG_PLL_CTL0_OFFSET;
    pll.ctl1_base = base + REG_PLL_CTL1_OFFSET;
    pll.ctl2_base = base + REG_PLL_CTL2_OFFSET;
    init.name = name;
    init.flags = 0;
    pdata.hw = parent_hw;
    init.parent_data = &pdata;
    init.num_parents = 1;
    if (id == CAPLL || id == DDRPLL)
    init.ops = &ma35d1_clk_fixed_pll_ops;
    else
    init.ops = &ma35d1_clk_pll_ops;
    pll.hw.init = &init;
    hw = &pll.hw;
    ret = devm_clk_hw_register(dev, hw);
    if (ret)
    return ERR_PTR(ret);
    return hw;
    }
    EXPORT_SYMBOL_GPL(ma35d1_reg_clk_pll);
