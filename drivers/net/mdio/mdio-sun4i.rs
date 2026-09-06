//! Automatically rewritten from C to Rust
//! Source: drivers/net/mdio/mdio-sun4i.c
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
// Allwinner EMAC MDIO interface driver
//
// Copyright 2012-2013 Stefan Roese <sr@denx.de>
// Copyright 2013 Maxime Ripard <maxime.ripard@free-electrons.com>
//
// Based on the Linux driver provided by Allwinner:
// Copyright (C) 1997  Sten Wang
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sun4i_mdio_data {
    pub membase: *mut void __iomem,
    pub regulator: *mut regulator,
}

#[no_mangle]
unsafe extern "C" fn sun4i_mdio_read(bus: *mut mii_bus, mii_id: c_int, regnum: c_int) -> c_int {
    static int sun4i_mdio_read(struct mii_bus *bus, int mii_id, int regnum)
    {
    struct sun4i_mdio_data *data = bus.priv;
    unsigned long timeout_jiffies;
    int value;
// issue the phy address and reg
    writel((mii_id << 8) | regnum, data.membase + EMAC_MAC_MADR_REG);
// pull up the phy io line
    writel(0x1, data.membase + EMAC_MAC_MCMD_REG);
// Wait read complete
    timeout_jiffies = jiffies + MDIO_TIMEOUT;
    while (readl(data.membase + EMAC_MAC_MIND_REG) & 0x1) {
    if (time_is_before_jiffies(timeout_jiffies))
    return -ETIMEDOUT;
    msleep(1);
    }
// push down the phy io line
    writel(0x0, data.membase + EMAC_MAC_MCMD_REG);
// and read data
    value = readl(data.membase + EMAC_MAC_MRDD_REG);
    return value;
    }
    static int sun4i_mdio_write(struct mii_bus *bus, int mii_id, int regnum,
    u16 value)
    {
    struct sun4i_mdio_data *data = bus.priv;
    unsigned long timeout_jiffies;
// issue the phy address and reg
    writel((mii_id << 8) | regnum, data.membase + EMAC_MAC_MADR_REG);
// pull up the phy io line
    writel(0x1, data.membase + EMAC_MAC_MCMD_REG);
// Wait read complete
    timeout_jiffies = jiffies + MDIO_TIMEOUT;
    while (readl(data.membase + EMAC_MAC_MIND_REG) & 0x1) {
    if (time_is_before_jiffies(timeout_jiffies))
    return -ETIMEDOUT;
    msleep(1);
    }
// push down the phy io line
    writel(0x0, data.membase + EMAC_MAC_MCMD_REG);
// and write data
    writel(value, data.membase + EMAC_MAC_MWTD_REG);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sun4i_mdio_probe(pdev: *mut platform_device) -> c_int {
    static int sun4i_mdio_probe(struct platform_device *pdev)
    {
    struct device_node *np = pdev.dev.of_node;
    struct mii_bus *bus;
    struct sun4i_mdio_data *data;
    int ret;
    bus = mdiobus_alloc_size(sizeof(*data));
    if (!bus)
    return -ENOMEM;
    bus.name = "sun4i_mii_bus";
    bus.read = &sun4i_mdio_read;
    bus.write = &sun4i_mdio_write;
    snprintf(bus.id, MII_BUS_ID_SIZE, "%s-mii", dev_name(&pdev.dev));
    bus.parent = &pdev.dev;
    data = bus.priv;
    data.membase = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(data.membase)) {
    ret = PTR_ERR(data.membase);
    goto err_out_free_mdiobus;
    }
    data.regulator = devm_regulator_get(&pdev.dev, "phy");
    if (IS_ERR(data.regulator)) {
    if (PTR_ERR(data.regulator) == -EPROBE_DEFER) {
    ret = -EPROBE_DEFER;
    goto err_out_free_mdiobus;
    }
    dev_info(&pdev.dev, "no regulator found\n");
    data.regulator = core::ptr::null_mut();
    } else {
    ret = regulator_enable(data.regulator);
    if (ret)
    goto err_out_free_mdiobus;
    }
    ret = of_mdiobus_register(bus, np);
    if (ret < 0)
    goto err_out_disable_regulator;
    platform_set_drvdata(pdev, bus);
    return 0;
    err_out_disable_regulator:
    if (data.regulator)
    regulator_disable(data.regulator);
    err_out_free_mdiobus:
    mdiobus_free(bus);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn sun4i_mdio_remove(pdev: *mut platform_device) {
    static void sun4i_mdio_remove(struct platform_device *pdev)
    {
    struct mii_bus *bus = platform_get_drvdata(pdev);
    struct sun4i_mdio_data *data = bus.priv;
    mdiobus_unregister(bus);
    if (data.regulator)
    regulator_disable(data.regulator);
    mdiobus_free(bus);
    }
    static const struct of_device_id sun4i_mdio_dt_ids[] = {
    { .compatible = "allwinner,sun4i-a10-mdio" },
// Deprecated
    { .compatible = "allwinner,sun4i-mdio" },
    { }
    };
    MODULE_DEVICE_TABLE(of, sun4i_mdio_dt_ids);
    static struct platform_driver sun4i_mdio_driver = {
    .probe = sun4i_mdio_probe,
    .remove = sun4i_mdio_remove,
    .driver = {
    .name = "sun4i-mdio",
    .of_match_table = sun4i_mdio_dt_ids,
    },
    };
    module_platform_driver(sun4i_mdio_driver);
    MODULE_DESCRIPTION("Allwinner EMAC MDIO interface driver");
    MODULE_AUTHOR("Maxime Ripard <maxime.ripard@free-electrons.com>");
    MODULE_LICENSE("GPL v2");
