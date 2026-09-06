//! Automatically rewritten from C to Rust
//! Source: drivers/clk/imx/clk-sscg-pll.c
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


// SPDX-License-Identifier: (GPL-2.0 OR MIT)
//
// Copyright 2018 NXP.
//
// This driver supports the SCCG plls found in the imx8m SOCs
//
// Documentation for this SCCG pll can be found at:
// https://www.nxp.com/docs/en/reference-manual/IMX8MDQLQRM.pdf#page=834
//

// PLL CFGs
pub const PLL_CFG0: c_uint = 0x0;
pub const PLL_CFG1: c_uint = 0x4;
pub const PLL_CFG2: c_uint = 0x8;

// These are the specification limits for the SSCG PLL

pub const PLL_DIVR1_MAX: c_int = 7;
pub const PLL_DIVR2_MAX: c_int = 63;
pub const PLL_DIVF1_MAX: c_int = 63;
pub const PLL_DIVF2_MAX: c_int = 63;
pub const PLL_DIVQ_MAX: c_int = 63;
pub const PLL_BYPASS_NONE: c_uint = 0x0;
pub const PLL_BYPASS1: c_uint = 0x2;
pub const PLL_BYPASS2: c_uint = 0x1;

pub const PLL_SCCG_LOCK_TIMEOUT: c_int = 70;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_sscg_pll_setup {
    pub divf1: int divr1,,
    pub divf2: int divr2,,
    pub divq: c_int,
    pub bypass: c_int,
    pub vco1: u64,
    pub vco2: u64,
    pub fout: u64,
    pub ref: u64,
    pub ref_div1: u64,
    pub ref_div2: u64,
    pub fout_request: u64,
    pub fout_error: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_sscg_pll {
    pub hw: clk_hw,
    pub ops: clk_ops,
    pub base: *mut void __iomem,
    pub setup: clk_sscg_pll_setup,
    pub parent: u8,
    pub bypass1: u8,
    pub bypass2: u8,
}

#[no_mangle]
unsafe extern "C" fn clk_sscg_pll_wait_lock(pll: *mut clk_sscg_pll) -> c_int {
    static int clk_sscg_pll_wait_lock(struct clk_sscg_pll *pll)
    {
    u32 val;
    val = readl_relaxed(pll.base + PLL_CFG0);
// don't wait for lock if all plls are bypassed
    if (!(val & SSCG_PLL_BYPASS2_MASK))
    return readl_poll_timeout(pll.base, val, val & PLL_LOCK_MASK,
    0, PLL_SCCG_LOCK_TIMEOUT);
    return 0;
    }
    static int clk_sscg_pll2_check_match(struct clk_sscg_pll_setup *setup,
    struct clk_sscg_pll_setup *temp_setup)
    {
    let mut new_diff: c_int = temp_setup.fout - temp_setup.fout_request;
    let mut diff: c_int = temp_setup.fout_error;
    if (abs(diff) > abs(new_diff)) {
    temp_setup.fout_error = new_diff;
    memcpy(setup, temp_setup, sizeof(struct clk_sscg_pll_setup));
    if (temp_setup.fout_request == temp_setup.fout)
    return 0;
    }
    return -1;
    }
    static int clk_sscg_divq_lookup(struct clk_sscg_pll_setup *setup,
    struct clk_sscg_pll_setup *temp_setup)
    {
    let mut ret: c_int = -EINVAL;
    for (temp_setup.divq = 0; temp_setup.divq <= PLL_DIVQ_MAX;
    temp_setup.divq++) {
    temp_setup.vco2 = temp_setup.vco1;
    do_div(temp_setup.vco2, temp_setup.divr2 + 1);
    temp_setup.vco2 *= 2;
    temp_setup.vco2 *= temp_setup.divf2 + 1;
    if (temp_setup.vco2 >= PLL_STAGE2_MIN_FREQ &&
    temp_setup.vco2 <= PLL_STAGE2_MAX_FREQ) {
    temp_setup.fout = temp_setup.vco2;
    do_div(temp_setup.fout, 2 * (temp_setup.divq + 1));
    ret = clk_sscg_pll2_check_match(setup, temp_setup);
    if (!ret) {
    temp_setup.bypass = PLL_BYPASS1;
    return ret;
    }
    }
    }
    return ret;
    }
    static int clk_sscg_divf2_lookup(struct clk_sscg_pll_setup *setup,
    struct clk_sscg_pll_setup *temp_setup)
    {
    let mut ret: c_int = -EINVAL;
    for (temp_setup.divf2 = 0; temp_setup.divf2 <= PLL_DIVF2_MAX;
    temp_setup.divf2++) {
    ret = clk_sscg_divq_lookup(setup, temp_setup);
    if (!ret)
    return ret;
    }
    return ret;
    }
    static int clk_sscg_divr2_lookup(struct clk_sscg_pll_setup *setup,
    struct clk_sscg_pll_setup *temp_setup)
    {
    let mut ret: c_int = -EINVAL;
    for (temp_setup.divr2 = 0; temp_setup.divr2 <= PLL_DIVR2_MAX;
    temp_setup.divr2++) {
    temp_setup.ref_div2 = temp_setup.vco1;
    do_div(temp_setup.ref_div2, temp_setup.divr2 + 1);
    if (temp_setup.ref_div2 >= PLL_STAGE2_REF_MIN_FREQ &&
    temp_setup.ref_div2 <= PLL_STAGE2_REF_MAX_FREQ) {
    ret = clk_sscg_divf2_lookup(setup, temp_setup);
    if (!ret)
    return ret;
    }
    }
    return ret;
    }
    static int clk_sscg_pll2_find_setup(struct clk_sscg_pll_setup *setup,
    struct clk_sscg_pll_setup *temp_setup,
    uint64_t ref)
    {
    int ret;
    if (ref < PLL_STAGE1_MIN_FREQ || ref > PLL_STAGE1_MAX_FREQ)
    return -EINVAL;
    temp_setup.vco1 = ref;
    ret = clk_sscg_divr2_lookup(setup, temp_setup);
    return ret;
    }
    static int clk_sscg_divf1_lookup(struct clk_sscg_pll_setup *setup,
    struct clk_sscg_pll_setup *temp_setup)
    {
    let mut ret: c_int = -EINVAL;
    for (temp_setup.divf1 = 0; temp_setup.divf1 <= PLL_DIVF1_MAX;
    temp_setup.divf1++) {
    let mut vco1: u64 = temp_setup.ref;
    do_div(vco1, temp_setup.divr1 + 1);
    vco1 *= 2;
    vco1 *= temp_setup.divf1 + 1;
    ret = clk_sscg_pll2_find_setup(setup, temp_setup, vco1);
    if (!ret) {
    temp_setup.bypass = PLL_BYPASS_NONE;
    return ret;
    }
    }
    return ret;
    }
    static int clk_sscg_divr1_lookup(struct clk_sscg_pll_setup *setup,
    struct clk_sscg_pll_setup *temp_setup)
    {
    let mut ret: c_int = -EINVAL;
    for (temp_setup.divr1 = 0; temp_setup.divr1 <= PLL_DIVR1_MAX;
    temp_setup.divr1++) {
    temp_setup.ref_div1 = temp_setup.ref;
    do_div(temp_setup.ref_div1, temp_setup.divr1 + 1);
    if (temp_setup.ref_div1 >= PLL_STAGE1_REF_MIN_FREQ &&
    temp_setup.ref_div1 <= PLL_STAGE1_REF_MAX_FREQ) {
    ret = clk_sscg_divf1_lookup(setup, temp_setup);
    if (!ret)
    return ret;
    }
    }
    return ret;
    }
    static int clk_sscg_pll1_find_setup(struct clk_sscg_pll_setup *setup,
    struct clk_sscg_pll_setup *temp_setup,
    uint64_t ref)
    {
    int ret;
    if (ref < PLL_REF_MIN_FREQ || ref > PLL_REF_MAX_FREQ)
    return -EINVAL;
    temp_setup.ref = ref;
    ret = clk_sscg_divr1_lookup(setup, temp_setup);
    return ret;
    }
    static int clk_sscg_pll_find_setup(struct clk_sscg_pll_setup *setup,
    uint64_t prate,
    uint64_t rate, int try_bypass)
    {
    struct clk_sscg_pll_setup temp_setup;
    let mut ret: c_int = -EINVAL;
    memset(&temp_setup, 0, sizeof(struct clk_sscg_pll_setup));
    memset(setup, 0, sizeof(struct clk_sscg_pll_setup));
    temp_setup.fout_error = PLL_OUT_MAX_FREQ;
    temp_setup.fout_request = rate;
    switch (try_bypass) {
    case PLL_BYPASS2:
    if (prate == rate) {
    setup.bypass = PLL_BYPASS2;
    setup.fout = rate;
    ret = 0;
    }
    break;
    case PLL_BYPASS1:
    ret = clk_sscg_pll2_find_setup(setup, &temp_setup, prate);
    break;
    case PLL_BYPASS_NONE:
    ret = clk_sscg_pll1_find_setup(setup, &temp_setup, prate);
    break;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn clk_sscg_pll_is_prepared(hw: *mut clk_hw) -> c_int {
    static int clk_sscg_pll_is_prepared(struct clk_hw *hw)
    {
    struct clk_sscg_pll *pll = to_clk_sscg_pll(hw);
    let mut val: u32 = readl_relaxed(pll.base + PLL_CFG0);
    return (val & PLL_PD_MASK) ? 0 : 1;
    }
#[no_mangle]
unsafe extern "C" fn clk_sscg_pll_prepare(hw: *mut clk_hw) -> c_int {
    static int clk_sscg_pll_prepare(struct clk_hw *hw)
    {
    struct clk_sscg_pll *pll = to_clk_sscg_pll(hw);
    u32 val;
    val = readl_relaxed(pll.base + PLL_CFG0);
    val &= ~PLL_PD_MASK;
    writel_relaxed(val, pll.base + PLL_CFG0);
    return clk_sscg_pll_wait_lock(pll);
    }
#[no_mangle]
unsafe extern "C" fn clk_sscg_pll_unprepare(hw: *mut clk_hw) {
    static void clk_sscg_pll_unprepare(struct clk_hw *hw)
    {
    struct clk_sscg_pll *pll = to_clk_sscg_pll(hw);
    u32 val;
    val = readl_relaxed(pll.base + PLL_CFG0);
    val |= PLL_PD_MASK;
    writel_relaxed(val, pll.base + PLL_CFG0);
    }
    static unsigned long clk_sscg_pll_recalc_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    struct clk_sscg_pll *pll = to_clk_sscg_pll(hw);
    u32 val, divr1, divf1, divr2, divf2, divq;
    u64 temp64;
    val = readl_relaxed(pll.base + PLL_CFG2);
    divr1 = FIELD_GET(PLL_DIVR1_MASK, val);
    divr2 = FIELD_GET(PLL_DIVR2_MASK, val);
    divf1 = FIELD_GET(PLL_DIVF1_MASK, val);
    divf2 = FIELD_GET(PLL_DIVF2_MASK, val);
    divq = FIELD_GET(PLL_DIVQ_MASK, val);
    temp64 = parent_rate;
    val = readl(pll.base + PLL_CFG0);
    if (val & SSCG_PLL_BYPASS2_MASK) {
    temp64 = parent_rate;
    } else if (val & SSCG_PLL_BYPASS1_MASK) {
    temp64 *= divf2;
    do_div(temp64, (divr2 + 1) * (divq + 1));
    } else {
    temp64 *= 2;
    temp64 *= (divf1 + 1) * (divf2 + 1);
    do_div(temp64, (divr1 + 1) * (divr2 + 1) * (divq + 1));
    }
    return temp64;
    }
    static int clk_sscg_pll_set_rate(struct clk_hw *hw, unsigned long rate,
    unsigned long parent_rate)
    {
    struct clk_sscg_pll *pll = to_clk_sscg_pll(hw);
    struct clk_sscg_pll_setup *setup = &pll.setup;
    u32 val;
// set bypass here too since the parent might be the same
    val = readl(pll.base + PLL_CFG0);
    val &= ~SSCG_PLL_BYPASS_MASK;
    val |= FIELD_PREP(SSCG_PLL_BYPASS_MASK, setup.bypass);
    writel(val, pll.base + PLL_CFG0);
    val = readl_relaxed(pll.base + PLL_CFG2);
    val &= ~(PLL_DIVF1_MASK | PLL_DIVF2_MASK);
    val &= ~(PLL_DIVR1_MASK | PLL_DIVR2_MASK | PLL_DIVQ_MASK);
    val |= FIELD_PREP(PLL_DIVF1_MASK, setup.divf1);
    val |= FIELD_PREP(PLL_DIVF2_MASK, setup.divf2);
    val |= FIELD_PREP(PLL_DIVR1_MASK, setup.divr1);
    val |= FIELD_PREP(PLL_DIVR2_MASK, setup.divr2);
    val |= FIELD_PREP(PLL_DIVQ_MASK, setup.divq);
    writel_relaxed(val, pll.base + PLL_CFG2);
    return clk_sscg_pll_wait_lock(pll);
    }
#[no_mangle]
unsafe extern "C" fn clk_sscg_pll_get_parent(hw: *mut clk_hw) -> u8 {
    static u8 clk_sscg_pll_get_parent(struct clk_hw *hw)
    {
    struct clk_sscg_pll *pll = to_clk_sscg_pll(hw);
    u32 val;
    let mut ret: u8 = pll.parent;
    val = readl(pll.base + PLL_CFG0);
    if (val & SSCG_PLL_BYPASS2_MASK)
    ret = pll.bypass2;
#[no_mangle]
pub unsafe extern "C" fn if(SSCG_PLL_BYPASS1_MASK: val &) -> else {
    else if (val & SSCG_PLL_BYPASS1_MASK)
    ret = pll.bypass1;
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn clk_sscg_pll_set_parent(hw: *mut clk_hw, index: u8) -> c_int {
    static int clk_sscg_pll_set_parent(struct clk_hw *hw, u8 index)
    {
    struct clk_sscg_pll *pll = to_clk_sscg_pll(hw);
    u32 val;
    val = readl(pll.base + PLL_CFG0);
    val &= ~SSCG_PLL_BYPASS_MASK;
    val |= FIELD_PREP(SSCG_PLL_BYPASS_MASK, pll.setup.bypass);
    writel(val, pll.base + PLL_CFG0);
    return clk_sscg_pll_wait_lock(pll);
    }
    static int __clk_sscg_pll_determine_rate(struct clk_hw *hw,
    struct clk_rate_request *req,
    uint64_t min,
    uint64_t max,
    uint64_t rate,
    int bypass)
    {
    struct clk_sscg_pll *pll = to_clk_sscg_pll(hw);
    struct clk_sscg_pll_setup *setup = &pll.setup;
    struct clk_hw *parent_hw = core::ptr::null_mut();
    int bypass_parent_index;
    int ret;
    req.max_rate = max;
    req.min_rate = min;
    switch (bypass) {
    case PLL_BYPASS2:
    bypass_parent_index = pll.bypass2;
    break;
    case PLL_BYPASS1:
    bypass_parent_index = pll.bypass1;
    break;
    default:
    bypass_parent_index = pll.parent;
    break;
    }
    parent_hw = clk_hw_get_parent_by_index(hw, bypass_parent_index);
    ret = __clk_determine_rate(parent_hw, req);
    if (!ret) {
    ret = clk_sscg_pll_find_setup(setup, req.rate,
    rate, bypass);
    }
    req.best_parent_hw = parent_hw;
    req.best_parent_rate = req.rate;
    req.rate = setup.fout;
    return ret;
    }
    static int clk_sscg_pll_determine_rate(struct clk_hw *hw,
    struct clk_rate_request *req)
    {
    struct clk_sscg_pll *pll = to_clk_sscg_pll(hw);
    struct clk_sscg_pll_setup *setup = &pll.setup;
    let mut rate: u64 = req.rate;
    let mut min: u64 = req.min_rate;
    let mut max: u64 = req.max_rate;
    int ret;
    if (rate < PLL_OUT_MIN_FREQ || rate > PLL_OUT_MAX_FREQ)
    return -EINVAL;
    ret = __clk_sscg_pll_determine_rate(hw, req, req.rate, req.rate,
    rate, PLL_BYPASS2);
    if (!ret)
    return ret;
    ret = __clk_sscg_pll_determine_rate(hw, req, PLL_STAGE1_REF_MIN_FREQ,
    PLL_STAGE1_REF_MAX_FREQ, rate,
    PLL_BYPASS1);
    if (!ret)
    return ret;
    ret = __clk_sscg_pll_determine_rate(hw, req, PLL_REF_MIN_FREQ,
    PLL_REF_MAX_FREQ, rate,
    PLL_BYPASS_NONE);
    if (!ret)
    return ret;
    if (setup.fout >= min && setup.fout <= max)
    ret = 0;
    return ret;
    }
    static const struct clk_ops clk_sscg_pll_ops = {
    .prepare	= clk_sscg_pll_prepare,
    .unprepare	= clk_sscg_pll_unprepare,
    .is_prepared	= clk_sscg_pll_is_prepared,
    .recalc_rate	= clk_sscg_pll_recalc_rate,
    .set_rate	= clk_sscg_pll_set_rate,
    .set_parent	= clk_sscg_pll_set_parent,
    .get_parent	= clk_sscg_pll_get_parent,
    .determine_rate	= clk_sscg_pll_determine_rate,
    };
    struct clk_hw *imx_clk_hw_sscg_pll(const char *name,
    const char * const *parent_names,
    u8 num_parents,
    u8 parent, u8 bypass1, u8 bypass2,
    void __iomem *base,
    unsigned long flags)
    {
    struct clk_sscg_pll *pll;
    struct clk_init_data init;
    struct clk_hw *hw;
    int ret;
    pll = kzalloc_obj(*pll);
    if (!pll)
    return ERR_PTR(-ENOMEM);
    pll.parent = parent;
    pll.bypass1 = bypass1;
    pll.bypass2 = bypass2;
    pll.base = base;
    init.name = name;
    init.ops = &clk_sscg_pll_ops;
    init.flags = flags;
    init.parent_names = parent_names;
    init.num_parents = num_parents;
    pll.hw.init = &init;
    hw = &pll.hw;
    ret = clk_hw_register(core::ptr::null_mut(), hw);
    if (ret) {
    kfree(pll);
    return ERR_PTR(ret);
    }
    return hw;
    }
    EXPORT_SYMBOL_GPL(imx_clk_hw_sscg_pll);
