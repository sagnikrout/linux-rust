//! Automatically rewritten from C to Rust
//! Source: drivers/net/mdio/fwnode_mdio.c
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
// fwnode helpers for the MDIO (Ethernet PHY) API
//
// This file provides helper functions for extracting PHY device information
// out of the fwnode and using it to populate an mii_bus.
//

    MODULE_AUTHOR("Calvin Johnson <calvin.johnson@oss.nxp.com>");
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("FWNODE MDIO bus (Ethernet PHY) accessors");
    static struct pse_control *
    fwnode_find_pse_control(struct fwnode_handle *fwnode,
    struct phy_device *phydev)
    {
    struct pse_control *psec;
    struct device_node *np;
    if (!IS_ENABLED(CONFIG_PSE_CONTROLLER))
    return core::ptr::null_mut();
    np = to_of_node(fwnode);
    if (!np)
    return core::ptr::null_mut();
    psec = of_pse_control_get(np, phydev);
    if (PTR_ERR(psec) == -ENOENT)
    return core::ptr::null_mut();
    return psec;
    }
    static struct mii_timestamper *
    fwnode_find_mii_timestamper(struct fwnode_handle *fwnode)
    {
    struct mii_timestamper *mii_ts;
    struct of_phandle_args arg;
    int err;
    if (is_acpi_node(fwnode))
    return core::ptr::null_mut();
    err = of_parse_phandle_with_fixed_args(to_of_node(fwnode),
    "timestamper", 1, 0, &arg);
    if (err == -ENOENT)
    return core::ptr::null_mut();
#[no_mangle]
pub unsafe extern "C" fn if(_arg: err) -> else {
    else if (err)
    return ERR_PTR(err);
    if (arg.args_count != 1) {
    mii_ts = ERR_PTR(-EINVAL);
    goto put_node;
    }
    mii_ts = register_mii_timestamper(arg.np, arg.args[0]);
    put_node:
    of_node_put(arg.np);
    return mii_ts;
    }
    int fwnode_mdiobus_phy_device_register(struct mii_bus *mdio,
    struct phy_device *phy,
    struct fwnode_handle *child, u32 addr)
    {
    int rc;
    rc = fwnode_irq_get(child, 0);
// Don't wait forever if the IRQ provider doesn't become available,
// just fall back to poll mode
//
    if (rc == -EPROBE_DEFER)
    rc = driver_deferred_probe_check_state(&phy.mdio.dev);
    if (rc == -EPROBE_DEFER)
    return rc;
    if (rc > 0) {
    phy.irq = rc;
    mdio.irq[addr] = rc;
    } else {
    phy.irq = mdio.irq[addr];
    }
    if (fwnode_property_read_bool(child, "broken-turn-around"))
    mdio.phy_ignore_ta_mask |= 1 << addr;
// Associate the fwnode with the device structure so it
// can be looked up later
//
    fwnode_handle_get(child);
    device_set_node(&phy.mdio.dev, child);
// All data is now stored in the phy struct;
// register it
//
    rc = phy_device_register(phy);
    if (rc) {
    device_set_node(&phy.mdio.dev, core::ptr::null_mut());
    fwnode_handle_put(child);
    return rc;
    }
    dev_dbg(&mdio.dev, "registered phy fwnode %pfw at address %i\n",
    child, addr);
    return 0;
    }
    EXPORT_SYMBOL(fwnode_mdiobus_phy_device_register);
    int fwnode_mdiobus_register_phy(struct mii_bus *bus,
    struct fwnode_handle *child, u32 addr)
    {
    struct mii_timestamper *mii_ts = core::ptr::null_mut();
    struct pse_control *psec = core::ptr::null_mut();
    struct phy_device *phy;
    bool is_c45;
    u32 phy_id;
    int rc;
    mii_ts = fwnode_find_mii_timestamper(child);
    if (IS_ERR(mii_ts))
    return PTR_ERR(mii_ts);
    is_c45 = fwnode_device_is_compatible(child, "ethernet-phy-ieee802.3-c45");
    if (is_c45 || fwnode_get_phy_id(child, &phy_id))
    phy = get_phy_device(bus, addr, is_c45);
    else
    phy = phy_device_create(bus, addr, phy_id, 0, core::ptr::null_mut());
    if (IS_ERR(phy)) {
    rc = PTR_ERR(phy);
    goto clean_mii_ts;
    }
    if (is_acpi_node(child)) {
    phy.irq = bus.irq[addr];
// Associate the fwnode with the device structure so it
// can be looked up later.
//
    phy.mdio.dev.fwnode = fwnode_handle_get(child);
// All data is now stored in the phy struct, so register it
    rc = phy_device_register(phy);
    if (rc) {
    phy.mdio.dev.fwnode = core::ptr::null_mut();
    fwnode_handle_put(child);
    goto clean_phy;
    }
    } else if (is_of_node(child)) {
    rc = fwnode_mdiobus_phy_device_register(bus, phy, child, addr);
    if (rc)
    goto clean_phy;
    }
    psec = fwnode_find_pse_control(child, phy);
    if (IS_ERR(psec)) {
    rc = PTR_ERR(psec);
    goto unregister_phy;
    }
    phy.psec = psec;
// phy->mii_ts may already be defined by the PHY driver. A
// mii_timestamper probed via the device tree will still have
// precedence.
//
    if (mii_ts)
    phy.mii_ts = mii_ts;
    return 0;
    unregister_phy:
    if (is_acpi_node(child) || is_of_node(child))
    phy_device_remove(phy);
    clean_phy:
    phy_device_free(phy);
    clean_mii_ts:
    unregister_mii_timestamper(mii_ts);
    return rc;
    }
    EXPORT_SYMBOL(fwnode_mdiobus_register_phy);
