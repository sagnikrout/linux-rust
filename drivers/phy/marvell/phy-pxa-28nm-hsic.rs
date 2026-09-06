//! Automatically rewritten from C to Rust
//! Source: drivers/phy/marvell/phy-pxa-28nm-hsic.c
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
// Copyright (C) 2015 Linaro, Ltd.
// Rob Herring <robh@kernel.org>
//
// Based on vendor driver:
// Copyright (C) 2013 Marvell Inc.
// Author: Chao Xie <xiechao.mail@gmail.com>
//

pub const PHY_28NM_HSIC_CTRL: c_uint = 0x08;
pub const PHY_28NM_HSIC_IMPCAL_CAL: c_uint = 0x18;
pub const PHY_28NM_HSIC_PLL_CTRL01: c_uint = 0x1c;
pub const PHY_28NM_HSIC_PLL_CTRL2: c_uint = 0x20;
pub const PHY_28NM_HSIC_INT: c_uint = 0x28;
pub const PHY_28NM_HSIC_PLL_SELLPFR_SHIFT: c_int = 26;
pub const PHY_28NM_HSIC_PLL_FBDIV_SHIFT: c_int = 0;
pub const PHY_28NM_HSIC_PLL_REFDIV_SHIFT: c_int = 9;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mv_hsic_phy {
    pub phy: *mut phy,
    pub pdev: *mut platform_device,
    pub base: *mut void __iomem,
    pub clk: *mut clk,
}

#[no_mangle]
unsafe extern "C" fn wait_for_reg(reg: *mut void __iomem, mask: u32, ms: u32) -> c_int {
    static int wait_for_reg(void __iomem *reg, u32 mask, u32 ms)
    {
    u32 val;
    return readl_poll_timeout(reg, val, ((val & mask) == mask),
    1000, 1000 * ms);
    }
#[no_mangle]
unsafe extern "C" fn mv_hsic_phy_init(phy: *mut phy) -> c_int {
    static int mv_hsic_phy_init(struct phy *phy)
    {
    struct mv_hsic_phy *mv_phy = phy_get_drvdata(phy);
    struct platform_device *pdev = mv_phy.pdev;
    void __iomem *base = mv_phy.base;
    int ret;
    clk_prepare_enable(mv_phy.clk);
// Set reference clock
    writel(0x1 << PHY_28NM_HSIC_PLL_SELLPFR_SHIFT |
    0xf0 << PHY_28NM_HSIC_PLL_FBDIV_SHIFT |
    0xd << PHY_28NM_HSIC_PLL_REFDIV_SHIFT,
    base + PHY_28NM_HSIC_PLL_CTRL01);
// Turn on PLL
    writel(readl(base + PHY_28NM_HSIC_PLL_CTRL2) |
    PHY_28NM_HSIC_S2H_PU_PLL,
    base + PHY_28NM_HSIC_PLL_CTRL2);
// Make sure PHY PLL is locked
    ret = wait_for_reg(base + PHY_28NM_HSIC_PLL_CTRL2,
    PHY_28NM_HSIC_H2S_PLL_LOCK, 100);
    if (ret) {
    dev_err(&pdev.dev, "HSIC PHY PLL not locked after 100mS.");
    clk_disable_unprepare(mv_phy.clk);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn mv_hsic_phy_power_on(phy: *mut phy) -> c_int {
    static int mv_hsic_phy_power_on(struct phy *phy)
    {
    struct mv_hsic_phy *mv_phy = phy_get_drvdata(phy);
    struct platform_device *pdev = mv_phy.pdev;
    void __iomem *base = mv_phy.base;
    u32 reg;
    int ret;
    reg = readl(base + PHY_28NM_HSIC_CTRL);
// Avoid SE0 state when resume for some device will take it as reset
    reg &= ~S2H_DRV_SE0_4RESUME;
    reg |= PHY_28NM_HSIC_S2H_HSIC_EN;	/* Enable HSIC PHY */
    writel(reg, base + PHY_28NM_HSIC_CTRL);
//
// Calibration Timing
// ____________________________
// CAL START   ___|
// ____________________
// CAL_DONE    ___________|
// | 400us |
//
// Make sure PHY Calibration is ready
    ret = wait_for_reg(base + PHY_28NM_HSIC_IMPCAL_CAL,
    PHY_28NM_HSIC_H2S_IMPCAL_DONE, 100);
    if (ret) {
    dev_warn(&pdev.dev, "HSIC PHY READY not set after 100mS.");
    return ret;
    }
// Waiting for HSIC connect int
    ret = wait_for_reg(base + PHY_28NM_HSIC_INT,
    PHY_28NM_HSIC_CONNECT_INT, 200);
    if (ret)
    dev_warn(&pdev.dev, "HSIC wait for connect interrupt timeout.");
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn mv_hsic_phy_power_off(phy: *mut phy) -> c_int {
    static int mv_hsic_phy_power_off(struct phy *phy)
    {
    struct mv_hsic_phy *mv_phy = phy_get_drvdata(phy);
    void __iomem *base = mv_phy.base;
    writel(readl(base + PHY_28NM_HSIC_CTRL) & ~PHY_28NM_HSIC_S2H_HSIC_EN,
    base + PHY_28NM_HSIC_CTRL);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mv_hsic_phy_exit(phy: *mut phy) -> c_int {
    static int mv_hsic_phy_exit(struct phy *phy)
    {
    struct mv_hsic_phy *mv_phy = phy_get_drvdata(phy);
    void __iomem *base = mv_phy.base;
// Turn off PLL
    writel(readl(base + PHY_28NM_HSIC_PLL_CTRL2) &
    ~PHY_28NM_HSIC_S2H_PU_PLL,
    base + PHY_28NM_HSIC_PLL_CTRL2);
    clk_disable_unprepare(mv_phy.clk);
    return 0;
    }
    static const struct phy_ops hsic_ops = {
    .init		= mv_hsic_phy_init,
    .power_on	= mv_hsic_phy_power_on,
    .power_off	= mv_hsic_phy_power_off,
    .exit		= mv_hsic_phy_exit,
    .owner		= THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn mv_hsic_phy_probe(pdev: *mut platform_device) -> c_int {
    static int mv_hsic_phy_probe(struct platform_device *pdev)
    {
    struct phy_provider *phy_provider;
    struct mv_hsic_phy *mv_phy;
    mv_phy = devm_kzalloc(&pdev.dev, sizeof(*mv_phy), GFP_KERNEL);
    if (!mv_phy)
    return -ENOMEM;
    mv_phy.pdev = pdev;
    mv_phy.clk = devm_clk_get(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(mv_phy.clk)) {
    dev_err(&pdev.dev, "failed to get clock.\n");
    return PTR_ERR(mv_phy.clk);
    }
    mv_phy.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(mv_phy.base))
    return PTR_ERR(mv_phy.base);
    mv_phy.phy = devm_phy_create(&pdev.dev, pdev.dev.of_node, &hsic_ops);
    if (IS_ERR(mv_phy.phy))
    return PTR_ERR(mv_phy.phy);
    phy_set_drvdata(mv_phy.phy, mv_phy);
    phy_provider = devm_of_phy_provider_register(&pdev.dev, of_phy_simple_xlate);
    return PTR_ERR_OR_ZERO(phy_provider);
    }
    static const struct of_device_id mv_hsic_phy_dt_match[] = {
    { .compatible = "marvell,pxa1928-hsic-phy", },
    {},
    };
    MODULE_DEVICE_TABLE(of, mv_hsic_phy_dt_match);
    static struct platform_driver mv_hsic_phy_driver = {
    .probe	= mv_hsic_phy_probe,
    .driver = {
    .name   = "mv-hsic-phy",
    .of_match_table = mv_hsic_phy_dt_match,
    },
    };
    module_platform_driver(mv_hsic_phy_driver);
    MODULE_AUTHOR("Rob Herring <robh@kernel.org>");
    MODULE_DESCRIPTION("Marvell HSIC phy driver");
    MODULE_LICENSE("GPL v2");
