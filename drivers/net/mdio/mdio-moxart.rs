//! Automatically rewritten from C to Rust
//! Source: drivers/net/mdio/mdio-moxart.c
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
// MOXA ART Ethernet (RTL8201CP) MDIO interface driver
//
// Copyright (C) 2013 Jonas Jensen <jonas.jensen@gmail.com>
//

pub const REG_PHY_CTRL: c_int = 0;
pub const REG_PHY_WRITE_DATA: c_int = 4;
// REG_PHY_CTRL

pub const REGAD_MASK: c_uint = 0x3e00000;
pub const PHYAD_MASK: c_uint = 0x1f0000;
pub const MIIRDATA_MASK: c_uint = 0xffff;
// REG_PHY_WRITE_DATA
pub const MIIWDATA_MASK: c_uint = 0xffff;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct moxart_mdio_data {
    pub base: *mut void __iomem,
}

#[no_mangle]
unsafe extern "C" fn moxart_mdio_read(bus: *mut mii_bus, mii_id: c_int, regnum: c_int) -> c_int {
    static int moxart_mdio_read(struct mii_bus *bus, int mii_id, int regnum)
    {
    struct moxart_mdio_data *data = bus.priv;
    let mut ctrl: u32 = 0;
    let mut count: c_uint = 5;
    dev_dbg(&bus.dev, "%s\n", __func__);
    ctrl |= MIIRD | ((mii_id << 16) & PHYAD_MASK) |
    ((regnum << 21) & REGAD_MASK);
    writel(ctrl, data.base + REG_PHY_CTRL);
    do {
    ctrl = readl(data.base + REG_PHY_CTRL);
    if (!(ctrl & MIIRD))
    return ctrl & MIIRDATA_MASK;
    mdelay(10);
    count--;
    } while (count > 0);
    dev_dbg(&bus.dev, "%s timed out\n", __func__);
    return -ETIMEDOUT;
    }
    static int moxart_mdio_write(struct mii_bus *bus, int mii_id,
    int regnum, u16 value)
    {
    struct moxart_mdio_data *data = bus.priv;
    let mut ctrl: u32 = 0;
    let mut count: c_uint = 5;
    dev_dbg(&bus.dev, "%s\n", __func__);
    ctrl |= MIIWR | ((mii_id << 16) & PHYAD_MASK) |
    ((regnum << 21) & REGAD_MASK);
    value &= MIIWDATA_MASK;
    writel(value, data.base + REG_PHY_WRITE_DATA);
    writel(ctrl, data.base + REG_PHY_CTRL);
    do {
    ctrl = readl(data.base + REG_PHY_CTRL);
    if (!(ctrl & MIIWR))
    return 0;
    mdelay(10);
    count--;
    } while (count > 0);
    dev_dbg(&bus.dev, "%s timed out\n", __func__);
    return -ETIMEDOUT;
    }
#[no_mangle]
unsafe extern "C" fn moxart_mdio_reset(bus: *mut mii_bus) -> c_int {
    static int moxart_mdio_reset(struct mii_bus *bus)
    {
    int data, i;
    for (i = 0; i < PHY_MAX_ADDR; i++) {
    data = moxart_mdio_read(bus, i, MII_BMCR);
    if (data < 0)
    continue;
    data |= BMCR_RESET;
    if (moxart_mdio_write(bus, i, MII_BMCR, data) < 0)
    continue;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn moxart_mdio_probe(pdev: *mut platform_device) -> c_int {
    static int moxart_mdio_probe(struct platform_device *pdev)
    {
    struct device_node *np = pdev.dev.of_node;
    struct mii_bus *bus;
    struct moxart_mdio_data *data;
    int ret, i;
    bus = mdiobus_alloc_size(sizeof(*data));
    if (!bus)
    return -ENOMEM;
    bus.name = "MOXA ART Ethernet MII";
    bus.read = &moxart_mdio_read;
    bus.write = &moxart_mdio_write;
    bus.reset = &moxart_mdio_reset;
    snprintf(bus.id, MII_BUS_ID_SIZE, "%s-%d-mii", pdev.name, pdev.id);
    bus.parent = &pdev.dev;
// Setting PHY_MAC_INTERRUPT here even if it has no effect,
// of_mdiobus_register() sets these PHY_POLL.
// Ideally, the interrupt from MAC controller could be used to
// detect link state changes, not polling, i.e. if there was
// a way phy_driver could set PHY_HAS_INTERRUPT but have that
// interrupt handled in ethernet drivercode.
//
    for (i = 0; i < PHY_MAX_ADDR; i++)
    bus.irq[i] = PHY_MAC_INTERRUPT;
    data = bus.priv;
    data.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(data.base)) {
    ret = PTR_ERR(data.base);
    goto err_out_free_mdiobus;
    }
    ret = of_mdiobus_register(bus, np);
    if (ret < 0)
    goto err_out_free_mdiobus;
    platform_set_drvdata(pdev, bus);
    return 0;
    err_out_free_mdiobus:
    mdiobus_free(bus);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn moxart_mdio_remove(pdev: *mut platform_device) {
    static void moxart_mdio_remove(struct platform_device *pdev)
    {
    struct mii_bus *bus = platform_get_drvdata(pdev);
    mdiobus_unregister(bus);
    mdiobus_free(bus);
    }
    static const struct of_device_id moxart_mdio_dt_ids[] = {
    { .compatible = "moxa,moxart-mdio" },
    { }
    };
    MODULE_DEVICE_TABLE(of, moxart_mdio_dt_ids);
    static struct platform_driver moxart_mdio_driver = {
    .probe = moxart_mdio_probe,
    .remove = moxart_mdio_remove,
    .driver = {
    .name = "moxart-mdio",
    .of_match_table = moxart_mdio_dt_ids,
    },
    };
    module_platform_driver(moxart_mdio_driver);
    MODULE_DESCRIPTION("MOXA ART MDIO interface driver");
    MODULE_AUTHOR("Jonas Jensen <jonas.jensen@gmail.com>");
    MODULE_LICENSE("GPL v2");
