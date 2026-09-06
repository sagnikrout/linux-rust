//! Automatically rewritten from C to Rust
//! Source: drivers/clk/imx/clk-pllv4.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// Copyright (C) 2016 Freescale Semiconductor, Inc.
// Copyright 2017~2018 NXP
//
// Author: Dong Aisheng <aisheng.dong@nxp.com>
//

// PLL Control Status Register (xPLLCSR)
pub const PLL_CSR_OFFSET: c_uint = 0x0;

// PLL Configuration Register (xPLLCFG)
pub const PLL_CFG_OFFSET: c_uint = 0x08;
pub const IMX8ULP_PLL_CFG_OFFSET: c_uint = 0x10;
pub const BP_PLL_MULT: c_int = 16;

// PLL Numerator Register (xPLLNUM)
pub const PLL_NUM_OFFSET: c_uint = 0x10;
pub const IMX8ULP_PLL_NUM_OFFSET: c_uint = 0x1c;
// PLL Denominator Register (xPLLDENOM)
pub const PLL_DENOM_OFFSET: c_uint = 0x14;
pub const IMX8ULP_PLL_DENOM_OFFSET: c_uint = 0x18;
pub const MAX_MFD: c_uint = 0x3fffffff;
pub const DEFAULT_MFD: c_int = 1000000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_pllv4 {
    pub hw: clk_hw,
    pub base: *mut void __iomem,
    pub cfg_offset: u32,
    pub num_offset: u32,
    pub denom_offset: u32,
    pub use_mult_range: bool,
}

// Valid PLL MULT Table
    static const int pllv4_mult_table[] = {33, 27, 22, 20, 17, 16};
// Valid PLL MULT range, (max, min)
    static const int pllv4_mult_range[] = {54, 27};

#[no_mangle]
pub unsafe extern "C" fn clk_pllv4_wait_lock(pll: *mut clk_pllv4) -> c_int {
    static inline int clk_pllv4_wait_lock(struct clk_pllv4 *pll)
    {
    u32 csr;
    return readl_poll_timeout(pll.base  + PLL_CSR_OFFSET,
    csr, csr & PLL_VLD, 0, LOCK_TIMEOUT_US);
    }
#[no_mangle]
unsafe extern "C" fn clk_pllv4_is_prepared(hw: *mut clk_hw) -> c_int {
    static int clk_pllv4_is_prepared(struct clk_hw *hw)
    {
    struct clk_pllv4 *pll = to_clk_pllv4(hw);
    if (readl_relaxed(pll.base) & PLL_EN)
    return 1;
    return 0;
    }
    static unsigned long clk_pllv4_recalc_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    struct clk_pllv4 *pll = to_clk_pllv4(hw);
    u32 mult, mfn, mfd;
    u64 temp64;
    mult = readl_relaxed(pll.base + pll.cfg_offset);
    mult &= BM_PLL_MULT;
    mult >>= BP_PLL_MULT;
    mfn = readl_relaxed(pll.base + pll.num_offset);
    mfd = readl_relaxed(pll.base + pll.denom_offset);
    temp64 = parent_rate;
    temp64 *= mfn;
    do_div(temp64, mfd);
    return (parent_rate * mult) + (u32)temp64;
    }
    static int clk_pllv4_determine_rate(struct clk_hw *hw,
    struct clk_rate_request *req)
    {
    struct clk_pllv4 *pll = to_clk_pllv4(hw);
    let mut parent_rate: c_ulong = req.best_parent_rate;
    unsigned long round_rate, i;
    u32 mfn, mfd = DEFAULT_MFD;
    let mut found: bool = false;
    u64 temp64;
    u32 mult;
    if (pll.use_mult_range) {
    temp64 = (u64) req.rate;
    do_div(temp64, parent_rate);
    mult = temp64;
    if (mult >= pllv4_mult_range[1] &&
    mult <= pllv4_mult_range[0]) {
    round_rate = parent_rate * mult;
    found = true;
    }
    } else {
    for (i = 0; i < ARRAY_SIZE(pllv4_mult_table); i++) {
    round_rate = parent_rate * pllv4_mult_table[i];
    if (req.rate >= round_rate) {
    found = true;
    break;
    }
    }
    }
    if (!found) {
    pr_warn("%s: unable to round rate %lu, parent rate %lu\n",
    clk_hw_get_name(hw), req.rate, parent_rate);
    req.rate = 0;
    return 0;
    }
    if (parent_rate <= MAX_MFD)
    mfd = parent_rate;
    temp64 = (u64)(req.rate - round_rate);
    temp64 *= mfd;
    do_div(temp64, parent_rate);
    mfn = temp64;
//
// NOTE: The value of numerator must always be configured to be
// less than the value of the denominator. If we can't get a proper
// pair of mfn/mfd, we simply return the round_rate without using
// the frac part.
//
    if (mfn >= mfd) {
    req.rate = round_rate;
    return 0;
    }
    temp64 = (u64)parent_rate;
    temp64 *= mfn;
    do_div(temp64, mfd);
    req.rate = round_rate + (u32)temp64;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn clk_pllv4_is_valid_mult(pll: *mut clk_pllv4, mult: c_uint) -> bool {
    static bool clk_pllv4_is_valid_mult(struct clk_pllv4 *pll, unsigned int mult)
    {
    int i;
// check if mult is in valid MULT table
    if (pll.use_mult_range) {
    if (mult >= pllv4_mult_range[1] &&
    mult <= pllv4_mult_range[0])
    return true;
    } else {
    for (i = 0; i < ARRAY_SIZE(pllv4_mult_table); i++) {
    if (pllv4_mult_table[i] == mult)
    return true;
    }
    }
    return false;
    }
    static int clk_pllv4_set_rate(struct clk_hw *hw, unsigned long rate,
    unsigned long parent_rate)
    {
    struct clk_pllv4 *pll = to_clk_pllv4(hw);
    u32 val, mult, mfn, mfd = DEFAULT_MFD;
    u64 temp64;
    mult = rate / parent_rate;
    if (!clk_pllv4_is_valid_mult(pll, mult))
    return -EINVAL;
    if (parent_rate <= MAX_MFD)
    mfd = parent_rate;
    temp64 = (u64)(rate - mult * parent_rate);
    temp64 *= mfd;
    do_div(temp64, parent_rate);
    mfn = temp64;
    val = readl_relaxed(pll.base + pll.cfg_offset);
    val &= ~BM_PLL_MULT;
    val |= mult << BP_PLL_MULT;
    writel_relaxed(val, pll.base + pll.cfg_offset);
    writel_relaxed(mfn, pll.base + pll.num_offset);
    writel_relaxed(mfd, pll.base + pll.denom_offset);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn clk_pllv4_prepare(hw: *mut clk_hw) -> c_int {
    static int clk_pllv4_prepare(struct clk_hw *hw)
    {
    u32 val;
    struct clk_pllv4 *pll = to_clk_pllv4(hw);
    val = readl_relaxed(pll.base);
    val |= PLL_EN;
    writel_relaxed(val, pll.base);
    return clk_pllv4_wait_lock(pll);
    }
#[no_mangle]
unsafe extern "C" fn clk_pllv4_unprepare(hw: *mut clk_hw) {
    static void clk_pllv4_unprepare(struct clk_hw *hw)
    {
    u32 val;
    struct clk_pllv4 *pll = to_clk_pllv4(hw);
    val = readl_relaxed(pll.base);
    val &= ~PLL_EN;
    writel_relaxed(val, pll.base);
    }
    static const struct clk_ops clk_pllv4_ops = {
    .recalc_rate	= clk_pllv4_recalc_rate,
    .determine_rate = clk_pllv4_determine_rate,
    .set_rate	= clk_pllv4_set_rate,
    .prepare	= clk_pllv4_prepare,
    .unprepare	= clk_pllv4_unprepare,
    .is_prepared	= clk_pllv4_is_prepared,
    };
    struct clk_hw *imx_clk_hw_pllv4(enum imx_pllv4_type type, const char *name,
    const char *parent_name, void __iomem *base)
    {
    struct clk_pllv4 *pll;
    struct clk_hw *hw;
    struct clk_init_data init;
    int ret;
    pll = kzalloc_obj(*pll);
    if (!pll)
    return ERR_PTR(-ENOMEM);
    pll.base = base;
    if (type == IMX_PLLV4_IMX8ULP ||
    type == IMX_PLLV4_IMX8ULP_1GHZ) {
    pll.cfg_offset = IMX8ULP_PLL_CFG_OFFSET;
    pll.num_offset = IMX8ULP_PLL_NUM_OFFSET;
    pll.denom_offset = IMX8ULP_PLL_DENOM_OFFSET;
    if (type == IMX_PLLV4_IMX8ULP_1GHZ)
    pll.use_mult_range = true;
    } else {
    pll.cfg_offset = PLL_CFG_OFFSET;
    pll.num_offset = PLL_NUM_OFFSET;
    pll.denom_offset = PLL_DENOM_OFFSET;
    }
    init.name = name;
    init.ops = &clk_pllv4_ops;
    init.parent_names = &parent_name;
    init.num_parents = 1;
    init.flags = CLK_SET_RATE_GATE;
    pll.hw.init = &init;
    hw = &pll.hw;
    ret = clk_hw_register(core::ptr::null_mut(), hw);
    if (ret) {
    kfree(pll);
    hw = ERR_PTR(ret);
    }
    return hw;
    }
    EXPORT_SYMBOL_GPL(imx_clk_hw_pllv4);
