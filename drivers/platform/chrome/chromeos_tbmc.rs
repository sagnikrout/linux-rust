//! Automatically rewritten from C to Rust
//! Source: drivers/platform/chrome/chromeos_tbmc.c
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
// Driver to detect Tablet Mode for ChromeOS convertible.
//
// Copyright (C) 2017 Google, Inc.
// Author: Gwendal Grignou <gwendal@chromium.org>
//
// On Chromebook using ACPI, this device listens for notification
// from GOOG0006 and issue method TBMC to retrieve the status.
//
// GOOG0006 issues the notification when it receives EC_HOST_EVENT_MODE_CHANGE
// from the EC.
// Method TBMC reads EC_ACPI_MEM_DEVICE_ORIENTATION byte from the shared
// memory region.

    static int chromeos_tbmc_query_switch(struct acpi_device *adev,
    struct input_dev *idev)
    {
    unsigned long long state;
    acpi_status status;
    status = acpi_evaluate_integer(adev.handle, "TBMC", core::ptr::null_mut(), &state);
    if (ACPI_FAILURE(status))
    return -ENODEV;
// input layer checks if event is redundant
    input_report_switch(idev, SW_TABLET_MODE, state);
    input_sync(idev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn chromeos_tbmc_resume(dev: *mut device) -> __maybe_unused int {
    static __maybe_unused int chromeos_tbmc_resume(struct device *dev)
    {
    return chromeos_tbmc_query_switch(ACPI_COMPANION(dev), dev_get_drvdata(dev));
    }
#[no_mangle]
unsafe extern "C" fn chromeos_tbmc_notify(handle: acpi_handle, event: u32, data: *mut c_void) {
    static void chromeos_tbmc_notify(acpi_handle handle, u32 event, void *data)
    {
    struct device *dev = data;
    acpi_pm_wakeup_event(dev);
    switch (event) {
    case 0x80:
    chromeos_tbmc_query_switch(ACPI_COMPANION(dev), dev_get_drvdata(dev));
    break;
    default:
    dev_err(dev, "Unexpected event: 0x%08X\n", event);
    }
    }
#[no_mangle]
unsafe extern "C" fn chromeos_tbmc_open(idev: *mut input_dev) -> c_int {
    static int chromeos_tbmc_open(struct input_dev *idev)
    {
    struct acpi_device *adev = input_get_drvdata(idev);
    return chromeos_tbmc_query_switch(adev, idev);
    }
#[no_mangle]
unsafe extern "C" fn chromeos_tbmc_probe(pdev: *mut platform_device) -> c_int {
    static int chromeos_tbmc_probe(struct platform_device *pdev)
    {
    struct input_dev *idev;
    struct device *dev = &pdev.dev;
    struct acpi_device *adev;
    int ret;
    adev = ACPI_COMPANION(dev);
    if (!adev)
    return -ENODEV;
    idev = devm_input_allocate_device(dev);
    if (!idev)
    return -ENOMEM;
    idev.name = "Tablet Mode Switch";
    idev.phys = acpi_device_hid(adev);
    idev.id.bustype = BUS_HOST;
    idev.id.version = 1;
    idev.id.product = 0;
    idev.open = chromeos_tbmc_open;
    input_set_drvdata(idev, adev);
    platform_set_drvdata(pdev, idev);
    input_set_capability(idev, EV_SW, SW_TABLET_MODE);
    ret = input_register_device(idev);
    if (ret) {
    dev_err(dev, "cannot register input device\n");
    return ret;
    }
    device_init_wakeup(dev, true);
    ret = acpi_dev_install_notify_handler(adev, ACPI_DEVICE_NOTIFY,
    chromeos_tbmc_notify, dev);
    if (ret) {
    dev_err(dev, "cannot install ACPI notify handler\n");
    device_init_wakeup(dev, false);
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn chromeos_tbmc_remove(pdev: *mut platform_device) {
    static void chromeos_tbmc_remove(struct platform_device *pdev)
    {
    acpi_dev_remove_notify_handler(ACPI_COMPANION(&pdev.dev),
    ACPI_DEVICE_NOTIFY, chromeos_tbmc_notify);
    device_init_wakeup(&pdev.dev, false);
    }
    static const struct acpi_device_id chromeos_tbmc_acpi_device_ids[] = {
    { ACPI_DRV_NAME, 0 },
    { }
    };
    MODULE_DEVICE_TABLE(acpi, chromeos_tbmc_acpi_device_ids);
    static SIMPLE_DEV_PM_OPS(chromeos_tbmc_pm_ops, core::ptr::null_mut(),
    chromeos_tbmc_resume);
    static struct platform_driver chromeos_tbmc_driver = {
    .probe = chromeos_tbmc_probe,
    .remove = chromeos_tbmc_remove,
    .driver = {
    .name = DRV_NAME,
    .acpi_match_table = chromeos_tbmc_acpi_device_ids,
    .pm = &chromeos_tbmc_pm_ops,
    },
    };
    module_platform_driver(chromeos_tbmc_driver);
    MODULE_LICENSE("GPL v2");
    MODULE_DESCRIPTION("ChromeOS ACPI tablet switch driver");
