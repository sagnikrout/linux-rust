//! Automatically rewritten from C to Rust
//! Source: drivers/phy/marvell/phy-armada375-usb2.c
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
// USB cluster support for Armada 375 platform.
//
// Copyright (C) 2014 Marvell
//
// Gregory CLEMENT <gregory.clement@free-electrons.com>
//
// Armada 375 comes with an USB2 host and device controller and an
// USB3 controller. The USB cluster control register allows to manage
// common features of both USB controllers.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct armada375_cluster_phy {
    pub phy: *mut phy,
    pub reg: *mut void __iomem,
    pub use_usb3: bool,
    pub phy_provided: c_int,
}

#[no_mangle]
unsafe extern "C" fn armada375_usb_phy_init(phy: *mut phy) -> c_int {
    static int armada375_usb_phy_init(struct phy *phy)
    {
    struct armada375_cluster_phy *cluster_phy;
    u32 reg;
    cluster_phy = phy_get_drvdata(phy);
    if (!cluster_phy)
    return -ENODEV;
    reg = readl(cluster_phy.reg);
    if (cluster_phy.use_usb3)
    reg |= USB2_PHY_CONFIG_DISABLE;
    else
    reg &= ~USB2_PHY_CONFIG_DISABLE;
    writel(reg, cluster_phy.reg);
    return 0;
    }
    static const struct phy_ops armada375_usb_phy_ops = {
    .init = armada375_usb_phy_init,
    .owner = THIS_MODULE,
    };
//
// Only one controller can use this PHY. We shouldn't have the case
// when two controllers want to use this PHY. But if this case occurs
// then we provide a phy to the first one and return an error for the
// next one. This error has also to be an error returned by
// devm_phy_optional_get() so different from ENODEV for USB2. In the
// USB3 case it still optional and we use ENODEV.
//
    static struct phy *armada375_usb_phy_xlate(struct device *dev,
    const struct of_phandle_args *args)
    {
    struct armada375_cluster_phy *cluster_phy = dev_get_drvdata(dev);
    if (!cluster_phy)
    return  ERR_PTR(-ENODEV);
//
// Either the phy had never been requested and then the first
// usb claiming it can get it, or it had already been
// requested in this case, we only allow to use it with the
// same configuration.
//
    if (WARN_ON((cluster_phy.phy_provided != PHY_NONE) &&
    (cluster_phy.phy_provided != args.args[0]))) {
    dev_err(dev, "This PHY has already been provided!\n");
    dev_err(dev, "Check your device tree, only one controller can use it\n.");
    if (args.args[0] == PHY_TYPE_USB2)
    return ERR_PTR(-EBUSY);
    else
    return ERR_PTR(-ENODEV);
    }
    if (args.args[0] == PHY_TYPE_USB2)
    cluster_phy.use_usb3 = false;
#[no_mangle]
pub unsafe extern "C" fn if(PHY_TYPE_USB3: args->args[0] ==) -> else {
    else if (args.args[0] == PHY_TYPE_USB3)
    cluster_phy.use_usb3 = true;
    else {
    dev_err(dev, "Invalid PHY mode\n");
    return ERR_PTR(-ENODEV);
    }
// Store which phy mode is used for next test
    cluster_phy.phy_provided = args.args[0];
    return cluster_phy.phy;
    }
#[no_mangle]
unsafe extern "C" fn armada375_usb_phy_probe(pdev: *mut platform_device) -> c_int {
    static int armada375_usb_phy_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct phy *phy;
    struct phy_provider *phy_provider;
    void __iomem *usb_cluster_base;
    struct armada375_cluster_phy *cluster_phy;
    cluster_phy = devm_kzalloc(dev, sizeof(*cluster_phy), GFP_KERNEL);
    if (!cluster_phy)
    return  -ENOMEM;
    usb_cluster_base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(usb_cluster_base))
    return PTR_ERR(usb_cluster_base);
    phy = devm_phy_create(dev, core::ptr::null_mut(), &armada375_usb_phy_ops);
    if (IS_ERR(phy)) {
    dev_err(dev, "failed to create PHY\n");
    return PTR_ERR(phy);
    }
    cluster_phy.phy = phy;
    cluster_phy.reg = usb_cluster_base;
    dev_set_drvdata(dev, cluster_phy);
    phy_set_drvdata(phy, cluster_phy);
    phy_provider = devm_of_phy_provider_register(&pdev.dev,
    armada375_usb_phy_xlate);
    return PTR_ERR_OR_ZERO(phy_provider);
    }
    static const struct of_device_id of_usb_cluster_table[] = {
    { .compatible = "marvell,armada-375-usb-cluster", },
    { /* end of list */ },
    };
    static struct platform_driver armada375_usb_phy_driver = {
    .probe	= armada375_usb_phy_probe,
    .driver = {
    .of_match_table	= of_usb_cluster_table,
    .name  = "armada-375-usb-cluster",
    }
    };
    builtin_platform_driver(armada375_usb_phy_driver);
