//! Automatically rewritten from C to Rust
//! Source: drivers/clk/mstar/clk-msc313-cpupll.c
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
// Copyright (C) 2019 Daniel Palmer <daniel@thingy.jp>
//

//
// This IP is not documented outside of the messy vendor driver.
// Below is what we think the registers look like based on looking at
// the vendor code and poking at the hardware:
//
// 0x140 -- LPF low. Seems to store one half of the clock transition
// 0x144
// 0x148 -- LPF high. Seems to store one half of the clock transition
// 0x14c
// 0x150 -- vendor code says "toggle lpf enable"
// 0x154 -- mu?
// 0x15c -- lpf_update_count?
// 0x160 -- vendor code says "switch to LPF". Clock source config? Register bank?
// 0x164 -- vendor code says "from low to high" which seems to mean transition from LPF low to
// LPF high.
// 0x174 -- Seems to be the PLL lock status bit
// 0x180 -- Seems to be the current frequency, this might need to be populated by software?
// 0x184 /  The vendor driver uses these to set the initial value of LPF low
//
// Frequency seems to be calculated like this:
// (parent clock (432mhz) / register_magic_value) * 16 * 524288
// Only the lower 24 bits of the resulting value will be used. In addition, the
// PLL doesn't seem to be able to lock on frequencies lower than 220 MHz, as
// divisor 0xfb586f (220 MHz) works but 0xfb7fff locks up.
//
// Vendor values:
// frequency - register value
//
// 400000000  - 0x0067AE14
// 600000000  - 0x00451EB8,
// 800000000  - 0x0033D70A,
// 1000000000 - 0x002978d4,
//
pub const REG_LPF_LOW_L: c_uint = 0x140;
pub const REG_LPF_LOW_H: c_uint = 0x144;
pub const REG_LPF_HIGH_BOTTOM: c_uint = 0x148;
pub const REG_LPF_HIGH_TOP: c_uint = 0x14c;
pub const REG_LPF_TOGGLE: c_uint = 0x150;
pub const REG_LPF_MYSTERYTWO: c_uint = 0x154;
pub const REG_LPF_UPDATE_COUNT: c_uint = 0x15c;
pub const REG_LPF_MYSTERYONE: c_uint = 0x160;
pub const REG_LPF_TRANSITIONCTRL: c_uint = 0x164;
pub const REG_LPF_LOCK: c_uint = 0x174;
pub const REG_CURRENT: c_uint = 0x180;
pub const LPF_LOCK_TIMEOUT: c_int = 100000000;
pub const MULTIPLIER_1: c_int = 16;
pub const MULTIPLIER_2: c_int = 524288;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msc313_cpupll {
    pub base: *mut void __iomem,
    pub clk_hw: clk_hw,
}

#[no_mangle]
unsafe extern "C" fn msc313_cpupll_reg_read32(cpupll: *mut msc313_cpupll, reg: c_uint) -> u32 {
    static u32 msc313_cpupll_reg_read32(struct msc313_cpupll *cpupll, unsigned int reg)
    {
    u32 value;
    value = ioread16(cpupll.base + reg + 4) << 16;
    value |= ioread16(cpupll.base + reg);
    return value;
    }
#[no_mangle]
unsafe extern "C" fn msc313_cpupll_reg_write32(cpupll: *mut msc313_cpupll, reg: c_uint, value: u32) {
    static void msc313_cpupll_reg_write32(struct msc313_cpupll *cpupll, unsigned int reg, u32 value)
    {
    let mut l: u16 = value & 0xffff, h = (value >> 16) & 0xffff;
    iowrite16(l, cpupll.base + reg);
    iowrite16(h, cpupll.base + reg + 4);
    }
#[no_mangle]
unsafe extern "C" fn msc313_cpupll_setfreq(cpupll: *mut msc313_cpupll, regvalue: u32) {
    static void msc313_cpupll_setfreq(struct msc313_cpupll *cpupll, u32 regvalue)
    {
    ktime_t timeout;
    msc313_cpupll_reg_write32(cpupll, REG_LPF_HIGH_BOTTOM, regvalue);
    iowrite16(0x1, cpupll.base + REG_LPF_MYSTERYONE);
    iowrite16(0x6, cpupll.base + REG_LPF_MYSTERYTWO);
    iowrite16(0x8, cpupll.base + REG_LPF_UPDATE_COUNT);
    iowrite16(BIT(12), cpupll.base + REG_LPF_TRANSITIONCTRL);
    iowrite16(0, cpupll.base + REG_LPF_TOGGLE);
    iowrite16(1, cpupll.base + REG_LPF_TOGGLE);
    timeout = ktime_add_ns(ktime_get(), LPF_LOCK_TIMEOUT);
    while (!(ioread16(cpupll.base + REG_LPF_LOCK))) {
    if (ktime_after(ktime_get(), timeout)) {
    pr_err("timeout waiting for LPF_LOCK\n");
    return;
    }
    cpu_relax();
    }
    iowrite16(0, cpupll.base + REG_LPF_TOGGLE);
    msc313_cpupll_reg_write32(cpupll, REG_LPF_LOW_L, regvalue);
    }
#[no_mangle]
unsafe extern "C" fn msc313_cpupll_frequencyforreg(reg: u32, parent_rate: c_ulong) -> c_ulong {
    static unsigned long msc313_cpupll_frequencyforreg(u32 reg, unsigned long parent_rate)
    {
    let mut prescaled: c_ulonglong = ((unsigned long long)parent_rate) * MULTIPLIER;
    if (prescaled == 0 || reg == 0)
    return 0;
    return DIV_ROUND_DOWN_ULL(prescaled, reg);
    }
#[no_mangle]
unsafe extern "C" fn msc313_cpupll_regforfrequecy(rate: c_ulong, parent_rate: c_ulong) -> u32 {
    static u32 msc313_cpupll_regforfrequecy(unsigned long rate, unsigned long parent_rate)
    {
    let mut prescaled: c_ulonglong = ((unsigned long long)parent_rate) * MULTIPLIER;
    if (prescaled == 0 || rate == 0)
    return 0;
    return DIV_ROUND_UP_ULL(prescaled, rate);
    }
#[no_mangle]
unsafe extern "C" fn msc313_cpupll_recalc_rate(hw: *mut clk_hw, parent_rate: c_ulong) -> c_ulong {
    static unsigned long msc313_cpupll_recalc_rate(struct clk_hw *hw, unsigned long parent_rate)
    {
    struct msc313_cpupll *cpupll = to_cpupll(hw);
    return msc313_cpupll_frequencyforreg(msc313_cpupll_reg_read32(cpupll, REG_LPF_LOW_L),
    parent_rate);
    }
    static int msc313_cpupll_determine_rate(struct clk_hw *hw,
    struct clk_rate_request *req)
    {
    let mut reg: u32 = msc313_cpupll_regforfrequecy(req.rate, req.best_parent_rate);
    let mut rounded: c_long = msc313_cpupll_frequencyforreg(reg, req.best_parent_rate);
//
// This is my poor attempt at making sure the resulting
// rate doesn't overshoot the requested rate.
//
    for (; rounded >= req.rate && reg > 0; reg--)
    rounded = msc313_cpupll_frequencyforreg(reg, req.best_parent_rate);
    req.rate = rounded;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn msc313_cpupll_set_rate(hw: *mut clk_hw, rate: c_ulong, parent_rate: c_ulong) -> c_int {
    static int msc313_cpupll_set_rate(struct clk_hw *hw, unsigned long rate, unsigned long parent_rate)
    {
    struct msc313_cpupll *cpupll = to_cpupll(hw);
    let mut reg: u32 = msc313_cpupll_regforfrequecy(rate, parent_rate);
    msc313_cpupll_setfreq(cpupll, reg);
    return 0;
    }
    static const struct clk_ops msc313_cpupll_ops = {
    .recalc_rate	= msc313_cpupll_recalc_rate,
    .determine_rate = msc313_cpupll_determine_rate,
    .set_rate	= msc313_cpupll_set_rate,
    };
    static const struct of_device_id msc313_cpupll_of_match[] = {
    { .compatible = "mstar,msc313-cpupll" },
    {}
    };
#[no_mangle]
unsafe extern "C" fn msc313_cpupll_probe(pdev: *mut platform_device) -> c_int {
    static int msc313_cpupll_probe(struct platform_device *pdev)
    {
    let mut clk_init: clk_init_data = {};
    let mut cpupll_parent: clk_parent_data = { .index	= 0 };
    struct device *dev = &pdev.dev;
    struct msc313_cpupll *cpupll;
    int ret;
    cpupll = devm_kzalloc(&pdev.dev, sizeof(*cpupll), GFP_KERNEL);
    if (!cpupll)
    return -ENOMEM;
    cpupll.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(cpupll.base))
    return PTR_ERR(cpupll.base);
// LPF might not contain the current frequency so fix that up
    msc313_cpupll_reg_write32(cpupll, REG_LPF_LOW_L,
    msc313_cpupll_reg_read32(cpupll, REG_CURRENT));
    clk_init.name = dev_name(dev);
    clk_init.ops = &msc313_cpupll_ops;
    clk_init.parent_data = &cpupll_parent;
    clk_init.num_parents = 1;
    cpupll.clk_hw.init = &clk_init;
    ret = devm_clk_hw_register(dev, &cpupll.clk_hw);
    if (ret)
    return ret;
    return devm_of_clk_add_hw_provider(&pdev.dev, of_clk_hw_simple_get, &cpupll.clk_hw);
    }
    static struct platform_driver msc313_cpupll_driver = {
    .driver = {
    .name = "mstar-msc313-cpupll",
    .of_match_table = msc313_cpupll_of_match,
    },
    .probe = msc313_cpupll_probe,
    };
    builtin_platform_driver(msc313_cpupll_driver);
