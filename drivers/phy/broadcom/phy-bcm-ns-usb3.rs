//! Automatically rewritten from C to Rust
//! Source: drivers/phy/broadcom/phy-bcm-ns-usb3.c
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
// Broadcom Northstar USB 3.0 PHY Driver
//
// Copyright (C) 2016 Rafał Miłecki <rafal@milecki.pl>
// Copyright (C) 2016 Broadcom
//
// All magic values used for initialization (and related comments) were obtained
// from Broadcom's SDK:
// Copyright (c) Broadcom Corp, 2012
//

pub const BCM_NS_USB3_PHY_BASE_ADDR_REG: c_uint = 0x1f;
pub const BCM_NS_USB3_PHY_PLL30_BLOCK: c_uint = 0x8000;
pub const BCM_NS_USB3_PHY_TX_PMD_BLOCK: c_uint = 0x8040;
pub const BCM_NS_USB3_PHY_PIPE_BLOCK: c_uint = 0x8060;
// Registers of PLL30 block
pub const BCM_NS_USB3_PLL_CONTROL: c_uint = 0x01;
pub const BCM_NS_USB3_PLLA_CONTROL0: c_uint = 0x0a;
pub const BCM_NS_USB3_PLLA_CONTROL1: c_uint = 0x0b;
// Registers of TX PMD block
pub const BCM_NS_USB3_TX_PMD_CONTROL1: c_uint = 0x01;
// Registers of PIPE block
pub const BCM_NS_USB3_LFPS_CMP: c_uint = 0x02;
pub const BCM_NS_USB3_LFPS_DEGLITCH: c_uint = 0x03;
    enum bcm_ns_family {
    BCM_NS_UNKNOWN,
    BCM_NS_AX,
    BCM_NS_BX,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm_ns_usb3 {
    pub dev: *mut device,
    pub family: enum bcm_ns_family,
    pub dmp: *mut void __iomem,
    pub mdiodev: *mut mdio_device,
    pub phy: *mut phy,
}

    static const struct of_device_id bcm_ns_usb3_id_table[] = {
    {
    .compatible = "brcm,ns-ax-usb3-phy",
    .data = (int *)BCM_NS_AX,
    },
    {
    .compatible = "brcm,ns-bx-usb3-phy",
    .data = (int *)BCM_NS_BX,
    },
    {},
    };
    MODULE_DEVICE_TABLE(of, bcm_ns_usb3_id_table);
    static int bcm_ns_usb3_mdio_phy_write(struct bcm_ns_usb3 *usb3, u16 reg,
    u16 value);
#[no_mangle]
unsafe extern "C" fn bcm_ns_usb3_phy_init_ns_bx(usb3: *mut bcm_ns_usb3) -> c_int {
    static int bcm_ns_usb3_phy_init_ns_bx(struct bcm_ns_usb3 *usb3)
    {
    int err;
// USB3 PLL Block
    err = bcm_ns_usb3_mdio_phy_write(usb3, BCM_NS_USB3_PHY_BASE_ADDR_REG,
    BCM_NS_USB3_PHY_PLL30_BLOCK);
    if (err < 0)
    return err;
// Assert Ana_Pllseq start
    bcm_ns_usb3_mdio_phy_write(usb3, BCM_NS_USB3_PLL_CONTROL, 0x1000);
// Assert CML Divider ratio to 26
    bcm_ns_usb3_mdio_phy_write(usb3, BCM_NS_USB3_PLLA_CONTROL0, 0x6400);
// Asserting PLL Reset
    bcm_ns_usb3_mdio_phy_write(usb3, BCM_NS_USB3_PLLA_CONTROL1, 0xc000);
// Deaaserting PLL Reset
    bcm_ns_usb3_mdio_phy_write(usb3, BCM_NS_USB3_PLLA_CONTROL1, 0x8000);
// Deasserting USB3 system reset
    writel(0, usb3.dmp + BCMA_RESET_CTL);
// PLL frequency monitor enable
    bcm_ns_usb3_mdio_phy_write(usb3, BCM_NS_USB3_PLL_CONTROL, 0x9000);
// PIPE Block
    bcm_ns_usb3_mdio_phy_write(usb3, BCM_NS_USB3_PHY_BASE_ADDR_REG,
    BCM_NS_USB3_PHY_PIPE_BLOCK);
// CMPMAX & CMPMINTH setting
    bcm_ns_usb3_mdio_phy_write(usb3, BCM_NS_USB3_LFPS_CMP, 0xf30d);
// DEGLITCH MIN & MAX setting
    bcm_ns_usb3_mdio_phy_write(usb3, BCM_NS_USB3_LFPS_DEGLITCH, 0x6302);
// TXPMD block
    bcm_ns_usb3_mdio_phy_write(usb3, BCM_NS_USB3_PHY_BASE_ADDR_REG,
    BCM_NS_USB3_PHY_TX_PMD_BLOCK);
// Enabling SSC
    bcm_ns_usb3_mdio_phy_write(usb3, BCM_NS_USB3_TX_PMD_CONTROL1, 0x1003);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bcm_ns_usb3_phy_init_ns_ax(usb3: *mut bcm_ns_usb3) -> c_int {
    static int bcm_ns_usb3_phy_init_ns_ax(struct bcm_ns_usb3 *usb3)
    {
    int err;
// PLL30 block
    err = bcm_ns_usb3_mdio_phy_write(usb3, BCM_NS_USB3_PHY_BASE_ADDR_REG,
    BCM_NS_USB3_PHY_PLL30_BLOCK);
    if (err < 0)
    return err;
    bcm_ns_usb3_mdio_phy_write(usb3, BCM_NS_USB3_PLLA_CONTROL0, 0x6400);
    bcm_ns_usb3_mdio_phy_write(usb3, BCM_NS_USB3_PHY_BASE_ADDR_REG, 0x80e0);
    bcm_ns_usb3_mdio_phy_write(usb3, 0x02, 0x009c);
// Enable SSC
    bcm_ns_usb3_mdio_phy_write(usb3, BCM_NS_USB3_PHY_BASE_ADDR_REG,
    BCM_NS_USB3_PHY_TX_PMD_BLOCK);
    bcm_ns_usb3_mdio_phy_write(usb3, 0x02, 0x21d3);
    bcm_ns_usb3_mdio_phy_write(usb3, BCM_NS_USB3_TX_PMD_CONTROL1, 0x1003);
// Deasserting USB3 system reset
    writel(0, usb3.dmp + BCMA_RESET_CTL);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bcm_ns_usb3_phy_init(phy: *mut phy) -> c_int {
    static int bcm_ns_usb3_phy_init(struct phy *phy)
    {
    struct bcm_ns_usb3 *usb3 = phy_get_drvdata(phy);
    int err;
// Perform USB3 system soft reset
    writel(BCMA_RESET_CTL_RESET, usb3.dmp + BCMA_RESET_CTL);
    switch (usb3.family) {
    case BCM_NS_AX:
    err = bcm_ns_usb3_phy_init_ns_ax(usb3);
    break;
    case BCM_NS_BX:
    err = bcm_ns_usb3_phy_init_ns_bx(usb3);
    break;
    default:
    WARN_ON(1);
    err = -ENOTSUPP;
    }
    return err;
    }
    static const struct phy_ops ops = {
    .init		= bcm_ns_usb3_phy_init,
    .owner		= THIS_MODULE,
    };
//
// MDIO driver code
//
    static int bcm_ns_usb3_mdio_phy_write(struct bcm_ns_usb3 *usb3, u16 reg,
    u16 value)
    {
    struct mdio_device *mdiodev = usb3.mdiodev;
    return mdiodev_write(mdiodev, reg, value);
    }
#[no_mangle]
unsafe extern "C" fn bcm_ns_usb3_mdio_probe(mdiodev: *mut mdio_device) -> c_int {
    static int bcm_ns_usb3_mdio_probe(struct mdio_device *mdiodev)
    {
    struct device *dev = &mdiodev.dev;
    struct phy_provider *phy_provider;
    struct device_node *syscon_np;
    struct bcm_ns_usb3 *usb3;
    struct resource res;
    int err;
    usb3 = devm_kzalloc(dev, sizeof(*usb3), GFP_KERNEL);
    if (!usb3)
    return -ENOMEM;
    usb3.dev = dev;
    usb3.mdiodev = mdiodev;
    usb3.family = (unsigned long)device_get_match_data(dev);
    syscon_np = of_parse_phandle(dev.of_node, "usb3-dmp-syscon", 0);
    err = of_address_to_resource(syscon_np, 0, &res);
    of_node_put(syscon_np);
    if (err)
    return err;
    usb3.dmp = devm_ioremap_resource(dev, &res);
    if (IS_ERR(usb3.dmp))
    return PTR_ERR(usb3.dmp);
    usb3.phy = devm_phy_create(dev, core::ptr::null_mut(), &ops);
    if (IS_ERR(usb3.phy)) {
    dev_err(dev, "Failed to create PHY\n");
    return PTR_ERR(usb3.phy);
    }
    phy_set_drvdata(usb3.phy, usb3);
    phy_provider = devm_of_phy_provider_register(dev, of_phy_simple_xlate);
    return PTR_ERR_OR_ZERO(phy_provider);
    }
    static struct mdio_driver bcm_ns_usb3_mdio_driver = {
    .mdiodrv = {
    .driver = {
    .name = "bcm_ns_mdio_usb3",
    .of_match_table = bcm_ns_usb3_id_table,
    },
    },
    .probe = bcm_ns_usb3_mdio_probe,
    };
    mdio_module_driver(bcm_ns_usb3_mdio_driver);
    MODULE_DESCRIPTION("Broadcom Northstar USB 3.0 PHY Driver");
    MODULE_LICENSE("GPL v2");
