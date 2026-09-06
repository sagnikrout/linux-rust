//! Automatically rewritten from C to Rust
//! Source: drivers/platform/x86/intel/rst.c
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
// Copyright 2013 Matthew Garrett <mjg59@srcf.ucam.org>
//

    MODULE_DESCRIPTION("Intel Rapid Start Technology Driver");
    MODULE_LICENSE("GPL");
    static ssize_t irst_show_wakeup_events(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    struct acpi_device *acpi;
    unsigned long long value;
    acpi_status status;
    acpi = to_acpi_device(dev);
    status = acpi_evaluate_integer(acpi.handle, "GFFS", core::ptr::null_mut(), &value);
    if (ACPI_FAILURE(status))
    return -EINVAL;
    return sprintf(buf, "%lld\n", value);
    }
    static ssize_t irst_store_wakeup_events(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t count)
    {
    struct acpi_device *acpi;
    acpi_status status;
    unsigned long value;
    int error;
    acpi = to_acpi_device(dev);
    error = kstrtoul(buf, 0, &value);
    if (error)
    return error;
    status = acpi_execute_simple_method(acpi.handle, "SFFS", value);
    if (ACPI_FAILURE(status))
    return -EINVAL;
    return count;
    }
    static struct device_attribute irst_wakeup_attr = {
    .attr = { .name = "wakeup_events", .mode = 0600 },
    .show = irst_show_wakeup_events,
    .store = irst_store_wakeup_events
    };
    static ssize_t irst_show_wakeup_time(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct acpi_device *acpi;
    unsigned long long value;
    acpi_status status;
    acpi = to_acpi_device(dev);
    status = acpi_evaluate_integer(acpi.handle, "GFTV", core::ptr::null_mut(), &value);
    if (ACPI_FAILURE(status))
    return -EINVAL;
    return sprintf(buf, "%lld\n", value);
    }
    static ssize_t irst_store_wakeup_time(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t count)
    {
    struct acpi_device *acpi;
    acpi_status status;
    unsigned long value;
    int error;
    acpi = to_acpi_device(dev);
    error = kstrtoul(buf, 0, &value);
    if (error)
    return error;
    status = acpi_execute_simple_method(acpi.handle, "SFTV", value);
    if (ACPI_FAILURE(status))
    return -EINVAL;
    return count;
    }
    static struct device_attribute irst_timeout_attr = {
    .attr = { .name = "wakeup_time", .mode = 0600 },
    .show = irst_show_wakeup_time,
    .store = irst_store_wakeup_time
    };
#[no_mangle]
unsafe extern "C" fn irst_probe(pdev: *mut platform_device) -> c_int {
    static int irst_probe(struct platform_device *pdev)
    {
    struct acpi_device *acpi;
    int error;
    acpi = ACPI_COMPANION(&pdev.dev);
    if (!acpi)
    return -ENODEV;
    error = device_create_file(&acpi.dev, &irst_timeout_attr);
    if (unlikely(error))
    return error;
    error = device_create_file(&acpi.dev, &irst_wakeup_attr);
    if (unlikely(error))
    device_remove_file(&acpi.dev, &irst_timeout_attr);
    return error;
    }
#[no_mangle]
unsafe extern "C" fn irst_remove(pdev: *mut platform_device) {
    static void irst_remove(struct platform_device *pdev)
    {
    struct acpi_device *acpi = ACPI_COMPANION(&pdev.dev);
    device_remove_file(&acpi.dev, &irst_wakeup_attr);
    device_remove_file(&acpi.dev, &irst_timeout_attr);
    }
    static const struct acpi_device_id irst_ids[] = {
    {"INT3392", 0},
    {"", 0}
    };
    static struct platform_driver irst_driver = {
    .probe = irst_probe,
    .remove = irst_remove,
    .driver = {
    .name = "intel_rapid_start",
    .acpi_match_table = irst_ids,
    },
    };
    module_platform_driver(irst_driver);
    MODULE_DEVICE_TABLE(acpi, irst_ids);
