//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/arc/emac_mdio.c
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
// Copyright (C) 2004-2013 Synopsys, Inc. (www.synopsys.com)
//
// MDIO implementation for ARC EMAC
//

// Number of seconds we wait for "MDIO complete" flag to appear
pub const ARC_MDIO_COMPLETE_POLL_COUNT: c_int = 1;
//
// arc_mdio_complete_wait - Waits until MDIO transaction is completed.
// @priv:	Pointer to ARC EMAC private data structure.
//
// returns:	0 on success, -ETIMEDOUT on a timeout.
//
#[no_mangle]
unsafe extern "C" fn arc_mdio_complete_wait(priv: *mut arc_emac_priv) -> c_int {
    static int arc_mdio_complete_wait(struct arc_emac_priv *priv)
    {
    unsigned int i;
    for (i = 0; i < ARC_MDIO_COMPLETE_POLL_COUNT * 40; i++) {
    let mut status: c_uint = arc_reg_get(priv, R_STATUS);
    status &= MDIO_MASK;
    if (status) {
// Reset "MDIO complete" flag
    arc_reg_set(priv, R_STATUS, status);
    return 0;
    }
    msleep(25);
    }
    return -ETIMEDOUT;
    }
//
// arc_mdio_read - MDIO interface read function.
// @bus:	Pointer to MII bus structure.
// @phy_addr:	Address of the PHY device.
// @reg_num:	PHY register to read.
//
// returns:	The register contents on success, -ETIMEDOUT on a timeout.
//
// Reads the contents of the requested register from the requested PHY
// address.
//
#[no_mangle]
unsafe extern "C" fn arc_mdio_read(bus: *mut mii_bus, phy_addr: c_int, reg_num: c_int) -> c_int {
    static int arc_mdio_read(struct mii_bus *bus, int phy_addr, int reg_num)
    {
    struct arc_emac_priv *priv = bus.priv;
    unsigned int value;
    int error;
    arc_reg_set(priv, R_MDIO,
    0x60020000 | (phy_addr << 23) | (reg_num << 18));
    error = arc_mdio_complete_wait(priv);
    if (error < 0)
    return error;
    value = arc_reg_get(priv, R_MDIO) & 0xffff;
    dev_dbg(priv.dev, "arc_mdio_read(phy_addr=%i, reg_num=%x) = %x\n",
    phy_addr, reg_num, value);
    return value;
    }
//
// arc_mdio_write - MDIO interface write function.
// @bus:	Pointer to MII bus structure.
// @phy_addr:	Address of the PHY device.
// @reg_num:	PHY register to write to.
// @value:	Value to be written into the register.
//
// returns:	0 on success, -ETIMEDOUT on a timeout.
//
// Writes the value to the requested register.
//
    static int arc_mdio_write(struct mii_bus *bus, int phy_addr,
    int reg_num, u16 value)
    {
    struct arc_emac_priv *priv = bus.priv;
    dev_dbg(priv.dev,
    "arc_mdio_write(phy_addr=%i, reg_num=%x, value=%x)\n",
    phy_addr, reg_num, value);
    arc_reg_set(priv, R_MDIO,
    0x50020000 | (phy_addr << 23) | (reg_num << 18) | value);
    return arc_mdio_complete_wait(priv);
    }
//
// arc_mdio_reset
// @bus: points to the mii_bus structure
// Description: reset the MII bus
//
#[no_mangle]
unsafe extern "C" fn arc_mdio_reset(bus: *mut mii_bus) -> c_int {
    static int arc_mdio_reset(struct mii_bus *bus)
    {
    struct arc_emac_priv *priv = bus.priv;
    struct arc_emac_mdio_bus_data *data = &priv.bus_data;
    if (data.reset_gpio) {
    gpiod_set_value_cansleep(data.reset_gpio, 1);
    msleep(data.msec);
    gpiod_set_value_cansleep(data.reset_gpio, 0);
    }
    return 0;
    }
//
// arc_mdio_probe - MDIO probe function.
// @priv:	Pointer to ARC EMAC private data structure.
//
// returns:	0 on success, -ENOMEM when mdiobus_alloc
// (to allocate memory for MII bus structure) fails.
//
// Sets up and registers the MDIO interface.
//
#[no_mangle]
pub unsafe extern "C" fn arc_mdio_probe(priv: *mut arc_emac_priv) -> c_int {
    int arc_mdio_probe(struct arc_emac_priv *priv)
    {
    struct arc_emac_mdio_bus_data *data = &priv.bus_data;
    struct device_node *np = priv.dev.of_node;
    const char *name = "Synopsys MII Bus";
    struct device_node *mdio_node;
    struct mii_bus *bus;
    int error;
    bus = mdiobus_alloc();
    if (!bus)
    return -ENOMEM;
    priv.bus = bus;
    bus.priv = priv;
    bus.parent = priv.dev;
    bus.name = name;
    bus.read = &arc_mdio_read;
    bus.write = &arc_mdio_write;
    bus.reset = &arc_mdio_reset;
// optional reset-related properties
    data.reset_gpio = devm_gpiod_get_optional(priv.dev, "phy-reset",
    GPIOD_OUT_LOW);
    if (IS_ERR(data.reset_gpio)) {
    mdiobus_free(bus);
    return dev_err_probe(priv.dev, PTR_ERR(data.reset_gpio),
    "Failed to request gpio\n");
    }
    of_property_read_u32(np, "phy-reset-duration", &data.msec);
// A sane reset duration should not be longer than 1s
    if (data.msec > 1000)
    data.msec = 1;
    snprintf(bus.id, MII_BUS_ID_SIZE, "%s", bus.name);
// Backwards compatibility for EMAC nodes without MDIO subnode.
    mdio_node = of_get_child_by_name(np, "mdio");
    if (!mdio_node)
    mdio_node = of_node_get(np);
    error = of_mdiobus_register(bus, mdio_node);
    of_node_put(mdio_node);
    if (error) {
    mdiobus_free(bus);
    return dev_err_probe(priv.dev, error,
    "cannot register MDIO bus %s\n", name);
    }
    return 0;
    }
//
// arc_mdio_remove - MDIO remove function.
// @priv:	Pointer to ARC EMAC private data structure.
//
// Unregisters the MDIO and frees any associate memory for MII bus.
//
#[no_mangle]
pub unsafe extern "C" fn arc_mdio_remove(priv: *mut arc_emac_priv) -> c_int {
    int arc_mdio_remove(struct arc_emac_priv *priv)
    {
    mdiobus_unregister(priv.bus);
    mdiobus_free(priv.bus);
    priv.bus = core::ptr::null_mut();
    return 0;
    }
