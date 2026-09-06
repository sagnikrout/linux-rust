//! Automatically rewritten from C to Rust
//! Source: drivers/vfio/mdev/mdev_driver.c
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
// MDEV driver
//
// Copyright (c) 2016, NVIDIA CORPORATION. All rights reserved.
// Author: Neo Jia <cjia@nvidia.com>
// Kirti Wankhede <kwankhede@nvidia.com>
//

#[no_mangle]
unsafe extern "C" fn mdev_probe(dev: *mut device) -> c_int {
    static int mdev_probe(struct device *dev)
    {
    struct mdev_driver *drv =
    container_of(dev.driver, struct mdev_driver, driver);
    if (!drv.probe)
    return 0;
    return drv.probe(to_mdev_device(dev));
    }
#[no_mangle]
unsafe extern "C" fn mdev_remove(dev: *mut device) {
    static void mdev_remove(struct device *dev)
    {
    struct mdev_driver *drv =
    container_of(dev.driver, struct mdev_driver, driver);
    if (drv.remove)
    drv.remove(to_mdev_device(dev));
    }
#[no_mangle]
unsafe extern "C" fn mdev_match(dev: *mut device, drv: *const device_driver) -> c_int {
    static int mdev_match(struct device *dev, const struct device_driver *drv)
    {
//
// No drivers automatically match. Drivers are only bound by explicit
// device_driver_attach()
//
    return 0;
    }
    const struct bus_type mdev_bus_type = {
    .name		= "mdev",
    .probe		= mdev_probe,
    .remove		= mdev_remove,
    .match		= mdev_match,
    };
//
// mdev_register_driver - register a new MDEV driver
// @drv: the driver to register
//
// Returns a negative value on error, otherwise 0.
//
#[no_mangle]
pub unsafe extern "C" fn mdev_register_driver(drv: *mut mdev_driver) -> c_int {
    int mdev_register_driver(struct mdev_driver *drv)
    {
    if (!drv.device_api)
    return -EINVAL;
// initialize common driver fields
    drv.driver.bus = &mdev_bus_type;
    return driver_register(&drv.driver);
    }
    EXPORT_SYMBOL(mdev_register_driver);
//
// mdev_unregister_driver - unregister MDEV driver
// @drv: the driver to unregister
//
#[no_mangle]
pub unsafe extern "C" fn mdev_unregister_driver(drv: *mut mdev_driver) {
    void mdev_unregister_driver(struct mdev_driver *drv)
    {
    driver_unregister(&drv.driver);
    }
    EXPORT_SYMBOL(mdev_unregister_driver);
