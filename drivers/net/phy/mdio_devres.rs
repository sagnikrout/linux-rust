//! Automatically rewritten from C to Rust
//! Source: drivers/net/phy/mdio_devres.c
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


// SPDX-License-Identifier: GPL-2.0-or-later

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdiobus_devres {
    pub mii: *mut mii_bus,
}

#[no_mangle]
unsafe extern "C" fn devm_mdiobus_free(dev: *mut device, this: *mut c_void) {
    static void devm_mdiobus_free(struct device *dev, void *this)
    {
    struct mdiobus_devres *dr = this;
    mdiobus_free(dr.mii);
    }
//
// devm_mdiobus_alloc_size - Resource-managed mdiobus_alloc_size()
// @dev:		Device to allocate mii_bus for
// @sizeof_priv:	Space to allocate for private structure
//
// Managed mdiobus_alloc_size. mii_bus allocated with this function is
// automatically freed on driver detach.
//
// RETURNS:
// Pointer to allocated mii_bus on success, NULL on out-of-memory error.
//
    struct mii_bus *devm_mdiobus_alloc_size(struct device *dev, int sizeof_priv)
    {
    struct mdiobus_devres *dr;
    dr = devres_alloc(devm_mdiobus_free, sizeof(*dr), GFP_KERNEL);
    if (!dr)
    return core::ptr::null_mut();
    dr.mii = mdiobus_alloc_size(sizeof_priv);
    if (!dr.mii) {
    devres_free(dr);
    return core::ptr::null_mut();
    }
    devres_add(dev, dr);
    return dr.mii;
    }
    EXPORT_SYMBOL(devm_mdiobus_alloc_size);
#[no_mangle]
unsafe extern "C" fn devm_mdiobus_unregister(dev: *mut device, this: *mut c_void) {
    static void devm_mdiobus_unregister(struct device *dev, void *this)
    {
    struct mdiobus_devres *dr = this;
    mdiobus_unregister(dr.mii);
    }
    static int mdiobus_devres_match(struct device *dev,
    void *this, void *match_data)
    {
    struct mdiobus_devres *res = this;
    struct mii_bus *mii = match_data;
    let mut mii: return = = res.mii;
    }
//
// __devm_mdiobus_register - Resource-managed variant of mdiobus_register()
// @dev:	Device to register mii_bus for
// @bus:	MII bus structure to register
// @owner:	Owning module
//
// Returns 0 on success, negative error number on failure.
//
    int __devm_mdiobus_register(struct device *dev, struct mii_bus *bus,
    struct module *owner)
    {
    struct mdiobus_devres *dr;
    int ret;
    if (WARN_ON(!devres_find(dev, devm_mdiobus_free,
    mdiobus_devres_match, bus)))
    return -EINVAL;
    dr = devres_alloc(devm_mdiobus_unregister, sizeof(*dr), GFP_KERNEL);
    if (!dr)
    return -ENOMEM;
    ret = __mdiobus_register(bus, owner);
    if (ret) {
    devres_free(dr);
    return ret;
    }
    dr.mii = bus;
    devres_add(dev, dr);
    return 0;
    }
    EXPORT_SYMBOL(__devm_mdiobus_register);

//
// __devm_of_mdiobus_register - Resource managed variant of of_mdiobus_register()
// @dev:	Device to register mii_bus for
// @mdio:	MII bus structure to register
// @np:		Device node to parse
// @owner:	Owning module
//
    int __devm_of_mdiobus_register(struct device *dev, struct mii_bus *mdio,
    struct device_node *np, struct module *owner)
    {
    struct mdiobus_devres *dr;
    int ret;
    if (WARN_ON(!devres_find(dev, devm_mdiobus_free,
    mdiobus_devres_match, mdio)))
    return -EINVAL;
    dr = devres_alloc(devm_mdiobus_unregister, sizeof(*dr), GFP_KERNEL);
    if (!dr)
    return -ENOMEM;
    ret = __of_mdiobus_register(mdio, np, owner);
    if (ret) {
    devres_free(dr);
    return ret;
    }
    dr.mii = mdio;
    devres_add(dev, dr);
    return 0;
    }
    EXPORT_SYMBOL(__devm_of_mdiobus_register);

    MODULE_DESCRIPTION("Network MDIO bus devres helpers");
    MODULE_LICENSE("GPL");
