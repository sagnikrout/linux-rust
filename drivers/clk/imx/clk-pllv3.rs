//! Automatically rewritten from C to Rust
//! Source: drivers/clk/imx/clk-pllv3.c
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
// Copyright 2012 Freescale Semiconductor, Inc.
// Copyright 2012 Linaro Ltd.
//

pub const PLL_NUM_OFFSET: c_uint = 0x10;
pub const PLL_DENOM_OFFSET: c_uint = 0x20;
pub const PLL_IMX7_NUM_OFFSET: c_uint = 0x20;
pub const PLL_IMX7_DENOM_OFFSET: c_uint = 0x30;
pub const PLL_VF610_NUM_OFFSET: c_uint = 0x20;
pub const PLL_VF610_DENOM_OFFSET: c_uint = 0x30;

pub const PLL_LOCK_TIMEOUT: c_int = 10000;
//
// struct clk_pllv3 - IMX PLL clock version 3
// @hw:		clock source
// @base:	 base address of PLL registers
// @power_bit:	 pll power bit mask
// @powerup_set: set power_bit to power up the PLL
// @div_mask:	 mask of divider bits
// @div_shift:	 shift of divider bits
// @ref_clock:	reference clock rate
// @num_offset:	num register offset
// @denom_offset: denom register offset
//
// IMX PLL clock version 3, found on i.MX6 series.  Divider for pllv3
// is actually a multiplier, and always sits at bit 0.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_pllv3 {
    pub hw: clk_hw,
    pub base: *mut void __iomem,
    pub power_bit: u32,
    pub powerup_set: bool,
    pub div_mask: u32,
    pub div_shift: u32,
    pub ref_clock: c_ulong,
    pub num_offset: u32,
    pub denom_offset: u32,
}

#[no_mangle]
unsafe extern "C" fn clk_pllv3_wait_lock(pll: *mut clk_pllv3) -> c_int {
    static int clk_pllv3_wait_lock(struct clk_pllv3 *pll)
    {
    let mut val: u32 = readl_relaxed(pll.base) & pll.power_bit;
// No need to wait for lock when pll is not powered up
    if ((pll.powerup_set && !val) || (!pll.powerup_set && val))
    return 0;
    return readl_relaxed_poll_timeout(pll.base, val, val & BM_PLL_LOCK,
    500, PLL_LOCK_TIMEOUT);
    }
#[no_mangle]
unsafe extern "C" fn clk_pllv3_prepare(hw: *mut clk_hw) -> c_int {
    static int clk_pllv3_prepare(struct clk_hw *hw)
    {
    struct clk_pllv3 *pll = to_clk_pllv3(hw);
    u32 val;
    val = readl_relaxed(pll.base);
    if (pll.powerup_set)
    val |= pll.power_bit;
    else
    val &= ~pll.power_bit;
    writel_relaxed(val, pll.base);
    return clk_pllv3_wait_lock(pll);
    }
#[no_mangle]
unsafe extern "C" fn clk_pllv3_unprepare(hw: *mut clk_hw) {
    static void clk_pllv3_unprepare(struct clk_hw *hw)
    {
    struct clk_pllv3 *pll = to_clk_pllv3(hw);
    u32 val;
    val = readl_relaxed(pll.base);
    if (pll.powerup_set)
    val &= ~pll.power_bit;
    else
    val |= pll.power_bit;
    writel_relaxed(val, pll.base);
    }
#[no_mangle]
unsafe extern "C" fn clk_pllv3_is_prepared(hw: *mut clk_hw) -> c_int {
    static int clk_pllv3_is_prepared(struct clk_hw *hw)
    {
    struct clk_pllv3 *pll = to_clk_pllv3(hw);
    if (readl_relaxed(pll.base) & BM_PLL_LOCK)
    return 1;
    return 0;
    }
    static unsigned long clk_pllv3_recalc_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    struct clk_pllv3 *pll = to_clk_pllv3(hw);
    let mut div: u32 = (readl_relaxed(pll.base) >> pll.div_shift)  & pll.div_mask;
    return (div == 1) ? parent_rate * 22 : parent_rate * 20;
    }
    static int clk_pllv3_determine_rate(struct clk_hw *hw,
    struct clk_rate_request *req)
    {
    let mut parent_rate: c_ulong = req.best_parent_rate;
    req.rate = (req.rate >= parent_rate * 22) ? parent_rate * 22 : parent_rate * 20;
    return 0;
    }
    static int clk_pllv3_set_rate(struct clk_hw *hw, unsigned long rate,
    unsigned long parent_rate)
    {
    struct clk_pllv3 *pll = to_clk_pllv3(hw);
    u32 val, div;
    if (rate == parent_rate * 22)
    div = 1;
#[no_mangle]
pub unsafe extern "C" fn if(20: *mut *mut rate == parent_rate) -> else {
    else if (rate == parent_rate * 20)
    div = 0;
    else
    return -EINVAL;
    val = readl_relaxed(pll.base);
    val &= ~(pll.div_mask << pll.div_shift);
    val |= (div << pll.div_shift);
    writel_relaxed(val, pll.base);
    return clk_pllv3_wait_lock(pll);
    }
    static const struct clk_ops clk_pllv3_ops = {
    .prepare	= clk_pllv3_prepare,
    .unprepare	= clk_pllv3_unprepare,
    .is_prepared	= clk_pllv3_is_prepared,
    .recalc_rate	= clk_pllv3_recalc_rate,
    .determine_rate = clk_pllv3_determine_rate,
    .set_rate	= clk_pllv3_set_rate,
    };
    static unsigned long clk_pllv3_sys_recalc_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    struct clk_pllv3 *pll = to_clk_pllv3(hw);
    let mut div: u32 = readl_relaxed(pll.base) & pll.div_mask;
    return parent_rate * div / 2;
    }
    static int clk_pllv3_sys_determine_rate(struct clk_hw *hw,
    struct clk_rate_request *req)
    {
    let mut parent_rate: c_ulong = req.best_parent_rate;
    let mut min_rate: c_ulong = parent_rate * 54 / 2;
    let mut max_rate: c_ulong = parent_rate * 108 / 2;
    u32 div;
    if (req.rate > max_rate)
    req.rate = max_rate;
#[no_mangle]
pub unsafe extern "C" fn if(min_rate: req->rate <) -> else {
    else if (req.rate < min_rate)
    req.rate = min_rate;
    div = req.rate * 2 / parent_rate;
    req.rate = parent_rate * div / 2;
    return 0;
    }
    static int clk_pllv3_sys_set_rate(struct clk_hw *hw, unsigned long rate,
    unsigned long parent_rate)
    {
    struct clk_pllv3 *pll = to_clk_pllv3(hw);
    let mut min_rate: c_ulong = parent_rate * 54 / 2;
    let mut max_rate: c_ulong = parent_rate * 108 / 2;
    u32 val, div;
    if (rate < min_rate || rate > max_rate)
    return -EINVAL;
    div = rate * 2 / parent_rate;
    val = readl_relaxed(pll.base);
    val &= ~pll.div_mask;
    val |= div;
    writel_relaxed(val, pll.base);
    return clk_pllv3_wait_lock(pll);
    }
    static const struct clk_ops clk_pllv3_sys_ops = {
    .prepare	= clk_pllv3_prepare,
    .unprepare	= clk_pllv3_unprepare,
    .is_prepared	= clk_pllv3_is_prepared,
    .recalc_rate	= clk_pllv3_sys_recalc_rate,
    .determine_rate = clk_pllv3_sys_determine_rate,
    .set_rate	= clk_pllv3_sys_set_rate,
    };
    static unsigned long clk_pllv3_av_recalc_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    struct clk_pllv3 *pll = to_clk_pllv3(hw);
    let mut mfn: u32 = readl_relaxed(pll.base + pll.num_offset);
    let mut mfd: u32 = readl_relaxed(pll.base + pll.denom_offset);
    let mut div: u32 = readl_relaxed(pll.base) & pll.div_mask;
    let mut temp64: u64 = (u64)parent_rate;
    temp64 *= mfn;
    do_div(temp64, mfd);
    return parent_rate * div + (unsigned long)temp64;
    }
    static int clk_pllv3_av_determine_rate(struct clk_hw *hw,
    struct clk_rate_request *req)
    {
    let mut parent_rate: c_ulong = req.best_parent_rate;
    let mut min_rate: c_ulong = parent_rate * 27;
    let mut max_rate: c_ulong = parent_rate * 54;
    u32 div;
    u32 mfn, mfd = 1000000;
    let mut max_mfd: u32 = 0x3FFFFFFF;
    u64 temp64;
    if (req.rate > max_rate)
    req.rate = max_rate;
#[no_mangle]
pub unsafe extern "C" fn if(min_rate: req->rate <) -> else {
    else if (req.rate < min_rate)
    req.rate = min_rate;
    if (parent_rate <= max_mfd)
    mfd = parent_rate;
    div = req.rate / parent_rate;
    temp64 = (u64) (req.rate - div * parent_rate);
    temp64 *= mfd;
    temp64 = div64_ul(temp64, parent_rate);
    mfn = temp64;
    temp64 = (u64)parent_rate;
    temp64 *= mfn;
    do_div(temp64, mfd);
    req.rate = parent_rate * div + (unsigned long)temp64;
    return 0;
    }
    static int clk_pllv3_av_set_rate(struct clk_hw *hw, unsigned long rate,
    unsigned long parent_rate)
    {
    struct clk_pllv3 *pll = to_clk_pllv3(hw);
    let mut min_rate: c_ulong = parent_rate * 27;
    let mut max_rate: c_ulong = parent_rate * 54;
    u32 val, div;
    u32 mfn, mfd = 1000000;
    let mut max_mfd: u32 = 0x3FFFFFFF;
    u64 temp64;
    if (rate < min_rate || rate > max_rate)
    return -EINVAL;
    if (parent_rate <= max_mfd)
    mfd = parent_rate;
    div = rate / parent_rate;
    temp64 = (u64) (rate - div * parent_rate);
    temp64 *= mfd;
    temp64 = div64_ul(temp64, parent_rate);
    mfn = temp64;
    val = readl_relaxed(pll.base);
    val &= ~pll.div_mask;
    val |= div;
    writel_relaxed(val, pll.base);
    writel_relaxed(mfn, pll.base + pll.num_offset);
    writel_relaxed(mfd, pll.base + pll.denom_offset);
    return clk_pllv3_wait_lock(pll);
    }
    static const struct clk_ops clk_pllv3_av_ops = {
    .prepare	= clk_pllv3_prepare,
    .unprepare	= clk_pllv3_unprepare,
    .is_prepared	= clk_pllv3_is_prepared,
    .recalc_rate	= clk_pllv3_av_recalc_rate,
    .determine_rate = clk_pllv3_av_determine_rate,
    .set_rate	= clk_pllv3_av_set_rate,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_pllv3_vf610_mf {
    pub /: *mut *mut u32 mfi; / integer part, can be 20 or 22,
    pub /: *mut *mut u32 mfn; / numerator, 30-bit value,
    pub /: *mut *mut u32 mfd; / denominator, 30-bit value, must be less than mfn,
}

    static unsigned long clk_pllv3_vf610_mf_to_rate(unsigned long parent_rate,
    struct clk_pllv3_vf610_mf mf)
    {
    u64 temp64;
    temp64 = parent_rate;
    temp64 *= mf.mfn;
    do_div(temp64, mf.mfd);
    return (parent_rate * mf.mfi) + temp64;
    }
    static struct clk_pllv3_vf610_mf clk_pllv3_vf610_rate_to_mf(
    unsigned long parent_rate, unsigned long rate)
    {
    struct clk_pllv3_vf610_mf mf;
    u64 temp64;
    mf.mfi = (rate >= 22 * parent_rate) ? 22 : 20;
    mf.mfd = 0x3fffffff;	/* use max supported value for best accuracy */
    if (rate <= parent_rate * mf.mfi)
    mf.mfn = 0;
#[no_mangle]
pub unsafe extern "C" fn if(1): *mut *mut rate >= parent_rate  (mf.mfi +) -> else {
    else if (rate >= parent_rate * (mf.mfi + 1))
    mf.mfn = mf.mfd - 1;
    else {
// rate = parent_rate * (mfi + mfn/mfd)
    temp64 = rate - parent_rate * mf.mfi;
    temp64 *= mf.mfd;
    temp64 = div64_ul(temp64, parent_rate);
    mf.mfn = temp64;
    }
    return mf;
    }
    static unsigned long clk_pllv3_vf610_recalc_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    struct clk_pllv3 *pll = to_clk_pllv3(hw);
    struct clk_pllv3_vf610_mf mf;
    mf.mfn = readl_relaxed(pll.base + pll.num_offset);
    mf.mfd = readl_relaxed(pll.base + pll.denom_offset);
    mf.mfi = (readl_relaxed(pll.base) & pll.div_mask) ? 22 : 20;
    return clk_pllv3_vf610_mf_to_rate(parent_rate, mf);
    }
    static int clk_pllv3_vf610_determine_rate(struct clk_hw *hw,
    struct clk_rate_request *req)
    {
    struct clk_pllv3_vf610_mf mf = clk_pllv3_vf610_rate_to_mf(req.best_parent_rate,
    req.rate);
    req.rate = clk_pllv3_vf610_mf_to_rate(req.best_parent_rate, mf);
    return 0;
    }
    static int clk_pllv3_vf610_set_rate(struct clk_hw *hw, unsigned long rate,
    unsigned long parent_rate)
    {
    struct clk_pllv3 *pll = to_clk_pllv3(hw);
    struct clk_pllv3_vf610_mf mf =
    clk_pllv3_vf610_rate_to_mf(parent_rate, rate);
    u32 val;
    val = readl_relaxed(pll.base);
    if (mf.mfi == 20)
    val &= ~pll.div_mask;	/* clear bit for mfi=20 */
    else
    val |= pll.div_mask;	/* set bit for mfi=22 */
    writel_relaxed(val, pll.base);
    writel_relaxed(mf.mfn, pll.base + pll.num_offset);
    writel_relaxed(mf.mfd, pll.base + pll.denom_offset);
    return clk_pllv3_wait_lock(pll);
    }
    static const struct clk_ops clk_pllv3_vf610_ops = {
    .prepare	= clk_pllv3_prepare,
    .unprepare	= clk_pllv3_unprepare,
    .is_prepared	= clk_pllv3_is_prepared,
    .recalc_rate	= clk_pllv3_vf610_recalc_rate,
    .determine_rate = clk_pllv3_vf610_determine_rate,
    .set_rate	= clk_pllv3_vf610_set_rate,
    };
    static unsigned long clk_pllv3_enet_recalc_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    struct clk_pllv3 *pll = to_clk_pllv3(hw);
    return pll.ref_clock;
    }
    static const struct clk_ops clk_pllv3_enet_ops = {
    .prepare	= clk_pllv3_prepare,
    .unprepare	= clk_pllv3_unprepare,
    .is_prepared	= clk_pllv3_is_prepared,
    .recalc_rate	= clk_pllv3_enet_recalc_rate,
    };
    struct clk_hw *imx_clk_hw_pllv3(enum imx_pllv3_type type, const char *name,
    const char *parent_name, void __iomem *base,
    u32 div_mask)
    {
    struct clk_pllv3 *pll;
    const struct clk_ops *ops;
    struct clk_hw *hw;
    struct clk_init_data init;
    int ret;
    pll = kzalloc_obj(*pll);
    if (!pll)
    return ERR_PTR(-ENOMEM);
    pll.power_bit = BM_PLL_POWER;
    pll.num_offset = PLL_NUM_OFFSET;
    pll.denom_offset = PLL_DENOM_OFFSET;
    switch (type) {
    case IMX_PLLV3_SYS:
    ops = &clk_pllv3_sys_ops;
    break;
    case IMX_PLLV3_SYS_VF610:
    ops = &clk_pllv3_vf610_ops;
    pll.num_offset = PLL_VF610_NUM_OFFSET;
    pll.denom_offset = PLL_VF610_DENOM_OFFSET;
    break;
    case IMX_PLLV3_USB_VF610:
    pll.div_shift = 1;
    fallthrough;
    case IMX_PLLV3_USB:
    ops = &clk_pllv3_ops;
    pll.powerup_set = true;
    break;
    case IMX_PLLV3_AV_IMX7:
    pll.num_offset = PLL_IMX7_NUM_OFFSET;
    pll.denom_offset = PLL_IMX7_DENOM_OFFSET;
    fallthrough;
    case IMX_PLLV3_AV:
    ops = &clk_pllv3_av_ops;
    break;
    case IMX_PLLV3_ENET_IMX7:
    pll.power_bit = IMX7_ENET_PLL_POWER;
    pll.ref_clock = 1000000000;
    ops = &clk_pllv3_enet_ops;
    break;
    case IMX_PLLV3_ENET:
    pll.ref_clock = 500000000;
    ops = &clk_pllv3_enet_ops;
    break;
    case IMX_PLLV3_DDR_IMX7:
    pll.power_bit = IMX7_DDR_PLL_POWER;
    pll.num_offset = PLL_IMX7_NUM_OFFSET;
    pll.denom_offset = PLL_IMX7_DENOM_OFFSET;
    ops = &clk_pllv3_av_ops;
    break;
    default:
    ops = &clk_pllv3_ops;
    }
    pll.base = base;
    pll.div_mask = div_mask;
    init.name = name;
    init.ops = ops;
    init.flags = 0;
    init.parent_names = &parent_name;
    init.num_parents = 1;
    pll.hw.init = &init;
    hw = &pll.hw;
    ret = clk_hw_register(core::ptr::null_mut(), hw);
    if (ret) {
    kfree(pll);
    return ERR_PTR(ret);
    }
    return hw;
    }
    EXPORT_SYMBOL_GPL(imx_clk_hw_pllv3);
