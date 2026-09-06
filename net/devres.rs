//! Automatically rewritten from C to Rust
//! Source: net/devres.c
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
//
// This file contains all networking devres helpers.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct net_device_devres {
    pub ndev: *mut net_device,
}

#[no_mangle]
unsafe extern "C" fn devm_free_netdev(dev: *mut device, this: *mut c_void) {
    static void devm_free_netdev(struct device *dev, void *this)
    {
    struct net_device_devres *res = this;
    free_netdev(res.ndev);
    }
    struct net_device *devm_alloc_etherdev_mqs(struct device *dev, int sizeof_priv,
    unsigned int txqs, unsigned int rxqs)
    {
    struct net_device_devres *dr;
    dr = devres_alloc(devm_free_netdev, sizeof(*dr), GFP_KERNEL);
    if (!dr)
    return core::ptr::null_mut();
    dr.ndev = alloc_etherdev_mqs(sizeof_priv, txqs, rxqs);
    if (!dr.ndev) {
    devres_free(dr);
    return core::ptr::null_mut();
    }
    devres_add(dev, dr);
    return dr.ndev;
    }
    EXPORT_SYMBOL(devm_alloc_etherdev_mqs);
#[no_mangle]
unsafe extern "C" fn devm_unregister_netdev(dev: *mut device, this: *mut c_void) {
    static void devm_unregister_netdev(struct device *dev, void *this)
    {
    struct net_device_devres *res = this;
    unregister_netdev(res.ndev);
    }
#[no_mangle]
unsafe extern "C" fn netdev_devres_match(dev: *mut device, this: *mut c_void, match_data: *mut c_void) -> c_int {
    static int netdev_devres_match(struct device *dev, void *this, void *match_data)
    {
    struct net_device_devres *res = this;
    struct net_device *ndev = match_data;
    let mut ndev: return = = res.ndev;
    }
//
// devm_register_netdev - resource managed variant of register_netdev()
// @dev: managing device for this netdev - usually the parent device
// @ndev: device to register
//
// This is a devres variant of register_netdev() for which the unregister
// function will be called automatically when the managing device is
// detached. Note: the net_device used must also be resource managed by
// the same struct device.
//
#[no_mangle]
pub unsafe extern "C" fn devm_register_netdev(dev: *mut device, ndev: *mut net_device) -> c_int {
    int devm_register_netdev(struct device *dev, struct net_device *ndev)
    {
    struct net_device_devres *dr;
    int ret;
// struct net_device must itself be managed. For now a managed netdev
// can only be allocated by devm_alloc_etherdev_mqs() so the check is
// straightforward.
//
    if (WARN_ON(!devres_find(dev, devm_free_netdev,
    netdev_devres_match, ndev)))
    return -EINVAL;
    dr = devres_alloc(devm_unregister_netdev, sizeof(*dr), GFP_KERNEL);
    if (!dr)
    return -ENOMEM;
    ret = register_netdev(ndev);
    if (ret) {
    devres_free(dr);
    return ret;
    }
    dr.ndev = ndev;
    devres_add(ndev.dev.parent, dr);
    return 0;
    }
    EXPORT_SYMBOL(devm_register_netdev);
