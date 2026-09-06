//! Automatically rewritten from C to Rust
//! Source: drivers/phy/ralink/phy-ralink-usb.c
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
// Copyright (C) 2017 John Crispin <john@phrozen.org>
//
// Based on code from
// Allwinner Technology Co., Ltd. <www.allwinnertech.com>
//

pub const RT_SYSC_REG_SYSCFG1: c_uint = 0x014;
pub const RT_SYSC_REG_CLKCFG1: c_uint = 0x030;
pub const RT_SYSC_REG_USB_PHY_CFG: c_uint = 0x05c;
pub const OFS_U2_PHY_AC0: c_uint = 0x800;
pub const OFS_U2_PHY_AC1: c_uint = 0x804;
pub const OFS_U2_PHY_AC2: c_uint = 0x808;
pub const OFS_U2_PHY_ACR0: c_uint = 0x810;
pub const OFS_U2_PHY_ACR1: c_uint = 0x814;
pub const OFS_U2_PHY_ACR2: c_uint = 0x818;
pub const OFS_U2_PHY_ACR3: c_uint = 0x81C;
pub const OFS_U2_PHY_ACR4: c_uint = 0x820;
pub const OFS_U2_PHY_AMON0: c_uint = 0x824;
pub const OFS_U2_PHY_DCR0: c_uint = 0x860;
pub const OFS_U2_PHY_DCR1: c_uint = 0x864;
pub const OFS_U2_PHY_DTM0: c_uint = 0x868;
pub const OFS_U2_PHY_DTM1: c_uint = 0x86C;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ralink_usb_phy {
    pub rstdev: *mut reset_control,
    pub rsthost: *mut reset_control,
    pub clk: u32,
    pub phy: *mut phy,
    pub base: *mut void __iomem,
    pub sysctl: *mut regmap,
}

#[no_mangle]
unsafe extern "C" fn u2_phy_w32(phy: *mut ralink_usb_phy, val: u32, reg: u32) {
    static void u2_phy_w32(struct ralink_usb_phy *phy, u32 val, u32 reg)
    {
    writel(val, phy.base + reg);
    }
#[no_mangle]
unsafe extern "C" fn u2_phy_r32(phy: *mut ralink_usb_phy, reg: u32) -> u32 {
    static u32 u2_phy_r32(struct ralink_usb_phy *phy, u32 reg)
    {
    return readl(phy.base + reg);
    }
#[no_mangle]
unsafe extern "C" fn ralink_usb_phy_init(phy: *mut ralink_usb_phy) {
    static void ralink_usb_phy_init(struct ralink_usb_phy *phy)
    {
    u2_phy_r32(phy, OFS_U2_PHY_AC2);
    u2_phy_r32(phy, OFS_U2_PHY_ACR0);
    u2_phy_r32(phy, OFS_U2_PHY_DCR0);
    u2_phy_w32(phy, 0x00ffff02, OFS_U2_PHY_DCR0);
    u2_phy_r32(phy, OFS_U2_PHY_DCR0);
    u2_phy_w32(phy, 0x00555502, OFS_U2_PHY_DCR0);
    u2_phy_r32(phy, OFS_U2_PHY_DCR0);
    u2_phy_w32(phy, 0x00aaaa02, OFS_U2_PHY_DCR0);
    u2_phy_r32(phy, OFS_U2_PHY_DCR0);
    u2_phy_w32(phy, 0x00000402, OFS_U2_PHY_DCR0);
    u2_phy_r32(phy, OFS_U2_PHY_DCR0);
    u2_phy_w32(phy, 0x0048086a, OFS_U2_PHY_AC0);
    u2_phy_w32(phy, 0x4400001c, OFS_U2_PHY_AC1);
    u2_phy_w32(phy, 0xc0200000, OFS_U2_PHY_ACR3);
    u2_phy_w32(phy, 0x02000000, OFS_U2_PHY_DTM0);
    }
#[no_mangle]
unsafe extern "C" fn ralink_usb_phy_power_on(_phy: *mut phy) -> c_int {
    static int ralink_usb_phy_power_on(struct phy *_phy)
    {
    struct ralink_usb_phy *phy = phy_get_drvdata(_phy);
    u32 t;
// enable the phy
    regmap_update_bits(phy.sysctl, RT_SYSC_REG_CLKCFG1,
    phy.clk, phy.clk);
// setup host mode
    regmap_update_bits(phy.sysctl, RT_SYSC_REG_SYSCFG1,
    RT_SYSCFG1_USB0_HOST_MODE,
    RT_SYSCFG1_USB0_HOST_MODE);
// deassert the reset lines
    reset_control_deassert(phy.rsthost);
    reset_control_deassert(phy.rstdev);
//
// The SDK kernel had a delay of 100ms. however on device
// testing showed that 10ms is enough
//
    mdelay(10);
    if (phy.base)
    ralink_usb_phy_init(phy);
// print some status info
    regmap_read(phy.sysctl, RT_SYSC_REG_USB_PHY_CFG, &t);
    dev_info(&phy.phy.dev, "remote usb device wakeup %s\n",
    (t & UDEV_WAKEUP) ? ("enabled") : ("disabled"));
    if (t & USB_PHY_UTMI_8B60M)
    dev_info(&phy.phy.dev, "UTMI 8bit 60MHz\n");
    else
    dev_info(&phy.phy.dev, "UTMI 16bit 30MHz\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ralink_usb_phy_power_off(_phy: *mut phy) -> c_int {
    static int ralink_usb_phy_power_off(struct phy *_phy)
    {
    struct ralink_usb_phy *phy = phy_get_drvdata(_phy);
// disable the phy
    regmap_update_bits(phy.sysctl, RT_SYSC_REG_CLKCFG1,
    phy.clk, 0);
// assert the reset lines
    reset_control_assert(phy.rstdev);
    reset_control_assert(phy.rsthost);
    return 0;
    }
    static const struct phy_ops ralink_usb_phy_ops = {
    .power_on	= ralink_usb_phy_power_on,
    .power_off	= ralink_usb_phy_power_off,
    .owner		= THIS_MODULE,
    };
    static const struct of_device_id ralink_usb_phy_of_match[] = {
    {
    .compatible = "ralink,rt3352-usbphy",
    .data = (void *)(uintptr_t)(RT_CLKCFG1_UPHY1_CLK_EN |
    RT_CLKCFG1_UPHY0_CLK_EN)
    },
    {
    .compatible = "mediatek,mt7620-usbphy",
    .data = (void *)(uintptr_t)(MT7620_CLKCFG1_UPHY1_CLK_EN |
    MT7620_CLKCFG1_UPHY0_CLK_EN)
    },
    {
    .compatible = "mediatek,mt7628-usbphy",
    .data = (void *)(uintptr_t)(MT7620_CLKCFG1_UPHY1_CLK_EN |
    MT7620_CLKCFG1_UPHY0_CLK_EN) },
    { },
    };
    MODULE_DEVICE_TABLE(of, ralink_usb_phy_of_match);
#[no_mangle]
unsafe extern "C" fn ralink_usb_phy_probe(pdev: *mut platform_device) -> c_int {
    static int ralink_usb_phy_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct phy_provider *phy_provider;
    struct ralink_usb_phy *phy;
    phy = devm_kzalloc(dev, sizeof(*phy), GFP_KERNEL);
    if (!phy)
    return -ENOMEM;
    phy.clk = (uintptr_t)device_get_match_data(&pdev.dev);
    phy.base = core::ptr::null_mut();
    phy.sysctl = syscon_regmap_lookup_by_phandle(dev.of_node, "ralink,sysctl");
    if (IS_ERR(phy.sysctl)) {
    dev_err(dev, "failed to get sysctl registers\n");
    return PTR_ERR(phy.sysctl);
    }
// The MT7628 and MT7688 require extra setup of PHY registers.
    if (of_device_is_compatible(dev.of_node, "mediatek,mt7628-usbphy")) {
    phy.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(phy.base)) {
    dev_err(dev, "failed to remap register memory\n");
    return PTR_ERR(phy.base);
    }
    }
    phy.rsthost = devm_reset_control_get(&pdev.dev, "host");
    if (IS_ERR(phy.rsthost)) {
    dev_err(dev, "host reset is missing\n");
    return PTR_ERR(phy.rsthost);
    }
    phy.rstdev = devm_reset_control_get(&pdev.dev, "device");
    if (IS_ERR(phy.rstdev)) {
    dev_err(dev, "device reset is missing\n");
    return PTR_ERR(phy.rstdev);
    }
    phy.phy = devm_phy_create(dev, core::ptr::null_mut(), &ralink_usb_phy_ops);
    if (IS_ERR(phy.phy)) {
    dev_err(dev, "failed to create PHY\n");
    return PTR_ERR(phy.phy);
    }
    phy_set_drvdata(phy.phy, phy);
    phy_provider = devm_of_phy_provider_register(dev, of_phy_simple_xlate);
    return PTR_ERR_OR_ZERO(phy_provider);
    }
    static struct platform_driver ralink_usb_phy_driver = {
    .probe	= ralink_usb_phy_probe,
    .driver = {
    .of_match_table	= ralink_usb_phy_of_match,
    .name  = "ralink-usb-phy",
    }
    };
    module_platform_driver(ralink_usb_phy_driver);
    MODULE_DESCRIPTION("Ralink USB phy driver");
    MODULE_AUTHOR("John Crispin <john@phrozen.org>");
    MODULE_LICENSE("GPL v2");
