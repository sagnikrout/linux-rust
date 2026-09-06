//! Automatically rewritten from C to Rust
//! Source: drivers/net/phy/fixed_phy.c
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
// Fixed MDIO bus (MDIO bus emulation with fixed PHYs)
//
// Author: Vitaly Bordug <vbordug@ru.mvista.com>
// Anton Vorontsov <avorontsov@ru.mvista.com>
//
// Copyright (c) 2006-2007 MontaVista Software, Inc.
//

// The DSA loop driver may allocate 4 fixed PHY's, and 4 additional
// fixed PHY's for a system should be sufficient.
//
pub const NUM_FP: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fixed_phy {
    pub phydev: *mut phy_device,
    pub status: fixed_phy_status,
    pub ): *mut *mut *mut int (link_update)(struct net_device , struct fixed_phy_status,
}

    static DECLARE_BITMAP(fixed_phy_ids, NUM_FP);
    static struct fixed_phy fmb_fixed_phys[NUM_FP];
    static struct mii_bus *fmb_mii_bus;
    static struct fixed_phy *fixed_phy_find(int addr)
    {
    return test_bit(addr, fixed_phy_ids) ? fmb_fixed_phys + addr : core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn fixed_phy_change_carrier(dev: *mut net_device, new_carrier: bool) -> c_int {
    int fixed_phy_change_carrier(struct net_device *dev, bool new_carrier)
    {
    struct phy_device *phydev = dev.phydev;
    struct fixed_phy *fp;
    if (!phydev || !phydev.mdio.bus)
    return -EINVAL;
    fp = fixed_phy_find(phydev.mdio.addr);
    if (!fp)
    return -EINVAL;
    fp.status.link = new_carrier;
    return 0;
    }
    EXPORT_SYMBOL_GPL(fixed_phy_change_carrier);
#[no_mangle]
unsafe extern "C" fn fixed_mdio_read(bus: *mut mii_bus, phy_addr: c_int, reg_num: c_int) -> c_int {
    static int fixed_mdio_read(struct mii_bus *bus, int phy_addr, int reg_num)
    {
    struct fixed_phy *fp;
    fp = fixed_phy_find(phy_addr);
    if (!fp)
    return 0xffff;
    if (fp.link_update)
    fp.link_update(fp.phydev.attached_dev, &fp.status);
    return swphy_read_reg(reg_num, &fp.status);
    }
    static int fixed_mdio_write(struct mii_bus *bus, int phy_addr, int reg_num,
    u16 val)
    {
    return 0;
    }
//
// If something weird is required to be done with link/speed,
// network driver is able to assign a function to implement this.
// May be useful for PHY's that need to be software-driven.
//
    int fixed_phy_set_link_update(struct phy_device *phydev,
    int (*link_update)(struct net_device *,
    struct fixed_phy_status *))
    {
    struct fixed_phy *fp;
    if (!phydev || !phydev.mdio.bus)
    return -EINVAL;
    fp = fixed_phy_find(phydev.mdio.addr);
    if (!fp)
    return -ENOENT;
    fp.link_update = link_update;
    fp.phydev = phydev;
    return 0;
    }
    EXPORT_SYMBOL_GPL(fixed_phy_set_link_update);
#[no_mangle]
unsafe extern "C" fn fixed_phy_del(phy_addr: c_int) {
    static void fixed_phy_del(int phy_addr)
    {
    struct fixed_phy *fp;
    fp = fixed_phy_find(phy_addr);
    if (!fp)
    return;
    memset(fp, 0, sizeof(*fp));
    clear_bit(phy_addr, fixed_phy_ids);
    }
#[no_mangle]
unsafe extern "C" fn fixed_phy_get_free_addr() -> c_int {
    static int fixed_phy_get_free_addr(void)
    {
    int addr;
    do {
    addr = find_first_zero_bit(fixed_phy_ids, NUM_FP);
    if (addr == NUM_FP)
    return -ENOSPC;
    } while (test_and_set_bit(addr, fixed_phy_ids));
    return addr;
    }
    struct phy_device *fixed_phy_register(const struct fixed_phy_status *status,
    struct device_node *np)
    {
    struct phy_device *phy;
    int phy_addr;
    int ret;
    ret = swphy_validate_state(status);
    if (ret < 0)
    return ERR_PTR(ret);
    if (!fmb_mii_bus || fmb_mii_bus.state != MDIOBUS_REGISTERED)
    return ERR_PTR(-EPROBE_DEFER);
// Get the next available PHY address, up to NUM_FP
    phy_addr = fixed_phy_get_free_addr();
    if (phy_addr < 0)
    return ERR_PTR(phy_addr);
    fmb_fixed_phys[phy_addr].status = *status;
    fmb_fixed_phys[phy_addr].status.link = true;
    phy = get_phy_device(fmb_mii_bus, phy_addr, false);
    if (IS_ERR(phy)) {
    fixed_phy_del(phy_addr);
    return ERR_PTR(-EINVAL);
    }
    of_node_get(np);
    phy.mdio.dev.of_node = np;
    phy.is_pseudo_fixed_link = true;
    ret = phy_device_register(phy);
    if (ret) {
    phy_device_free(phy);
    of_node_put(np);
    fixed_phy_del(phy_addr);
    return ERR_PTR(ret);
    }
    return phy;
    }
    EXPORT_SYMBOL_GPL(fixed_phy_register);
    struct phy_device *fixed_phy_register_100fd(void)
    {
    static const struct fixed_phy_status status = {
    .speed	= SPEED_100,
    .duplex	= DUPLEX_FULL,
    };
    return fixed_phy_register(&status, core::ptr::null_mut());
    }
    EXPORT_SYMBOL_GPL(fixed_phy_register_100fd);
#[no_mangle]
pub unsafe extern "C" fn fixed_phy_unregister(phy: *mut phy_device) {
    void fixed_phy_unregister(struct phy_device *phy)
    {
    phy_device_remove(phy);
    of_node_put(phy.mdio.dev.of_node);
    fixed_phy_del(phy.mdio.addr);
    phy_device_free(phy);
    }
    EXPORT_SYMBOL_GPL(fixed_phy_unregister);
#[no_mangle]
unsafe extern "C" fn fixed_mdio_bus_init() -> int __init {
    static int __init fixed_mdio_bus_init(void)
    {
    int ret;
    fmb_mii_bus = mdiobus_alloc();
    if (!fmb_mii_bus)
    return -ENOMEM;
    snprintf(fmb_mii_bus.id, MII_BUS_ID_SIZE, "fixed-0");
    fmb_mii_bus.name = "Fixed MDIO Bus";
    fmb_mii_bus.read = &fixed_mdio_read;
    fmb_mii_bus.write = &fixed_mdio_write;
    fmb_mii_bus.phy_mask = ~0;
    ret = mdiobus_register(fmb_mii_bus);
    if (ret)
    goto err_mdiobus_alloc;
    return 0;
    err_mdiobus_alloc:
    mdiobus_free(fmb_mii_bus);
    return ret;
    }
    module_init(fixed_mdio_bus_init);
#[no_mangle]
unsafe extern "C" fn fixed_mdio_bus_exit() -> void __exit {
    static void __exit fixed_mdio_bus_exit(void)
    {
    mdiobus_unregister(fmb_mii_bus);
    mdiobus_free(fmb_mii_bus);
    }
    module_exit(fixed_mdio_bus_exit);
    MODULE_DESCRIPTION("Fixed MDIO bus (MDIO bus emulation with fixed PHYs)");
    MODULE_AUTHOR("Vitaly Bordug");
    MODULE_LICENSE("GPL");
