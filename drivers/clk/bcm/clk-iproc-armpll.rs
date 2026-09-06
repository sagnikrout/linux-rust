//! Automatically rewritten from C to Rust
//! Source: drivers/clk/bcm/clk-iproc-armpll.c
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
// Copyright (C) 2014 Broadcom Corporation

pub const IPROC_CLK_MAX_FREQ_POLICY: c_uint = 0x3;
pub const IPROC_CLK_POLICY_FREQ_OFFSET: c_uint = 0x008;
pub const IPROC_CLK_POLICY_FREQ_POLICY_FREQ_SHIFT: c_int = 8;
pub const IPROC_CLK_POLICY_FREQ_POLICY_FREQ_MASK: c_uint = 0x7;
pub const IPROC_CLK_PLLARMA_OFFSET: c_uint = 0xc00;
pub const IPROC_CLK_PLLARMA_LOCK_SHIFT: c_int = 28;
pub const IPROC_CLK_PLLARMA_PDIV_SHIFT: c_int = 24;
pub const IPROC_CLK_PLLARMA_PDIV_MASK: c_uint = 0xf;
pub const IPROC_CLK_PLLARMA_NDIV_INT_SHIFT: c_int = 8;
pub const IPROC_CLK_PLLARMA_NDIV_INT_MASK: c_uint = 0x3ff;
pub const IPROC_CLK_PLLARMB_OFFSET: c_uint = 0xc04;
pub const IPROC_CLK_PLLARMB_NDIV_FRAC_MASK: c_uint = 0xfffff;
pub const IPROC_CLK_PLLARMC_OFFSET: c_uint = 0xc08;
pub const IPROC_CLK_PLLARMC_BYPCLK_EN_SHIFT: c_int = 8;
pub const IPROC_CLK_PLLARMC_MDIV_MASK: c_uint = 0xff;
pub const IPROC_CLK_PLLARMCTL5_OFFSET: c_uint = 0xc20;
pub const IPROC_CLK_PLLARMCTL5_H_MDIV_MASK: c_uint = 0xff;
pub const IPROC_CLK_PLLARM_OFFSET_OFFSET: c_uint = 0xc24;
pub const IPROC_CLK_PLLARM_SW_CTL_SHIFT: c_int = 29;
pub const IPROC_CLK_PLLARM_NDIV_INT_OFFSET_SHIFT: c_int = 20;
pub const IPROC_CLK_PLLARM_NDIV_INT_OFFSET_MASK: c_uint = 0xff;
pub const IPROC_CLK_PLLARM_NDIV_FRAC_OFFSET_MASK: c_uint = 0xfffff;
pub const IPROC_CLK_ARM_DIV_OFFSET: c_uint = 0xe00;
pub const IPROC_CLK_ARM_DIV_PLL_SELECT_OVERRIDE_SHIFT: c_int = 4;
pub const IPROC_CLK_ARM_DIV_ARM_PLL_SELECT_MASK: c_uint = 0xf;
pub const IPROC_CLK_POLICY_DBG_OFFSET: c_uint = 0xec0;
pub const IPROC_CLK_POLICY_DBG_ACT_FREQ_SHIFT: c_int = 12;
pub const IPROC_CLK_POLICY_DBG_ACT_FREQ_MASK: c_uint = 0x7;
    enum iproc_arm_pll_fid {
    ARM_PLL_FID_CRYSTAL_CLK   = 0,
    ARM_PLL_FID_SYS_CLK       = 2,
    ARM_PLL_FID_CH0_SLOW_CLK  = 6,
    ARM_PLL_FID_CH1_FAST_CLK  = 7
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iproc_arm_pll {
    pub hw: clk_hw,
    pub base: *mut void __iomem,
    pub rate: c_ulong,
}

#[no_mangle]
unsafe extern "C" fn __get_fid(pll: *mut iproc_arm_pll) -> c_uint {
    static unsigned int __get_fid(struct iproc_arm_pll *pll)
    {
    u32 val;
    unsigned int policy, fid, active_fid;
    val = readl(pll.base + IPROC_CLK_ARM_DIV_OFFSET);
    if (val & (1 << IPROC_CLK_ARM_DIV_PLL_SELECT_OVERRIDE_SHIFT))
    policy = val & IPROC_CLK_ARM_DIV_ARM_PLL_SELECT_MASK;
    else
    policy = 0;
// something is seriously wrong
    BUG_ON(policy > IPROC_CLK_MAX_FREQ_POLICY);
    val = readl(pll.base + IPROC_CLK_POLICY_FREQ_OFFSET);
    fid = (val >> (IPROC_CLK_POLICY_FREQ_POLICY_FREQ_SHIFT * policy)) &
    IPROC_CLK_POLICY_FREQ_POLICY_FREQ_MASK;
    val = readl(pll.base + IPROC_CLK_POLICY_DBG_OFFSET);
    active_fid = IPROC_CLK_POLICY_DBG_ACT_FREQ_MASK &
    (val >> IPROC_CLK_POLICY_DBG_ACT_FREQ_SHIFT);
    if (fid != active_fid) {
    pr_debug("%s: fid override %u.%u\n", __func__,	fid,
    active_fid);
    fid = active_fid;
    }
    pr_debug("%s: active fid: %u\n", __func__, fid);
    return fid;
    }
//
// Determine the mdiv (post divider) based on the frequency ID being used.
// There are 4 sources that can be used to derive the output clock rate:
// - 25 MHz Crystal
// - System clock
// - PLL channel 0 (slow clock)
// - PLL channel 1 (fast clock)
//
#[no_mangle]
unsafe extern "C" fn __get_mdiv(pll: *mut iproc_arm_pll) -> c_int {
    static int __get_mdiv(struct iproc_arm_pll *pll)
    {
    unsigned int fid;
    int mdiv;
    u32 val;
    fid = __get_fid(pll);
    switch (fid) {
    case ARM_PLL_FID_CRYSTAL_CLK:
    case ARM_PLL_FID_SYS_CLK:
    mdiv = 1;
    break;
    case ARM_PLL_FID_CH0_SLOW_CLK:
    val = readl(pll.base + IPROC_CLK_PLLARMC_OFFSET);
    mdiv = val & IPROC_CLK_PLLARMC_MDIV_MASK;
    if (mdiv == 0)
    mdiv = 256;
    break;
    case ARM_PLL_FID_CH1_FAST_CLK:
    val = readl(pll.base +	IPROC_CLK_PLLARMCTL5_OFFSET);
    mdiv = val & IPROC_CLK_PLLARMCTL5_H_MDIV_MASK;
    if (mdiv == 0)
    mdiv = 256;
    break;
    default:
    mdiv = -EFAULT;
    }
    return mdiv;
    }
#[no_mangle]
unsafe extern "C" fn __get_ndiv(pll: *mut iproc_arm_pll) -> c_uint {
    static unsigned int __get_ndiv(struct iproc_arm_pll *pll)
    {
    u32 val;
    unsigned int ndiv_int, ndiv_frac, ndiv;
    val = readl(pll.base + IPROC_CLK_PLLARM_OFFSET_OFFSET);
    if (val & (1 << IPROC_CLK_PLLARM_SW_CTL_SHIFT)) {
//
// offset mode is active. Read the ndiv from the PLLARM OFFSET
// register
//
    ndiv_int = (val >> IPROC_CLK_PLLARM_NDIV_INT_OFFSET_SHIFT) &
    IPROC_CLK_PLLARM_NDIV_INT_OFFSET_MASK;
    if (ndiv_int == 0)
    ndiv_int = 256;
    ndiv_frac = val & IPROC_CLK_PLLARM_NDIV_FRAC_OFFSET_MASK;
    } else {
// offset mode not active
    val = readl(pll.base + IPROC_CLK_PLLARMA_OFFSET);
    ndiv_int = (val >> IPROC_CLK_PLLARMA_NDIV_INT_SHIFT) &
    IPROC_CLK_PLLARMA_NDIV_INT_MASK;
    if (ndiv_int == 0)
    ndiv_int = 1024;
    val = readl(pll.base + IPROC_CLK_PLLARMB_OFFSET);
    ndiv_frac = val & IPROC_CLK_PLLARMB_NDIV_FRAC_MASK;
    }
    ndiv = (ndiv_int << 20) | ndiv_frac;
    return ndiv;
    }
//
// The output frequency of the ARM PLL is calculated based on the ARM PLL
// divider values:
// pdiv = ARM PLL pre-divider
// ndiv = ARM PLL multiplier
// mdiv = ARM PLL post divider
//
// The frequency is calculated by:
// ((ndiv * parent clock rate) / pdiv) / mdiv
//
    static unsigned long iproc_arm_pll_recalc_rate(struct clk_hw *hw,
    unsigned long parent_rate)
    {
    struct iproc_arm_pll *pll = to_iproc_arm_pll(hw);
    u32 val;
    int mdiv;
    u64 ndiv;
    unsigned int pdiv;
// in bypass mode, use parent rate
    val = readl(pll.base + IPROC_CLK_PLLARMC_OFFSET);
    if (val & (1 << IPROC_CLK_PLLARMC_BYPCLK_EN_SHIFT)) {
    pll.rate = parent_rate;
    return pll.rate;
    }
// PLL needs to be locked
    val = readl(pll.base + IPROC_CLK_PLLARMA_OFFSET);
    if (!(val & (1 << IPROC_CLK_PLLARMA_LOCK_SHIFT))) {
    pll.rate = 0;
    return 0;
    }
    pdiv = (val >> IPROC_CLK_PLLARMA_PDIV_SHIFT) &
    IPROC_CLK_PLLARMA_PDIV_MASK;
    if (pdiv == 0)
    pdiv = 16;
    ndiv = __get_ndiv(pll);
    mdiv = __get_mdiv(pll);
    if (mdiv <= 0) {
    pll.rate = 0;
    return 0;
    }
    pll.rate = (ndiv * parent_rate) >> 20;
    pll.rate = (pll.rate / pdiv) / mdiv;
    pr_debug("%s: ARM PLL rate: %lu. parent rate: %lu\n", __func__,
    pll.rate, parent_rate);
    pr_debug("%s: ndiv_int: %u, pdiv: %u, mdiv: %d\n", __func__,
    (unsigned int)(ndiv >> 20), pdiv, mdiv);
    return pll.rate;
    }
    static const struct clk_ops iproc_arm_pll_ops = {
    .recalc_rate = iproc_arm_pll_recalc_rate,
    };
#[no_mangle]
pub unsafe extern "C" fn iproc_armpll_setup(node: *mut device_node) -> void __init {
    void __init iproc_armpll_setup(struct device_node *node)
    {
    int ret;
    struct iproc_arm_pll *pll;
    struct clk_init_data init;
    const char *parent_name;
    pll = kzalloc_obj(*pll);
    if (WARN_ON(!pll))
    return;
    pll.base = of_iomap(node, 0);
    if (WARN_ON(!pll.base))
    goto err_free_pll;
    init.name = node.name;
    init.ops = &iproc_arm_pll_ops;
    init.flags = 0;
    parent_name = of_clk_get_parent_name(node, 0);
    init.parent_names = (parent_name ? &parent_name : core::ptr::null_mut());
    init.num_parents = (parent_name ? 1 : 0);
    pll.hw.init = &init;
    ret = clk_hw_register(core::ptr::null_mut(), &pll.hw);
    if (WARN_ON(ret))
    goto err_iounmap;
    ret = of_clk_add_hw_provider(node, of_clk_hw_simple_get, &pll.hw);
    if (WARN_ON(ret))
    goto err_clk_unregister;
    return;
    err_clk_unregister:
    clk_hw_unregister(&pll.hw);
    err_iounmap:
    iounmap(pll.base);
    err_free_pll:
    kfree(pll);
    }
