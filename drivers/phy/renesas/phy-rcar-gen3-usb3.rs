//! Automatically rewritten from C to Rust
//! Source: drivers/phy/renesas/phy-rcar-gen3-usb3.c
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
// Renesas R-Car Gen3 for USB3.0 PHY driver
//
// Copyright (C) 2017 Renesas Electronics Corporation
//

pub const USB30_CLKSET0: c_uint = 0x034;
pub const USB30_CLKSET1: c_uint = 0x036;
pub const USB30_SSC_SET: c_uint = 0x038;
pub const USB30_PHY_ENABLE: c_uint = 0x060;
pub const USB30_VBUS_EN: c_uint = 0x064;
// USB30_CLKSET0
pub const CLKSET0_PRIVATE: c_uint = 0x05c0;
pub const CLKSET0_USB30_FSEL_USB_EXTAL: c_uint = 0x0002;
// USB30_CLKSET1
pub const CLKSET1_USB30_PLL_MULTI_SHIFT: c_int = 6;

    CLKSET1_USB30_PLL_MULTI_SHIFT)

// USB30_SSC_SET

pub const SSC_SET_RANGE_SHIFT: c_int = 9;

// USB30_PHY_ENABLE

// USB30_VBUS_EN

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rcar_gen3_usb3 {
    pub base: *mut void __iomem,
    pub phy: *mut phy,
    pub ssc_range: u32,
    pub usb3s_clk: bool,
    pub usb_extal: bool,
}

#[no_mangle]
unsafe extern "C" fn write_clkset1_for_usb_extal(r: *mut rcar_gen3_usb3, reset: bool) {
    static void write_clkset1_for_usb_extal(struct rcar_gen3_usb3 *r, bool reset)
    {
    u16 val = CLKSET1_USB30_PLL_MULTI_USB_EXTAL |
    CLKSET1_REF_CLKDIV | CLKSET1_PRIVATE_2_1;
    if (reset)
    val |= CLKSET1_PHYRESET;
    writew(val, r.base + USB30_CLKSET1);
    }
#[no_mangle]
unsafe extern "C" fn rcar_gen3_phy_usb3_enable_ssc(r: *mut rcar_gen3_usb3) {
    static void rcar_gen3_phy_usb3_enable_ssc(struct rcar_gen3_usb3 *r)
    {
    let mut val: u16 = SSC_SET_SSC_EN;
    switch (r.ssc_range) {
    case 4980:
    val |= SSC_SET_RANGE_4980;
    break;
    case 4492:
    val |= SSC_SET_RANGE_4492;
    break;
    case 4003:
    val |= SSC_SET_RANGE_4003;
    break;
    default:
    dev_err(&r.phy.dev, "%s: unsupported range (%x)\n", __func__,
    r.ssc_range);
    return;
    }
    writew(val, r.base + USB30_SSC_SET);
    }
#[no_mangle]
unsafe extern "C" fn rcar_gen3_phy_usb3_select_usb_extal(r: *mut rcar_gen3_usb3) {
    static void rcar_gen3_phy_usb3_select_usb_extal(struct rcar_gen3_usb3 *r)
    {
    write_clkset1_for_usb_extal(r, false);
    if (r.ssc_range)
    rcar_gen3_phy_usb3_enable_ssc(r);
    writew(CLKSET0_PRIVATE | CLKSET0_USB30_FSEL_USB_EXTAL,
    r.base + USB30_CLKSET0);
    writew(PHY_ENABLE_RESET_EN, r.base + USB30_PHY_ENABLE);
    write_clkset1_for_usb_extal(r, true);
    usleep_range(10, 20);
    write_clkset1_for_usb_extal(r, false);
    }
#[no_mangle]
unsafe extern "C" fn rcar_gen3_phy_usb3_init(p: *mut phy) -> c_int {
    static int rcar_gen3_phy_usb3_init(struct phy *p)
    {
    struct rcar_gen3_usb3 *r = phy_get_drvdata(p);
    dev_vdbg(&r.phy.dev, "%s: enter (%d, %d, %d)\n", __func__,
    r.usb3s_clk, r.usb_extal, r.ssc_range);
    if (!r.usb3s_clk && r.usb_extal)
    rcar_gen3_phy_usb3_select_usb_extal(r);
// Enables VBUS detection anyway
    writew(VBUS_EN_VBUS_EN, r.base + USB30_VBUS_EN);
    return 0;
    }
    static const struct phy_ops rcar_gen3_phy_usb3_ops = {
    .init		= rcar_gen3_phy_usb3_init,
    .owner		= THIS_MODULE,
    };
    static const struct of_device_id rcar_gen3_phy_usb3_match_table[] = {
    { .compatible = "renesas,rcar-gen3-usb3-phy" },
    { }
    };
    MODULE_DEVICE_TABLE(of, rcar_gen3_phy_usb3_match_table);
#[no_mangle]
unsafe extern "C" fn rcar_gen3_phy_usb3_probe(pdev: *mut platform_device) -> c_int {
    static int rcar_gen3_phy_usb3_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct rcar_gen3_usb3 *r;
    struct phy_provider *provider;
    let mut ret: c_int = 0;
    struct clk *clk;
    if (!dev.of_node) {
    dev_err(dev, "This driver needs device tree\n");
    return -EINVAL;
    }
    r = devm_kzalloc(dev, sizeof(*r), GFP_KERNEL);
    if (!r)
    return -ENOMEM;
    r.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(r.base))
    return PTR_ERR(r.base);
    clk = devm_clk_get(dev, "usb3s_clk");
    if (!IS_ERR(clk) && !clk_prepare_enable(clk)) {
    r.usb3s_clk = !!clk_get_rate(clk);
    clk_disable_unprepare(clk);
    }
    clk = devm_clk_get(dev, "usb_extal");
    if (!IS_ERR(clk) && !clk_prepare_enable(clk)) {
    r.usb_extal = !!clk_get_rate(clk);
    clk_disable_unprepare(clk);
    }
    if (!r.usb3s_clk && !r.usb_extal) {
    dev_err(dev, "This driver needs usb3s_clk and/or usb_extal\n");
    ret = -EINVAL;
    goto error;
    }
//
// devm_phy_create() will call pm_runtime_enable(&phy->dev);
// And then, phy-core will manage runtime pm for this device.
//
    pm_runtime_enable(dev);
    r.phy = devm_phy_create(dev, core::ptr::null_mut(), &rcar_gen3_phy_usb3_ops);
    if (IS_ERR(r.phy)) {
    dev_err(dev, "Failed to create USB3 PHY\n");
    ret = PTR_ERR(r.phy);
    goto error;
    }
    of_property_read_u32(dev.of_node, "renesas,ssc-range", &r.ssc_range);
    platform_set_drvdata(pdev, r);
    phy_set_drvdata(r.phy, r);
    provider = devm_of_phy_provider_register(dev, of_phy_simple_xlate);
    if (IS_ERR(provider)) {
    dev_err(dev, "Failed to register PHY provider\n");
    ret = PTR_ERR(provider);
    goto error;
    }
    return 0;
    error:
    pm_runtime_disable(dev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn rcar_gen3_phy_usb3_remove(pdev: *mut platform_device) {
    static void rcar_gen3_phy_usb3_remove(struct platform_device *pdev)
    {
    pm_runtime_disable(&pdev.dev);
    }
    static struct platform_driver rcar_gen3_phy_usb3_driver = {
    .driver = {
    .name = "phy_rcar_gen3_usb3",
    .of_match_table = rcar_gen3_phy_usb3_match_table,
    },
    .probe = rcar_gen3_phy_usb3_probe,
    .remove = rcar_gen3_phy_usb3_remove,
    };
    module_platform_driver(rcar_gen3_phy_usb3_driver);
    MODULE_LICENSE("GPL v2");
    MODULE_DESCRIPTION("Renesas R-Car Gen3 USB 3.0 PHY");
    MODULE_AUTHOR("Yoshihiro Shimoda <yoshihiro.shimoda.uh@renesas.com>");
