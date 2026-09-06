//! Automatically rewritten from C to Rust
//! Source: drivers/phy/qualcomm/phy-ath79-usb.c
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
// Atheros AR71XX/9XXX USB PHY driver
//
// Copyright (C) 2015-2018 Alban Bedel <albeu@free.fr>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath79_usb_phy {
    pub reset: *mut reset_control,
// The suspend override logic is inverted, hence the no prefix
// to make the code a bit easier to understand.
//
    pub no_suspend_override: *mut reset_control,
}

#[no_mangle]
unsafe extern "C" fn ath79_usb_phy_power_on(phy: *mut phy) -> c_int {
    static int ath79_usb_phy_power_on(struct phy *phy)
    {
    struct ath79_usb_phy *priv = phy_get_drvdata(phy);
    let mut err: c_int = 0;
    if (priv.no_suspend_override) {
    err = reset_control_assert(priv.no_suspend_override);
    if (err)
    return err;
    }
    err = reset_control_deassert(priv.reset);
    if (err && priv.no_suspend_override)
    reset_control_deassert(priv.no_suspend_override);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn ath79_usb_phy_power_off(phy: *mut phy) -> c_int {
    static int ath79_usb_phy_power_off(struct phy *phy)
    {
    struct ath79_usb_phy *priv = phy_get_drvdata(phy);
    let mut err: c_int = 0;
    err = reset_control_assert(priv.reset);
    if (err)
    return err;
    if (priv.no_suspend_override) {
    err = reset_control_deassert(priv.no_suspend_override);
    if (err)
    reset_control_deassert(priv.reset);
    }
    return err;
    }
    static const struct phy_ops ath79_usb_phy_ops = {
    .power_on	= ath79_usb_phy_power_on,
    .power_off	= ath79_usb_phy_power_off,
    .owner		= THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn ath79_usb_phy_probe(pdev: *mut platform_device) -> c_int {
    static int ath79_usb_phy_probe(struct platform_device *pdev)
    {
    struct ath79_usb_phy *priv;
    struct phy *phy;
    priv = devm_kzalloc(&pdev.dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.reset = devm_reset_control_get(&pdev.dev, "phy");
    if (IS_ERR(priv.reset))
    return PTR_ERR(priv.reset);
    priv.no_suspend_override = devm_reset_control_get_optional(
    &pdev.dev, "usb-suspend-override");
    if (IS_ERR(priv.no_suspend_override))
    return PTR_ERR(priv.no_suspend_override);
    phy = devm_phy_create(&pdev.dev, core::ptr::null_mut(), &ath79_usb_phy_ops);
    if (IS_ERR(phy))
    return PTR_ERR(phy);
    phy_set_drvdata(phy, priv);
    return PTR_ERR_OR_ZERO(devm_of_phy_provider_register(
    &pdev.dev, of_phy_simple_xlate));
    }
    static const struct of_device_id ath79_usb_phy_of_match[] = {
    { .compatible = "qca,ar7100-usb-phy" },
    {}
    };
    MODULE_DEVICE_TABLE(of, ath79_usb_phy_of_match);
    static struct platform_driver ath79_usb_phy_driver = {
    .probe	= ath79_usb_phy_probe,
    .driver = {
    .of_match_table	= ath79_usb_phy_of_match,
    .name		= "ath79-usb-phy",
    }
    };
    module_platform_driver(ath79_usb_phy_driver);
    MODULE_DESCRIPTION("ATH79 USB PHY driver");
    MODULE_AUTHOR("Alban Bedel <albeu@free.fr>");
    MODULE_LICENSE("GPL");
