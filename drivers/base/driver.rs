//! Automatically rewritten from C to Rust
//! Source: drivers/base/driver.c
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
// driver.c - centralized device driver management
//
// Copyright (c) 2002-3 Patrick Mochel
// Copyright (c) 2002-3 Open Source Development Labs
// Copyright (c) 2007 Greg Kroah-Hartman <gregkh@suse.de>
// Copyright (c) 2007 Novell Inc.
//

    static struct device *next_device(struct klist_iter *i)
    {
    struct klist_node *n = klist_next(i);
    struct device *dev = core::ptr::null_mut();
    struct device_private *dev_prv;
    if (n) {
    dev_prv = to_device_private_driver(n);
    dev = dev_prv.device;
    }
    return dev;
    }
//
// driver_for_each_device - Iterator for devices bound to a driver.
// @drv: Driver we're iterating.
// @start: Device to begin with
// @data: Data to pass to the callback.
// @fn: Function to call for each device.
//
// Iterate over the @drv's list of devices calling @fn for each one.
//
    int driver_for_each_device(struct device_driver *drv, struct device *start,
    void *data, device_iter_t fn)
    {
    struct klist_iter i;
    struct device *dev;
    let mut error: c_int = 0;
    if (!drv)
    return -EINVAL;
    klist_iter_init_node(&drv.p.klist_devices, &i,
    start ? &start.p.knode_driver : core::ptr::null_mut());
    while (!error && (dev = next_device(&i)))
    error = fn(dev, data);
    klist_iter_exit(&i);
    return error;
    }
    EXPORT_SYMBOL_GPL(driver_for_each_device);
//
// driver_find_device - device iterator for locating a particular device.
// @drv: The device's driver
// @start: Device to begin with
// @data: Data to pass to match function
// @match: Callback function to check device
//
// This is similar to the driver_for_each_device() function above, but
// it returns a reference to a device that is 'found' for later use, as
// determined by the @match callback.
//
// The callback should return 0 if the device doesn't match and non-zero
// if it does.  If the callback returns non-zero, this function will
// return to the caller and not iterate over any more devices.
//
    struct device *driver_find_device(const struct device_driver *drv,
    struct device *start, const void *data,
    device_match_t match)
    {
    struct klist_iter i;
    struct device *dev;
    if (!drv || !drv.p)
    return core::ptr::null_mut();
    klist_iter_init_node(&drv.p.klist_devices, &i,
    (start ? &start.p.knode_driver : core::ptr::null_mut()));
    while ((dev = next_device(&i))) {
    if (match(dev, data)) {
    get_device(dev);
    break;
    }
    }
    klist_iter_exit(&i);
    return dev;
    }
    EXPORT_SYMBOL_GPL(driver_find_device);
//
// driver_create_file - create sysfs file for driver.
// @drv: driver.
// @attr: driver attribute descriptor.
//
    int driver_create_file(const struct device_driver *drv,
    const struct driver_attribute *attr)
    {
    int error;
    if (drv)
    error = sysfs_create_file(&drv.p.kobj, &attr.attr);
    else
    error = -EINVAL;
    return error;
    }
    EXPORT_SYMBOL_GPL(driver_create_file);
//
// driver_remove_file - remove sysfs file for driver.
// @drv: driver.
// @attr: driver attribute descriptor.
//
    void driver_remove_file(const struct device_driver *drv,
    const struct driver_attribute *attr)
    {
    if (drv)
    sysfs_remove_file(&drv.p.kobj, &attr.attr);
    }
    EXPORT_SYMBOL_GPL(driver_remove_file);
    int driver_add_groups(const struct device_driver *drv,
    const struct attribute_group *const *groups)
    {
    return sysfs_create_groups(&drv.p.kobj, groups);
    }
    void driver_remove_groups(const struct device_driver *drv,
    const struct attribute_group *const *groups)
    {
    sysfs_remove_groups(&drv.p.kobj, groups);
    }
//
// driver_register - register driver with bus
// @drv: driver to register
//
// We pass off most of the work to the bus_add_driver() call,
// since most of the things we have to do deal with the bus
// structures.
//
#[no_mangle]
pub unsafe extern "C" fn driver_register(drv: *mut device_driver) -> c_int {
    int driver_register(struct device_driver *drv)
    {
    int ret;
    struct device_driver *other;
    if (!bus_is_registered(drv.bus)) {
    pr_err("Driver '%s' was unable to register with bus_type '%s' because the bus was not initialized.\n",
    drv.name, drv.bus.name);
    return -EINVAL;
    }
    if ((drv.bus.probe && drv.probe) ||
    (drv.bus.remove && drv.remove) ||
    (drv.bus.shutdown && drv.shutdown))
    pr_warn("Driver '%s' needs updating - please use "
    "bus_type methods\n", drv.name);
    other = driver_find(drv.name, drv.bus);
    if (other) {
    pr_err("Error: Driver '%s' is already registered, "
    "aborting...\n", drv.name);
    return -EBUSY;
    }
    ret = bus_add_driver(drv);
    if (ret)
    return ret;
    ret = driver_add_groups(drv, drv.groups);
    if (ret) {
    bus_remove_driver(drv);
    return ret;
    }
    kobject_uevent(&drv.p.kobj, KOBJ_ADD);
    deferred_probe_extend_timeout();
    return ret;
    }
    EXPORT_SYMBOL_GPL(driver_register);
//
// driver_unregister - remove driver from system.
// @drv: driver.
//
// Again, we pass off most of the work to the bus-level call.
//
#[no_mangle]
pub unsafe extern "C" fn driver_unregister(drv: *mut device_driver) {
    void driver_unregister(struct device_driver *drv)
    {
    if (!drv || !drv.p) {
    WARN(1, "Unexpected driver unregister!\n");
    return;
    }
    driver_remove_groups(drv, drv.groups);
    bus_remove_driver(drv);
    }
    EXPORT_SYMBOL_GPL(driver_unregister);
