//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/samsung/sxgbe/sxgbe_mdio.c
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
// 10G controller driver for Samsung SoCs
//
// Copyright (C) 2013 Samsung Electronics Co., Ltd.
// http://www.samsung.com
//
// Author: Siva Reddy Kallam <siva.kallam@samsung.com>
//

pub const SXGBE_SMA_WRITE_CMD: c_uint = 0x01 /* write command */;
pub const SXGBE_SMA_PREAD_CMD: c_uint = 0x02 /* post read  increament address */;
pub const SXGBE_SMA_READ_CMD: c_uint = 0x03 /* read command */;
pub const SXGBE_SMA_SKIP_ADDRFRM: c_uint = 0x00040000 /* skip the address frame */;
pub const SXGBE_MII_BUSY: c_uint = 0x00400000 /* mii busy */;
#[no_mangle]
unsafe extern "C" fn sxgbe_mdio_busy_wait(ioaddr: *mut void __iomem, mii_data: c_uint) -> c_int {
    static int sxgbe_mdio_busy_wait(void __iomem *ioaddr, unsigned int mii_data)
    {
    unsigned long fin_time = jiffies + 3 * HZ; /* 3 seconds */
    while (!time_after(jiffies, fin_time)) {
    if (!(readl(ioaddr + mii_data) & SXGBE_MII_BUSY))
    return 0;
    cpu_relax();
    }
    return -EBUSY;
    }
    static void sxgbe_mdio_ctrl_data(struct sxgbe_priv_data *sp, u32 cmd,
    u16 phydata)
    {
    let mut reg: u32 = phydata;
    reg |= (cmd << 16) | SXGBE_SMA_SKIP_ADDRFRM |
    ((sp.clk_csr & 0x7) << 19) | SXGBE_MII_BUSY;
    writel(reg, sp.ioaddr + sp.hw.mii.data);
    }
    static void sxgbe_mdio_c45(struct sxgbe_priv_data *sp, u32 cmd, int phyaddr,
    int devad, int phyreg, u16 phydata)
    {
    u32 reg;
// set mdio address register
    reg = (devad & 0x1f) << 21;
    reg |= (phyaddr << 16) | (phyreg & 0xffff);
    writel(reg, sp.ioaddr + sp.hw.mii.addr);
    sxgbe_mdio_ctrl_data(sp, cmd, phydata);
    }
    static void sxgbe_mdio_c22(struct sxgbe_priv_data *sp, u32 cmd, int phyaddr,
    int phyreg, u16 phydata)
    {
    u32 reg;
    writel(1 << phyaddr, sp.ioaddr + SXGBE_MDIO_CLAUSE22_PORT_REG);
// set mdio address register
    reg = (phyaddr << 16) | (phyreg & 0x1f);
    writel(reg, sp.ioaddr + sp.hw.mii.addr);
    sxgbe_mdio_ctrl_data(sp, cmd, phydata);
    }
    static int sxgbe_mdio_access_c22(struct sxgbe_priv_data *sp, u32 cmd,
    int phyaddr, int phyreg, u16 phydata)
    {
    const struct mii_regs *mii = &sp.hw.mii;
    int rc;
    rc = sxgbe_mdio_busy_wait(sp.ioaddr, mii.data);
    if (rc < 0)
    return rc;
// Ports 0-3 only support C22.
    if (phyaddr >= 4)
    return -ENODEV;
    sxgbe_mdio_c22(sp, cmd, phyaddr, phyreg, phydata);
    return sxgbe_mdio_busy_wait(sp.ioaddr, mii.data);
    }
    static int sxgbe_mdio_access_c45(struct sxgbe_priv_data *sp, u32 cmd,
    int phyaddr, int devad, int phyreg,
    u16 phydata)
    {
    const struct mii_regs *mii = &sp.hw.mii;
    int rc;
    rc = sxgbe_mdio_busy_wait(sp.ioaddr, mii.data);
    if (rc < 0)
    return rc;
    sxgbe_mdio_c45(sp, cmd, phyaddr, devad, phyreg, phydata);
    return sxgbe_mdio_busy_wait(sp.ioaddr, mii.data);
    }
//
// sxgbe_mdio_read_c22
// @bus: points to the mii_bus structure
// @phyaddr: address of phy port
// @phyreg: address of register with in phy register
// Description: this function used for C22 MDIO Read
//
#[no_mangle]
unsafe extern "C" fn sxgbe_mdio_read_c22(bus: *mut mii_bus, phyaddr: c_int, phyreg: c_int) -> c_int {
    static int sxgbe_mdio_read_c22(struct mii_bus *bus, int phyaddr, int phyreg)
    {
    struct net_device *ndev = bus.priv;
    struct sxgbe_priv_data *priv = netdev_priv(ndev);
    int rc;
    rc = sxgbe_mdio_access_c22(priv, SXGBE_SMA_READ_CMD, phyaddr,
    phyreg, 0);
    if (rc < 0)
    return rc;
    return readl(priv.ioaddr + priv.hw.mii.data) & 0xffff;
    }
//
// sxgbe_mdio_read_c45
// @bus: points to the mii_bus structure
// @phyaddr: address of phy port
// @devad: device (MMD) address
// @phyreg: address of register with in phy register
// Description: this function used for C45 MDIO Read
//
    static int sxgbe_mdio_read_c45(struct mii_bus *bus, int phyaddr, int devad,
    int phyreg)
    {
    struct net_device *ndev = bus.priv;
    struct sxgbe_priv_data *priv = netdev_priv(ndev);
    int rc;
    rc = sxgbe_mdio_access_c45(priv, SXGBE_SMA_READ_CMD, phyaddr,
    devad, phyreg, 0);
    if (rc < 0)
    return rc;
    return readl(priv.ioaddr + priv.hw.mii.data) & 0xffff;
    }
//
// sxgbe_mdio_write_c22
// @bus: points to the mii_bus structure
// @phyaddr: address of phy port
// @phyreg: address of phy registers
// @phydata: data to be written into phy register
// Description: this function is used for C22 MDIO write
//
    static int sxgbe_mdio_write_c22(struct mii_bus *bus, int phyaddr, int phyreg,
    u16 phydata)
    {
    struct net_device *ndev = bus.priv;
    struct sxgbe_priv_data *priv = netdev_priv(ndev);
    return sxgbe_mdio_access_c22(priv, SXGBE_SMA_WRITE_CMD, phyaddr, phyreg,
    phydata);
    }
//
// sxgbe_mdio_write_c45
// @bus: points to the mii_bus structure
// @phyaddr: address of phy port
// @phyreg: address of phy registers
// @devad: device (MMD) address
// @phydata: data to be written into phy register
// Description: this function is used for C45 MDIO write
//
    static int sxgbe_mdio_write_c45(struct mii_bus *bus, int phyaddr, int devad,
    int phyreg, u16 phydata)
    {
    struct net_device *ndev = bus.priv;
    struct sxgbe_priv_data *priv = netdev_priv(ndev);
    return sxgbe_mdio_access_c45(priv, SXGBE_SMA_WRITE_CMD, phyaddr,
    devad, phyreg, phydata);
    }
#[no_mangle]
pub unsafe extern "C" fn sxgbe_mdio_register(ndev: *mut net_device) -> c_int {
    int sxgbe_mdio_register(struct net_device *ndev)
    {
    struct mii_bus *mdio_bus;
    struct sxgbe_priv_data *priv = netdev_priv(ndev);
    struct sxgbe_mdio_bus_data *mdio_data = priv.plat.mdio_bus_data;
    int err, phy_addr;
    int *irqlist;
    let mut phy_found: bool = false;
    bool act;
// allocate the new mdio bus
    mdio_bus = mdiobus_alloc();
    if (!mdio_bus) {
    netdev_err(ndev, "%s: mii bus allocation failed\n", __func__);
    return -ENOMEM;
    }
    if (mdio_data.irqs)
    irqlist = mdio_data.irqs;
    else
    irqlist = priv.mii_irq;
// assign mii bus fields
    mdio_bus.name = "sxgbe";
    mdio_bus.read = sxgbe_mdio_read_c22;
    mdio_bus.write = sxgbe_mdio_write_c22;
    mdio_bus.read_c45 = sxgbe_mdio_read_c45;
    mdio_bus.write_c45 = sxgbe_mdio_write_c45;
    snprintf(mdio_bus.id, MII_BUS_ID_SIZE, "%s-%x",
    mdio_bus.name, priv.plat.bus_id);
    mdio_bus.priv = ndev;
    mdio_bus.phy_mask = mdio_data.phy_mask;
    mdio_bus.parent = priv.device;
// register with kernel subsystem
    err = mdiobus_register(mdio_bus);
    if (err != 0) {
    netdev_err(ndev, "mdiobus register failed\n");
    goto mdiobus_err;
    }
    for (phy_addr = 0; phy_addr < PHY_MAX_ADDR; phy_addr++) {
    struct phy_device *phy = mdiobus_get_phy(mdio_bus, phy_addr);
    if (phy) {
    char irq_num[4];
    char *irq_str;
// If an IRQ was provided to be assigned after
// the bus probe, do it here.
//
    if ((mdio_data.irqs == core::ptr::null_mut()) &&
    (mdio_data.probed_phy_irq > 0)) {
    irqlist[phy_addr] = mdio_data.probed_phy_irq;
    phy.irq = mdio_data.probed_phy_irq;
    }
// If we're  going to bind the MAC to this PHY bus,
// and no PHY number was provided to the MAC,
// use the one probed here.
//
    if (priv.plat.phy_addr == -1)
    priv.plat.phy_addr = phy_addr;
    act = (priv.plat.phy_addr == phy_addr);
    switch (phy.irq) {
    case PHY_POLL:
    irq_str = "POLL";
    break;
    case PHY_MAC_INTERRUPT:
    irq_str = "MAC";
    break;
    default:
    sprintf(irq_num, "%d", phy.irq);
    irq_str = irq_num;
    break;
    }
    netdev_info(ndev, "PHY ID %08x at %d IRQ %s (%s)%s\n",
    phy.phy_id, phy_addr, irq_str,
    phydev_name(phy), act ? " active" : "");
    phy_found = true;
    }
    }
    if (!phy_found) {
    netdev_err(ndev, "PHY not found\n");
    goto phyfound_err;
    }
    priv.mii = mdio_bus;
    return 0;
    phyfound_err:
    err = -ENODEV;
    mdiobus_unregister(mdio_bus);
    mdiobus_err:
    mdiobus_free(mdio_bus);
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn sxgbe_mdio_unregister(ndev: *mut net_device) -> c_int {
    int sxgbe_mdio_unregister(struct net_device *ndev)
    {
    struct sxgbe_priv_data *priv = netdev_priv(ndev);
    if (!priv.mii)
    return 0;
    mdiobus_unregister(priv.mii);
    priv.mii.priv = core::ptr::null_mut();
    mdiobus_free(priv.mii);
    priv.mii = core::ptr::null_mut();
    return 0;
    }
