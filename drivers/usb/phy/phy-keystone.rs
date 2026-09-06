//! Automatically rewritten from C to Rust
//! Source: drivers/usb/phy/phy-keystone.c
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
// phy-keystone - USB PHY, talking to dwc3 controller in Keystone.
//
// Copyright (C) 2013 Texas Instruments Incorporated - https://www.ti.com
//
// Author: WingMan Kwok <w-kwok2@ti.com>
//

// USB PHY control register offsets
pub const USB_PHY_CTL_UTMI: c_uint = 0x0000;
pub const USB_PHY_CTL_PIPE: c_uint = 0x0004;
pub const USB_PHY_CTL_PARAM_1: c_uint = 0x0008;
pub const USB_PHY_CTL_PARAM_2: c_uint = 0x000c;
pub const USB_PHY_CTL_CLOCK: c_uint = 0x0010;
pub const USB_PHY_CTL_PLL: c_uint = 0x0014;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct keystone_usbphy {
    pub usb_phy_gen: usb_phy_generic,
    pub phy_ctrl: *mut void __iomem,
}

#[no_mangle]
pub unsafe extern "C" fn keystone_usbphy_readl(base: *mut void __iomem, offset: u32) -> u32 {
    static inline u32 keystone_usbphy_readl(void __iomem *base, u32 offset)
    {
    return readl(base + offset);
    }
    static inline void keystone_usbphy_writel(void __iomem *base,
    u32 offset, u32 value)
    {
    writel(value, base + offset);
    }
#[no_mangle]
unsafe extern "C" fn keystone_usbphy_init(phy: *mut usb_phy) -> c_int {
    static int keystone_usbphy_init(struct usb_phy *phy)
    {
    struct keystone_usbphy *k_phy = dev_get_drvdata(phy.dev);
    u32 val;
    val  = keystone_usbphy_readl(k_phy.phy_ctrl, USB_PHY_CTL_CLOCK);
    keystone_usbphy_writel(k_phy.phy_ctrl, USB_PHY_CTL_CLOCK,
    val | PHY_REF_SSP_EN);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn keystone_usbphy_shutdown(phy: *mut usb_phy) {
    static void keystone_usbphy_shutdown(struct usb_phy *phy)
    {
    struct keystone_usbphy *k_phy = dev_get_drvdata(phy.dev);
    u32 val;
    val  = keystone_usbphy_readl(k_phy.phy_ctrl, USB_PHY_CTL_CLOCK);
    keystone_usbphy_writel(k_phy.phy_ctrl, USB_PHY_CTL_CLOCK,
    val & ~PHY_REF_SSP_EN);
    }
#[no_mangle]
unsafe extern "C" fn keystone_usbphy_probe(pdev: *mut platform_device) -> c_int {
    static int keystone_usbphy_probe(struct platform_device *pdev)
    {
    struct device		*dev = &pdev.dev;
    struct keystone_usbphy	*k_phy;
    int ret;
    k_phy = devm_kzalloc(dev, sizeof(*k_phy), GFP_KERNEL);
    if (!k_phy)
    return -ENOMEM;
    k_phy.phy_ctrl = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(k_phy.phy_ctrl))
    return PTR_ERR(k_phy.phy_ctrl);
    ret = usb_phy_gen_create_phy(dev, &k_phy.usb_phy_gen);
    if (ret)
    return ret;
    k_phy.usb_phy_gen.phy.init = keystone_usbphy_init;
    k_phy.usb_phy_gen.phy.shutdown = keystone_usbphy_shutdown;
    platform_set_drvdata(pdev, k_phy);
    return usb_add_phy_dev(&k_phy.usb_phy_gen.phy);
    }
#[no_mangle]
unsafe extern "C" fn keystone_usbphy_remove(pdev: *mut platform_device) {
    static void keystone_usbphy_remove(struct platform_device *pdev)
    {
    struct keystone_usbphy *k_phy = platform_get_drvdata(pdev);
    usb_remove_phy(&k_phy.usb_phy_gen.phy);
    }
    static const struct of_device_id keystone_usbphy_ids[] = {
    { .compatible = "ti,keystone-usbphy" },
    { }
    };
    MODULE_DEVICE_TABLE(of, keystone_usbphy_ids);
    static struct platform_driver keystone_usbphy_driver = {
    .probe          = keystone_usbphy_probe,
    .remove         = keystone_usbphy_remove,
    .driver         = {
    .name   = "keystone-usbphy",
    .of_match_table = keystone_usbphy_ids,
    },
    };
    module_platform_driver(keystone_usbphy_driver);
    MODULE_ALIAS("platform:keystone-usbphy");
    MODULE_AUTHOR("Texas Instruments Inc.");
    MODULE_DESCRIPTION("Keystone USB phy driver");
    MODULE_LICENSE("GPL v2");
