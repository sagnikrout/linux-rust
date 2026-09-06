//! Automatically rewritten from C to Rust
//! Source: drivers/phy/hisilicon/phy-hi6220-usb.c
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
// Copyright (c) 2015 Linaro Ltd.
// Copyright (c) 2015 HiSilicon Limited.
//

pub const SC_PERIPH_CTRL4: c_uint = 0x00c;

pub const SC_PERIPH_CTRL5: c_uint = 0x010;

pub const SC_PERIPH_CTRL8: c_uint = 0x018;
pub const SC_PERIPH_RSTEN0: c_uint = 0x300;
pub const SC_PERIPH_RSTDIS0: c_uint = 0x304;

pub const EYE_PATTERN_PARA: c_uint = 0x7053348c;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hi6220_priv {
    pub reg: *mut regmap,
    pub dev: *mut device,
}

#[no_mangle]
unsafe extern "C" fn hi6220_phy_init(priv: *mut hi6220_priv) {
    static void hi6220_phy_init(struct hi6220_priv *priv)
    {
    struct regmap *reg = priv.reg;
    u32 val, mask;
    val = RST0_USBOTG_BUS | RST0_POR_PICOPHY |
    RST0_USBOTG | RST0_USBOTG_32K;
    mask = val;
    regmap_update_bits(reg, SC_PERIPH_RSTEN0, mask, val);
    regmap_update_bits(reg, SC_PERIPH_RSTDIS0, mask, val);
    }
#[no_mangle]
unsafe extern "C" fn hi6220_phy_setup(priv: *mut hi6220_priv, on: bool) -> c_int {
    static int hi6220_phy_setup(struct hi6220_priv *priv, bool on)
    {
    struct regmap *reg = priv.reg;
    u32 val, mask;
    int ret;
    if (on) {
    val = CTRL5_USBOTG_RES_SEL | CTRL5_PICOPHY_ACAENB;
    mask = val | CTRL5_PICOPHY_BC_MODE;
    ret = regmap_update_bits(reg, SC_PERIPH_CTRL5, mask, val);
    if (ret)
    goto out;
    val =  CTRL4_PICO_VBUSVLDEXT | CTRL4_PICO_VBUSVLDEXTSEL |
    CTRL4_OTG_PHY_SEL;
    mask = val | CTRL4_PICO_SIDDQ | CTRL4_PICO_OGDISABLE;
    ret = regmap_update_bits(reg, SC_PERIPH_CTRL4, mask, val);
    if (ret)
    goto out;
    ret = regmap_write(reg, SC_PERIPH_CTRL8, EYE_PATTERN_PARA);
    if (ret)
    goto out;
    } else {
    val = CTRL4_PICO_SIDDQ;
    mask = val;
    ret = regmap_update_bits(reg, SC_PERIPH_CTRL4, mask, val);
    if (ret)
    goto out;
    }
    return 0;
    out:
    dev_err(priv.dev, "failed to setup phy ret: %d\n", ret);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn hi6220_phy_start(phy: *mut phy) -> c_int {
    static int hi6220_phy_start(struct phy *phy)
    {
    struct hi6220_priv *priv = phy_get_drvdata(phy);
    return hi6220_phy_setup(priv, true);
    }
#[no_mangle]
unsafe extern "C" fn hi6220_phy_exit(phy: *mut phy) -> c_int {
    static int hi6220_phy_exit(struct phy *phy)
    {
    struct hi6220_priv *priv = phy_get_drvdata(phy);
    return hi6220_phy_setup(priv, false);
    }
    static const struct phy_ops hi6220_phy_ops = {
    .init		= hi6220_phy_start,
    .exit		= hi6220_phy_exit,
    .owner		= THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn hi6220_phy_probe(pdev: *mut platform_device) -> c_int {
    static int hi6220_phy_probe(struct platform_device *pdev)
    {
    struct phy_provider *phy_provider;
    struct device *dev = &pdev.dev;
    struct phy *phy;
    struct hi6220_priv *priv;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.dev = dev;
    priv.reg = syscon_regmap_lookup_by_phandle(dev.of_node,
    "hisilicon,peripheral-syscon");
    if (IS_ERR(priv.reg)) {
    dev_err(dev, "no hisilicon,peripheral-syscon\n");
    return PTR_ERR(priv.reg);
    }
    hi6220_phy_init(priv);
    phy = devm_phy_create(dev, core::ptr::null_mut(), &hi6220_phy_ops);
    if (IS_ERR(phy))
    return PTR_ERR(phy);
    phy_set_drvdata(phy, priv);
    phy_provider = devm_of_phy_provider_register(dev, of_phy_simple_xlate);
    return PTR_ERR_OR_ZERO(phy_provider);
    }
    static const struct of_device_id hi6220_phy_of_match[] = {
    {.compatible = "hisilicon,hi6220-usb-phy",},
    { },
    };
    MODULE_DEVICE_TABLE(of, hi6220_phy_of_match);
    static struct platform_driver hi6220_phy_driver = {
    .probe	= hi6220_phy_probe,
    .driver = {
    .name	= "hi6220-usb-phy",
    .of_match_table	= hi6220_phy_of_match,
    }
    };
    module_platform_driver(hi6220_phy_driver);
    MODULE_DESCRIPTION("HISILICON HI6220 USB PHY driver");
    MODULE_LICENSE("GPL");
