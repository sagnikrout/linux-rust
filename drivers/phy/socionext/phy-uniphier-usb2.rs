//! Automatically rewritten from C to Rust
//! Source: drivers/phy/socionext/phy-uniphier-usb2.c
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
// phy-uniphier-usb2.c - PHY driver for UniPhier USB2 controller
// Copyright 2015-2018 Socionext Inc.
// Author:
// Kunihiko Hayashi <hayashi.kunihiko@socionext.com>
//

pub const SG_USBPHY1CTRL: c_uint = 0x500;
pub const SG_USBPHY1CTRL2: c_uint = 0x504;
pub const SG_USBPHY2CTRL: c_uint = 0x508;
pub const SG_USBPHY2CTRL2: c_uint = 0x50c	/* LD11 */;
pub const SG_USBPHY12PLL: c_uint = 0x50c	/* Pro4 */;
pub const SG_USBPHY3CTRL: c_uint = 0x510;
pub const SG_USBPHY3CTRL2: c_uint = 0x514;
pub const SG_USBPHY4CTRL: c_uint = 0x518	/* Pro4 */;
pub const SG_USBPHY4CTRL2: c_uint = 0x51c	/* Pro4 */;
pub const SG_USBPHY34PLL: c_uint = 0x51c	/* Pro4 */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uniphier_u2phy_param {
    pub offset: u32,
    pub value: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uniphier_u2phy_soc_data {
    pub config0: uniphier_u2phy_param,
    pub config1: uniphier_u2phy_param,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uniphier_u2phy_priv {
    pub regmap: *mut regmap,
    pub phy: *mut phy,
    pub vbus: *mut regulator,
    pub data: *const uniphier_u2phy_soc_data,
    pub next: *mut uniphier_u2phy_priv,
}

#[no_mangle]
unsafe extern "C" fn uniphier_u2phy_power_on(phy: *mut phy) -> c_int {
    static int uniphier_u2phy_power_on(struct phy *phy)
    {
    struct uniphier_u2phy_priv *priv = phy_get_drvdata(phy);
    let mut ret: c_int = 0;
    if (priv.vbus)
    ret = regulator_enable(priv.vbus);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn uniphier_u2phy_power_off(phy: *mut phy) -> c_int {
    static int uniphier_u2phy_power_off(struct phy *phy)
    {
    struct uniphier_u2phy_priv *priv = phy_get_drvdata(phy);
    if (priv.vbus)
    regulator_disable(priv.vbus);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn uniphier_u2phy_init(phy: *mut phy) -> c_int {
    static int uniphier_u2phy_init(struct phy *phy)
    {
    struct uniphier_u2phy_priv *priv = phy_get_drvdata(phy);
    if (!priv.data)
    return 0;
    regmap_write(priv.regmap, priv.data.config0.offset,
    priv.data.config0.value);
    regmap_write(priv.regmap, priv.data.config1.offset,
    priv.data.config1.value);
    return 0;
    }
    static struct phy *uniphier_u2phy_xlate(struct device *dev,
    const struct of_phandle_args *args)
    {
    struct uniphier_u2phy_priv *priv = dev_get_drvdata(dev);
    while (priv && args.np != priv.phy.dev.of_node)
    priv = priv.next;
    if (!priv) {
    dev_err(dev, "Failed to find appropriate phy\n");
    return ERR_PTR(-EINVAL);
    }
    return priv.phy;
    }
    static const struct phy_ops uniphier_u2phy_ops = {
    .init      = uniphier_u2phy_init,
    .power_on  = uniphier_u2phy_power_on,
    .power_off = uniphier_u2phy_power_off,
    .owner = THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn uniphier_u2phy_probe(pdev: *mut platform_device) -> c_int {
    static int uniphier_u2phy_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct device_node *parent;
    struct uniphier_u2phy_priv *priv = core::ptr::null_mut(), *next = core::ptr::null_mut();
    struct phy_provider *phy_provider;
    struct regmap *regmap;
    const struct uniphier_u2phy_soc_data *data;
    int ret, data_idx, ndatas;
    data = of_device_get_match_data(dev);
    if (WARN_ON(!data))
    return -EINVAL;
// get number of data
    for (ndatas = 0; data[ndatas].config0.offset; ndatas++)
    ;
    parent = of_get_parent(dev.of_node);
    regmap = syscon_node_to_regmap(parent);
    of_node_put(parent);
    if (IS_ERR(regmap)) {
    dev_err(dev, "Failed to get regmap\n");
    return PTR_ERR(regmap);
    }
    for_each_child_of_node_scoped(dev.of_node, child) {
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.regmap = regmap;
    priv.vbus = devm_regulator_get_optional(dev, "vbus");
    if (IS_ERR(priv.vbus)) {
    if (PTR_ERR(priv.vbus) == -EPROBE_DEFER)
    return PTR_ERR(priv.vbus);
    priv.vbus = core::ptr::null_mut();
    }
    priv.phy = devm_phy_create(dev, child, &uniphier_u2phy_ops);
    if (IS_ERR(priv.phy)) {
    dev_err(dev, "Failed to create phy\n");
    return PTR_ERR(priv.phy);
    }
    ret = of_property_read_u32(child, "reg", &data_idx);
    if (ret) {
    dev_err(dev, "Failed to get reg property\n");
    return ret;
    }
    if (data_idx < ndatas)
    priv.data = &data[data_idx];
    else
    dev_warn(dev, "No phy configuration: %s\n",
    child.full_name);
    phy_set_drvdata(priv.phy, priv);
    priv.next = next;
    next = priv;
    }
    dev_set_drvdata(dev, priv);
    phy_provider = devm_of_phy_provider_register(dev,
    uniphier_u2phy_xlate);
    return PTR_ERR_OR_ZERO(phy_provider);
    }
    static const struct uniphier_u2phy_soc_data uniphier_pro4_data[] = {
    {
    .config0 = { SG_USBPHY1CTRL, 0x05142400 },
    .config1 = { SG_USBPHY12PLL, 0x00010010 },
    },
    {
    .config0 = { SG_USBPHY2CTRL, 0x05142400 },
    .config1 = { SG_USBPHY12PLL, 0x00010010 },
    },
    {
    .config0 = { SG_USBPHY3CTRL, 0x05142400 },
    .config1 = { SG_USBPHY34PLL, 0x00010010 },
    },
    {
    .config0 = { SG_USBPHY4CTRL, 0x05142400 },
    .config1 = { SG_USBPHY34PLL, 0x00010010 },
    },
    { /* sentinel */ }
    };
    static const struct uniphier_u2phy_soc_data uniphier_ld11_data[] = {
    {
    .config0 = { SG_USBPHY1CTRL,  0x82280000 },
    .config1 = { SG_USBPHY1CTRL2, 0x00000106 },
    },
    {
    .config0 = { SG_USBPHY2CTRL,  0x82280000 },
    .config1 = { SG_USBPHY2CTRL2, 0x00000106 },
    },
    {
    .config0 = { SG_USBPHY3CTRL,  0x82280000 },
    .config1 = { SG_USBPHY3CTRL2, 0x00000106 },
    },
    { /* sentinel */ }
    };
    static const struct of_device_id uniphier_u2phy_match[] = {
    {
    .compatible = "socionext,uniphier-pro4-usb2-phy",
    .data = &uniphier_pro4_data,
    },
    {
    .compatible = "socionext,uniphier-ld11-usb2-phy",
    .data = &uniphier_ld11_data,
    },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, uniphier_u2phy_match);
    static struct platform_driver uniphier_u2phy_driver = {
    .probe = uniphier_u2phy_probe,
    .driver = {
    .name = "uniphier-usb2-phy",
    .of_match_table = uniphier_u2phy_match,
    },
    };
    module_platform_driver(uniphier_u2phy_driver);
    MODULE_AUTHOR("Kunihiko Hayashi <hayashi.kunihiko@socionext.com>");
    MODULE_DESCRIPTION("UniPhier PHY driver for USB2 controller");
    MODULE_LICENSE("GPL v2");
