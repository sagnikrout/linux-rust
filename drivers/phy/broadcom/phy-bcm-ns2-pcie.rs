//! Automatically rewritten from C to Rust
//! Source: drivers/phy/broadcom/phy-bcm-ns2-pcie.c
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
// Copyright (C) 2016 Broadcom

pub const BLK_ADDR_REG_OFFSET: c_uint = 0x1f;
pub const PLL_AFE1_100MHZ_BLK: c_uint = 0x2100;
pub const PLL_CLK_AMP_OFFSET: c_uint = 0x03;
pub const PLL_CLK_AMP_2P05V: c_uint = 0x2b18;
#[no_mangle]
unsafe extern "C" fn ns2_pci_phy_init(p: *mut phy) -> c_int {
    static int ns2_pci_phy_init(struct phy *p)
    {
    struct mdio_device *mdiodev = phy_get_drvdata(p);
    int rc;
// select the AFE 100MHz block page
    rc = mdiodev_write(mdiodev, BLK_ADDR_REG_OFFSET, PLL_AFE1_100MHZ_BLK);
    if (rc)
    goto err;
// set the 100 MHz reference clock amplitude to 2.05 v
    rc = mdiodev_write(mdiodev, PLL_CLK_AMP_OFFSET, PLL_CLK_AMP_2P05V);
    if (rc)
    goto err;
    return 0;
    err:
    dev_err(&mdiodev.dev, "Error %d writing to phy\n", rc);
    return rc;
    }
    static const struct phy_ops ns2_pci_phy_ops = {
    .init = ns2_pci_phy_init,
    .owner = THIS_MODULE,
    };
#[no_mangle]
unsafe extern "C" fn ns2_pci_phy_probe(mdiodev: *mut mdio_device) -> c_int {
    static int ns2_pci_phy_probe(struct mdio_device *mdiodev)
    {
    struct device *dev = &mdiodev.dev;
    struct phy_provider *provider;
    struct phy *phy;
    phy = devm_phy_create(dev, dev.of_node, &ns2_pci_phy_ops);
    if (IS_ERR(phy)) {
    dev_err(dev, "failed to create Phy\n");
    return PTR_ERR(phy);
    }
    phy_set_drvdata(phy, mdiodev);
    provider = devm_of_phy_provider_register(&phy.dev,
    of_phy_simple_xlate);
    if (IS_ERR(provider)) {
    dev_err(dev, "failed to register Phy provider\n");
    return PTR_ERR(provider);
    }
    return 0;
    }
    static const struct of_device_id ns2_pci_phy_of_match[] = {
    { .compatible = "brcm,ns2-pcie-phy", },
    { /* sentinel */ },
    };
    MODULE_DEVICE_TABLE(of, ns2_pci_phy_of_match);
    static struct mdio_driver ns2_pci_phy_driver = {
    .mdiodrv = {
    .driver = {
    .name = "phy-bcm-ns2-pci",
    .of_match_table = ns2_pci_phy_of_match,
    },
    },
    .probe = ns2_pci_phy_probe,
    };
    mdio_module_driver(ns2_pci_phy_driver);
    MODULE_AUTHOR("Broadcom");
    MODULE_DESCRIPTION("Broadcom Northstar2 PCI Phy driver");
    MODULE_LICENSE("GPL v2");
    MODULE_ALIAS("platform:phy-bcm-ns2-pci");
