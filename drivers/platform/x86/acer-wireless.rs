//! Automatically rewritten from C to Rust
//! Source: drivers/platform/x86/acer-wireless.c
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
// Acer Wireless Radio Control Driver
//
// Copyright (C) 2017 Endless Mobile, Inc.
//

    static const struct acpi_device_id acer_wireless_acpi_ids[] = {
    {"10251229", 0},
    {"", 0},
    };
    MODULE_DEVICE_TABLE(acpi, acer_wireless_acpi_ids);
#[no_mangle]
unsafe extern "C" fn acer_wireless_notify(handle: acpi_handle, event: u32, data: *mut c_void) {
    static void acer_wireless_notify(acpi_handle handle, u32 event, void *data)
    {
    struct device *dev = data;
    struct input_dev *idev = dev_get_drvdata(dev);
    dev_dbg(dev, "event=%#x\n", event);
    if (event != 0x80) {
    dev_notice(dev, "Unknown SMKB event: %#x\n", event);
    return;
    }
    input_report_key(idev, KEY_RFKILL, 1);
    input_sync(idev);
    input_report_key(idev, KEY_RFKILL, 0);
    input_sync(idev);
    }
#[no_mangle]
unsafe extern "C" fn acer_wireless_probe(pdev: *mut platform_device) -> c_int {
    static int acer_wireless_probe(struct platform_device *pdev)
    {
    struct acpi_device *adev;
    struct input_dev *idev;
    int ret;
    adev = ACPI_COMPANION(&pdev.dev);
    if (!adev)
    return -ENODEV;
    idev = devm_input_allocate_device(&pdev.dev);
    if (!idev)
    return -ENOMEM;
    platform_set_drvdata(pdev, idev);
    idev.name = "Acer Wireless Radio Control";
    idev.phys = "acer-wireless/input0";
    idev.id.bustype = BUS_HOST;
    idev.id.vendor = PCI_VENDOR_ID_AI;
    idev.id.product = 0x1229;
    set_bit(EV_KEY, idev.evbit);
    set_bit(KEY_RFKILL, idev.keybit);
    ret = input_register_device(idev);
    if (ret)
    return ret;
    return acpi_dev_install_notify_handler(adev, ACPI_DEVICE_NOTIFY,
    acer_wireless_notify,
    &pdev.dev);
    }
#[no_mangle]
unsafe extern "C" fn acer_wireless_remove(pdev: *mut platform_device) {
    static void acer_wireless_remove(struct platform_device *pdev)
    {
    acpi_dev_remove_notify_handler(ACPI_COMPANION(&pdev.dev),
    ACPI_DEVICE_NOTIFY,
    acer_wireless_notify);
    }
    static struct platform_driver acer_wireless_driver = {
    .probe = acer_wireless_probe,
    .remove = acer_wireless_remove,
    .driver = {
    .name = "Acer Wireless Radio Control Driver",
    .acpi_match_table = acer_wireless_acpi_ids,
    },
    };
    module_platform_driver(acer_wireless_driver);
    MODULE_DESCRIPTION("Acer Wireless Radio Control Driver");
    MODULE_AUTHOR("Chris Chiu <chiu@gmail.com>");
    MODULE_LICENSE("GPL v2");
