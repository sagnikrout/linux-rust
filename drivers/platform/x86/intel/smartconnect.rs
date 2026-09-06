//! Automatically rewritten from C to Rust
//! Source: drivers/platform/x86/intel/smartconnect.c
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

    MODULE_DESCRIPTION("Intel Smart Connect disabling driver");
    MODULE_LICENSE("GPL");
#[no_mangle]
unsafe extern "C" fn smartconnect_acpi_probe(pdev: *mut platform_device) -> c_int {
    static int smartconnect_acpi_probe(struct platform_device *pdev)
    {
    unsigned long long value;
    acpi_handle handle;
    acpi_status status;
    handle = ACPI_HANDLE(&pdev.dev);
    if (!handle)
    return -ENODEV;
    status = acpi_evaluate_integer(handle, "GAOS", core::ptr::null_mut(), &value);
    if (ACPI_FAILURE(status))
    return -EINVAL;
    if (value & 0x1) {
    dev_info(&pdev.dev, "Disabling Intel Smart Connect\n");
    status = acpi_execute_simple_method(handle, "SAOS", 0);
    }
    return 0;
    }
    static const struct acpi_device_id smartconnect_ids[] = {
    {"INT33A0", 0},
    {"", 0}
    };
    MODULE_DEVICE_TABLE(acpi, smartconnect_ids);
    static struct platform_driver smartconnect_driver = {
    .probe = smartconnect_acpi_probe,
    .driver = {
    .name = "intel_smart_connect",
    .acpi_match_table = smartconnect_ids,
    },
    };
    module_platform_driver(smartconnect_driver);
