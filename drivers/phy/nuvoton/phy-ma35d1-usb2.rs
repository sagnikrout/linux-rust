//! Automatically rewritten from C to Rust
//! Source: drivers/phy/nuvoton/phy-ma35d1-usb2.c
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
// Copyright (C) 2024 Nuvoton Technology Corp.
//

// USB PHY Miscellaneous Control Register
pub const MA35_SYS_REG_USBPMISCR: c_uint = 0x60;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ma35_usb_phy {
    pub clk: *mut clk,
    pub dev: *mut device,
    pub sysreg: *mut regmap,
}

#[no_mangle]
unsafe extern "C" fn ma35_usb_phy_power_on(phy: *mut phy) -> c_int {
    static int ma35_usb_phy_power_on(struct phy *phy)
    {
    struct ma35_usb_phy *p_phy = phy_get_drvdata(phy);
    unsigned int val;
    int ret;
    ret = clk_prepare_enable(p_phy.clk);
    if (ret < 0) {
    dev_err(p_phy.dev, "Failed to enable PHY clock: %d\n", ret);
    return ret;
    }
    regmap_read(p_phy.sysreg, MA35_SYS_REG_USBPMISCR, &val);
    if (val & PHY0SUSPEND) {
//
// USB PHY0 is in operation mode already
// make sure USB PHY 60 MHz UTMI Interface Clock ready
//
    ret = regmap_read_poll_timeout(p_phy.sysreg, MA35_SYS_REG_USBPMISCR, val,
    val & PHY0DEVCKSTB, 10, 1000);
    if (ret == 0)
    return 0;
    }
//
// reset USB PHY0.
// wait until USB PHY0 60 MHz UTMI Interface Clock ready
//
    regmap_update_bits(p_phy.sysreg, MA35_SYS_REG_USBPMISCR, 0x7, (PHY0POR | PHY0SUSPEND));
    udelay(20);
// make USB PHY0 enter operation mode
    regmap_update_bits(p_phy.sysreg, MA35_SYS_REG_USBPMISCR, 0x7, PHY0SUSPEND);
// make sure USB PHY 60 MHz UTMI Interface Clock ready
    ret = regmap_read_poll_timeout(p_phy.sysreg, MA35_SYS_REG_USBPMISCR, val,
    val & PHY0DEVCKSTB, 10, 1000);
    if (ret == -ETIMEDOUT) {
    dev_err(p_phy.dev, "Check PHY clock, Timeout: %d\n", ret);
    clk_disable_unprepare(p_phy.clk);
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ma35_usb_phy_power_off(phy: *mut phy) -> c_int {
    static int ma35_usb_phy_power_off(struct phy *phy)
    {
    struct ma35_usb_phy *p_phy = phy_get_drvdata(phy);
    clk_disable_unprepare(p_phy.clk);
    return 0;
    }
    static const struct phy_ops ma35_usb_phy_ops = {
    .power_on = ma35_usb_phy_power_on,
    .power_off = ma35_usb_phy_power_off,
    .owner = THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn ma35_usb_phy_probe(pdev: *mut platform_device) -> c_int {
    static int ma35_usb_phy_probe(struct platform_device *pdev)
    {
    struct phy_provider *provider;
    struct ma35_usb_phy *p_phy;
    struct phy *phy;
    p_phy = devm_kzalloc(&pdev.dev, sizeof(*p_phy), GFP_KERNEL);
    if (!p_phy)
    return -ENOMEM;
    p_phy.dev = &pdev.dev;
    platform_set_drvdata(pdev, p_phy);
    p_phy.sysreg = syscon_regmap_lookup_by_phandle(pdev.dev.of_node, "nuvoton,sys");
    if (IS_ERR(p_phy.sysreg))
    return dev_err_probe(&pdev.dev, PTR_ERR(p_phy.sysreg),
    "Failed to get SYS registers\n");
    p_phy.clk = of_clk_get(pdev.dev.of_node, 0);
    if (IS_ERR(p_phy.clk))
    return dev_err_probe(&pdev.dev, PTR_ERR(p_phy.clk),
    "failed to find usb_phy clock\n");
    phy = devm_phy_create(&pdev.dev, core::ptr::null_mut(), &ma35_usb_phy_ops);
    if (IS_ERR(phy))
    return dev_err_probe(&pdev.dev, PTR_ERR(phy), "Failed to create PHY\n");
    phy_set_drvdata(phy, p_phy);
    provider = devm_of_phy_provider_register(&pdev.dev, of_phy_simple_xlate);
    if (IS_ERR(provider))
    return dev_err_probe(&pdev.dev, PTR_ERR(provider),
    "Failed to register PHY provider\n");
    return 0;
    }
    static const struct of_device_id ma35_usb_phy_of_match[] = {
    { .compatible = "nuvoton,ma35d1-usb2-phy", },
    { },
    };
    MODULE_DEVICE_TABLE(of, ma35_usb_phy_of_match);
    static struct platform_driver ma35_usb_phy_driver = {
    .probe		= ma35_usb_phy_probe,
    .driver	= {
    .name	= "ma35d1-usb2-phy",
    .of_match_table = ma35_usb_phy_of_match,
    },
    };
    module_platform_driver(ma35_usb_phy_driver);
    MODULE_DESCRIPTION("Nuvoton ma35d1 USB2.0 PHY driver");
    MODULE_AUTHOR("Hui-Ping Chen <hpchen0nvt@gmail.com>");
    MODULE_LICENSE("GPL");
