//! Automatically rewritten from C to Rust
//! Source: drivers/phy/sophgo/phy-cv1800-usb2.c
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
// Copyright (C) 2025 Inochi Amaoto <inochiama@outlook.com>
//

pub const REG_USB_PHY_CTRL: c_uint = 0x048;

pub const PHY_APP_CLK_RATE: c_int = 125000000;
pub const PHY_LPM_CLK_RATE: c_int = 12000000;
pub const PHY_STB_CLK_RATE: c_int = 333334;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cv1800_usb_phy {
    pub phy: *mut phy,
    pub syscon: *mut regmap,
    pub lock: spinlock_t,
    pub usb_app_clk: *mut clk,
    pub usb_lpm_clk: *mut clk,
    pub usb_stb_clk: *mut clk,
    pub support_otg: bool,
}

    static int cv1800_usb_phy_set_mode(struct phy *_phy,
    enum phy_mode mode, int submode)
    {
    struct cv1800_usb_phy *phy = phy_get_drvdata(_phy);
    let mut regval: c_uint = 0;
    int ret;
    dev_info(&phy.phy.dev, "set mode %d", (int)mode);
    switch (mode) {
    case PHY_MODE_USB_DEVICE:
    regval = PHY_ID_OVERWRITE_EN | PHY_ID_OVERWRITE_MODE_DEVICE;
    regmap_clear_bits(phy.syscon, REG_USB_PHY_CTRL, PHY_VBUS_POWER);
    break;
    case PHY_MODE_USB_HOST:
    regval = PHY_ID_OVERWRITE_EN | PHY_ID_OVERWRITE_MODE_HOST;
    regmap_set_bits(phy.syscon, REG_USB_PHY_CTRL, PHY_VBUS_POWER);
    break;
    case PHY_MODE_USB_OTG:
    if (!phy.support_otg)
    return 0;
    ret = regmap_read(phy.syscon, REG_USB_PHY_CTRL, &regval);
    if (ret)
    return ret;
    regval = FIELD_GET(PHY_ID_OVERWRITE_MODE, regval);
    break;
    default:
    return -EINVAL;
    }
    return regmap_update_bits(phy.syscon, REG_USB_PHY_CTRL,
    PHY_ID_OVERWRITE_EN | PHY_ID_OVERWRITE_MODE,
    regval);
    }
#[no_mangle]
unsafe extern "C" fn cv1800_usb_phy_set_clock(phy: *mut cv1800_usb_phy) -> c_int {
    static int cv1800_usb_phy_set_clock(struct cv1800_usb_phy *phy)
    {
    int ret;
    ret = clk_set_rate(phy.usb_app_clk, PHY_APP_CLK_RATE);
    if (ret)
    return ret;
    ret = clk_set_rate(phy.usb_lpm_clk, PHY_LPM_CLK_RATE);
    if (ret)
    return ret;
    return clk_set_rate(phy.usb_stb_clk, PHY_STB_CLK_RATE);
    }
    static const struct phy_ops cv1800_usb_phy_ops = {
    .set_mode	= cv1800_usb_phy_set_mode,
    .owner		= THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn cv1800_usb_phy_probe(pdev: *mut platform_device) -> c_int {
    static int cv1800_usb_phy_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct device *parent = dev.parent;
    struct cv1800_usb_phy *phy;
    struct phy_provider *phy_provider;
    int ret;
    if (!parent)
    return -ENODEV;
    phy = devm_kmalloc(dev, sizeof(*phy), GFP_KERNEL);
    if (!phy)
    return -ENOMEM;
    phy.syscon = syscon_node_to_regmap(parent.of_node);
    if (IS_ERR_OR_NULL(phy.syscon))
    return -ENODEV;
    phy.support_otg = false;
    spin_lock_init(&phy.lock);
    phy.usb_app_clk = devm_clk_get_enabled(dev, "app");
    if (IS_ERR(phy.usb_app_clk))
    return dev_err_probe(dev, PTR_ERR(phy.usb_app_clk),
    "Failed to get app clock\n");
    phy.usb_lpm_clk = devm_clk_get_enabled(dev, "lpm");
    if (IS_ERR(phy.usb_lpm_clk))
    return dev_err_probe(dev, PTR_ERR(phy.usb_lpm_clk),
    "Failed to get lpm clock\n");
    phy.usb_stb_clk = devm_clk_get_enabled(dev, "stb");
    if (IS_ERR(phy.usb_stb_clk))
    return dev_err_probe(dev, PTR_ERR(phy.usb_stb_clk),
    "Failed to get stb clock\n");
    phy.phy = devm_phy_create(dev, core::ptr::null_mut(), &cv1800_usb_phy_ops);
    if (IS_ERR(phy.phy))
    return dev_err_probe(dev, PTR_ERR(phy.phy),
    "Failed to create phy\n");
    ret = cv1800_usb_phy_set_clock(phy);
    if (ret)
    return ret;
    phy_set_drvdata(phy.phy, phy);
    phy_provider = devm_of_phy_provider_register(dev, of_phy_simple_xlate);
    return PTR_ERR_OR_ZERO(phy_provider);
    }
    static const struct of_device_id cv1800_usb_phy_ids[] = {
    { .compatible = "sophgo,cv1800b-usb2-phy" },
    { },
    };
    MODULE_DEVICE_TABLE(of, cv1800_usb_phy_ids);
    static struct platform_driver cv1800_usb_phy_driver = {
    .probe = cv1800_usb_phy_probe,
    .driver = {
    .name = "cv1800-usb2-phy",
    .of_match_table = cv1800_usb_phy_ids,
    },
    };
    module_platform_driver(cv1800_usb_phy_driver);
    MODULE_AUTHOR("Inochi Amaoto <inochiama@outlook.com>");
    MODULE_DESCRIPTION("CV1800/SG2000 SoC USB 2.0 PHY driver");
    MODULE_LICENSE("GPL");
