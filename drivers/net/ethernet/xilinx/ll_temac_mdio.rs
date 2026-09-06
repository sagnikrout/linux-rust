//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/xilinx/ll_temac_mdio.c
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
// MDIO bus driver for the Xilinx TEMAC device
//
// Copyright (c) 2009 Secret Lab Technologies, Ltd.
//

// ---------------------------------------------------------------------
// MDIO Bus functions
//
#[no_mangle]
unsafe extern "C" fn temac_mdio_read(bus: *mut mii_bus, phy_id: c_int, reg: c_int) -> c_int {
    static int temac_mdio_read(struct mii_bus *bus, int phy_id, int reg)
    {
    struct temac_local *lp = bus.priv;
    u32 rc;
    unsigned long flags;
// Write the PHY address to the MIIM Access Initiator register.
// When the transfer completes, the PHY register value will appear
// in the LSW0 register
//
    spin_lock_irqsave(lp.indirect_lock, flags);
    temac_iow(lp, XTE_LSW0_OFFSET, (phy_id << 5) | reg);
    rc = temac_indirect_in32_locked(lp, XTE_MIIMAI_OFFSET);
    spin_unlock_irqrestore(lp.indirect_lock, flags);
    dev_dbg(lp.dev, "temac_mdio_read(phy_id=%i, reg=%x) == %x\n",
    phy_id, reg, rc);
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn temac_mdio_write(bus: *mut mii_bus, phy_id: c_int, reg: c_int, val: u16) -> c_int {
    static int temac_mdio_write(struct mii_bus *bus, int phy_id, int reg, u16 val)
    {
    struct temac_local *lp = bus.priv;
    unsigned long flags;
    dev_dbg(lp.dev, "temac_mdio_write(phy_id=%i, reg=%x, val=%x)\n",
    phy_id, reg, val);
// First write the desired value into the write data register
// and then write the address into the access initiator register
//
    spin_lock_irqsave(lp.indirect_lock, flags);
    temac_indirect_out32_locked(lp, XTE_MGTDR_OFFSET, val);
    temac_indirect_out32_locked(lp, XTE_MIIMAI_OFFSET, (phy_id << 5) | reg);
    spin_unlock_irqrestore(lp.indirect_lock, flags);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn temac_mdio_setup(lp: *mut temac_local, pdev: *mut platform_device) -> c_int {
    int temac_mdio_setup(struct temac_local *lp, struct platform_device *pdev)
    {
    struct ll_temac_platform_data *pdata = dev_get_platdata(&pdev.dev);
    struct device_node *np = dev_of_node(&pdev.dev);
    struct mii_bus *bus;
    u32 bus_hz;
    int clk_div;
    int rc;
    struct resource res;
// Get MDIO bus frequency (if specified)
    bus_hz = 0;
    if (np)
    of_property_read_u32(np, "clock-frequency", &bus_hz);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: pdata) -> else {
    else if (pdata)
    bus_hz = pdata.mdio_clk_freq;
// Calculate a reasonable divisor for the clock rate
    clk_div = 0x3f; /* worst-case default setting */
    if (bus_hz != 0) {
    clk_div = bus_hz / (2500 * 1000 * 2) - 1;
    if (clk_div < 1)
    clk_div = 1;
    if (clk_div > 0x3f)
    clk_div = 0x3f;
    }
// Enable the MDIO bus by asserting the enable bit and writing
// in the clock config
//
    temac_indirect_out32(lp, XTE_MC_OFFSET, 1 << 6 | clk_div);
    bus = devm_mdiobus_alloc(&pdev.dev);
    if (!bus)
    return -ENOMEM;
    if (np) {
    of_address_to_resource(np, 0, &res);
    snprintf(bus.id, MII_BUS_ID_SIZE, "%.8llx",
    (unsigned long long)res.start);
    } else if (pdata) {
    snprintf(bus.id, MII_BUS_ID_SIZE, "%.8llx",
    pdata.mdio_bus_id);
    }
    bus.priv = lp;
    bus.name = "Xilinx TEMAC MDIO";
    bus.read = temac_mdio_read;
    bus.write = temac_mdio_write;
    bus.parent = lp.dev;
    lp.mii_bus = bus;
    rc = of_mdiobus_register(bus, np);
    if (rc)
    return rc;
    dev_dbg(lp.dev, "MDIO bus registered;  MC:%x\n",
    temac_indirect_in32(lp, XTE_MC_OFFSET));
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn temac_mdio_teardown(lp: *mut temac_local) {
    void temac_mdio_teardown(struct temac_local *lp)
    {
    mdiobus_unregister(lp.mii_bus);
    }
