//! Automatically rewritten from C to Rust
//! Source: drivers/clk/imx/clk-pfdv2.c
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

//
// struct clk_pfdv2 - IMX PFD clock
// @hw:		clock source
// @reg:	PFD register address
// @gate_bit:	Gate bit offset
// @vld_bit:	Valid bit offset
// @frac_off:	PLL Fractional Divider offset
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_pfdv2 {
    pub hw: clk_hw,
    pub reg: *mut void __iomem,
    pub gate_bit: u8,
    pub vld_bit: u8,
    pub frac_off: u8,
}

pub const CLK_PFDV2_FRAC_MASK: c_uint = 0x3f;

    static DEFINE_SPINLOCK(pfd_lock);
#[no_mangle]
unsafe extern "C" fn clk_pfdv2_wait(pfd: *mut clk_pfdv2) -> c_int {
    static int clk_pfdv2_wait(struct clk_pfdv2 *pfd)
    {
    u32 val;
    return readl_poll_timeout(pfd.reg, val, val & (1 << pfd.vld_bit),
    0, LOCK_TIMEOUT_US);
    }
#[no_mangle]
unsafe extern "C" fn clk_pfdv2_enable(hw: *mut clk_hw) -> c_int {
    static int clk_pfdv2_enable(struct clk_hw *hw)
    {
    struct clk_pfdv2 *pfd = to_clk_pfdv2(hw);
    unsigned long flags;
    u32 val;
    spin_lock_irqsave(&pfd_lock, flags);
    val = readl_relaxed(pfd.reg);
    val &= ~(1 << pfd.gate_bit);
    writel_relaxed(val, pfd.reg);
    spin_unlock_irqrestore(&pfd_lock, flags);
    return clk_pfdv2_wait(pfd);
    }
#[no_mangle]
unsafe extern "C" fn clk_pfdv2_disable(hw: *mut clk_hw) {
    static void clk_pfdv2_disable(struct clk_hw *hw)
    {
    struct clk_pfdv2 *pfd = to_clk_pfdv2(hw);
    unsigned long flags;
    u32 val;
    spin_lock_irqsave(&pfd_lock, flags);
    val = readl_relaxed(pfd.reg);
    val |= (1 << pfd.gate_bit);
    writel_relaxed(val, pfd.reg);
    spin_unlock_irqrestore(&pfd_lock, flags);
    }
    static unsigned long clk_pfdv2_recalc_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    struct clk_pfdv2 *pfd = to_clk_pfdv2(hw);
    let mut tmp: u64 = parent_rate;
    u8 frac;
    frac = (readl_relaxed(pfd.reg) >> pfd.frac_off)
    & CLK_PFDV2_FRAC_MASK;
    if (!frac) {
    pr_debug("clk_pfdv2: %s invalid pfd frac value 0\n",
    clk_hw_get_name(hw));
    return 0;
    }
    tmp *= 18;
    do_div(tmp, frac);
    return tmp;
    }
    static int clk_pfdv2_determine_rate(struct clk_hw *hw,
    struct clk_rate_request *req)
    {
    unsigned long parent_rates[] = {
    480000000,
    528000000,
    req.best_parent_rate
    };
    let mut best_rate: c_ulong = -1UL, rate = req.rate;
    let mut best_parent_rate: c_ulong = req.best_parent_rate;
    u64 tmp;
    u8 frac;
    int i;
    for (i = 0; i < ARRAY_SIZE(parent_rates); i++) {
    tmp = parent_rates[i];
    tmp = tmp * 18 + rate / 2;
    do_div(tmp, rate);
    frac = tmp;
    if (frac < 12)
    frac = 12;
#[no_mangle]
pub unsafe extern "C" fn if(35: frac >) -> else {
    else if (frac > 35)
    frac = 35;
    tmp = parent_rates[i];
    tmp *= 18;
    do_div(tmp, frac);
    if (abs(tmp - req.rate) < abs(best_rate - req.rate)) {
    best_rate = tmp;
    best_parent_rate = parent_rates[i];
    }
    }
    req.best_parent_rate = best_parent_rate;
    req.rate = best_rate;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn clk_pfdv2_is_enabled(hw: *mut clk_hw) -> c_int {
    static int clk_pfdv2_is_enabled(struct clk_hw *hw)
    {
    struct clk_pfdv2 *pfd = to_clk_pfdv2(hw);
    if (readl_relaxed(pfd.reg) & (1 << pfd.gate_bit))
    return 0;
    return 1;
    }
    static int clk_pfdv2_set_rate(struct clk_hw *hw, unsigned long rate,
    unsigned long parent_rate)
    {
    struct clk_pfdv2 *pfd = to_clk_pfdv2(hw);
    unsigned long flags;
    let mut tmp: u64 = parent_rate;
    u32 val;
    u8 frac;
    if (!rate)
    return -EINVAL;
//
// PFD can NOT change rate without gating.
// as the PFDs may enabled in HW by default but no
// consumer used it, the enable count is '0', so the
// 'SET_RATE_GATE' can NOT help on blocking the set_rate
// ops especially for 'assigned-clock-xxx'. In order
// to simplify the case, just disable the PFD if it is
// enabled in HW but not in SW.
//
    if (clk_pfdv2_is_enabled(hw))
    clk_pfdv2_disable(hw);
    tmp = tmp * 18 + rate / 2;
    do_div(tmp, rate);
    frac = tmp;
    if (frac < 12)
    frac = 12;
#[no_mangle]
pub unsafe extern "C" fn if(35: frac >) -> else {
    else if (frac > 35)
    frac = 35;
    spin_lock_irqsave(&pfd_lock, flags);
    val = readl_relaxed(pfd.reg);
    val &= ~(CLK_PFDV2_FRAC_MASK << pfd.frac_off);
    val |= frac << pfd.frac_off;
    writel_relaxed(val, pfd.reg);
    spin_unlock_irqrestore(&pfd_lock, flags);
    return 0;
    }
    static const struct clk_ops clk_pfdv2_ops = {
    .enable		= clk_pfdv2_enable,
    .disable	= clk_pfdv2_disable,
    .recalc_rate	= clk_pfdv2_recalc_rate,
    .determine_rate	= clk_pfdv2_determine_rate,
    .set_rate	= clk_pfdv2_set_rate,
    .is_enabled     = clk_pfdv2_is_enabled,
    };
    struct clk_hw *imx_clk_hw_pfdv2(enum imx_pfdv2_type type, const char *name,
    const char *parent_name, void __iomem *reg, u8 idx)
    {
    struct clk_init_data init;
    struct clk_pfdv2 *pfd;
    struct clk_hw *hw;
    int ret;
    WARN_ON(idx > 3);
    pfd = kzalloc_obj(*pfd);
    if (!pfd)
    return ERR_PTR(-ENOMEM);
    pfd.reg = reg;
    pfd.gate_bit = (idx + 1) * 8 - 1;
    pfd.vld_bit = pfd.gate_bit - 1;
    pfd.frac_off = idx * 8;
    init.name = name;
    init.ops = &clk_pfdv2_ops;
    init.parent_names = &parent_name;
    init.num_parents = 1;
    if (type == IMX_PFDV2_IMX7ULP)
    init.flags = CLK_SET_RATE_GATE | CLK_SET_RATE_PARENT;
    else
    init.flags = CLK_SET_RATE_GATE;
    pfd.hw.init = &init;
    hw = &pfd.hw;
    ret = clk_hw_register(core::ptr::null_mut(), hw);
    if (ret) {
    kfree(pfd);
    hw = ERR_PTR(ret);
    }
    return hw;
    }
    EXPORT_SYMBOL_GPL(imx_clk_hw_pfdv2);
