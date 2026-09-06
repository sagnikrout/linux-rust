//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/freescale/fs_enet/mii-fec.c
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
// Combined Ethernet driver for Motorola MPC8xx and MPC82xx.
//
// Copyright (c) 2003 Intracom S.A.
// by Pantelis Antoniou <panto@intracom.gr>
//
// 2005 (c) MontaVista Software, Inc.
// Vitaly Bordug <vbordug@ru.mvista.com>
//

// Make MII read/write commands for the FEC.
//

pub const mk_mii_end: c_int = 0;
pub const FEC_MII_LOOPS: c_int = 10000;
#[no_mangle]
unsafe extern "C" fn fs_enet_fec_mii_read(bus: *mut mii_bus, phy_id: c_int, location: c_int) -> c_int {
    static int fs_enet_fec_mii_read(struct mii_bus *bus , int phy_id, int location)
    {
    let mut fec: *mut fec_info = bus.priv;
    struct fec __iomem *fecp = fec.fecp;
    int i, ret = -1;
    BUG_ON((in_be32(&fecp.fec_r_cntrl) & FEC_RCNTRL_MII_MODE) == 0);
// Add PHY address to register command.
    out_be32(&fecp.fec_mii_data, (phy_id << 23) | mk_mii_read(location));
    for (i = 0; i < FEC_MII_LOOPS; i++)
    if ((in_be32(&fecp.fec_ievent) & FEC_ENET_MII) != 0)
    break;
    if (i < FEC_MII_LOOPS) {
    out_be32(&fecp.fec_ievent, FEC_ENET_MII);
    ret = in_be32(&fecp.fec_mii_data) & 0xffff;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn fs_enet_fec_mii_write(bus: *mut mii_bus, phy_id: c_int, location: c_int, val: u16) -> c_int {
    static int fs_enet_fec_mii_write(struct mii_bus *bus, int phy_id, int location, u16 val)
    {
    let mut fec: *mut fec_info = bus.priv;
    struct fec __iomem *fecp = fec.fecp;
    int i;
// this must never happen
    BUG_ON((in_be32(&fecp.fec_r_cntrl) & FEC_RCNTRL_MII_MODE) == 0);
// Add PHY address to register command.
    out_be32(&fecp.fec_mii_data, (phy_id << 23) | mk_mii_write(location, val));
    for (i = 0; i < FEC_MII_LOOPS; i++)
    if ((in_be32(&fecp.fec_ievent) & FEC_ENET_MII) != 0)
    break;
    if (i < FEC_MII_LOOPS)
    out_be32(&fecp.fec_ievent, FEC_ENET_MII);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn fs_enet_mdio_probe(ofdev: *mut platform_device) -> c_int {
    static int fs_enet_mdio_probe(struct platform_device *ofdev)
    {
    struct resource res;
    struct mii_bus *new_bus;
    struct fec_info *fec;
    int (*get_bus_freq)(struct device *);
    let mut ret: c_int = -ENOMEM, clock, speed;
    get_bus_freq = device_get_match_data(&ofdev.dev);
    new_bus = mdiobus_alloc();
    if (!new_bus)
    goto out;
    fec = kzalloc_obj(struct fec_info);
    if (!fec)
    goto out_mii;
    new_bus.priv = fec;
    new_bus.name = "FEC MII Bus";
    new_bus.read = &fs_enet_fec_mii_read;
    new_bus.write = &fs_enet_fec_mii_write;
    ret = of_address_to_resource(ofdev.dev.of_node, 0, &res);
    if (ret)
    goto out_res;
    snprintf(new_bus.id, MII_BUS_ID_SIZE, "%pap", &res.start);
    fec.fecp = ioremap(res.start, resource_size(&res));
    if (!fec.fecp) {
    ret = -ENOMEM;
    goto out_fec;
    }
    if (get_bus_freq) {
    clock = get_bus_freq(&ofdev.dev);
    if (!clock) {
// Use maximum divider if clock is unknown
    dev_warn(&ofdev.dev, "could not determine IPS clock\n");
    clock = 0x3F * 5000000;
    }
    } else
    clock = ppc_proc_freq;
//
// Scale for a MII clock <= 2.5 MHz
// Note that only 6 bits (25:30) are available for MII speed.
//
    speed = (clock + 4999999) / 5000000;
    if (speed > 0x3F) {
    speed = 0x3F;
    dev_err(&ofdev.dev,
    "MII clock (%d Hz) exceeds max (2.5 MHz)\n",
    clock / speed);
    }
    fec.mii_speed = speed << 1;
    setbits32(&fec.fecp.fec_r_cntrl, FEC_RCNTRL_MII_MODE);
    setbits32(&fec.fecp.fec_ecntrl, FEC_ECNTRL_PINMUX |
    FEC_ECNTRL_ETHER_EN);
    out_be32(&fec.fecp.fec_ievent, FEC_ENET_MII);
    clrsetbits_be32(&fec.fecp.fec_mii_speed, 0x7E, fec.mii_speed);
    new_bus.phy_mask = ~0;
    new_bus.parent = &ofdev.dev;
    platform_set_drvdata(ofdev, new_bus);
    ret = of_mdiobus_register(new_bus, ofdev.dev.of_node);
    if (ret)
    goto out_unmap_regs;
    return 0;
    out_unmap_regs:
    iounmap(fec.fecp);
    out_res:
    out_fec:
    kfree(fec);
    out_mii:
    mdiobus_free(new_bus);
    out:
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn fs_enet_mdio_remove(ofdev: *mut platform_device) {
    static void fs_enet_mdio_remove(struct platform_device *ofdev)
    {
    struct mii_bus *bus = platform_get_drvdata(ofdev);
    struct fec_info *fec = bus.priv;
    mdiobus_unregister(bus);
    iounmap(fec.fecp);
    kfree(fec);
    mdiobus_free(bus);
    }
    static const struct of_device_id fs_enet_mdio_fec_match[] = {
    {
    .compatible = "fsl,pq1-fec-mdio",
    },

    {
    .compatible = "fsl,mpc5121-fec-mdio",
    .data = mpc5xxx_get_bus_frequency,
    },

    {},
    };
    MODULE_DEVICE_TABLE(of, fs_enet_mdio_fec_match);
    static struct platform_driver fs_enet_fec_mdio_driver = {
    .driver = {
    .name = "fsl-fec-mdio",
    .of_match_table = fs_enet_mdio_fec_match,
    },
    .probe = fs_enet_mdio_probe,
    .remove = fs_enet_mdio_remove,
    };
    module_platform_driver(fs_enet_fec_mdio_driver);
    MODULE_LICENSE("GPL");
