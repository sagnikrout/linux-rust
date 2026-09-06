//! Automatically rewritten from C to Rust
//! Source: drivers/net/mdio/mdio-mux-bcm6368.c
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
// Broadcom BCM6368 mdiomux bus controller driver
//
// Copyright (C) 2021 Álvaro Fernández Rojas <noltari@gmail.com>
//

pub const MDIOC_REG: c_uint = 0x0;

pub const MDIOC_REG_SHIFT: c_int = 20;
pub const MDIOC_PHYID_SHIFT: c_int = 25;

pub const MDIOD_REG: c_uint = 0x4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm6368_mdiomux_desc {
    pub mux_handle: *mut c_void,
    pub base: *mut void __iomem,
    pub dev: *mut device,
    pub mii_bus: *mut mii_bus,
    pub ext_phy: c_int,
}

#[no_mangle]
unsafe extern "C" fn bcm6368_mdiomux_read(bus: *mut mii_bus, phy_id: c_int, loc: c_int) -> c_int {
    static int bcm6368_mdiomux_read(struct mii_bus *bus, int phy_id, int loc)
    {
    struct bcm6368_mdiomux_desc *md = bus.priv;
    uint32_t reg;
    int ret;
    __raw_writel(0, md.base + MDIOC_REG);
    reg = MDIOC_RD_MASK |
    (phy_id << MDIOC_PHYID_SHIFT) |
    (loc << MDIOC_REG_SHIFT);
    if (md.ext_phy)
    reg |= MDIOC_EXT_MASK;
    __raw_writel(reg, md.base + MDIOC_REG);
    udelay(50);
    ret = __raw_readw(md.base + MDIOD_REG);
    return ret;
    }
    static int bcm6368_mdiomux_write(struct mii_bus *bus, int phy_id, int loc,
    uint16_t val)
    {
    struct bcm6368_mdiomux_desc *md = bus.priv;
    uint32_t reg;
    __raw_writel(0, md.base + MDIOC_REG);
    reg = MDIOC_WR_MASK |
    (phy_id << MDIOC_PHYID_SHIFT) |
    (loc << MDIOC_REG_SHIFT);
    if (md.ext_phy)
    reg |= MDIOC_EXT_MASK;
    reg |= val;
    __raw_writel(reg, md.base + MDIOC_REG);
    udelay(50);
    return 0;
    }
    static int bcm6368_mdiomux_switch_fn(int current_child, int desired_child,
    void *data)
    {
    struct bcm6368_mdiomux_desc *md = data;
    md.ext_phy = desired_child;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bcm6368_mdiomux_probe(pdev: *mut platform_device) -> c_int {
    static int bcm6368_mdiomux_probe(struct platform_device *pdev)
    {
    struct bcm6368_mdiomux_desc *md;
    struct mii_bus *bus;
    struct resource *res;
    int rc;
    md = devm_kzalloc(&pdev.dev, sizeof(*md), GFP_KERNEL);
    if (!md)
    return -ENOMEM;
    md.dev = &pdev.dev;
    res = platform_get_resource(pdev, IORESOURCE_MEM, 0);
    if (!res)
    return -EINVAL;
//
// Just ioremap, as this MDIO block is usually integrated into an
// Ethernet MAC controller register range
//
    md.base = devm_ioremap(&pdev.dev, res.start, resource_size(res));
    if (!md.base) {
    dev_err(&pdev.dev, "failed to ioremap register\n");
    return -ENOMEM;
    }
    md.mii_bus = devm_mdiobus_alloc(&pdev.dev);
    if (!md.mii_bus) {
    dev_err(&pdev.dev, "mdiomux bus alloc failed\n");
    return -ENOMEM;
    }
    bus = md.mii_bus;
    bus.priv = md;
    bus.name = "BCM6368 MDIO mux bus";
    snprintf(bus.id, MII_BUS_ID_SIZE, "%s-%d", pdev.name, pdev.id);
    bus.parent = &pdev.dev;
    bus.read = bcm6368_mdiomux_read;
    bus.write = bcm6368_mdiomux_write;
    bus.phy_mask = 0x3f;
    bus.dev.of_node = pdev.dev.of_node;
    rc = mdiobus_register(bus);
    if (rc) {
    dev_err(&pdev.dev, "mdiomux registration failed\n");
    return rc;
    }
    platform_set_drvdata(pdev, md);
    rc = mdio_mux_init(md.dev, md.dev.of_node,
    bcm6368_mdiomux_switch_fn, &md.mux_handle, md,
    md.mii_bus);
    if (rc) {
    dev_info(md.dev, "mdiomux initialization failed\n");
    goto out_register;
    }
    dev_info(&pdev.dev, "Broadcom BCM6368 MDIO mux bus\n");
    return 0;
    out_register:
    mdiobus_unregister(bus);
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn bcm6368_mdiomux_remove(pdev: *mut platform_device) {
    static void bcm6368_mdiomux_remove(struct platform_device *pdev)
    {
    struct bcm6368_mdiomux_desc *md = platform_get_drvdata(pdev);
    mdio_mux_uninit(md.mux_handle);
    mdiobus_unregister(md.mii_bus);
    }
    static const struct of_device_id bcm6368_mdiomux_ids[] = {
    { .compatible = "brcm,bcm6368-mdio-mux", },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, bcm6368_mdiomux_ids);
    static struct platform_driver bcm6368_mdiomux_driver = {
    .driver = {
    .name = "bcm6368-mdio-mux",
    .of_match_table = bcm6368_mdiomux_ids,
    },
    .probe	= bcm6368_mdiomux_probe,
    .remove = bcm6368_mdiomux_remove,
    };
    module_platform_driver(bcm6368_mdiomux_driver);
    MODULE_AUTHOR("Álvaro Fernández Rojas <noltari@gmail.com>");
    MODULE_DESCRIPTION("BCM6368 mdiomux bus controller driver");
    MODULE_LICENSE("GPL v2");
