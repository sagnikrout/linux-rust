//! Automatically rewritten from C to Rust
//! Source: drivers/net/phy/xilinx_gmii2rgmii.c
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
// Xilinx GMII2RGMII Converter driver
//
// Copyright (C) 2016 Xilinx, Inc.
// Copyright (C) 2016 Andrew Lunn <andrew@lunn.ch>
//
// Author: Andrew Lunn <andrew@lunn.ch>
// Author: Kedareswara rao Appana <appanad@xilinx.com>
//
// Description:
// This driver is developed for Xilinx GMII2RGMII Converter
//

pub const XILINX_GMII2RGMII_REG: c_uint = 0x10;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gmii2rgmii {
    pub phy_dev: *mut phy_device,
    pub phy_drv: *const phy_driver,
    pub conv_phy_drv: phy_driver,
    pub mdio: *mut mdio_device,
}

#[no_mangle]
unsafe extern "C" fn xgmiitorgmii_configure(priv: *mut gmii2rgmii, speed: c_int) {
    static void xgmiitorgmii_configure(struct gmii2rgmii *priv, int speed)
    {
    struct mii_bus *bus = priv.mdio.bus;
    let mut addr: c_int = priv.mdio.addr;
    u16 val;
    val = mdiobus_read(bus, addr, XILINX_GMII2RGMII_REG);
    val &= ~XILINX_GMII2RGMII_SPEED_MASK;
    if (speed == SPEED_1000)
    val |= BMCR_SPEED1000;
#[no_mangle]
pub unsafe extern "C" fn if(SPEED_100: speed ==) -> else {
    else if (speed == SPEED_100)
    val |= BMCR_SPEED100;
    else
    val |= BMCR_SPEED10;
    mdiobus_write(bus, addr, XILINX_GMII2RGMII_REG, val);
    }
#[no_mangle]
unsafe extern "C" fn xgmiitorgmii_read_status(phydev: *mut phy_device) -> c_int {
    static int xgmiitorgmii_read_status(struct phy_device *phydev)
    {
    struct gmii2rgmii *priv = mdiodev_get_drvdata(&phydev.mdio);
    int err;
    if (priv.phy_drv.read_status)
    err = priv.phy_drv.read_status(phydev);
    else
    err = genphy_read_status(phydev);
    if (err < 0)
    return err;
    xgmiitorgmii_configure(priv, phydev.speed);
    return 0;
    }
    static int xgmiitorgmii_set_loopback(struct phy_device *phydev, bool enable,
    int speed)
    {
    struct gmii2rgmii *priv = mdiodev_get_drvdata(&phydev.mdio);
    int err;
    if (priv.phy_drv.set_loopback)
    err = priv.phy_drv.set_loopback(phydev, enable, speed);
    else
    err = genphy_loopback(phydev, enable, speed);
    if (err < 0)
    return err;
    xgmiitorgmii_configure(priv, phydev.speed);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn xgmiitorgmii_probe(mdiodev: *mut mdio_device) -> c_int {
    static int xgmiitorgmii_probe(struct mdio_device *mdiodev)
    {
    struct device *dev = &mdiodev.dev;
    struct device_node *np = dev.of_node, *phy_node;
    struct gmii2rgmii *priv;
    struct clk *clkin;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    clkin = devm_clk_get_optional_enabled(dev, core::ptr::null_mut());
    if (IS_ERR(clkin))
    return dev_err_probe(dev, PTR_ERR(clkin),
    "Failed to get and enable clock from Device Tree\n");
    phy_node = of_parse_phandle(np, "phy-handle", 0);
    if (!phy_node) {
    dev_err(dev, "Couldn't parse phy-handle\n");
    return -ENODEV;
    }
    priv.phy_dev = of_phy_find_device(phy_node);
    of_node_put(phy_node);
    if (!priv.phy_dev) {
    dev_info(dev, "Couldn't find phydev\n");
    return -EPROBE_DEFER;
    }
    if (!priv.phy_dev.drv) {
    dev_info(dev, "Attached phy not ready\n");
    put_device(&priv.phy_dev.mdio.dev);
    return -EPROBE_DEFER;
    }
    priv.mdio = mdiodev;
    priv.phy_drv = priv.phy_dev.drv;
    memcpy(&priv.conv_phy_drv, priv.phy_dev.drv,
    sizeof(struct phy_driver));
    priv.conv_phy_drv.read_status = xgmiitorgmii_read_status;
    priv.conv_phy_drv.set_loopback = xgmiitorgmii_set_loopback;
    mdiodev_set_drvdata(&priv.phy_dev.mdio, priv);
    priv.phy_dev.drv = &priv.conv_phy_drv;
    return 0;
    }
    static const struct of_device_id xgmiitorgmii_of_match[] = {
    { .compatible = "xlnx,gmii-to-rgmii-1.0" },
    {},
    };
    MODULE_DEVICE_TABLE(of, xgmiitorgmii_of_match);
    static struct mdio_driver xgmiitorgmii_driver = {
    .probe	= xgmiitorgmii_probe,
    .mdiodrv.driver = {
    .name = "xgmiitorgmii",
    .of_match_table = xgmiitorgmii_of_match,
    },
    };
    mdio_module_driver(xgmiitorgmii_driver);
    MODULE_DESCRIPTION("Xilinx GMII2RGMII converter driver");
    MODULE_LICENSE("GPL");
