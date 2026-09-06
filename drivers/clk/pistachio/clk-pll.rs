//! Automatically rewritten from C to Rust
//! Source: drivers/clk/pistachio/clk-pll.c
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
// Copyright (C) 2014 Google, Inc.
//

pub const PLL_STATUS: c_uint = 0x0;

pub const PLL_CTRL1: c_uint = 0x4;
pub const PLL_CTRL1_REFDIV_SHIFT: c_int = 0;
pub const PLL_CTRL1_REFDIV_MASK: c_uint = 0x3f;
pub const PLL_CTRL1_FBDIV_SHIFT: c_int = 6;
pub const PLL_CTRL1_FBDIV_MASK: c_uint = 0xfff;
pub const PLL_INT_CTRL1_POSTDIV1_SHIFT: c_int = 18;
pub const PLL_INT_CTRL1_POSTDIV1_MASK: c_uint = 0x7;
pub const PLL_INT_CTRL1_POSTDIV2_SHIFT: c_int = 21;
pub const PLL_INT_CTRL1_POSTDIV2_MASK: c_uint = 0x7;

pub const PLL_CTRL2: c_uint = 0x8;
pub const PLL_FRAC_CTRL2_FRAC_SHIFT: c_int = 0;
pub const PLL_FRAC_CTRL2_FRAC_MASK: c_uint = 0xffffff;
pub const PLL_FRAC_CTRL2_POSTDIV1_SHIFT: c_int = 24;
pub const PLL_FRAC_CTRL2_POSTDIV1_MASK: c_uint = 0x7;
pub const PLL_FRAC_CTRL2_POSTDIV2_SHIFT: c_int = 27;
pub const PLL_FRAC_CTRL2_POSTDIV2_MASK: c_uint = 0x7;

pub const PLL_CTRL3: c_uint = 0xc;

pub const PLL_CTRL4: c_uint = 0x10;

// Fractional PLL operating modes
    enum pll_mode {
    PLL_MODE_FRAC,
    PLL_MODE_INT,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pistachio_clk_pll {
    pub hw: clk_hw,
    pub base: *mut void __iomem,
    pub rates: *mut pistachio_pll_rate_table,
    pub nr_rates: c_uint,
}

#[no_mangle]
pub unsafe extern "C" fn pll_readl(pll: *mut pistachio_clk_pll, reg: u32) -> u32 {
    static inline u32 pll_readl(struct pistachio_clk_pll *pll, u32 reg)
    {
    return readl(pll.base + reg);
    }
#[no_mangle]
pub unsafe extern "C" fn pll_writel(pll: *mut pistachio_clk_pll, val: u32, reg: u32) {
    static inline void pll_writel(struct pistachio_clk_pll *pll, u32 val, u32 reg)
    {
    writel(val, pll.base + reg);
    }
#[no_mangle]
pub unsafe extern "C" fn pll_lock(pll: *mut pistachio_clk_pll) {
    static inline void pll_lock(struct pistachio_clk_pll *pll)
    {
    while (!(pll_readl(pll, PLL_STATUS) & PLL_STATUS_LOCK))
    cpu_relax();
    }
#[no_mangle]
pub unsafe extern "C" fn do_div_round_closest(dividend: u64, divisor: u64) -> u64 {
    static inline u64 do_div_round_closest(u64 dividend, u64 divisor)
    {
    dividend += divisor / 2;
    return div64_u64(dividend, divisor);
    }
    static inline struct pistachio_clk_pll *to_pistachio_pll(struct clk_hw *hw)
    {
    return container_of(hw, struct pistachio_clk_pll, hw);
    }
#[no_mangle]
pub unsafe extern "C" fn pll_frac_get_mode(hw: *mut clk_hw) -> enum pll_mode {
    static inline enum pll_mode pll_frac_get_mode(struct clk_hw *hw)
    {
    struct pistachio_clk_pll *pll = to_pistachio_pll(hw);
    u32 val;
    val = pll_readl(pll, PLL_CTRL3) & PLL_FRAC_CTRL3_DSMPD;
    return val ? PLL_MODE_INT : PLL_MODE_FRAC;
    }
#[no_mangle]
pub unsafe extern "C" fn pll_frac_set_mode(hw: *mut clk_hw, mode: enum pll_mode) {
    static inline void pll_frac_set_mode(struct clk_hw *hw, enum pll_mode mode)
    {
    struct pistachio_clk_pll *pll = to_pistachio_pll(hw);
    u32 val;
    val = pll_readl(pll, PLL_CTRL3);
    if (mode == PLL_MODE_INT)
    val |= PLL_FRAC_CTRL3_DSMPD | PLL_FRAC_CTRL3_DACPD;
    else
    val &= ~(PLL_FRAC_CTRL3_DSMPD | PLL_FRAC_CTRL3_DACPD);
    pll_writel(pll, val, PLL_CTRL3);
    }
    static struct pistachio_pll_rate_table *
    pll_get_params(struct pistachio_clk_pll *pll, unsigned long fref,
    unsigned long fout)
    {
    unsigned int i;
    for (i = 0; i < pll.nr_rates; i++) {
    if (pll.rates[i].fref == fref && pll.rates[i].fout == fout)
    return &pll.rates[i];
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn pll_determine_rate(hw: *mut clk_hw, req: *mut clk_rate_request) -> c_int {
    static int pll_determine_rate(struct clk_hw *hw, struct clk_rate_request *req)
    {
    struct pistachio_clk_pll *pll = to_pistachio_pll(hw);
    unsigned int i;
    for (i = 0; i < pll.nr_rates; i++) {
    if (i > 0 && pll.rates[i].fref == req.best_parent_rate &&
    pll.rates[i].fout <= req.rate) {
    req.rate = pll.rates[i - 1].fout;
    return 0;
    }
    }
    req.rate = pll.rates[0].fout;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pll_gf40lp_frac_enable(hw: *mut clk_hw) -> c_int {
    static int pll_gf40lp_frac_enable(struct clk_hw *hw)
    {
    struct pistachio_clk_pll *pll = to_pistachio_pll(hw);
    u32 val;
    val = pll_readl(pll, PLL_CTRL3);
    val &= ~(PLL_FRAC_CTRL3_PD | PLL_FRAC_CTRL3_FOUTPOSTDIVPD |
    PLL_FRAC_CTRL3_FOUT4PHASEPD | PLL_FRAC_CTRL3_FOUTVCOPD);
    pll_writel(pll, val, PLL_CTRL3);
    val = pll_readl(pll, PLL_CTRL4);
    val &= ~PLL_FRAC_CTRL4_BYPASS;
    pll_writel(pll, val, PLL_CTRL4);
    pll_lock(pll);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pll_gf40lp_frac_disable(hw: *mut clk_hw) {
    static void pll_gf40lp_frac_disable(struct clk_hw *hw)
    {
    struct pistachio_clk_pll *pll = to_pistachio_pll(hw);
    u32 val;
    val = pll_readl(pll, PLL_CTRL3);
    val |= PLL_FRAC_CTRL3_PD;
    pll_writel(pll, val, PLL_CTRL3);
    }
#[no_mangle]
unsafe extern "C" fn pll_gf40lp_frac_is_enabled(hw: *mut clk_hw) -> c_int {
    static int pll_gf40lp_frac_is_enabled(struct clk_hw *hw)
    {
    struct pistachio_clk_pll *pll = to_pistachio_pll(hw);
    return !(pll_readl(pll, PLL_CTRL3) & PLL_FRAC_CTRL3_PD);
    }
    static int pll_gf40lp_frac_set_rate(struct clk_hw *hw, unsigned long rate,
    unsigned long parent_rate)
    {
    struct pistachio_clk_pll *pll = to_pistachio_pll(hw);
    struct pistachio_pll_rate_table *params;
    let mut enabled: c_int = pll_gf40lp_frac_is_enabled(hw);
    u64 val, vco, old_postdiv1, old_postdiv2;
    const char *name = clk_hw_get_name(hw);
    if (rate < MIN_OUTPUT_FRAC || rate > MAX_OUTPUT_FRAC)
    return -EINVAL;
    params = pll_get_params(pll, parent_rate, rate);
    if (!params || !params.refdiv)
    return -EINVAL;
// calculate vco
    vco = params.fref;
    vco *= (params.fbdiv << 24) + params.frac;
    vco = div64_u64(vco, params.refdiv << 24);
    if (vco < MIN_VCO_FRAC_FRAC || vco > MAX_VCO_FRAC_FRAC)
    pr_warn("%s: VCO %llu is out of range %lu..%lu\n", name, vco,
    MIN_VCO_FRAC_FRAC, MAX_VCO_FRAC_FRAC);
    val = div64_u64(params.fref, params.refdiv);
    if (val < MIN_PFD)
    pr_warn("%s: PFD %llu is too low (min %lu)\n",
    name, val, MIN_PFD);
    if (val > vco / 16)
    pr_warn("%s: PFD %llu is too high (max %llu)\n",
    name, val, vco / 16);
    val = pll_readl(pll, PLL_CTRL1);
    val &= ~((PLL_CTRL1_REFDIV_MASK << PLL_CTRL1_REFDIV_SHIFT) |
    (PLL_CTRL1_FBDIV_MASK << PLL_CTRL1_FBDIV_SHIFT));
    val |= (params.refdiv << PLL_CTRL1_REFDIV_SHIFT) |
    (params.fbdiv << PLL_CTRL1_FBDIV_SHIFT);
    pll_writel(pll, val, PLL_CTRL1);
    val = pll_readl(pll, PLL_CTRL2);
    old_postdiv1 = (val >> PLL_FRAC_CTRL2_POSTDIV1_SHIFT) &
    PLL_FRAC_CTRL2_POSTDIV1_MASK;
    old_postdiv2 = (val >> PLL_FRAC_CTRL2_POSTDIV2_SHIFT) &
    PLL_FRAC_CTRL2_POSTDIV2_MASK;
    if (enabled &&
    (params.postdiv1 != old_postdiv1 ||
    params.postdiv2 != old_postdiv2))
    pr_warn("%s: changing postdiv while PLL is enabled\n", name);
    if (params.postdiv2 > params.postdiv1)
    pr_warn("%s: postdiv2 should not exceed postdiv1\n", name);
    val &= ~((PLL_FRAC_CTRL2_FRAC_MASK << PLL_FRAC_CTRL2_FRAC_SHIFT) |
    (PLL_FRAC_CTRL2_POSTDIV1_MASK <<
    PLL_FRAC_CTRL2_POSTDIV1_SHIFT) |
    (PLL_FRAC_CTRL2_POSTDIV2_MASK <<
    PLL_FRAC_CTRL2_POSTDIV2_SHIFT));
    val |= (params.frac << PLL_FRAC_CTRL2_FRAC_SHIFT) |
    (params.postdiv1 << PLL_FRAC_CTRL2_POSTDIV1_SHIFT) |
    (params.postdiv2 << PLL_FRAC_CTRL2_POSTDIV2_SHIFT);
    pll_writel(pll, val, PLL_CTRL2);
// set operating mode
    if (params.frac)
    pll_frac_set_mode(hw, PLL_MODE_FRAC);
    else
    pll_frac_set_mode(hw, PLL_MODE_INT);
    if (enabled)
    pll_lock(pll);
    return 0;
    }
    static unsigned long pll_gf40lp_frac_recalc_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    struct pistachio_clk_pll *pll = to_pistachio_pll(hw);
    u64 val, prediv, fbdiv, frac, postdiv1, postdiv2, rate;
    val = pll_readl(pll, PLL_CTRL1);
    prediv = (val >> PLL_CTRL1_REFDIV_SHIFT) & PLL_CTRL1_REFDIV_MASK;
    fbdiv = (val >> PLL_CTRL1_FBDIV_SHIFT) & PLL_CTRL1_FBDIV_MASK;
    val = pll_readl(pll, PLL_CTRL2);
    postdiv1 = (val >> PLL_FRAC_CTRL2_POSTDIV1_SHIFT) &
    PLL_FRAC_CTRL2_POSTDIV1_MASK;
    postdiv2 = (val >> PLL_FRAC_CTRL2_POSTDIV2_SHIFT) &
    PLL_FRAC_CTRL2_POSTDIV2_MASK;
    frac = (val >> PLL_FRAC_CTRL2_FRAC_SHIFT) & PLL_FRAC_CTRL2_FRAC_MASK;
// get operating mode (int/frac) and calculate rate accordingly
    rate = parent_rate;
    if (pll_frac_get_mode(hw) == PLL_MODE_FRAC)
    rate *= (fbdiv << 24) + frac;
    else
    rate *= (fbdiv << 24);
    rate = do_div_round_closest(rate, (prediv * postdiv1 * postdiv2) << 24);
    return rate;
    }
    static const struct clk_ops pll_gf40lp_frac_ops = {
    .enable = pll_gf40lp_frac_enable,
    .disable = pll_gf40lp_frac_disable,
    .is_enabled = pll_gf40lp_frac_is_enabled,
    .recalc_rate = pll_gf40lp_frac_recalc_rate,
    .determine_rate = pll_determine_rate,
    .set_rate = pll_gf40lp_frac_set_rate,
    };
    static const struct clk_ops pll_gf40lp_frac_fixed_ops = {
    .enable = pll_gf40lp_frac_enable,
    .disable = pll_gf40lp_frac_disable,
    .is_enabled = pll_gf40lp_frac_is_enabled,
    .recalc_rate = pll_gf40lp_frac_recalc_rate,
    };
#[no_mangle]
unsafe extern "C" fn pll_gf40lp_laint_enable(hw: *mut clk_hw) -> c_int {
    static int pll_gf40lp_laint_enable(struct clk_hw *hw)
    {
    struct pistachio_clk_pll *pll = to_pistachio_pll(hw);
    u32 val;
    val = pll_readl(pll, PLL_CTRL1);
    val &= ~(PLL_INT_CTRL1_PD |
    PLL_INT_CTRL1_FOUTPOSTDIVPD | PLL_INT_CTRL1_FOUTVCOPD);
    pll_writel(pll, val, PLL_CTRL1);
    val = pll_readl(pll, PLL_CTRL2);
    val &= ~PLL_INT_CTRL2_BYPASS;
    pll_writel(pll, val, PLL_CTRL2);
    pll_lock(pll);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pll_gf40lp_laint_disable(hw: *mut clk_hw) {
    static void pll_gf40lp_laint_disable(struct clk_hw *hw)
    {
    struct pistachio_clk_pll *pll = to_pistachio_pll(hw);
    u32 val;
    val = pll_readl(pll, PLL_CTRL1);
    val |= PLL_INT_CTRL1_PD;
    pll_writel(pll, val, PLL_CTRL1);
    }
#[no_mangle]
unsafe extern "C" fn pll_gf40lp_laint_is_enabled(hw: *mut clk_hw) -> c_int {
    static int pll_gf40lp_laint_is_enabled(struct clk_hw *hw)
    {
    struct pistachio_clk_pll *pll = to_pistachio_pll(hw);
    return !(pll_readl(pll, PLL_CTRL1) & PLL_INT_CTRL1_PD);
    }
    static int pll_gf40lp_laint_set_rate(struct clk_hw *hw, unsigned long rate,
    unsigned long parent_rate)
    {
    struct pistachio_clk_pll *pll = to_pistachio_pll(hw);
    struct pistachio_pll_rate_table *params;
    let mut enabled: c_int = pll_gf40lp_laint_is_enabled(hw);
    u32 val, vco, old_postdiv1, old_postdiv2;
    const char *name = clk_hw_get_name(hw);
    if (rate < MIN_OUTPUT_LA || rate > MAX_OUTPUT_LA)
    return -EINVAL;
    params = pll_get_params(pll, parent_rate, rate);
    if (!params || !params.refdiv)
    return -EINVAL;
    vco = div_u64(params.fref * params.fbdiv, params.refdiv);
    if (vco < MIN_VCO_LA || vco > MAX_VCO_LA)
    pr_warn("%s: VCO %u is out of range %lu..%lu\n", name, vco,
    MIN_VCO_LA, MAX_VCO_LA);
    val = div_u64(params.fref, params.refdiv);
    if (val < MIN_PFD)
    pr_warn("%s: PFD %u is too low (min %lu)\n",
    name, val, MIN_PFD);
    if (val > vco / 16)
    pr_warn("%s: PFD %u is too high (max %u)\n",
    name, val, vco / 16);
    val = pll_readl(pll, PLL_CTRL1);
    old_postdiv1 = (val >> PLL_INT_CTRL1_POSTDIV1_SHIFT) &
    PLL_INT_CTRL1_POSTDIV1_MASK;
    old_postdiv2 = (val >> PLL_INT_CTRL1_POSTDIV2_SHIFT) &
    PLL_INT_CTRL1_POSTDIV2_MASK;
    if (enabled &&
    (params.postdiv1 != old_postdiv1 ||
    params.postdiv2 != old_postdiv2))
    pr_warn("%s: changing postdiv while PLL is enabled\n", name);
    if (params.postdiv2 > params.postdiv1)
    pr_warn("%s: postdiv2 should not exceed postdiv1\n", name);
    val &= ~((PLL_CTRL1_REFDIV_MASK << PLL_CTRL1_REFDIV_SHIFT) |
    (PLL_CTRL1_FBDIV_MASK << PLL_CTRL1_FBDIV_SHIFT) |
    (PLL_INT_CTRL1_POSTDIV1_MASK << PLL_INT_CTRL1_POSTDIV1_SHIFT) |
    (PLL_INT_CTRL1_POSTDIV2_MASK << PLL_INT_CTRL1_POSTDIV2_SHIFT));
    val |= (params.refdiv << PLL_CTRL1_REFDIV_SHIFT) |
    (params.fbdiv << PLL_CTRL1_FBDIV_SHIFT) |
    (params.postdiv1 << PLL_INT_CTRL1_POSTDIV1_SHIFT) |
    (params.postdiv2 << PLL_INT_CTRL1_POSTDIV2_SHIFT);
    pll_writel(pll, val, PLL_CTRL1);
    if (enabled)
    pll_lock(pll);
    return 0;
    }
    static unsigned long pll_gf40lp_laint_recalc_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    struct pistachio_clk_pll *pll = to_pistachio_pll(hw);
    u32 val, prediv, fbdiv, postdiv1, postdiv2;
    let mut rate: u64 = parent_rate;
    val = pll_readl(pll, PLL_CTRL1);
    prediv = (val >> PLL_CTRL1_REFDIV_SHIFT) & PLL_CTRL1_REFDIV_MASK;
    fbdiv = (val >> PLL_CTRL1_FBDIV_SHIFT) & PLL_CTRL1_FBDIV_MASK;
    postdiv1 = (val >> PLL_INT_CTRL1_POSTDIV1_SHIFT) &
    PLL_INT_CTRL1_POSTDIV1_MASK;
    postdiv2 = (val >> PLL_INT_CTRL1_POSTDIV2_SHIFT) &
    PLL_INT_CTRL1_POSTDIV2_MASK;
    rate *= fbdiv;
    rate = do_div_round_closest(rate, prediv * postdiv1 * postdiv2);
    return rate;
    }
    static const struct clk_ops pll_gf40lp_laint_ops = {
    .enable = pll_gf40lp_laint_enable,
    .disable = pll_gf40lp_laint_disable,
    .is_enabled = pll_gf40lp_laint_is_enabled,
    .recalc_rate = pll_gf40lp_laint_recalc_rate,
    .determine_rate = pll_determine_rate,
    .set_rate = pll_gf40lp_laint_set_rate,
    };
    static const struct clk_ops pll_gf40lp_laint_fixed_ops = {
    .enable = pll_gf40lp_laint_enable,
    .disable = pll_gf40lp_laint_disable,
    .is_enabled = pll_gf40lp_laint_is_enabled,
    .recalc_rate = pll_gf40lp_laint_recalc_rate,
    };
    static struct clk *pll_register(const char *name, const char *parent_name,
    unsigned long flags, void __iomem *base,
    enum pistachio_pll_type type,
    struct pistachio_pll_rate_table *rates,
    unsigned int nr_rates)
    {
    struct pistachio_clk_pll *pll;
    struct clk_init_data init;
    struct clk *clk;
    pll = kzalloc_obj(*pll);
    if (!pll)
    return ERR_PTR(-ENOMEM);
    init.name = name;
    init.flags = flags | CLK_GET_RATE_NOCACHE;
    init.parent_names = &parent_name;
    init.num_parents = 1;
    switch (type) {
    case PLL_GF40LP_FRAC:
    if (rates)
    init.ops = &pll_gf40lp_frac_ops;
    else
    init.ops = &pll_gf40lp_frac_fixed_ops;
    break;
    case PLL_GF40LP_LAINT:
    if (rates)
    init.ops = &pll_gf40lp_laint_ops;
    else
    init.ops = &pll_gf40lp_laint_fixed_ops;
    break;
    default:
    pr_err("Unrecognized PLL type %u\n", type);
    kfree(pll);
    return ERR_PTR(-EINVAL);
    }
    pll.hw.init = &init;
    pll.base = base;
    pll.rates = rates;
    pll.nr_rates = nr_rates;
    clk = clk_register(core::ptr::null_mut(), &pll.hw);
    if (IS_ERR(clk))
    kfree(pll);
    return clk;
    }
    void pistachio_clk_register_pll(struct pistachio_clk_provider *p,
    struct pistachio_pll *pll,
    unsigned int num)
    {
    struct clk *clk;
    unsigned int i;
    for (i = 0; i < num; i++) {
    clk = pll_register(pll[i].name, pll[i].parent,
    0, p.base + pll[i].reg_base,
    pll[i].type, pll[i].rates,
    pll[i].nr_rates);
    p.clk_data.clks[pll[i].id] = clk;
    }
    }
