//! Automatically rewritten from C to Rust
//! Source: drivers/phy/hisilicon/phy-hi3660-usb3.c
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
// Phy provider for USB 3.0 controller on HiSilicon 3660 platform
//
// Copyright (C) 2017-2018 Hilisicon Electronics Co., Ltd.
// http://www.huawei.com
//
// Authors: Yu Chen <chenyu56@huawei.com>
//

pub const PERI_CRG_CLK_EN4: c_uint = 0x40;
pub const PERI_CRG_CLK_DIS4: c_uint = 0x44;

pub const PERI_CRG_RSTEN4: c_uint = 0x90;
pub const PERI_CRG_RSTDIS4: c_uint = 0x94;

pub const PERI_CRG_ISODIS: c_uint = 0x148;

pub const PCTRL_PERI_CTRL3: c_uint = 0x10;
pub const PCTRL_PERI_CTRL3_MSK_START: c_int = 16;

pub const PCTRL_PERI_CTRL24: c_uint = 0x64;

pub const USBOTG3_CTRL0: c_uint = 0x00;

pub const USBOTG3_CTRL2: c_uint = 0x08;

pub const USBOTG3_CTRL3: c_uint = 0x0C;

pub const USBOTG3_CTRL4: c_uint = 0x10;
pub const USBOTG3_CTRL7: c_uint = 0x1c;

// This value config the default txtune parameter of the usb 2.0 phy
pub const HI3660_USB_DEFAULT_PHY_PARAM: c_uint = 0x1c466e3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hi3660_priv {
    pub dev: *mut device,
    pub peri_crg: *mut regmap,
    pub pctrl: *mut regmap,
    pub otg_bc: *mut regmap,
    pub eye_diagram_param: u32,
}

#[no_mangle]
unsafe extern "C" fn hi3660_phy_init(phy: *mut phy) -> c_int {
    static int hi3660_phy_init(struct phy *phy)
    {
    struct hi3660_priv *priv = phy_get_drvdata(phy);
    u32 val, mask;
    int ret;
// usb refclk iso disable
    ret = regmap_write(priv.peri_crg, PERI_CRG_ISODIS, USB_REFCLK_ISO_EN);
    if (ret)
    goto out;
// enable usb_tcxo_en
    val = USB_TCXO_EN | (USB_TCXO_EN << PCTRL_PERI_CTRL3_MSK_START);
    ret = regmap_write(priv.pctrl, PCTRL_PERI_CTRL3, val);
    if (ret)
    goto out;
// assert phy
    val = IP_RST_USB3OTGPHY_POR | IP_RST_USB3OTG;
    ret = regmap_write(priv.peri_crg, PERI_CRG_RSTEN4, val);
    if (ret)
    goto out;
// enable phy ref clk
    val = SC_USB3PHY_ABB_GT_EN;
    mask = val;
    ret = regmap_update_bits(priv.otg_bc, USBOTG3_CTRL0, mask, val);
    if (ret)
    goto out;
    val = REF_SSP_EN;
    mask = val;
    ret = regmap_update_bits(priv.otg_bc, USBOTG3_CTRL7, mask, val);
    if (ret)
    goto out;
// exit from IDDQ mode
    mask = USBOTG3CTRL2_POWERDOWN_HSP | USBOTG3CTRL2_POWERDOWN_SSP;
    ret = regmap_update_bits(priv.otg_bc, USBOTG3_CTRL2, mask, 0);
    if (ret)
    goto out;
// delay for exit from IDDQ mode
    usleep_range(100, 120);
// deassert phy
    val = IP_RST_USB3OTGPHY_POR | IP_RST_USB3OTG;
    ret = regmap_write(priv.peri_crg, PERI_CRG_RSTDIS4, val);
    if (ret)
    goto out;
// delay for phy deasserted
    usleep_range(10000, 15000);
// fake vbus valid signal
    val = USBOTG3_CTRL3_VBUSVLDEXT | USBOTG3_CTRL3_VBUSVLDEXTSEL;
    mask = val;
    ret = regmap_update_bits(priv.otg_bc, USBOTG3_CTRL3, mask, val);
    if (ret)
    goto out;
// delay for vbus valid
    usleep_range(100, 120);
    ret = regmap_write(priv.otg_bc, USBOTG3_CTRL4,
    priv.eye_diagram_param);
    if (ret)
    goto out;
    return 0;
    out:
    dev_err(priv.dev, "failed to init phy ret: %d\n", ret);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn hi3660_phy_exit(phy: *mut phy) -> c_int {
    static int hi3660_phy_exit(struct phy *phy)
    {
    struct hi3660_priv *priv = phy_get_drvdata(phy);
    u32 val;
    int ret;
// assert phy
    val = IP_RST_USB3OTGPHY_POR;
    ret = regmap_write(priv.peri_crg, PERI_CRG_RSTEN4, val);
    if (ret)
    goto out;
// disable usb_tcxo_en
    val = USB_TCXO_EN << PCTRL_PERI_CTRL3_MSK_START;
    ret = regmap_write(priv.pctrl, PCTRL_PERI_CTRL3, val);
    if (ret)
    goto out;
    return 0;
    out:
    dev_err(priv.dev, "failed to exit phy ret: %d\n", ret);
    return ret;
    }
    static const struct phy_ops hi3660_phy_ops = {
    .init		= hi3660_phy_init,
    .exit		= hi3660_phy_exit,
    .owner		= THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn hi3660_phy_probe(pdev: *mut platform_device) -> c_int {
    static int hi3660_phy_probe(struct platform_device *pdev)
    {
    struct phy_provider *phy_provider;
    struct device *dev = &pdev.dev;
    struct phy *phy;
    struct hi3660_priv *priv;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.dev = dev;
    priv.peri_crg = syscon_regmap_lookup_by_phandle(dev.of_node,
    "hisilicon,pericrg-syscon");
    if (IS_ERR(priv.peri_crg)) {
    dev_err(dev, "no hisilicon,pericrg-syscon\n");
    return PTR_ERR(priv.peri_crg);
    }
    priv.pctrl = syscon_regmap_lookup_by_phandle(dev.of_node,
    "hisilicon,pctrl-syscon");
    if (IS_ERR(priv.pctrl)) {
    dev_err(dev, "no hisilicon,pctrl-syscon\n");
    return PTR_ERR(priv.pctrl);
    }
// node of hi3660 phy is a sub-node of usb3_otg_bc
    priv.otg_bc = syscon_node_to_regmap(dev.parent.of_node);
    if (IS_ERR(priv.otg_bc)) {
    dev_err(dev, "no hisilicon,usb3-otg-bc-syscon\n");
    return PTR_ERR(priv.otg_bc);
    }
    if (of_property_read_u32(dev.of_node, "hisilicon,eye-diagram-param",
    &(priv.eye_diagram_param)))
    priv.eye_diagram_param = HI3660_USB_DEFAULT_PHY_PARAM;
    phy = devm_phy_create(dev, core::ptr::null_mut(), &hi3660_phy_ops);
    if (IS_ERR(phy))
    return PTR_ERR(phy);
    phy_set_drvdata(phy, priv);
    phy_provider = devm_of_phy_provider_register(dev, of_phy_simple_xlate);
    return PTR_ERR_OR_ZERO(phy_provider);
    }
    static const struct of_device_id hi3660_phy_of_match[] = {
    {.compatible = "hisilicon,hi3660-usb-phy",},
    { }
    };
    MODULE_DEVICE_TABLE(of, hi3660_phy_of_match);
    static struct platform_driver hi3660_phy_driver = {
    .probe	= hi3660_phy_probe,
    .driver = {
    .name	= "hi3660-usb-phy",
    .of_match_table	= hi3660_phy_of_match,
    }
    };
    module_platform_driver(hi3660_phy_driver);
    MODULE_AUTHOR("Yu Chen <chenyu56@huawei.com>");
    MODULE_LICENSE("GPL v2");
    MODULE_DESCRIPTION("Hilisicon Hi3660 USB3 PHY Driver");
