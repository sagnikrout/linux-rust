//! Automatically rewritten from C to Rust
//! Source: drivers/phy/broadcom/phy-bcm-kona-usb2.c
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
// phy-bcm-kona-usb2.c - Broadcom Kona USB2 Phy Driver
//
// Copyright (C) 2013 Linaro Limited
// Matt Porter <mporter@linaro.org>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm_kona_usb {
    pub regs: *mut void __iomem,
}

#[no_mangle]
unsafe extern "C" fn bcm_kona_usb_phy_power(phy: *mut bcm_kona_usb, on: c_int) {
    static void bcm_kona_usb_phy_power(struct bcm_kona_usb *phy, int on)
    {
    u32 val;
    val = readl(phy.regs + OTGCTL);
    if (on) {
// Configure and power PHY
    val &= ~(OTGCTL_OTGSTAT2 | OTGCTL_OTGSTAT1 |
    OTGCTL_UTMI_LINE_STATE1 | OTGCTL_UTMI_LINE_STATE0);
    val |= OTGCTL_PRST_N_SW | OTGCTL_HRESET_N;
    } else {
    val &= ~(OTGCTL_PRST_N_SW | OTGCTL_HRESET_N);
    }
    writel(val, phy.regs + OTGCTL);
    }
#[no_mangle]
unsafe extern "C" fn bcm_kona_usb_phy_init(gphy: *mut phy) -> c_int {
    static int bcm_kona_usb_phy_init(struct phy *gphy)
    {
    struct bcm_kona_usb *phy = phy_get_drvdata(gphy);
    u32 val;
// Soft reset PHY
    val = readl(phy.regs + P1CTL);
    val &= ~P1CTL_NON_DRIVING;
    val |= P1CTL_SOFT_RESET;
    writel(val, phy.regs + P1CTL);
    writel(val & ~P1CTL_SOFT_RESET, phy.regs + P1CTL);
// Reset needs to be asserted for 2ms
    mdelay(2);
    writel(val | P1CTL_SOFT_RESET, phy.regs + P1CTL);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bcm_kona_usb_phy_power_on(gphy: *mut phy) -> c_int {
    static int bcm_kona_usb_phy_power_on(struct phy *gphy)
    {
    struct bcm_kona_usb *phy = phy_get_drvdata(gphy);
    bcm_kona_usb_phy_power(phy, 1);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bcm_kona_usb_phy_power_off(gphy: *mut phy) -> c_int {
    static int bcm_kona_usb_phy_power_off(struct phy *gphy)
    {
    struct bcm_kona_usb *phy = phy_get_drvdata(gphy);
    bcm_kona_usb_phy_power(phy, 0);
    return 0;
    }
    static const struct phy_ops ops = {
    .init		= bcm_kona_usb_phy_init,
    .power_on	= bcm_kona_usb_phy_power_on,
    .power_off	= bcm_kona_usb_phy_power_off,
    .owner		= THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn bcm_kona_usb2_probe(pdev: *mut platform_device) -> c_int {
    static int bcm_kona_usb2_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct bcm_kona_usb *phy;
    struct phy *gphy;
    struct phy_provider *phy_provider;
    phy = devm_kzalloc(dev, sizeof(*phy), GFP_KERNEL);
    if (!phy)
    return -ENOMEM;
    phy.regs = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(phy.regs))
    return PTR_ERR(phy.regs);
    platform_set_drvdata(pdev, phy);
    gphy = devm_phy_create(dev, core::ptr::null_mut(), &ops);
    if (IS_ERR(gphy))
    return PTR_ERR(gphy);
// The Kona PHY supports an 8-bit wide UTMI interface
    phy_set_bus_width(gphy, 8);
    phy_set_drvdata(gphy, phy);
    phy_provider = devm_of_phy_provider_register(dev,
    of_phy_simple_xlate);
    return PTR_ERR_OR_ZERO(phy_provider);
    }
    static const struct of_device_id bcm_kona_usb2_dt_ids[] = {
    { .compatible = "brcm,kona-usb2-phy" },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, bcm_kona_usb2_dt_ids);
    static struct platform_driver bcm_kona_usb2_driver = {
    .probe		= bcm_kona_usb2_probe,
    .driver		= {
    .name	= "bcm-kona-usb2",
    .of_match_table = bcm_kona_usb2_dt_ids,
    },
    };
    module_platform_driver(bcm_kona_usb2_driver);
    MODULE_ALIAS("platform:bcm-kona-usb2");
    MODULE_AUTHOR("Matt Porter <mporter@linaro.org>");
    MODULE_DESCRIPTION("BCM Kona USB 2.0 PHY driver");
    MODULE_LICENSE("GPL v2");
