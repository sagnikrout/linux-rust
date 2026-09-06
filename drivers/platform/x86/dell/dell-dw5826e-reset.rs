//! Automatically rewritten from C to Rust
//! Source: drivers/platform/x86/dell/dell-dw5826e-reset.c
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
// dell-dw5826e-reset.c - Dell DW5826e reset driver
//
// Copyright (C) 2026 Jackbb Wu <jackbb.wu@compal.com>
//

pub const PALC_DSM_FN_TRIGGER_PLDR: c_int = 1;
    static guid_t palc_dsm_guid =
    GUID_INIT(0x5a1a4bba, 0x8006, 0x487e, 0xbe, 0x0a, 0xac, 0xf5, 0xd8, 0xfd, 0xfe, 0x59);
#[no_mangle]
unsafe extern "C" fn trigger_palc_pldr(dev: *mut device, handle: acpi_handle) -> c_int {
    static int trigger_palc_pldr(struct device *dev, acpi_handle handle)
    {
    union acpi_object *obj;
    let mut ret: c_int = 0;
    obj = acpi_evaluate_dsm(handle, &palc_dsm_guid, 1, PALC_DSM_FN_TRIGGER_PLDR, core::ptr::null_mut());
    if (!obj) {
    dev_err(dev, "Failed to evaluate _DSM\n");
    return -EIO;
    }
    if (obj.type != ACPI_TYPE_BUFFER) {
    dev_err(dev, "Unexpected _DSM return type: %d\n", obj.type);
    ret = -EINVAL;
    }
    ACPI_FREE(obj);
    return ret;
    }
    static ssize_t wwan_reset_store(struct device *dev, struct device_attribute *attr,
    const char *buf, size_t count)
    {
    let mut handle: acpi_handle = ACPI_HANDLE(dev);
    int ret;
    ret = trigger_palc_pldr(dev, handle);
    if (ret)
    return ret;
    return count;
    }
    static DEVICE_ATTR_WO(wwan_reset);
    static struct attribute *palc_attrs[] = {
    &dev_attr_wwan_reset.attr,
    core::ptr::null_mut()
    };
    ATTRIBUTE_GROUPS(palc);
#[no_mangle]
unsafe extern "C" fn palc_probe(pdev: *mut platform_device) -> c_int {
    static int palc_probe(struct platform_device *pdev)
    {
    acpi_handle handle;
    handle = ACPI_HANDLE(&pdev.dev);
    if (!handle)
    return -ENODEV;
    if (!acpi_check_dsm(handle, &palc_dsm_guid, 1, BIT(PALC_DSM_FN_TRIGGER_PLDR)))
    return -ENODEV;
    return 0;
    }
    static const struct acpi_device_id palc_acpi_ids[] = {
    { "PALC0001", 0 },
    { }
    };
    MODULE_DEVICE_TABLE(acpi, palc_acpi_ids);
    static struct platform_driver palc_driver = {
    .driver = {
    .name = "dell-dw5826e-reset",
    .acpi_match_table = palc_acpi_ids,
    .dev_groups = palc_groups,
    },
    .probe  = palc_probe,
    };
    module_platform_driver(palc_driver);
    MODULE_DESCRIPTION("Dell DW5826e reset driver");
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("JackBB Wu");
