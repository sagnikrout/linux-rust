//! Automatically rewritten from C to Rust
//! Source: drivers/phy/marvell/phy-mvebu-a3700-utmi.c
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
// Copyright (C) 2018 Marvell
//
// Authors:
// Igal Liberman <igall@marvell.com>
// Miquèl Raynal <miquel.raynal@bootlin.com>
//
// Marvell A3700 UTMI PHY driver
//

// Armada 3700 UTMI PHY registers
pub const USB2_PHY_PLL_CTRL_REG0: c_uint = 0x0;
pub const PLL_REF_DIV_OFF: c_int = 0;

pub const PLL_REF_DIV_5: c_int = 5;
pub const PLL_FB_DIV_OFF: c_int = 16;

pub const PLL_FB_DIV_96: c_int = 96;
pub const PLL_SEL_LPFR_OFF: c_int = 28;

pub const USB2_PHY_CAL_CTRL: c_uint = 0x8;

pub const USB2_RX_CHAN_CTRL1: c_uint = 0x18;

pub const USB2_PHY_OTG_CTRL: c_uint = 0x34;

pub const USB2_PHY_CHRGR_DETECT: c_uint = 0x38;

// Armada 3700 USB miscellaneous registers

pub const PLL_LOCK_DELAY_US: c_int = 10000;
pub const PLL_LOCK_TIMEOUT_US: c_int = 1000000;
//
// struct mvebu_a3700_utmi_caps - PHY capabilities
//
// @usb32: Flag indicating which PHY is in use (impacts the register map):
// - The UTMI PHY wired to the USB3/USB2 controller (otg)
// - The UTMI PHY wired to the USB2 controller (host only)
// @ops: PHY operations
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mvebu_a3700_utmi_caps {
    pub usb32: c_int,
    pub ops: *const phy_ops,
}

//
// struct mvebu_a3700_utmi - PHY driver data
//
// @regs: PHY registers
// @usb_misc: Regmap with USB miscellaneous registers including PHY ones
// @caps: PHY capabilities
// @phy: PHY handle
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mvebu_a3700_utmi {
    pub regs: *mut void __iomem,
    pub usb_misc: *mut regmap,
    pub caps: *const mvebu_a3700_utmi_caps,
    pub phy: *mut phy,
}

#[no_mangle]
unsafe extern "C" fn mvebu_a3700_utmi_phy_power_on(phy: *mut phy) -> c_int {
    static int mvebu_a3700_utmi_phy_power_on(struct phy *phy)
    {
    struct mvebu_a3700_utmi *utmi = phy_get_drvdata(phy);
    struct device *dev = &phy.dev;
    let mut usb32: c_int = utmi.caps.usb32;
    let mut ret: c_int = 0;
    u32 reg;
//
// Setup PLL. 40MHz clock used to be the default, being 25MHz now.
// See "PLL Settings for Typical REFCLK" table.
//
    reg = readl(utmi.regs + USB2_PHY_PLL_CTRL_REG0);
    reg &= ~(PLL_REF_DIV_MASK | PLL_FB_DIV_MASK | PLL_SEL_LPFR_MASK);
    reg |= (PLL_REF_DIV_5 << PLL_REF_DIV_OFF) |
    (PLL_FB_DIV_96 << PLL_FB_DIV_OFF);
    writel(reg, utmi.regs + USB2_PHY_PLL_CTRL_REG0);
// Enable PHY pull up and disable USB2 suspend
    regmap_update_bits(utmi.usb_misc, USB2_PHY_CTRL(usb32),
    RB_USB2PHY_SUSPM(usb32) | RB_USB2PHY_PU,
    RB_USB2PHY_SUSPM(usb32) | RB_USB2PHY_PU);
    if (usb32) {
// Power up OTG module
    reg = readl(utmi.regs + USB2_PHY_OTG_CTRL);
    reg |= PHY_PU_OTG;
    writel(reg, utmi.regs + USB2_PHY_OTG_CTRL);
// Disable PHY charger detection
    reg = readl(utmi.regs + USB2_PHY_CHRGR_DETECT);
    reg &= ~(PHY_CDP_EN | PHY_DCP_EN | PHY_PD_EN | PHY_PU_CHRG_DTC |
    PHY_CDP_DM_AUTO | PHY_ENSWITCH_DP | PHY_ENSWITCH_DM);
    writel(reg, utmi.regs + USB2_PHY_CHRGR_DETECT);
// Disable PHY DP/DM pull-down (used for device mode)
    regmap_update_bits(utmi.usb_misc, USB2_PHY_CTRL(usb32),
    USB2_DP_PULLDN_DEV_MODE |
    USB2_DM_PULLDN_DEV_MODE, 0);
    }
// Wait for PLL calibration
    ret = readl_poll_timeout(utmi.regs + USB2_PHY_CAL_CTRL, reg,
    reg & PHY_PLLCAL_DONE,
    PLL_LOCK_DELAY_US, PLL_LOCK_TIMEOUT_US);
    if (ret) {
    dev_err(dev, "Failed to end USB2 PLL calibration\n");
    return ret;
    }
// Wait for impedance calibration
    ret = readl_poll_timeout(utmi.regs + USB2_PHY_CAL_CTRL, reg,
    reg & PHY_IMPCAL_DONE,
    PLL_LOCK_DELAY_US, PLL_LOCK_TIMEOUT_US);
    if (ret) {
    dev_err(dev, "Failed to end USB2 impedance calibration\n");
    return ret;
    }
// Wait for squelch calibration
    ret = readl_poll_timeout(utmi.regs + USB2_RX_CHAN_CTRL1, reg,
    reg & USB2PHY_SQCAL_DONE,
    PLL_LOCK_DELAY_US, PLL_LOCK_TIMEOUT_US);
    if (ret) {
    dev_err(dev, "Failed to end USB2 unknown calibration\n");
    return ret;
    }
// Wait for PLL to be locked
    ret = readl_poll_timeout(utmi.regs + USB2_PHY_PLL_CTRL_REG0, reg,
    reg & PLL_READY,
    PLL_LOCK_DELAY_US, PLL_LOCK_TIMEOUT_US);
    if (ret)
    dev_err(dev, "Failed to lock USB2 PLL\n");
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn mvebu_a3700_utmi_phy_power_off(phy: *mut phy) -> c_int {
    static int mvebu_a3700_utmi_phy_power_off(struct phy *phy)
    {
    struct mvebu_a3700_utmi *utmi = phy_get_drvdata(phy);
    let mut usb32: c_int = utmi.caps.usb32;
    u32 reg;
// Disable PHY pull-up and enable USB2 suspend
    regmap_update_bits(utmi.usb_misc, USB2_PHY_CTRL(usb32),
    RB_USB2PHY_PU | RB_USB2PHY_SUSPM(usb32), 0);
// Power down OTG module
    if (usb32) {
    reg = readl(utmi.regs + USB2_PHY_OTG_CTRL);
    reg &= ~PHY_PU_OTG;
    writel(reg, utmi.regs + USB2_PHY_OTG_CTRL);
    }
    return 0;
    }
    static const struct phy_ops mvebu_a3700_utmi_phy_ops = {
    .power_on = mvebu_a3700_utmi_phy_power_on,
    .power_off = mvebu_a3700_utmi_phy_power_off,
    .owner = THIS_MODULE,
    };
    static const struct mvebu_a3700_utmi_caps mvebu_a3700_utmi_otg_phy_caps = {
    .usb32 = true,
    .ops = &mvebu_a3700_utmi_phy_ops,
    };
    static const struct mvebu_a3700_utmi_caps mvebu_a3700_utmi_host_phy_caps = {
    .usb32 = false,
    .ops = &mvebu_a3700_utmi_phy_ops,
    };
    static const struct of_device_id mvebu_a3700_utmi_of_match[] = {
    {
    .compatible = "marvell,a3700-utmi-otg-phy",
    .data = &mvebu_a3700_utmi_otg_phy_caps,
    },
    {
    .compatible = "marvell,a3700-utmi-host-phy",
    .data = &mvebu_a3700_utmi_host_phy_caps,
    },
    {},
    };
    MODULE_DEVICE_TABLE(of, mvebu_a3700_utmi_of_match);
#[no_mangle]
unsafe extern "C" fn mvebu_a3700_utmi_phy_probe(pdev: *mut platform_device) -> c_int {
    static int mvebu_a3700_utmi_phy_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct mvebu_a3700_utmi *utmi;
    struct phy_provider *provider;
    utmi = devm_kzalloc(dev, sizeof(*utmi), GFP_KERNEL);
    if (!utmi)
    return -ENOMEM;
// Get UTMI memory region
    utmi.regs = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(utmi.regs))
    return PTR_ERR(utmi.regs);
// Get miscellaneous Host/PHY region
    utmi.usb_misc = syscon_regmap_lookup_by_phandle(dev.of_node,
    "marvell,usb-misc-reg");
    if (IS_ERR(utmi.usb_misc)) {
    dev_err(dev,
    "Missing USB misc purpose system controller\n");
    return PTR_ERR(utmi.usb_misc);
    }
// Retrieve PHY capabilities
    utmi.caps = of_device_get_match_data(dev);
// Instantiate the PHY
    utmi.phy = devm_phy_create(dev, core::ptr::null_mut(), utmi.caps.ops);
    if (IS_ERR(utmi.phy)) {
    dev_err(dev, "Failed to create the UTMI PHY\n");
    return PTR_ERR(utmi.phy);
    }
    phy_set_drvdata(utmi.phy, utmi);
// Ensure the PHY is powered off
    utmi.caps.ops.power_off(utmi.phy);
    provider = devm_of_phy_provider_register(dev, of_phy_simple_xlate);
    return PTR_ERR_OR_ZERO(provider);
    }
    static struct platform_driver mvebu_a3700_utmi_driver = {
    .probe	= mvebu_a3700_utmi_phy_probe,
    .driver	= {
    .name		= "mvebu-a3700-utmi-phy",
    .of_match_table	= mvebu_a3700_utmi_of_match,
    },
    };
    module_platform_driver(mvebu_a3700_utmi_driver);
    MODULE_AUTHOR("Igal Liberman <igall@marvell.com>");
    MODULE_AUTHOR("Miquel Raynal <miquel.raynal@bootlin.com>");
    MODULE_DESCRIPTION("Marvell EBU A3700 UTMI PHY driver");
    MODULE_LICENSE("GPL v2");
