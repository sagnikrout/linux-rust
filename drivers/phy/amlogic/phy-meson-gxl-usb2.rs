//! Automatically rewritten from C to Rust
//! Source: drivers/phy/amlogic/phy-meson-gxl-usb2.c
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
// Meson GXL and GXM USB2 PHY driver
//
// Copyright (C) 2017 Martin Blumenstingl <martin.blumenstingl@googlemail.com>
//

// bits [31:27] are read-only
pub const U2P_R0: c_uint = 0x0;

pub const U2P_R1: c_uint = 0x4;

// bits [31:14] are read-only
pub const U2P_R2: c_uint = 0x8;

pub const U2P_R3: c_uint = 0xc;
pub const RESET_COMPLETE_TIME: c_int = 500;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct phy_meson_gxl_usb2_priv {
    pub regmap: *mut regmap,
    pub mode: enum phy_mode,
    pub is_enabled: c_int,
    pub clk: *mut clk,
    pub reset: *mut reset_control,
}

    static const struct regmap_config phy_meson_gxl_usb2_regmap_conf = {
    .reg_bits = 8,
    .val_bits = 32,
    .reg_stride = 4,
    .max_register = U2P_R3,
    };
#[no_mangle]
unsafe extern "C" fn phy_meson_gxl_usb2_init(phy: *mut phy) -> c_int {
    static int phy_meson_gxl_usb2_init(struct phy *phy)
    {
    struct phy_meson_gxl_usb2_priv *priv = phy_get_drvdata(phy);
    int ret;
    ret = reset_control_reset(priv.reset);
    if (ret)
    return ret;
    ret = clk_prepare_enable(priv.clk);
    if (ret) {
    reset_control_rearm(priv.reset);
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn phy_meson_gxl_usb2_exit(phy: *mut phy) -> c_int {
    static int phy_meson_gxl_usb2_exit(struct phy *phy)
    {
    struct phy_meson_gxl_usb2_priv *priv = phy_get_drvdata(phy);
    clk_disable_unprepare(priv.clk);
    reset_control_rearm(priv.reset);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn phy_meson_gxl_usb2_reset(phy: *mut phy) -> c_int {
    static int phy_meson_gxl_usb2_reset(struct phy *phy)
    {
    struct phy_meson_gxl_usb2_priv *priv = phy_get_drvdata(phy);
    if (priv.is_enabled) {
// reset the PHY and wait until settings are stabilized
    regmap_update_bits(priv.regmap, U2P_R0, U2P_R0_POWER_ON_RESET,
    U2P_R0_POWER_ON_RESET);
    udelay(RESET_COMPLETE_TIME);
    regmap_update_bits(priv.regmap, U2P_R0, U2P_R0_POWER_ON_RESET,
    0);
    udelay(RESET_COMPLETE_TIME);
    }
    return 0;
    }
    static int phy_meson_gxl_usb2_set_mode(struct phy *phy,
    enum phy_mode mode, int submode)
    {
    struct phy_meson_gxl_usb2_priv *priv = phy_get_drvdata(phy);
    switch (mode) {
    case PHY_MODE_USB_HOST:
    case PHY_MODE_USB_OTG:
    regmap_update_bits(priv.regmap, U2P_R0, U2P_R0_DM_PULLDOWN,
    U2P_R0_DM_PULLDOWN);
    regmap_update_bits(priv.regmap, U2P_R0, U2P_R0_DP_PULLDOWN,
    U2P_R0_DP_PULLDOWN);
    regmap_update_bits(priv.regmap, U2P_R0, U2P_R0_ID_PULLUP,
    U2P_R0_ID_PULLUP);
    break;
    case PHY_MODE_USB_DEVICE:
    regmap_update_bits(priv.regmap, U2P_R0, U2P_R0_DM_PULLDOWN,
    0);
    regmap_update_bits(priv.regmap, U2P_R0, U2P_R0_DP_PULLDOWN,
    0);
    regmap_update_bits(priv.regmap, U2P_R0, U2P_R0_ID_PULLUP,
    U2P_R0_ID_PULLUP);
    break;
    default:
    return -EINVAL;
    }
    phy_meson_gxl_usb2_reset(phy);
    priv.mode = mode;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn phy_meson_gxl_usb2_power_off(phy: *mut phy) -> c_int {
    static int phy_meson_gxl_usb2_power_off(struct phy *phy)
    {
    struct phy_meson_gxl_usb2_priv *priv = phy_get_drvdata(phy);
    priv.is_enabled = 0;
// power off the PHY by putting it into reset mode
    regmap_update_bits(priv.regmap, U2P_R0, U2P_R0_POWER_ON_RESET,
    U2P_R0_POWER_ON_RESET);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn phy_meson_gxl_usb2_power_on(phy: *mut phy) -> c_int {
    static int phy_meson_gxl_usb2_power_on(struct phy *phy)
    {
    struct phy_meson_gxl_usb2_priv *priv = phy_get_drvdata(phy);
    int ret;
    priv.is_enabled = 1;
// power on the PHY by taking it out of reset mode
    regmap_update_bits(priv.regmap, U2P_R0, U2P_R0_POWER_ON_RESET, 0);
    ret = phy_meson_gxl_usb2_set_mode(phy, priv.mode, 0);
    if (ret) {
    phy_meson_gxl_usb2_power_off(phy);
    dev_err(&phy.dev, "Failed to initialize PHY with mode %d\n",
    priv.mode);
    return ret;
    }
    return 0;
    }
    static const struct phy_ops phy_meson_gxl_usb2_ops = {
    .init		= phy_meson_gxl_usb2_init,
    .exit		= phy_meson_gxl_usb2_exit,
    .power_on	= phy_meson_gxl_usb2_power_on,
    .power_off	= phy_meson_gxl_usb2_power_off,
    .set_mode	= phy_meson_gxl_usb2_set_mode,
    .reset		= phy_meson_gxl_usb2_reset,
    .owner		= THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn phy_meson_gxl_usb2_probe(pdev: *mut platform_device) -> c_int {
    static int phy_meson_gxl_usb2_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct phy_provider *phy_provider;
    struct phy_meson_gxl_usb2_priv *priv;
    struct phy *phy;
    void __iomem *base;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    platform_set_drvdata(pdev, priv);
    base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(base))
    return PTR_ERR(base);
// start in host mode
    priv.mode = PHY_MODE_USB_HOST;
    priv.regmap = devm_regmap_init_mmio(dev, base,
    &phy_meson_gxl_usb2_regmap_conf);
    if (IS_ERR(priv.regmap))
    return PTR_ERR(priv.regmap);
    priv.clk = devm_clk_get_optional(dev, "phy");
    if (IS_ERR(priv.clk))
    return PTR_ERR(priv.clk);
    priv.reset = devm_reset_control_get_optional_shared(dev, "phy");
    if (IS_ERR(priv.reset))
    return PTR_ERR(priv.reset);
    phy = devm_phy_create(dev, core::ptr::null_mut(), &phy_meson_gxl_usb2_ops);
    if (IS_ERR(phy))
    return dev_err_probe(dev, PTR_ERR(phy),
    "failed to create PHY\n");
    phy_set_drvdata(phy, priv);
    phy_provider = devm_of_phy_provider_register(dev, of_phy_simple_xlate);
    return PTR_ERR_OR_ZERO(phy_provider);
    }
    static const struct of_device_id phy_meson_gxl_usb2_of_match[] = {
    { .compatible = "amlogic,meson-gxl-usb2-phy", },
    { },
    };
    MODULE_DEVICE_TABLE(of, phy_meson_gxl_usb2_of_match);
    static struct platform_driver phy_meson_gxl_usb2_driver = {
    .probe	= phy_meson_gxl_usb2_probe,
    .driver	= {
    .name		= "phy-meson-gxl-usb2",
    .of_match_table	= phy_meson_gxl_usb2_of_match,
    },
    };
    module_platform_driver(phy_meson_gxl_usb2_driver);
    MODULE_AUTHOR("Martin Blumenstingl <martin.blumenstingl@googlemail.com>");
    MODULE_DESCRIPTION("Meson GXL and GXM USB2 PHY driver");
    MODULE_LICENSE("GPL v2");
