//! Automatically rewritten from C to Rust
//! Source: drivers/clk/at91/clk-audio-pll.c
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
// Copyright (C) 2016 Atmel Corporation,
// Songjun Wu <songjun.wu@atmel.com>,
// Nicolas Ferre <nicolas.ferre@atmel.com>
// Copyright (C) 2017 Free Electrons,
// Quentin Schulz <quentin.schulz@free-electrons.com>
//
// The Sama5d2 SoC has two audio PLLs (PMC and PAD) that shares the same parent
// (FRAC). FRAC can output between 620 and 700MHz and only multiply the rate of
// its own parent. PMC and PAD can then divide the FRAC rate to best match the
// asked rate.
//
// Traits of FRAC clock:
// enable - clk_enable writes nd, fracr parameters and enables PLL
// rate - rate is adjustable.
// clk->rate = parent->rate * ((nd + 1) + (fracr / 2^22))
// parent - fixed parent.  No clk_set_parent support
//
// Traits of PMC clock:
// enable - clk_enable writes qdpmc, and enables PMC output
// rate - rate is adjustable.
// clk->rate = parent->rate / (qdpmc + 1)
// parent - fixed parent.  No clk_set_parent support
//
// Traits of PAD clock:
// enable - clk_enable writes divisors and enables PAD output
// rate - rate is adjustable.
// clk->rate = parent->rate / (qdaudio * div))
// parent - fixed parent.  No clk_set_parent support
//

    AT91_PMC_AUDIO_PLL_ND_OFFSET)

    AT91_PMC_AUDIO_PLL_QDPAD_EXTDIV_MASK) | \
    (AT91_PMC_AUDIO_PLL_QDPAD_DIV(div) & \
    AT91_PMC_AUDIO_PLL_QDPAD_DIV_MASK))

    AT91_PMC_AUDIO_PLL_QDPMC_OFFSET)

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_audio_frac {
    pub hw: clk_hw,
    pub regmap: *mut regmap,
    pub fracr: u32,
    pub nd: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_audio_pad {
    pub hw: clk_hw,
    pub regmap: *mut regmap,
    pub qdaudio: u8,
    pub div: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_audio_pmc {
    pub hw: clk_hw,
    pub regmap: *mut regmap,
    pub qdpmc: u8,
}

#[no_mangle]
unsafe extern "C" fn clk_audio_pll_frac_enable(hw: *mut clk_hw) -> c_int {
    static int clk_audio_pll_frac_enable(struct clk_hw *hw)
    {
    struct clk_audio_frac *frac = to_clk_audio_frac(hw);
    regmap_update_bits(frac.regmap, AT91_PMC_AUDIO_PLL0,
    AT91_PMC_AUDIO_PLL_RESETN, 0);
    regmap_update_bits(frac.regmap, AT91_PMC_AUDIO_PLL0,
    AT91_PMC_AUDIO_PLL_RESETN,
    AT91_PMC_AUDIO_PLL_RESETN);
    regmap_update_bits(frac.regmap, AT91_PMC_AUDIO_PLL1,
    AT91_PMC_AUDIO_PLL_FRACR_MASK, frac.fracr);
//
// reset and enable have to be done in 2 separated writes
// for AT91_PMC_AUDIO_PLL0
//
    regmap_update_bits(frac.regmap, AT91_PMC_AUDIO_PLL0,
    AT91_PMC_AUDIO_PLL_PLLEN |
    AT91_PMC_AUDIO_PLL_ND_MASK,
    AT91_PMC_AUDIO_PLL_PLLEN |
    AT91_PMC_AUDIO_PLL_ND(frac.nd));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn clk_audio_pll_pad_enable(hw: *mut clk_hw) -> c_int {
    static int clk_audio_pll_pad_enable(struct clk_hw *hw)
    {
    struct clk_audio_pad *apad_ck = to_clk_audio_pad(hw);
    regmap_update_bits(apad_ck.regmap, AT91_PMC_AUDIO_PLL1,
    AT91_PMC_AUDIO_PLL_QDPAD_MASK,
    AUDIO_PLL_QDPAD(apad_ck.qdaudio, apad_ck.div));
    regmap_update_bits(apad_ck.regmap, AT91_PMC_AUDIO_PLL0,
    AT91_PMC_AUDIO_PLL_PADEN, AT91_PMC_AUDIO_PLL_PADEN);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn clk_audio_pll_pmc_enable(hw: *mut clk_hw) -> c_int {
    static int clk_audio_pll_pmc_enable(struct clk_hw *hw)
    {
    struct clk_audio_pmc *apmc_ck = to_clk_audio_pmc(hw);
    regmap_update_bits(apmc_ck.regmap, AT91_PMC_AUDIO_PLL0,
    AT91_PMC_AUDIO_PLL_PMCEN |
    AT91_PMC_AUDIO_PLL_QDPMC_MASK,
    AT91_PMC_AUDIO_PLL_PMCEN |
    AT91_PMC_AUDIO_PLL_QDPMC(apmc_ck.qdpmc));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn clk_audio_pll_frac_disable(hw: *mut clk_hw) {
    static void clk_audio_pll_frac_disable(struct clk_hw *hw)
    {
    struct clk_audio_frac *frac = to_clk_audio_frac(hw);
    regmap_update_bits(frac.regmap, AT91_PMC_AUDIO_PLL0,
    AT91_PMC_AUDIO_PLL_PLLEN, 0);
// do it in 2 separated writes
    regmap_update_bits(frac.regmap, AT91_PMC_AUDIO_PLL0,
    AT91_PMC_AUDIO_PLL_RESETN, 0);
    }
#[no_mangle]
unsafe extern "C" fn clk_audio_pll_pad_disable(hw: *mut clk_hw) {
    static void clk_audio_pll_pad_disable(struct clk_hw *hw)
    {
    struct clk_audio_pad *apad_ck = to_clk_audio_pad(hw);
    regmap_update_bits(apad_ck.regmap, AT91_PMC_AUDIO_PLL0,
    AT91_PMC_AUDIO_PLL_PADEN, 0);
    }
#[no_mangle]
unsafe extern "C" fn clk_audio_pll_pmc_disable(hw: *mut clk_hw) {
    static void clk_audio_pll_pmc_disable(struct clk_hw *hw)
    {
    struct clk_audio_pmc *apmc_ck = to_clk_audio_pmc(hw);
    regmap_update_bits(apmc_ck.regmap, AT91_PMC_AUDIO_PLL0,
    AT91_PMC_AUDIO_PLL_PMCEN, 0);
    }
    static unsigned long clk_audio_pll_fout(unsigned long parent_rate,
    unsigned long nd, unsigned long fracr)
    {
    let mut fr: c_ulonglong = (unsigned long long)parent_rate * fracr;
    pr_debug("A PLL: %s, fr = %llu\n", __func__, fr);
    fr = DIV_ROUND_CLOSEST_ULL(fr, AUDIO_PLL_DIV_FRAC);
    pr_debug("A PLL: %s, fr = %llu\n", __func__, fr);
    return parent_rate * (nd + 1) + fr;
    }
    static unsigned long clk_audio_pll_frac_recalc_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    struct clk_audio_frac *frac = to_clk_audio_frac(hw);
    unsigned long fout;
    fout = clk_audio_pll_fout(parent_rate, frac.nd, frac.fracr);
    pr_debug("A PLL: %s, fout = %lu (nd = %u, fracr = %lu)\n", __func__,
    fout, frac.nd, (unsigned long)frac.fracr);
    return fout;
    }
    static unsigned long clk_audio_pll_pad_recalc_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    struct clk_audio_pad *apad_ck = to_clk_audio_pad(hw);
    let mut apad_rate: c_ulong = 0;
    if (apad_ck.qdaudio && apad_ck.div)
    apad_rate = parent_rate / (apad_ck.qdaudio * apad_ck.div);
    pr_debug("A PLL/PAD: %s, apad_rate = %lu (div = %u, qdaudio = %u)\n",
    __func__, apad_rate, apad_ck.div, apad_ck.qdaudio);
    return apad_rate;
    }
    static unsigned long clk_audio_pll_pmc_recalc_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    struct clk_audio_pmc *apmc_ck = to_clk_audio_pmc(hw);
    let mut apmc_rate: c_ulong = 0;
    apmc_rate = parent_rate / (apmc_ck.qdpmc + 1);
    pr_debug("A PLL/PMC: %s, apmc_rate = %lu (qdpmc = %u)\n", __func__,
    apmc_rate, apmc_ck.qdpmc);
    return apmc_rate;
    }
    static int clk_audio_pll_frac_compute_frac(unsigned long rate,
    unsigned long parent_rate,
    unsigned long *nd,
    unsigned long *fracr)
    {
    unsigned long long tmp, rem;
    if (!rate)
    return -EINVAL;
    tmp = rate;
    rem = do_div(tmp, parent_rate);
    if (!tmp || tmp >= AUDIO_PLL_ND_MAX)
    return -EINVAL;
// nd = tmp - 1;
    tmp = rem * AUDIO_PLL_DIV_FRAC;
    tmp = DIV_ROUND_CLOSEST_ULL(tmp, parent_rate);
    if (tmp > AT91_PMC_AUDIO_PLL_FRACR_MASK)
    return -EINVAL;
// we can cast here as we verified the bounds just above
// fracr = (unsigned long)tmp;
    return 0;
    }
    static int clk_audio_pll_frac_determine_rate(struct clk_hw *hw,
    struct clk_rate_request *req)
    {
    unsigned long fracr, nd;
    int ret;
    pr_debug("A PLL: %s, rate = %lu (parent_rate = %lu)\n", __func__,
    req.rate, req.best_parent_rate);
    req.rate = clamp(req.rate, AUDIO_PLL_FOUT_MIN, AUDIO_PLL_FOUT_MAX);
    req.min_rate = max(req.min_rate, AUDIO_PLL_FOUT_MIN);
    req.max_rate = min(req.max_rate, AUDIO_PLL_FOUT_MAX);
    ret = clk_audio_pll_frac_compute_frac(req.rate, req.best_parent_rate,
    &nd, &fracr);
    if (ret)
    return ret;
    req.rate = clk_audio_pll_fout(req.best_parent_rate, nd, fracr);
    req.best_parent_hw = clk_hw_get_parent(hw);
    pr_debug("A PLL: %s, best_rate = %lu (nd = %lu, fracr = %lu)\n",
    __func__, req.rate, nd, fracr);
    return 0;
    }
    static int clk_audio_pll_pad_determine_rate(struct clk_hw *hw,
    struct clk_rate_request *req)
    {
    struct clk_hw *pclk = clk_hw_get_parent(hw);
    let mut best_rate: c_long = -EINVAL;
    unsigned long best_parent_rate;
    unsigned long tmp_qd;
    u32 div;
    long tmp_rate;
    int tmp_diff;
    let mut best_diff: c_int = -1;
    pr_debug("A PLL/PAD: %s, rate = %lu (parent_rate = %lu)\n", __func__,
    req.rate, req.best_parent_rate);
//
// Rate divisor is actually made of two different divisors, multiplied
// between themselves before dividing the rate.
// tmp_qd goes from 1 to 31 and div is either 2 or 3.
// In order to avoid testing twice the rate divisor (e.g. divisor 12 can
// be found with (tmp_qd, div) = (2, 6) or (3, 4)), we remove any loop
// for a rate divisor when div is 2 and tmp_qd is a multiple of 3.
// We cannot inverse it (condition div is 3 and tmp_qd is even) or we
// would miss some rate divisor that aren't reachable with div being 2
// (e.g. rate divisor 90 is made with div = 3 and tmp_qd = 30, thus
// tmp_qd is even so we skip it because we think div 2 could make this
// rate divisor which isn't possible since tmp_qd has to be <= 31).
//
    for (tmp_qd = 1; tmp_qd < AT91_PMC_AUDIO_PLL_QDPAD_EXTDIV_MAX; tmp_qd++)
    for (div = 2; div <= 3; div++) {
    if (div == 2 && tmp_qd % 3 == 0)
    continue;
    best_parent_rate = clk_hw_round_rate(pclk,
    req.rate * tmp_qd * div);
    tmp_rate = best_parent_rate / (div * tmp_qd);
    tmp_diff = abs(req.rate - tmp_rate);
    if (best_diff < 0 || best_diff > tmp_diff) {
    req.best_parent_rate = best_parent_rate;
    best_rate = tmp_rate;
    best_diff = tmp_diff;
    }
    }
    pr_debug("A PLL/PAD: %s, best_rate = %ld, best_parent_rate = %lu\n",
    __func__, best_rate, best_parent_rate);
    req.rate = best_rate;
    return 0;
    }
    static int clk_audio_pll_pmc_determine_rate(struct clk_hw *hw,
    struct clk_rate_request *req)
    {
    struct clk_hw *pclk = clk_hw_get_parent(hw);
    let mut best_rate: c_long = -EINVAL;
    let mut best_parent_rate: c_ulong = 0;
    let mut tmp_qd: u32 = 0, div;
    long tmp_rate;
    int tmp_diff;
    let mut best_diff: c_int = -1;
    pr_debug("A PLL/PMC: %s, rate = %lu (parent_rate = %lu)\n", __func__,
    req.rate, req.best_parent_rate);
    if (!req.rate)
    return 0;
    best_parent_rate = clk_round_rate(pclk.clk, 1);
    div = max(best_parent_rate / req.rate, 1UL);
    for (; div <= AUDIO_PLL_QDPMC_MAX; div++) {
    best_parent_rate = clk_round_rate(pclk.clk, req.rate * div);
    tmp_rate = best_parent_rate / div;
    tmp_diff = abs(req.rate - tmp_rate);
    if (best_diff < 0 || best_diff > tmp_diff) {
    req.best_parent_rate = best_parent_rate;
    best_rate = tmp_rate;
    best_diff = tmp_diff;
    tmp_qd = div;
    if (!best_diff)
    break;	/* got exact match */
    }
    }
    pr_debug("A PLL/PMC: %s, best_rate = %ld, best_parent_rate = %lu (qd = %d)\n",
    __func__, best_rate, req.best_parent_rate, tmp_qd - 1);
    req.rate = best_rate;
    return 0;
    }
    static int clk_audio_pll_frac_set_rate(struct clk_hw *hw, unsigned long rate,
    unsigned long parent_rate)
    {
    struct clk_audio_frac *frac = to_clk_audio_frac(hw);
    unsigned long fracr, nd;
    int ret;
    pr_debug("A PLL: %s, rate = %lu (parent_rate = %lu)\n", __func__, rate,
    parent_rate);
    if (rate < AUDIO_PLL_FOUT_MIN || rate > AUDIO_PLL_FOUT_MAX)
    return -EINVAL;
    ret = clk_audio_pll_frac_compute_frac(rate, parent_rate, &nd, &fracr);
    if (ret)
    return ret;
    frac.nd = nd;
    frac.fracr = fracr;
    return 0;
    }
    static int clk_audio_pll_pad_set_rate(struct clk_hw *hw, unsigned long rate,
    unsigned long parent_rate)
    {
    struct clk_audio_pad *apad_ck = to_clk_audio_pad(hw);
    u8 tmp_div;
    pr_debug("A PLL/PAD: %s, rate = %lu (parent_rate = %lu)\n", __func__,
    rate, parent_rate);
    if (!rate)
    return -EINVAL;
    tmp_div = parent_rate / rate;
    if (tmp_div % 3 == 0) {
    apad_ck.qdaudio = tmp_div / 3;
    apad_ck.div = 3;
    } else {
    apad_ck.qdaudio = tmp_div / 2;
    apad_ck.div = 2;
    }
    return 0;
    }
    static int clk_audio_pll_pmc_set_rate(struct clk_hw *hw, unsigned long rate,
    unsigned long parent_rate)
    {
    struct clk_audio_pmc *apmc_ck = to_clk_audio_pmc(hw);
    if (!rate)
    return -EINVAL;
    pr_debug("A PLL/PMC: %s, rate = %lu (parent_rate = %lu)\n", __func__,
    rate, parent_rate);
    apmc_ck.qdpmc = parent_rate / rate - 1;
    return 0;
    }
    static const struct clk_ops audio_pll_frac_ops = {
    .enable = clk_audio_pll_frac_enable,
    .disable = clk_audio_pll_frac_disable,
    .recalc_rate = clk_audio_pll_frac_recalc_rate,
    .determine_rate = clk_audio_pll_frac_determine_rate,
    .set_rate = clk_audio_pll_frac_set_rate,
    };
    static const struct clk_ops audio_pll_pad_ops = {
    .enable = clk_audio_pll_pad_enable,
    .disable = clk_audio_pll_pad_disable,
    .recalc_rate = clk_audio_pll_pad_recalc_rate,
    .determine_rate = clk_audio_pll_pad_determine_rate,
    .set_rate = clk_audio_pll_pad_set_rate,
    };
    static const struct clk_ops audio_pll_pmc_ops = {
    .enable = clk_audio_pll_pmc_enable,
    .disable = clk_audio_pll_pmc_disable,
    .recalc_rate = clk_audio_pll_pmc_recalc_rate,
    .determine_rate = clk_audio_pll_pmc_determine_rate,
    .set_rate = clk_audio_pll_pmc_set_rate,
    };
    struct clk_hw * __init
    at91_clk_register_audio_pll_frac(struct regmap *regmap, const char *name,
    const char *parent_name)
    {
    struct clk_audio_frac *frac_ck;
    let mut init: clk_init_data = {};
    int ret;
    frac_ck = kzalloc_obj(*frac_ck);
    if (!frac_ck)
    return ERR_PTR(-ENOMEM);
    init.name = name;
    init.ops = &audio_pll_frac_ops;
    init.parent_names = &parent_name;
    init.num_parents = 1;
    init.flags = CLK_SET_RATE_GATE;
    frac_ck.hw.init = &init;
    frac_ck.regmap = regmap;
    ret = clk_hw_register(core::ptr::null_mut(), &frac_ck.hw);
    if (ret) {
    kfree(frac_ck);
    return ERR_PTR(ret);
    }
    return &frac_ck.hw;
    }
    struct clk_hw * __init
    at91_clk_register_audio_pll_pad(struct regmap *regmap, const char *name,
    const char *parent_name)
    {
    struct clk_audio_pad *apad_ck;
    struct clk_init_data init;
    int ret;
    apad_ck = kzalloc_obj(*apad_ck);
    if (!apad_ck)
    return ERR_PTR(-ENOMEM);
    init.name = name;
    init.ops = &audio_pll_pad_ops;
    init.parent_names = &parent_name;
    init.num_parents = 1;
    init.flags = CLK_SET_RATE_GATE | CLK_SET_PARENT_GATE |
    CLK_SET_RATE_PARENT;
    apad_ck.hw.init = &init;
    apad_ck.regmap = regmap;
    ret = clk_hw_register(core::ptr::null_mut(), &apad_ck.hw);
    if (ret) {
    kfree(apad_ck);
    return ERR_PTR(ret);
    }
    return &apad_ck.hw;
    }
    struct clk_hw * __init
    at91_clk_register_audio_pll_pmc(struct regmap *regmap, const char *name,
    const char *parent_name)
    {
    struct clk_audio_pmc *apmc_ck;
    struct clk_init_data init;
    int ret;
    apmc_ck = kzalloc_obj(*apmc_ck);
    if (!apmc_ck)
    return ERR_PTR(-ENOMEM);
    init.name = name;
    init.ops = &audio_pll_pmc_ops;
    init.parent_names = &parent_name;
    init.num_parents = 1;
    init.flags = CLK_SET_RATE_GATE | CLK_SET_PARENT_GATE |
    CLK_SET_RATE_PARENT;
    apmc_ck.hw.init = &init;
    apmc_ck.regmap = regmap;
    ret = clk_hw_register(core::ptr::null_mut(), &apmc_ck.hw);
    if (ret) {
    kfree(apmc_ck);
    return ERR_PTR(ret);
    }
    return &apmc_ck.hw;
    }
