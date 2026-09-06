//! Automatically rewritten from C to Rust
//! Source: drivers/phy/allwinner/phy-sun9i-usb.c
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
// Allwinner sun9i USB phy driver
//
// Copyright (C) 2014-2015 Chen-Yu Tsai <wens@csie.org>
//
// Based on phy-sun4i-usb.c from
// Hans de Goede <hdegoede@redhat.com>
//
// and code from
// Allwinner Technology Co., Ltd. <www.allwinnertech.com>
//

// usb1 HSIC specific bits

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sun9i_usb_phy {
    pub phy: *mut phy,
    pub pmu: *mut void __iomem,
    pub reset: *mut reset_control,
    pub clk: *mut clk,
    pub hsic_clk: *mut clk,
    pub type: enum usb_phy_interface,
}

#[no_mangle]
unsafe extern "C" fn sun9i_usb_phy_passby(phy: *mut sun9i_usb_phy, enable: c_int) {
    static void sun9i_usb_phy_passby(struct sun9i_usb_phy *phy, int enable)
    {
    u32 bits, reg_value;
    bits = SUNXI_AHB_INCR16_BURST_EN | SUNXI_AHB_INCR8_BURST_EN |
    SUNXI_AHB_INCR4_BURST_EN | SUNXI_AHB_INCRX_ALIGN_EN |
    SUNXI_ULPI_BYPASS_EN;
    if (phy.type == USBPHY_INTERFACE_MODE_HSIC)
    bits |= SUNXI_HSIC | SUNXI_EHCI_HS_FORCE |
    SUNXI_HSIC_CONNECT_DET | SUNXI_HSIC_CONNECT_INT;
    reg_value = readl(phy.pmu);
    if (enable)
    reg_value |= bits;
    else
    reg_value &= ~bits;
    writel(reg_value, phy.pmu);
    }
#[no_mangle]
unsafe extern "C" fn sun9i_usb_phy_init(_phy: *mut phy) -> c_int {
    static int sun9i_usb_phy_init(struct phy *_phy)
    {
    struct sun9i_usb_phy *phy = phy_get_drvdata(_phy);
    int ret;
    ret = clk_prepare_enable(phy.clk);
    if (ret)
    goto err_clk;
    ret = clk_prepare_enable(phy.hsic_clk);
    if (ret)
    goto err_hsic_clk;
    ret = reset_control_deassert(phy.reset);
    if (ret)
    goto err_reset;
    sun9i_usb_phy_passby(phy, 1);
    return 0;
    err_reset:
    clk_disable_unprepare(phy.hsic_clk);
    err_hsic_clk:
    clk_disable_unprepare(phy.clk);
    err_clk:
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn sun9i_usb_phy_exit(_phy: *mut phy) -> c_int {
    static int sun9i_usb_phy_exit(struct phy *_phy)
    {
    struct sun9i_usb_phy *phy = phy_get_drvdata(_phy);
    sun9i_usb_phy_passby(phy, 0);
    reset_control_assert(phy.reset);
    clk_disable_unprepare(phy.hsic_clk);
    clk_disable_unprepare(phy.clk);
    return 0;
    }
    static const struct phy_ops sun9i_usb_phy_ops = {
    .init		= sun9i_usb_phy_init,
    .exit		= sun9i_usb_phy_exit,
    .owner		= THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn sun9i_usb_phy_probe(pdev: *mut platform_device) -> c_int {
    static int sun9i_usb_phy_probe(struct platform_device *pdev)
    {
    struct sun9i_usb_phy *phy;
    struct device *dev = &pdev.dev;
    struct device_node *np = dev.of_node;
    struct phy_provider *phy_provider;
    phy = devm_kzalloc(dev, sizeof(*phy), GFP_KERNEL);
    if (!phy)
    return -ENOMEM;
    phy.type = of_usb_get_phy_mode(np);
    if (phy.type == USBPHY_INTERFACE_MODE_HSIC) {
    phy.clk = devm_clk_get(dev, "hsic_480M");
    if (IS_ERR(phy.clk)) {
    dev_err(dev, "failed to get hsic_480M clock\n");
    return PTR_ERR(phy.clk);
    }
    phy.hsic_clk = devm_clk_get(dev, "hsic_12M");
    if (IS_ERR(phy.hsic_clk)) {
    dev_err(dev, "failed to get hsic_12M clock\n");
    return PTR_ERR(phy.hsic_clk);
    }
    phy.reset = devm_reset_control_get(dev, "hsic");
    if (IS_ERR(phy.reset)) {
    dev_err(dev, "failed to get reset control\n");
    return PTR_ERR(phy.reset);
    }
    } else {
    phy.clk = devm_clk_get(dev, "phy");
    if (IS_ERR(phy.clk)) {
    dev_err(dev, "failed to get phy clock\n");
    return PTR_ERR(phy.clk);
    }
    phy.reset = devm_reset_control_get(dev, "phy");
    if (IS_ERR(phy.reset)) {
    dev_err(dev, "failed to get reset control\n");
    return PTR_ERR(phy.reset);
    }
    }
    phy.pmu = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(phy.pmu))
    return PTR_ERR(phy.pmu);
    phy.phy = devm_phy_create(dev, core::ptr::null_mut(), &sun9i_usb_phy_ops);
    if (IS_ERR(phy.phy)) {
    dev_err(dev, "failed to create PHY\n");
    return PTR_ERR(phy.phy);
    }
    phy_set_drvdata(phy.phy, phy);
    phy_provider = devm_of_phy_provider_register(dev, of_phy_simple_xlate);
    return PTR_ERR_OR_ZERO(phy_provider);
    }
    static const struct of_device_id sun9i_usb_phy_of_match[] = {
    { .compatible = "allwinner,sun9i-a80-usb-phy" },
    { },
    };
    MODULE_DEVICE_TABLE(of, sun9i_usb_phy_of_match);
    static struct platform_driver sun9i_usb_phy_driver = {
    .probe	= sun9i_usb_phy_probe,
    .driver = {
    .of_match_table	= sun9i_usb_phy_of_match,
    .name  = "sun9i-usb-phy",
    }
    };
    module_platform_driver(sun9i_usb_phy_driver);
    MODULE_DESCRIPTION("Allwinner sun9i USB phy driver");
    MODULE_AUTHOR("Chen-Yu Tsai <wens@csie.org>");
    MODULE_LICENSE("GPL");
