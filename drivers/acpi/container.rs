//! Automatically rewritten from C to Rust
//! Source: drivers/acpi/container.c
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
// container.c  - ACPI Generic Container Driver
//
// Copyright (C) 2004 Anil S Keshavamurthy (anil.s.keshavamurthy@intel.com)
// Copyright (C) 2004 Keiichiro Tokunaga (tokunaga.keiich@jp.fujitsu.com)
// Copyright (C) 2004 Motoyuki Ito (motoyuki@soft.fujitsu.com)
// Copyright (C) 2004 FUJITSU LIMITED
// Copyright (C) 2004, 2013 Intel Corp.
// Author: Rafael J. Wysocki <rafael.j.wysocki@intel.com>
//

    static const struct acpi_device_id container_device_ids[] = {
    {"ACPI0004", 0},
    {"PNP0A05", 0},
    {"PNP0A06", 0},
    {"", 0},
    };

#[no_mangle]
unsafe extern "C" fn check_offline(adev: *mut acpi_device, not_used: *mut c_void) -> c_int {
    static int check_offline(struct acpi_device *adev, void *not_used)
    {
    if (acpi_scan_is_offline(adev, false))
    return 0;
    return -EBUSY;
    }
#[no_mangle]
unsafe extern "C" fn acpi_container_offline(cdev: *mut container_dev) -> c_int {
    static int acpi_container_offline(struct container_dev *cdev)
    {
// Check all of the dependent devices' physical companions.
    return acpi_dev_for_each_child(ACPI_COMPANION(&cdev.dev), check_offline, core::ptr::null_mut());
    }
#[no_mangle]
unsafe extern "C" fn acpi_container_release(dev: *mut device) {
    static void acpi_container_release(struct device *dev)
    {
    kfree(to_container_dev(dev));
    }
    static int container_device_attach(struct acpi_device *adev,
    const struct acpi_device_id *not_used)
    {
    struct container_dev *cdev;
    struct device *dev;
    int ret;
    if (adev.flags.is_dock_station)
    return 0;
    cdev = kzalloc_obj(*cdev);
    if (!cdev)
    return -ENOMEM;
    cdev.offline = acpi_container_offline;
    dev = &cdev.dev;
    dev.bus = &container_subsys;
    dev_set_name(dev, "%s", dev_name(&adev.dev));
    ACPI_COMPANION_SET(dev, adev);
    dev.release = acpi_container_release;
    ret = device_register(dev);
    if (ret) {
    put_device(dev);
    return ret;
    }
    adev.driver_data = dev;
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn container_device_detach(adev: *mut acpi_device) {
    static void container_device_detach(struct acpi_device *adev)
    {
    struct device *dev = acpi_driver_data(adev);
    adev.driver_data = core::ptr::null_mut();
    if (dev)
    device_unregister(dev);
    }
#[no_mangle]
unsafe extern "C" fn container_device_online(adev: *mut acpi_device) {
    static void container_device_online(struct acpi_device *adev)
    {
    struct device *dev = acpi_driver_data(adev);
    kobject_uevent(&dev.kobj, KOBJ_ONLINE);
    }
    static struct acpi_scan_handler container_handler = {
    .ids = container_device_ids,
    .attach = container_device_attach,
    .detach = container_device_detach,
    .hotplug = {
    .enabled = true,
    .demand_offline = true,
    .notify_online = container_device_online,
    },
    };
#[no_mangle]
pub unsafe extern "C" fn acpi_container_init() -> void __init {
    void __init acpi_container_init(void)
    {
    acpi_scan_add_handler(&container_handler);
    }

    static struct acpi_scan_handler container_handler = {
    .ids = container_device_ids,
    };
#[no_mangle]
pub unsafe extern "C" fn acpi_container_init() -> void __init {
    void __init acpi_container_init(void)
    {
    acpi_scan_add_handler_with_hotplug(&container_handler, "container");
    }
