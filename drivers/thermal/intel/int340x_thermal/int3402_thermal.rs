//! Automatically rewritten from C to Rust
//! Source: drivers/thermal/intel/int340x_thermal/int3402_thermal.c
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
// INT3402 thermal driver for memory temperature reporting
//
// Copyright (C) 2014, Intel Corporation
// Authors: Aaron Lu <aaron.lu@intel.com>
//

pub const INT3402_PERF_CHANGED_EVENT: c_uint = 0x80;
pub const INT3402_THERMAL_EVENT: c_uint = 0x90;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct int3402_thermal_data {
    pub handle: *mut acpi_handle,
    pub int340x_zone: *mut int34x_thermal_zone,
}

#[no_mangle]
unsafe extern "C" fn int3402_notify(handle: acpi_handle, event: u32, data: *mut c_void) {
    static void int3402_notify(acpi_handle handle, u32 event, void *data)
    {
    struct int3402_thermal_data *priv = data;
    if (!priv)
    return;
    switch (event) {
    case INT3402_PERF_CHANGED_EVENT:
    break;
    case INT3402_THERMAL_EVENT:
    int340x_thermal_zone_device_update(priv.int340x_zone,
    THERMAL_TRIP_VIOLATED);
    break;
    default:
    break;
    }
    }
#[no_mangle]
unsafe extern "C" fn int3402_thermal_probe(pdev: *mut platform_device) -> c_int {
    static int int3402_thermal_probe(struct platform_device *pdev)
    {
    struct acpi_device *adev = ACPI_COMPANION(&pdev.dev);
    struct int3402_thermal_data *d;
    int ret;
    if (!adev)
    return -ENODEV;
    if (!acpi_has_method(adev.handle, "_TMP"))
    return -ENODEV;
    d = devm_kzalloc(&pdev.dev, sizeof(*d), GFP_KERNEL);
    if (!d)
    return -ENOMEM;
    d.int340x_zone = int340x_thermal_zone_add(adev, core::ptr::null_mut());
    if (IS_ERR(d.int340x_zone))
    return PTR_ERR(d.int340x_zone);
    ret = acpi_install_notify_handler(adev.handle,
    ACPI_DEVICE_NOTIFY,
    int3402_notify,
    d);
    if (ret) {
    int340x_thermal_zone_remove(d.int340x_zone);
    return ret;
    }
    d.handle = adev.handle;
    platform_set_drvdata(pdev, d);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn int3402_thermal_remove(pdev: *mut platform_device) {
    static void int3402_thermal_remove(struct platform_device *pdev)
    {
    struct int3402_thermal_data *d = platform_get_drvdata(pdev);
    acpi_remove_notify_handler(d.handle,
    ACPI_DEVICE_NOTIFY, int3402_notify);
    int340x_thermal_zone_remove(d.int340x_zone);
    }
    static const struct acpi_device_id int3402_thermal_match[] = {
    {"INT3402", 0},
    {}
    };
    MODULE_DEVICE_TABLE(acpi, int3402_thermal_match);
    static struct platform_driver int3402_thermal_driver = {
    .probe = int3402_thermal_probe,
    .remove = int3402_thermal_remove,
    .driver = {
    .name = "int3402 thermal",
    .acpi_match_table = int3402_thermal_match,
    },
    };
    module_platform_driver(int3402_thermal_driver);
    MODULE_DESCRIPTION("INT3402 Thermal driver");
    MODULE_LICENSE("GPL");
