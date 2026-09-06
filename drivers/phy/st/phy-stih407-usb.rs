//! Automatically rewritten from C to Rust
//! Source: drivers/phy/st/phy-stih407-usb.c
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
// Copyright (C) 2014 STMicroelectronics
//
// STMicroelectronics Generic PHY driver for STiH407 USB2.
//
// Author: Giuseppe Cavallaro <peppe.cavallaro@st.com>
//

pub const PHYPARAM_REG: c_int = 0;
pub const PHYCTRL_REG: c_int = 1;
// Default PHY_SEL and REFCLKSEL configuration
pub const STIH407_USB_PICOPHY_CTRL_PORT_CONF: c_uint = 0x6;
pub const STIH407_USB_PICOPHY_CTRL_PORT_MASK: c_uint = 0x1f;
// ports parameters overriding
pub const STIH407_USB_PICOPHY_PARAM_DEF: c_uint = 0x39a4dc;
pub const STIH407_USB_PICOPHY_PARAM_MASK: c_uint = 0xffffffff;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stih407_usb2_picophy {
    pub phy: *mut phy,
    pub regmap: *mut regmap,
    pub dev: *mut device,
    pub rstc: *mut reset_control,
    pub rstport: *mut reset_control,
    pub ctrl: c_int,
    pub param: c_int,
}

#[no_mangle]
unsafe extern "C" fn stih407_usb2_pico_ctrl(phy_dev: *mut stih407_usb2_picophy) -> c_int {
    static int stih407_usb2_pico_ctrl(struct stih407_usb2_picophy *phy_dev)
    {
    reset_control_deassert(phy_dev.rstc);
    return regmap_update_bits(phy_dev.regmap, phy_dev.ctrl,
    STIH407_USB_PICOPHY_CTRL_PORT_MASK,
    STIH407_USB_PICOPHY_CTRL_PORT_CONF);
    }
#[no_mangle]
unsafe extern "C" fn stih407_usb2_init_port(phy: *mut phy) -> c_int {
    static int stih407_usb2_init_port(struct phy *phy)
    {
    int ret;
    struct stih407_usb2_picophy *phy_dev = phy_get_drvdata(phy);
    stih407_usb2_pico_ctrl(phy_dev);
    ret = regmap_update_bits(phy_dev.regmap,
    phy_dev.param,
    STIH407_USB_PICOPHY_PARAM_MASK,
    STIH407_USB_PICOPHY_PARAM_DEF);
    if (ret)
    return ret;
    return reset_control_deassert(phy_dev.rstport);
    }
#[no_mangle]
unsafe extern "C" fn stih407_usb2_exit_port(phy: *mut phy) -> c_int {
    static int stih407_usb2_exit_port(struct phy *phy)
    {
    struct stih407_usb2_picophy *phy_dev = phy_get_drvdata(phy);
//
// Only port reset is asserted, phy global reset is kept untouched
// as other ports may still be active. When all ports are in reset
// state, assumption is made that power will be cut off on the phy, in
// case of suspend for instance. Theoretically, asserting individual
// reset (like here) or global reset should be equivalent.
//
    return reset_control_assert(phy_dev.rstport);
    }
    static const struct phy_ops stih407_usb2_picophy_data = {
    .init = stih407_usb2_init_port,
    .exit = stih407_usb2_exit_port,
    .owner = THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn stih407_usb2_picophy_probe(pdev: *mut platform_device) -> c_int {
    static int stih407_usb2_picophy_probe(struct platform_device *pdev)
    {
    struct stih407_usb2_picophy *phy_dev;
    struct device *dev = &pdev.dev;
    struct device_node *np = dev.of_node;
    struct phy_provider *phy_provider;
    unsigned int syscon_args[2];
    struct phy *phy;
    phy_dev = devm_kzalloc(dev, sizeof(*phy_dev), GFP_KERNEL);
    if (!phy_dev)
    return -ENOMEM;
    phy_dev.dev = dev;
    dev_set_drvdata(dev, phy_dev);
    phy_dev.rstc = devm_reset_control_get_shared(dev, "global");
    if (IS_ERR(phy_dev.rstc)) {
    dev_err(dev, "failed to ctrl picoPHY reset\n");
    return PTR_ERR(phy_dev.rstc);
    }
    phy_dev.rstport = devm_reset_control_get_exclusive(dev, "port");
    if (IS_ERR(phy_dev.rstport)) {
    dev_err(dev, "failed to ctrl picoPHY reset\n");
    return PTR_ERR(phy_dev.rstport);
    }
// Reset port by default: only deassert it in phy init
    reset_control_assert(phy_dev.rstport);
    phy_dev.regmap = syscon_regmap_lookup_by_phandle_args(np, "st,syscfg",
    2, syscon_args);
    if (IS_ERR(phy_dev.regmap)) {
    dev_err(dev, "No syscfg phandle specified\n");
    return PTR_ERR(phy_dev.regmap);
    }
    phy_dev.param = syscon_args[PHYPARAM_REG];
    phy_dev.ctrl = syscon_args[PHYCTRL_REG];
    phy = devm_phy_create(dev, core::ptr::null_mut(), &stih407_usb2_picophy_data);
    if (IS_ERR(phy)) {
    dev_err(dev, "failed to create Display Port PHY\n");
    return PTR_ERR(phy);
    }
    phy_dev.phy = phy;
    phy_set_drvdata(phy, phy_dev);
    phy_provider = devm_of_phy_provider_register(dev, of_phy_simple_xlate);
    if (IS_ERR(phy_provider))
    return PTR_ERR(phy_provider);
    return 0;
    }
    static const struct of_device_id stih407_usb2_picophy_of_match[] = {
    { .compatible = "st,stih407-usb2-phy" },
    { /*sentinel */ },
    };
    MODULE_DEVICE_TABLE(of, stih407_usb2_picophy_of_match);
    static struct platform_driver stih407_usb2_picophy_driver = {
    .probe = stih407_usb2_picophy_probe,
    .driver = {
    .name = "stih407-usb-genphy",
    .of_match_table = stih407_usb2_picophy_of_match,
    }
    };
    module_platform_driver(stih407_usb2_picophy_driver);
    MODULE_AUTHOR("Giuseppe Cavallaro <peppe.cavallaro@st.com>");
    MODULE_DESCRIPTION("STMicroelectronics Generic picoPHY driver for STiH407");
    MODULE_LICENSE("GPL v2");
