//! Automatically rewritten from C to Rust
//! Source: drivers/phy/phy-lpc18xx-usb-otg.c
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
// PHY driver for NXP LPC18xx/43xx internal USB OTG PHY
//
// Copyright (C) 2015 Joachim Eastwood <manabian@gmail.com>
//

// USB OTG PHY register offset and bit in CREG
pub const LPC18XX_CREG_CREG0: c_uint = 0x004;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpc18xx_usb_otg_phy {
    pub phy: *mut phy,
    pub clk: *mut clk,
    pub reg: *mut regmap,
}

#[no_mangle]
unsafe extern "C" fn lpc18xx_usb_otg_phy_init(phy: *mut phy) -> c_int {
    static int lpc18xx_usb_otg_phy_init(struct phy *phy)
    {
    struct lpc18xx_usb_otg_phy *lpc = phy_get_drvdata(phy);
    int ret;
// The PHY must be clocked at 480 MHz
    ret = clk_set_rate(lpc.clk, 480000000);
    if (ret)
    return ret;
    return clk_prepare(lpc.clk);
    }
#[no_mangle]
unsafe extern "C" fn lpc18xx_usb_otg_phy_exit(phy: *mut phy) -> c_int {
    static int lpc18xx_usb_otg_phy_exit(struct phy *phy)
    {
    struct lpc18xx_usb_otg_phy *lpc = phy_get_drvdata(phy);
    clk_unprepare(lpc.clk);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lpc18xx_usb_otg_phy_power_on(phy: *mut phy) -> c_int {
    static int lpc18xx_usb_otg_phy_power_on(struct phy *phy)
    {
    struct lpc18xx_usb_otg_phy *lpc = phy_get_drvdata(phy);
    int ret;
    ret = clk_enable(lpc.clk);
    if (ret)
    return ret;
// The bit in CREG is cleared to enable the PHY
    ret = regmap_update_bits(lpc.reg, LPC18XX_CREG_CREG0,
    LPC18XX_CREG_CREG0_USB0PHY, 0);
    if (ret) {
    clk_disable(lpc.clk);
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lpc18xx_usb_otg_phy_power_off(phy: *mut phy) -> c_int {
    static int lpc18xx_usb_otg_phy_power_off(struct phy *phy)
    {
    struct lpc18xx_usb_otg_phy *lpc = phy_get_drvdata(phy);
    int ret;
    ret = regmap_update_bits(lpc.reg, LPC18XX_CREG_CREG0,
    LPC18XX_CREG_CREG0_USB0PHY,
    LPC18XX_CREG_CREG0_USB0PHY);
    if (ret)
    return ret;
    clk_disable(lpc.clk);
    return 0;
    }
    static const struct phy_ops lpc18xx_usb_otg_phy_ops = {
    .init		= lpc18xx_usb_otg_phy_init,
    .exit		= lpc18xx_usb_otg_phy_exit,
    .power_on	= lpc18xx_usb_otg_phy_power_on,
    .power_off	= lpc18xx_usb_otg_phy_power_off,
    .owner		= THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn lpc18xx_usb_otg_phy_probe(pdev: *mut platform_device) -> c_int {
    static int lpc18xx_usb_otg_phy_probe(struct platform_device *pdev)
    {
    struct phy_provider *phy_provider;
    struct lpc18xx_usb_otg_phy *lpc;
    lpc = devm_kzalloc(&pdev.dev, sizeof(*lpc), GFP_KERNEL);
    if (!lpc)
    return -ENOMEM;
    lpc.reg = syscon_node_to_regmap(pdev.dev.of_node.parent);
    if (IS_ERR(lpc.reg)) {
    dev_err(&pdev.dev, "failed to get syscon\n");
    return PTR_ERR(lpc.reg);
    }
    lpc.clk = devm_clk_get(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(lpc.clk)) {
    dev_err(&pdev.dev, "failed to get clock\n");
    return PTR_ERR(lpc.clk);
    }
    lpc.phy = devm_phy_create(&pdev.dev, core::ptr::null_mut(), &lpc18xx_usb_otg_phy_ops);
    if (IS_ERR(lpc.phy)) {
    dev_err(&pdev.dev, "failed to create PHY\n");
    return PTR_ERR(lpc.phy);
    }
    phy_set_drvdata(lpc.phy, lpc);
    phy_provider = devm_of_phy_provider_register(&pdev.dev,
    of_phy_simple_xlate);
    return PTR_ERR_OR_ZERO(phy_provider);
    }
    static const struct of_device_id lpc18xx_usb_otg_phy_match[] = {
    { .compatible = "nxp,lpc1850-usb-otg-phy" },
    { }
    };
    MODULE_DEVICE_TABLE(of, lpc18xx_usb_otg_phy_match);
    static struct platform_driver lpc18xx_usb_otg_phy_driver = {
    .probe		= lpc18xx_usb_otg_phy_probe,
    .driver		= {
    .name	= "lpc18xx-usb-otg-phy",
    .of_match_table = lpc18xx_usb_otg_phy_match,
    },
    };
    module_platform_driver(lpc18xx_usb_otg_phy_driver);
    MODULE_AUTHOR("Joachim Eastwood <manabian@gmail.com>");
    MODULE_DESCRIPTION("NXP LPC18xx/43xx USB OTG PHY driver");
    MODULE_LICENSE("GPL v2");
