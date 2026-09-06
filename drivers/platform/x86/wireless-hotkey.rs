//! Automatically rewritten from C to Rust
//! Source: drivers/platform/x86/wireless-hotkey.c
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
// Airplane mode button for AMD, HP & Xiaomi laptops
//
// Copyright (C) 2014-2017 Alex Hung <alex.hung@canonical.com>
// Copyright (C) 2021 Advanced Micro Devices
//

    MODULE_DESCRIPTION("Airplane mode button for AMD, HP & Xiaomi laptops");
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Alex Hung");
    MODULE_ALIAS("acpi*:HPQ6001:*");
    MODULE_ALIAS("acpi*:WSTADEF:*");
    MODULE_ALIAS("acpi*:AMDI0051:*");
    MODULE_ALIAS("acpi*:LGEX0815:*");
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wl_button {
    pub input_dev: *mut input_dev,
    pub phys: [c_char; 32],
}

    static const struct acpi_device_id wl_ids[] = {
    {"HPQ6001", 0},
    {"WSTADEF", 0},
    {"AMDI0051", 0},
    {"LGEX0815", 0},
    {"", 0},
    };
#[no_mangle]
unsafe extern "C" fn wireless_input_setup(dev: *mut device) -> c_int {
    static int wireless_input_setup(struct device *dev)
    {
    struct wl_button *button = dev_get_drvdata(dev);
    int err;
    button.input_dev = input_allocate_device();
    if (!button.input_dev)
    return -ENOMEM;
    snprintf(button.phys, sizeof(button.phys), "%s/input0",
    acpi_device_hid(ACPI_COMPANION(dev)));
    button.input_dev.name = "Wireless hotkeys";
    button.input_dev.phys = button.phys;
    button.input_dev.id.bustype = BUS_HOST;
    button.input_dev.evbit[0] = BIT(EV_KEY);
    set_bit(KEY_RFKILL, button.input_dev.keybit);
    err = input_register_device(button.input_dev);
    if (err)
    goto err_free_dev;
    return 0;
    err_free_dev:
    input_free_device(button.input_dev);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn wireless_input_destroy(dev: *mut device) {
    static void wireless_input_destroy(struct device *dev)
    {
    struct wl_button *button = dev_get_drvdata(dev);
    input_unregister_device(button.input_dev);
    kfree(button);
    }
#[no_mangle]
unsafe extern "C" fn wl_notify(handle: acpi_handle, event: u32, data: *mut c_void) {
    static void wl_notify(acpi_handle handle, u32 event, void *data)
    {
    struct wl_button *button = data;
    if (event != 0x80) {
    pr_info("Received unknown event (0x%x)\n", event);
    return;
    }
    input_report_key(button.input_dev, KEY_RFKILL, 1);
    input_sync(button.input_dev);
    input_report_key(button.input_dev, KEY_RFKILL, 0);
    input_sync(button.input_dev);
    }
#[no_mangle]
unsafe extern "C" fn wl_probe(pdev: *mut platform_device) -> c_int {
    static int wl_probe(struct platform_device *pdev)
    {
    struct acpi_device *adev;
    struct wl_button *button;
    int err;
    adev = ACPI_COMPANION(&pdev.dev);
    if (!adev)
    return -ENODEV;
    button = kzalloc_obj(struct wl_button);
    if (!button)
    return -ENOMEM;
    platform_set_drvdata(pdev, button);
    err = wireless_input_setup(&pdev.dev);
    if (err) {
    pr_err("Failed to setup wireless hotkeys\n");
    kfree(button);
    return err;
    }
    err = acpi_dev_install_notify_handler(adev, ACPI_DEVICE_NOTIFY,
    wl_notify, button);
    if (err) {
    pr_err("Failed to install ACPI notify handler\n");
    wireless_input_destroy(&pdev.dev);
    }
    return err;
    }
#[no_mangle]
unsafe extern "C" fn wl_remove(pdev: *mut platform_device) {
    static void wl_remove(struct platform_device *pdev)
    {
    acpi_dev_remove_notify_handler(ACPI_COMPANION(&pdev.dev),
    ACPI_DEVICE_NOTIFY, wl_notify);
    wireless_input_destroy(&pdev.dev);
    }
    static struct platform_driver wl_driver = {
    .probe = wl_probe,
    .remove = wl_remove,
    .driver = {
    .name = "wireless-hotkey",
    .acpi_match_table = wl_ids,
    },
    };
    module_platform_driver(wl_driver);
