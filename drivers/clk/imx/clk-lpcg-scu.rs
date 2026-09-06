//! Automatically rewritten from C to Rust
//! Source: drivers/clk/imx/clk-lpcg-scu.c
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
// Copyright 2018 NXP
// Dong Aisheng <aisheng.dong@nxp.com>
//

    static DEFINE_SPINLOCK(imx_lpcg_scu_lock);
pub const CLK_GATE_SCU_LPCG_MASK: c_uint = 0x3;

//
// struct clk_lpcg_scu - Description of LPCG clock
//
// @hw: clk_hw of this LPCG
// @reg: register of this LPCG clock
// @bit_idx: bit index of this LPCG clock
// @hw_gate: HW auto gate enable
//
// This structure describes one LPCG clock
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_lpcg_scu {
    pub hw: clk_hw,
    pub reg: *mut void __iomem,
    pub bit_idx: u8,
    pub hw_gate: bool,
// for state save&restore
    pub state: u32,
}

// e10858 -LPCG clock gating register synchronization errata
#[no_mangle]
unsafe extern "C" fn lpcg_e10858_writel(rate: c_ulong, reg: *mut void __iomem, val: u32) {
    static void lpcg_e10858_writel(unsigned long rate, void __iomem *reg, u32 val)
    {
    writel(val, reg);
    if (rate >= 24 * HZ_PER_MHZ || rate == 0) {
//
// The time taken to access the LPCG registers from the AP core
// through the interconnect is longer than the minimum delay
// of 4 clock cycles required by the errata.
// Adding a readl will provide sufficient delay to prevent
// back-to-back writes.
//
    readl(reg);
    } else {
//
// For clocks running below 24MHz, wait a minimum of
// 4 clock cycles.
//
    ndelay(4 * (DIV_ROUND_UP(1000 * HZ_PER_MHZ, rate)));
    }
    }
#[no_mangle]
unsafe extern "C" fn clk_lpcg_scu_enable(hw: *mut clk_hw) -> c_int {
    static int clk_lpcg_scu_enable(struct clk_hw *hw)
    {
    struct clk_lpcg_scu *clk = to_clk_lpcg_scu(hw);
    unsigned long flags;
    u32 reg, val;
    spin_lock_irqsave(&imx_lpcg_scu_lock, flags);
    reg = readl_relaxed(clk.reg);
    reg &= ~(CLK_GATE_SCU_LPCG_MASK << clk.bit_idx);
    val = CLK_GATE_SCU_LPCG_SW_SEL;
    if (clk.hw_gate)
    val |= CLK_GATE_SCU_LPCG_HW_SEL;
    reg |= val << clk.bit_idx;
    lpcg_e10858_writel(clk_hw_get_rate(hw), clk.reg, reg);
    spin_unlock_irqrestore(&imx_lpcg_scu_lock, flags);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn clk_lpcg_scu_disable(hw: *mut clk_hw) {
    static void clk_lpcg_scu_disable(struct clk_hw *hw)
    {
    struct clk_lpcg_scu *clk = to_clk_lpcg_scu(hw);
    unsigned long flags;
    u32 reg;
    spin_lock_irqsave(&imx_lpcg_scu_lock, flags);
    reg = readl_relaxed(clk.reg);
    reg &= ~(CLK_GATE_SCU_LPCG_MASK << clk.bit_idx);
    lpcg_e10858_writel(clk_hw_get_rate(hw), clk.reg, reg);
    spin_unlock_irqrestore(&imx_lpcg_scu_lock, flags);
    }
    static const struct clk_ops clk_lpcg_scu_ops = {
    .enable = clk_lpcg_scu_enable,
    .disable = clk_lpcg_scu_disable,
    };
    struct clk_hw *__imx_clk_lpcg_scu(struct device *dev, const char *name,
    const char *parent_name, unsigned long flags,
    void __iomem *reg, u8 bit_idx, bool hw_gate)
    {
    struct clk_lpcg_scu *clk;
    struct clk_init_data init;
    struct clk_hw *hw;
    int ret;
    clk = kzalloc_obj(*clk);
    if (!clk)
    return ERR_PTR(-ENOMEM);
    clk.reg = reg;
    clk.bit_idx = bit_idx;
    clk.hw_gate = hw_gate;
    init.name = name;
    init.ops = &clk_lpcg_scu_ops;
    init.flags = CLK_SET_RATE_PARENT | flags;
    init.parent_names = parent_name ? &parent_name : core::ptr::null_mut();
    init.num_parents = parent_name ? 1 : 0;
    clk.hw.init = &init;
    hw = &clk.hw;
    ret = clk_hw_register(dev, hw);
    if (ret) {
    kfree(clk);
    hw = ERR_PTR(ret);
    return hw;
    }
    if (dev)
    dev_set_drvdata(dev, clk);
    return hw;
    }
#[no_mangle]
pub unsafe extern "C" fn imx_clk_lpcg_scu_unregister(hw: *mut clk_hw) {
    void imx_clk_lpcg_scu_unregister(struct clk_hw *hw)
    {
    struct clk_lpcg_scu *clk = to_clk_lpcg_scu(hw);
    clk_hw_unregister(&clk.hw);
    kfree(clk);
    }
#[no_mangle]
unsafe extern "C" fn imx_clk_lpcg_scu_suspend(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused imx_clk_lpcg_scu_suspend(struct device *dev)
    {
    struct clk_lpcg_scu *clk = dev_get_drvdata(dev);
    if (!strncmp("hdmi_lpcg", clk_hw_get_name(&clk.hw), strlen("hdmi_lpcg")))
    return 0;
    clk.state = readl_relaxed(clk.reg);
    dev_dbg(dev, "save lpcg state 0x%x\n", clk.state);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn imx_clk_lpcg_scu_resume(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused imx_clk_lpcg_scu_resume(struct device *dev)
    {
    struct clk_lpcg_scu *clk = dev_get_drvdata(dev);
    if (!strncmp("hdmi_lpcg", clk_hw_get_name(&clk.hw), strlen("hdmi_lpcg")))
    return 0;
    writel(clk.state, clk.reg);
    lpcg_e10858_writel(0, clk.reg, clk.state);
    dev_dbg(dev, "restore lpcg state 0x%x\n", clk.state);
    return 0;
    }
    const struct dev_pm_ops imx_clk_lpcg_scu_pm_ops = {
    SET_NOIRQ_SYSTEM_SLEEP_PM_OPS(imx_clk_lpcg_scu_suspend,
    imx_clk_lpcg_scu_resume)
    };
