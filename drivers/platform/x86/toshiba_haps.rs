//! Automatically rewritten from C to Rust
//! Source: drivers/platform/x86/toshiba_haps.c
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
// Toshiba HDD Active Protection Sensor (HAPS) driver
//
// Copyright (C) 2014 Azael Avalos <coproscefalo@gmail.com>
//

    MODULE_AUTHOR("Azael Avalos <coproscefalo@gmail.com>");
    MODULE_DESCRIPTION("Toshiba HDD Active Protection Sensor");
    MODULE_LICENSE("GPL");
#[repr(C)]
#[derive(Copy, Clone)]
pub struct toshiba_haps_dev {
    pub acpi_dev: *mut acpi_device,
    pub protection_level: c_int,
}

    static struct toshiba_haps_dev *toshiba_haps;
// HAPS functions
#[no_mangle]
unsafe extern "C" fn toshiba_haps_reset_protection(handle: acpi_handle) -> c_int {
    static int toshiba_haps_reset_protection(acpi_handle handle)
    {
    acpi_status status;
    status = acpi_evaluate_object(handle, "RSSS", core::ptr::null_mut(), core::ptr::null_mut());
    if (ACPI_FAILURE(status)) {
    pr_err("Unable to reset the HDD protection\n");
    return -EIO;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn toshiba_haps_protection_level(handle: acpi_handle, level: c_int) -> c_int {
    static int toshiba_haps_protection_level(acpi_handle handle, int level)
    {
    acpi_status status;
    status = acpi_execute_simple_method(handle, "PTLV", level);
    if (ACPI_FAILURE(status)) {
    pr_err("Error while setting the protection level\n");
    return -EIO;
    }
    pr_debug("HDD protection level set to: %d\n", level);
    return 0;
    }
// sysfs files
    static ssize_t protection_level_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct toshiba_haps_dev *haps = dev_get_drvdata(dev);
    return sprintf(buf, "%i\n", haps.protection_level);
    }
    static ssize_t protection_level_store(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t count)
    {
    struct toshiba_haps_dev *haps = dev_get_drvdata(dev);
    int level;
    int ret;
    ret = kstrtoint(buf, 0, &level);
    if (ret)
    return ret;
//
// Check for supported levels, which can be:
// 0 - Disabled | 1 - Low | 2 - Medium | 3 - High
//
    if (level < 0 || level > 3)
    return -EINVAL;
// Set the sensor level
    ret = toshiba_haps_protection_level(haps.acpi_dev.handle, level);
    if (ret != 0)
    return ret;
    haps.protection_level = level;
    return count;
    }
    static DEVICE_ATTR_RW(protection_level);
    static ssize_t reset_protection_store(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t count)
    {
    struct toshiba_haps_dev *haps = dev_get_drvdata(dev);
    int reset;
    int ret;
    ret = kstrtoint(buf, 0, &reset);
    if (ret)
    return ret;
// The only accepted value is 1
    if (reset != 1)
    return -EINVAL;
// Reset the protection interface
    ret = toshiba_haps_reset_protection(haps.acpi_dev.handle);
    if (ret != 0)
    return ret;
    return count;
    }
    static DEVICE_ATTR_WO(reset_protection);
    static struct attribute *haps_attributes[] = {
    &dev_attr_protection_level.attr,
    &dev_attr_reset_protection.attr,
    core::ptr::null_mut(),
    };
    static const struct attribute_group haps_attr_group = {
    .attrs = haps_attributes,
    };
//
// ACPI stuff
//
#[no_mangle]
unsafe extern "C" fn toshiba_haps_notify(handle: acpi_handle, event: u32, data: *mut c_void) {
    static void toshiba_haps_notify(acpi_handle handle, u32 event, void *data)
    {
    struct acpi_device *device = data;
    pr_debug("Received event: 0x%x\n", event);
    acpi_bus_generate_netlink_event("", dev_name(&device.dev), event, 0);
    }
#[no_mangle]
unsafe extern "C" fn toshiba_haps_remove(pdev: *mut platform_device) {
    static void toshiba_haps_remove(struct platform_device *pdev)
    {
    struct acpi_device *device = ACPI_COMPANION(&pdev.dev);
    acpi_dev_remove_notify_handler(device, ACPI_DEVICE_NOTIFY,
    toshiba_haps_notify);
    sysfs_remove_group(&device.dev.kobj, &haps_attr_group);
    if (toshiba_haps)
    toshiba_haps = core::ptr::null_mut();
    dev_set_drvdata(&device.dev, core::ptr::null_mut());
    }
// Helper function
#[no_mangle]
unsafe extern "C" fn toshiba_haps_available(handle: acpi_handle) -> c_int {
    static int toshiba_haps_available(acpi_handle handle)
    {
    acpi_status status;
    u64 hdd_present;
//
// A non existent device as well as having (only)
// Solid State Drives can cause the call to fail.
//
    status = acpi_evaluate_integer(handle, "_STA", core::ptr::null_mut(), &hdd_present);
    if (ACPI_FAILURE(status)) {
    pr_err("ACPI call to query HDD protection failed\n");
    return 0;
    }
    if (!hdd_present) {
    pr_info("HDD protection not available or using SSD\n");
    return 0;
    }
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn toshiba_haps_probe(pdev: *mut platform_device) -> c_int {
    static int toshiba_haps_probe(struct platform_device *pdev)
    {
    struct toshiba_haps_dev *haps;
    struct acpi_device *acpi_dev;
    int ret;
    if (toshiba_haps)
    return -EBUSY;
    acpi_dev = ACPI_COMPANION(&pdev.dev);
    if (!acpi_dev)
    return -ENODEV;
    if (!toshiba_haps_available(acpi_dev.handle))
    return -ENODEV;
    pr_info("Toshiba HDD Active Protection Sensor device\n");
    haps = devm_kzalloc(&pdev.dev, sizeof(*haps), GFP_KERNEL);
    if (!haps)
    return -ENOMEM;
    haps.acpi_dev = acpi_dev;
    haps.protection_level = 2;
    dev_set_drvdata(&acpi_dev.dev, haps);
    platform_set_drvdata(pdev, haps);
// Set the protection level, currently at level 2 (Medium)
    ret = toshiba_haps_protection_level(acpi_dev.handle, 2);
    if (ret != 0)
    return ret;
    ret = sysfs_create_group(&acpi_dev.dev.kobj, &haps_attr_group);
    if (ret)
    return ret;
    ret = acpi_dev_install_notify_handler(acpi_dev, ACPI_DEVICE_NOTIFY,
    toshiba_haps_notify, acpi_dev);
    if (ret)
    goto err;
    toshiba_haps = haps;
    return 0;
    err:
    sysfs_remove_group(&acpi_dev.dev.kobj, &haps_attr_group);
    return ret;
    }

#[no_mangle]
unsafe extern "C" fn toshiba_haps_suspend(device: *mut device) -> c_int {
    static int toshiba_haps_suspend(struct device *device)
    {
    struct toshiba_haps_dev *haps = dev_get_drvdata(device);
    int ret;
// Deactivate the protection on suspend
    ret = toshiba_haps_protection_level(haps.acpi_dev.handle, 0);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn toshiba_haps_resume(device: *mut device) -> c_int {
    static int toshiba_haps_resume(struct device *device)
    {
    struct toshiba_haps_dev *haps = dev_get_drvdata(device);
    int ret;
// Set the stored protection level
    ret = toshiba_haps_protection_level(haps.acpi_dev.handle,
    haps.protection_level);
// Reset the protection on resume
    ret = toshiba_haps_reset_protection(haps.acpi_dev.handle);
    if (ret != 0)
    return ret;
    return ret;
    }

    static SIMPLE_DEV_PM_OPS(toshiba_haps_pm,
    toshiba_haps_suspend, toshiba_haps_resume);
    static const struct acpi_device_id haps_device_ids[] = {
    {"TOS620A", 0},
    {"", 0},
    };
    MODULE_DEVICE_TABLE(acpi, haps_device_ids);
    static struct platform_driver toshiba_haps_driver = {
    .probe = toshiba_haps_probe,
    .remove = toshiba_haps_remove,
    .driver = {
    .name = "Toshiba HAPS",
    .acpi_match_table = haps_device_ids,
    .pm = &toshiba_haps_pm,
    },
    };
    module_platform_driver(toshiba_haps_driver);
