//! Automatically rewritten from C to Rust
//! Source: drivers/phy/allwinner/phy-sun50i-usb3.c
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
// Allwinner sun50i(H6) USB 3.0 phy driver
//
// Copyright (C) 2017 Icenowy Zheng <icenowy@aosc.io>
//
// Based on phy-sun9i-usb.c, which is:
//
// Copyright (C) 2014-2015 Chen-Yu Tsai <wens@csie.org>
//
// Based on code from Allwinner BSP, which is:
//
// Copyright (c) 2010-2015 Allwinner Technology Co., Ltd.
//

// Interface Status and Control Registers
pub const SUNXI_ISCR: c_uint = 0x00;
pub const SUNXI_PIPE_CLOCK_CONTROL: c_uint = 0x14;
pub const SUNXI_PHY_TUNE_LOW: c_uint = 0x18;
pub const SUNXI_PHY_TUNE_HIGH: c_uint = 0x1c;
pub const SUNXI_PHY_EXTERNAL_CONTROL: c_uint = 0x20;
// USB2.0 Interface Status and Control Register

// PIPE Clock Control Register

// PHY External Control Register

// PHY Tune High Register

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sun50i_usb3_phy {
    pub phy: *mut phy,
    pub regs: *mut void __iomem,
    pub reset: *mut reset_control,
    pub clk: *mut clk,
}

#[no_mangle]
unsafe extern "C" fn sun50i_usb3_phy_open(phy: *mut sun50i_usb3_phy) {
    static void sun50i_usb3_phy_open(struct sun50i_usb3_phy *phy)
    {
    u32 val;
    val = readl(phy.regs + SUNXI_PHY_EXTERNAL_CONTROL);
    val |= SUNXI_PEC_EXTERN_VBUS;
    val |= SUNXI_PEC_SSC_EN | SUNXI_PEC_REF_SSP_EN;
    writel(val, phy.regs + SUNXI_PHY_EXTERNAL_CONTROL);
    val = readl(phy.regs + SUNXI_PIPE_CLOCK_CONTROL);
    val |= SUNXI_PCC_PIPE_CLK_OPEN;
    writel(val, phy.regs + SUNXI_PIPE_CLOCK_CONTROL);
    val = readl(phy.regs + SUNXI_ISCR);
    val |= SUNXI_ISCR_FORCE_VBUS;
    writel(val, phy.regs + SUNXI_ISCR);
//
// All the magic numbers written to the PHY_TUNE_{LOW_HIGH}
// registers are directly taken from the BSP USB3 driver from
// Allwiner.
//
    writel(0x0047fc87, phy.regs + SUNXI_PHY_TUNE_LOW);
    val = readl(phy.regs + SUNXI_PHY_TUNE_HIGH);
    val &= ~(SUNXI_TXVBOOSTLVL_MASK | SUNXI_LOS_BIAS_MASK |
    SUNXI_TX_SWING_FULL_MASK | SUNXI_TX_DEEMPH_6GB_MASK |
    SUNXI_TX_DEEMPH_3P5DB_MASK);
    val |= SUNXI_TXVBOOSTLVL(0x7);
    val |= SUNXI_LOS_BIAS(0x7);
    val |= SUNXI_TX_SWING_FULL(0x55);
    val |= SUNXI_TX_DEEMPH_6DB(0x20);
    val |= SUNXI_TX_DEEMPH_3P5DB(0x15);
    writel(val, phy.regs + SUNXI_PHY_TUNE_HIGH);
    }
#[no_mangle]
unsafe extern "C" fn sun50i_usb3_phy_init(_phy: *mut phy) -> c_int {
    static int sun50i_usb3_phy_init(struct phy *_phy)
    {
    struct sun50i_usb3_phy *phy = phy_get_drvdata(_phy);
    int ret;
    ret = clk_prepare_enable(phy.clk);
    if (ret)
    return ret;
    ret = reset_control_deassert(phy.reset);
    if (ret) {
    clk_disable_unprepare(phy.clk);
    return ret;
    }
    sun50i_usb3_phy_open(phy);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sun50i_usb3_phy_exit(_phy: *mut phy) -> c_int {
    static int sun50i_usb3_phy_exit(struct phy *_phy)
    {
    struct sun50i_usb3_phy *phy = phy_get_drvdata(_phy);
    reset_control_assert(phy.reset);
    clk_disable_unprepare(phy.clk);
    return 0;
    }
    static const struct phy_ops sun50i_usb3_phy_ops = {
    .init		= sun50i_usb3_phy_init,
    .exit		= sun50i_usb3_phy_exit,
    .owner		= THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn sun50i_usb3_phy_probe(pdev: *mut platform_device) -> c_int {
    static int sun50i_usb3_phy_probe(struct platform_device *pdev)
    {
    struct sun50i_usb3_phy *phy;
    struct device *dev = &pdev.dev;
    struct phy_provider *phy_provider;
    phy = devm_kzalloc(dev, sizeof(*phy), GFP_KERNEL);
    if (!phy)
    return -ENOMEM;
    phy.clk = devm_clk_get(dev, core::ptr::null_mut());
    if (IS_ERR(phy.clk)) {
    if (PTR_ERR(phy.clk) != -EPROBE_DEFER)
    dev_err(dev, "failed to get phy clock\n");
    return PTR_ERR(phy.clk);
    }
    phy.reset = devm_reset_control_get(dev, core::ptr::null_mut());
    if (IS_ERR(phy.reset)) {
    dev_err(dev, "failed to get reset control\n");
    return PTR_ERR(phy.reset);
    }
    phy.regs = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(phy.regs))
    return PTR_ERR(phy.regs);
    phy.phy = devm_phy_create(dev, core::ptr::null_mut(), &sun50i_usb3_phy_ops);
    if (IS_ERR(phy.phy)) {
    dev_err(dev, "failed to create PHY\n");
    return PTR_ERR(phy.phy);
    }
    phy_set_drvdata(phy.phy, phy);
    phy_provider = devm_of_phy_provider_register(dev, of_phy_simple_xlate);
    return PTR_ERR_OR_ZERO(phy_provider);
    }
    static const struct of_device_id sun50i_usb3_phy_of_match[] = {
    { .compatible = "allwinner,sun50i-h6-usb3-phy" },
    { },
    };
    MODULE_DEVICE_TABLE(of, sun50i_usb3_phy_of_match);
    static struct platform_driver sun50i_usb3_phy_driver = {
    .probe	= sun50i_usb3_phy_probe,
    .driver = {
    .of_match_table	= sun50i_usb3_phy_of_match,
    .name  = "sun50i-usb3-phy",
    }
    };
    module_platform_driver(sun50i_usb3_phy_driver);
    MODULE_DESCRIPTION("Allwinner H6 USB 3.0 phy driver");
    MODULE_AUTHOR("Icenowy Zheng <icenowy@aosc.io>");
    MODULE_LICENSE("GPL");
