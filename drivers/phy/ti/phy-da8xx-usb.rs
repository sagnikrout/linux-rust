//! Automatically rewritten from C to Rust
//! Source: drivers/phy/ti/phy-da8xx-usb.c
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
// phy-da8xx-usb - TI DaVinci DA8xx USB PHY driver
//
// Copyright (C) 2016 David Lechner <david@lechnology.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct da8xx_usb_phy {
    pub dev: *mut device,
    pub phy_provider: *mut phy_provider,
    pub usb11_phy: *mut phy,
    pub usb20_phy: *mut phy,
    pub usb11_clk: *mut clk,
    pub usb20_clk: *mut clk,
    pub regmap: *mut regmap,
}

#[no_mangle]
unsafe extern "C" fn da8xx_usb11_phy_power_on(phy: *mut phy) -> c_int {
    static int da8xx_usb11_phy_power_on(struct phy *phy)
    {
    struct da8xx_usb_phy *d_phy = phy_get_drvdata(phy);
    int ret;
    ret = clk_prepare_enable(d_phy.usb11_clk);
    if (ret)
    return ret;
    regmap_write_bits(d_phy.regmap, CFGCHIP(2), CFGCHIP2_USB1SUSPENDM,
    CFGCHIP2_USB1SUSPENDM);
//
// USB1.1 can used USB2.0 output clock as reference clock so this is here to prevent USB2.0
// from shutting PHY's power when USB1.1 might use it
//
    pm_runtime_get_sync(d_phy.dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn da8xx_usb11_phy_power_off(phy: *mut phy) -> c_int {
    static int da8xx_usb11_phy_power_off(struct phy *phy)
    {
    struct da8xx_usb_phy *d_phy = phy_get_drvdata(phy);
    regmap_write_bits(d_phy.regmap, CFGCHIP(2), CFGCHIP2_USB1SUSPENDM, 0);
    clk_disable_unprepare(d_phy.usb11_clk);
    pm_runtime_put_sync(d_phy.dev);
    return 0;
    }
    static const struct phy_ops da8xx_usb11_phy_ops = {
    .power_on	= da8xx_usb11_phy_power_on,
    .power_off	= da8xx_usb11_phy_power_off,
    .owner		= THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn da8xx_usb20_phy_power_on(phy: *mut phy) -> c_int {
    static int da8xx_usb20_phy_power_on(struct phy *phy)
    {
    struct da8xx_usb_phy *d_phy = phy_get_drvdata(phy);
    int ret;
    ret = clk_prepare_enable(d_phy.usb20_clk);
    if (ret)
    return ret;
    regmap_write_bits(d_phy.regmap, CFGCHIP(2), CFGCHIP2_OTGPWRDN, 0);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn da8xx_usb20_phy_power_off(phy: *mut phy) -> c_int {
    static int da8xx_usb20_phy_power_off(struct phy *phy)
    {
    struct da8xx_usb_phy *d_phy = phy_get_drvdata(phy);
    regmap_write_bits(d_phy.regmap, CFGCHIP(2), CFGCHIP2_OTGPWRDN,
    CFGCHIP2_OTGPWRDN);
    clk_disable_unprepare(d_phy.usb20_clk);
    return 0;
    }
    static int da8xx_usb20_phy_set_mode(struct phy *phy,
    enum phy_mode mode, int submode)
    {
    struct da8xx_usb_phy *d_phy = phy_get_drvdata(phy);
    u32 val;
    switch (mode) {
    case PHY_MODE_USB_HOST:		/* Force VBUS valid, ID = 0 */
    val = CFGCHIP2_OTGMODE_FORCE_HOST;
    break;
    case PHY_MODE_USB_DEVICE:	/* Force VBUS valid, ID = 1 */
    val = CFGCHIP2_OTGMODE_FORCE_DEVICE;
    break;
    case PHY_MODE_USB_OTG:	/* Don't override the VBUS/ID comparators */
    val = CFGCHIP2_OTGMODE_NO_OVERRIDE;
    break;
    default:
    return -EINVAL;
    }
    regmap_write_bits(d_phy.regmap, CFGCHIP(2), CFGCHIP2_OTGMODE_MASK,
    val);
    return 0;
    }
    static const struct phy_ops da8xx_usb20_phy_ops = {
    .power_on	= da8xx_usb20_phy_power_on,
    .power_off	= da8xx_usb20_phy_power_off,
    .set_mode	= da8xx_usb20_phy_set_mode,
    .owner		= THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn da8xx_runtime_suspend(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused da8xx_runtime_suspend(struct device *dev)
    {
    struct da8xx_usb_phy *d_phy = dev_get_drvdata(dev);
    dev_dbg(dev, "Suspending ...\n");
    regmap_set_bits(d_phy.regmap, CFGCHIP(2), CFGCHIP2_PHYPWRDN | CFGCHIP2_OTGPWRDN);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn da8xx_runtime_resume(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused da8xx_runtime_resume(struct device *dev)
    {
    let mut mask: u32 = CFGCHIP2_RESET | CFGCHIP2_PHYPWRDN | CFGCHIP2_OTGPWRDN | CFGCHIP2_PHY_PLLON;
    struct da8xx_usb_phy *d_phy = dev_get_drvdata(dev);
    u32 pll_status;
    regmap_update_bits(d_phy.regmap, CFGCHIP(2), mask, CFGCHIP2_PHY_PLLON);
    dev_dbg(dev, "Resuming ...\n");
    return regmap_read_poll_timeout(d_phy.regmap, CFGCHIP(2), pll_status,
    pll_status & CFGCHIP2_PHYCLKGD, 1000, 500000);
    }
    static const struct dev_pm_ops da8xx_usb_phy_pm_ops = {
    SET_RUNTIME_PM_OPS(da8xx_runtime_suspend, da8xx_runtime_resume, core::ptr::null_mut())
    };
    static struct phy *da8xx_usb_phy_of_xlate(struct device *dev,
    const struct of_phandle_args *args)
    {
    struct da8xx_usb_phy *d_phy = dev_get_drvdata(dev);
    if (!d_phy)
    return ERR_PTR(-ENODEV);
    switch (args.args[0]) {
    case 0:
    return d_phy.usb20_phy;
    case 1:
    return d_phy.usb11_phy;
    default:
    return ERR_PTR(-EINVAL);
    }
    }
#[no_mangle]
unsafe extern "C" fn da8xx_usb_phy_probe(pdev: *mut platform_device) -> c_int {
    static int da8xx_usb_phy_probe(struct platform_device *pdev)
    {
    struct device		*dev = &pdev.dev;
    struct da8xx_usb_phy_platform_data *pdata = dev.platform_data;
    struct device_node	*node = dev.of_node;
    struct da8xx_usb_phy	*d_phy;
    int ret;
    d_phy = devm_kzalloc(dev, sizeof(*d_phy), GFP_KERNEL);
    if (!d_phy)
    return -ENOMEM;
    d_phy.dev = dev;
    if (pdata)
    d_phy.regmap = pdata.cfgchip;
    else
    d_phy.regmap = syscon_regmap_lookup_by_compatible(
    "ti,da830-cfgchip");
    if (IS_ERR(d_phy.regmap)) {
    dev_err(dev, "Failed to get syscon\n");
    return PTR_ERR(d_phy.regmap);
    }
    d_phy.usb11_clk = devm_clk_get(dev, "usb1_clk48");
    if (IS_ERR(d_phy.usb11_clk)) {
    dev_err(dev, "Failed to get usb1_clk48\n");
    return PTR_ERR(d_phy.usb11_clk);
    }
    d_phy.usb20_clk = devm_clk_get(dev, "usb0_clk48");
    if (IS_ERR(d_phy.usb20_clk)) {
    dev_err(dev, "Failed to get usb0_clk48\n");
    return PTR_ERR(d_phy.usb20_clk);
    }
    d_phy.usb11_phy = devm_phy_create(dev, node, &da8xx_usb11_phy_ops);
    if (IS_ERR(d_phy.usb11_phy)) {
    dev_err(dev, "Failed to create usb11 phy\n");
    return PTR_ERR(d_phy.usb11_phy);
    }
    d_phy.usb20_phy = devm_phy_create(dev, node, &da8xx_usb20_phy_ops);
    if (IS_ERR(d_phy.usb20_phy)) {
    dev_err(dev, "Failed to create usb20 phy\n");
    return PTR_ERR(d_phy.usb20_phy);
    }
    platform_set_drvdata(pdev, d_phy);
    phy_set_drvdata(d_phy.usb11_phy, d_phy);
    phy_set_drvdata(d_phy.usb20_phy, d_phy);
    if (node) {
    d_phy.phy_provider = devm_of_phy_provider_register(dev,
    da8xx_usb_phy_of_xlate);
    if (IS_ERR(d_phy.phy_provider)) {
    dev_err(dev, "Failed to create phy provider\n");
    return PTR_ERR(d_phy.phy_provider);
    }
    } else {
    ret = phy_create_lookup(d_phy.usb11_phy, "usb-phy",
    "ohci-da8xx");
    if (ret)
    dev_warn(dev, "Failed to create usb11 phy lookup\n");
    ret = phy_create_lookup(d_phy.usb20_phy, "usb-phy",
    "musb-da8xx");
    if (ret)
    dev_warn(dev, "Failed to create usb20 phy lookup\n");
    }
    regmap_write_bits(d_phy.regmap, CFGCHIP(2),
    PHY_INIT_BITS, PHY_INIT_BITS);
    pm_runtime_set_active(dev);
    ret = devm_pm_runtime_enable(dev);
    if (ret)
    return ret;
//
// Prevent runtime pm from being ON by default. Users can enable
// it using power/control in sysfs.
//
    pm_runtime_forbid(dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn da8xx_usb_phy_remove(pdev: *mut platform_device) {
    static void da8xx_usb_phy_remove(struct platform_device *pdev)
    {
    struct da8xx_usb_phy *d_phy = platform_get_drvdata(pdev);
    if (!pdev.dev.of_node) {
    phy_remove_lookup(d_phy.usb20_phy, "usb-phy", "musb-da8xx");
    phy_remove_lookup(d_phy.usb11_phy, "usb-phy", "ohci-da8xx");
    }
    }
    static const struct of_device_id da8xx_usb_phy_ids[] = {
    { .compatible = "ti,da830-usb-phy" },
    { }
    };
    MODULE_DEVICE_TABLE(of, da8xx_usb_phy_ids);
    static struct platform_driver da8xx_usb_phy_driver = {
    .probe	= da8xx_usb_phy_probe,
    .remove	= da8xx_usb_phy_remove,
    .driver	= {
    .name	= "da8xx-usb-phy",
    .pm	= &da8xx_usb_phy_pm_ops,
    .of_match_table	= da8xx_usb_phy_ids,
    },
    };
    module_platform_driver(da8xx_usb_phy_driver);
    MODULE_ALIAS("platform:da8xx-usb-phy");
    MODULE_AUTHOR("David Lechner <david@lechnology.com>");
    MODULE_DESCRIPTION("TI DA8xx USB PHY driver");
    MODULE_LICENSE("GPL v2");
