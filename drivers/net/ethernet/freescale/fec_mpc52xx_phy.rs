//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/freescale/fec_mpc52xx_phy.c
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


//
// Driver for the MPC5200 Fast Ethernet Controller - MDIO bus driver
//
// Copyright (C) 2007  Domen Puncer, Telargo, Inc.
// Copyright (C) 2008  Wolfram Sang, Pengutronix
//
// This file is licensed under the terms of the GNU General Public License
// version 2. This program is licensed "as is" without any warranty of any
// kind, whether express or implied.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpc52xx_fec_mdio_priv {
    pub regs: *mut mpc52xx_fec __iomem,
}

    static int mpc52xx_fec_mdio_transfer(struct mii_bus *bus, int phy_id,
    int reg, u32 value)
    {
    struct mpc52xx_fec_mdio_priv *priv = bus.priv;
    struct mpc52xx_fec __iomem *fec = priv.regs;
    let mut tries: c_int = 3;
    value |= (phy_id << FEC_MII_DATA_PA_SHIFT) & FEC_MII_DATA_PA_MSK;
    value |= (reg << FEC_MII_DATA_RA_SHIFT) & FEC_MII_DATA_RA_MSK;
    out_be32(&fec.ievent, FEC_IEVENT_MII);
    out_be32(&fec.mii_data, value);
// wait for it to finish, this takes about 23 us on lite5200b
    while (!(in_be32(&fec.ievent) & FEC_IEVENT_MII) && --tries)
    msleep(1);
    if (!tries)
    return -ETIMEDOUT;
    return value & FEC_MII_DATA_OP_RD ?
    in_be32(&fec.mii_data) & FEC_MII_DATA_DATAMSK : 0;
    }
#[no_mangle]
unsafe extern "C" fn mpc52xx_fec_mdio_read(bus: *mut mii_bus, phy_id: c_int, reg: c_int) -> c_int {
    static int mpc52xx_fec_mdio_read(struct mii_bus *bus, int phy_id, int reg)
    {
    return mpc52xx_fec_mdio_transfer(bus, phy_id, reg, FEC_MII_READ_FRAME);
    }
    static int mpc52xx_fec_mdio_write(struct mii_bus *bus, int phy_id, int reg,
    u16 data)
    {
    return mpc52xx_fec_mdio_transfer(bus, phy_id, reg,
    data | FEC_MII_WRITE_FRAME);
    }
#[no_mangle]
unsafe extern "C" fn mpc52xx_fec_mdio_probe(of: *mut platform_device) -> c_int {
    static int mpc52xx_fec_mdio_probe(struct platform_device *of)
    {
    struct device *dev = &of.dev;
    struct device_node *np = of.dev.of_node;
    struct mii_bus *bus;
    struct mpc52xx_fec_mdio_priv *priv;
    struct resource res;
    int err;
    bus = mdiobus_alloc();
    if (bus == core::ptr::null_mut())
    return -ENOMEM;
    priv = kzalloc_obj(*priv);
    if (priv == core::ptr::null_mut()) {
    err = -ENOMEM;
    goto out_free;
    }
    bus.name = "mpc52xx MII bus";
    bus.read = mpc52xx_fec_mdio_read;
    bus.write = mpc52xx_fec_mdio_write;
// setup registers
    err = of_address_to_resource(np, 0, &res);
    if (err)
    goto out_free;
    priv.regs = ioremap(res.start, resource_size(&res));
    if (priv.regs == core::ptr::null_mut()) {
    err = -ENOMEM;
    goto out_free;
    }
    snprintf(bus.id, MII_BUS_ID_SIZE, "%pa", &res.start);
    bus.priv = priv;
    bus.parent = dev;
    dev_set_drvdata(dev, bus);
// set MII speed
    out_be32(&priv.regs.mii_speed, ((mpc5xxx_get_bus_frequency(dev) >> 20) / 5) << 1);
    err = of_mdiobus_register(bus, np);
    if (err)
    goto out_unmap;
    return 0;
    out_unmap:
    iounmap(priv.regs);
    out_free:
    kfree(priv);
    mdiobus_free(bus);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn mpc52xx_fec_mdio_remove(of: *mut platform_device) {
    static void mpc52xx_fec_mdio_remove(struct platform_device *of)
    {
    struct mii_bus *bus = platform_get_drvdata(of);
    struct mpc52xx_fec_mdio_priv *priv = bus.priv;
    mdiobus_unregister(bus);
    iounmap(priv.regs);
    kfree(priv);
    mdiobus_free(bus);
    }
    static const struct of_device_id mpc52xx_fec_mdio_match[] = {
    { .compatible = "fsl,mpc5200b-mdio", },
    { .compatible = "fsl,mpc5200-mdio", },
    { .compatible = "mpc5200b-fec-phy", },
    {}
    };
    MODULE_DEVICE_TABLE(of, mpc52xx_fec_mdio_match);
    struct platform_driver mpc52xx_fec_mdio_driver = {
    .driver = {
    .name = "mpc5200b-fec-phy",
    .owner = THIS_MODULE,
    .of_match_table = mpc52xx_fec_mdio_match,
    },
    .probe = mpc52xx_fec_mdio_probe,
    .remove = mpc52xx_fec_mdio_remove,
    };
// let fec driver call it, since this has to be registered before it
    EXPORT_SYMBOL_GPL(mpc52xx_fec_mdio_driver);
    MODULE_DESCRIPTION("MPC52xx FEC MDIO bus driver");
    MODULE_LICENSE("Dual BSD/GPL");
