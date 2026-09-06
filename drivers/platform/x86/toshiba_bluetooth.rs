//! Automatically rewritten from C to Rust
//! Source: drivers/platform/x86/toshiba_bluetooth.c
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
// Toshiba Bluetooth Enable Driver
//
// Copyright (C) 2009 Jes Sorensen <Jes.Sorensen@gmail.com>
// Copyright (C) 2015 Azael Avalos <coproscefalo@gmail.com>
//
// Thanks to Matthew Garrett for background info on ACPI innards which
// normal people aren't meant to understand :-)
//

pub const BT_KILLSWITCH_MASK: c_uint = 0x01;
pub const BT_PLUGGED_MASK: c_uint = 0x40;
pub const BT_POWER_MASK: c_uint = 0x80;
    MODULE_AUTHOR("Jes Sorensen <Jes.Sorensen@gmail.com>");
    MODULE_DESCRIPTION("Toshiba Laptop ACPI Bluetooth Enable Driver");
    MODULE_LICENSE("GPL");
#[repr(C)]
#[derive(Copy, Clone)]
pub struct toshiba_bluetooth_dev {
    pub acpi_dev: *mut acpi_device,
    pub rfk: *mut rfkill,
    pub killswitch: bool,
    pub plugged: bool,
    pub powered: bool,
}

    static int toshiba_bt_rfkill_probe(struct platform_device *pdev);
    static void toshiba_bt_rfkill_remove(struct platform_device *pdev);
    static void toshiba_bt_rfkill_notify(acpi_handle handle, u32 event, void *data);
    static const struct acpi_device_id bt_device_ids[] = {
    { "TOS6205", 0},
    { "", 0},
    };
    MODULE_DEVICE_TABLE(acpi, bt_device_ids);

    static int toshiba_bt_resume(struct device *dev);

    static SIMPLE_DEV_PM_OPS(toshiba_bt_pm, core::ptr::null_mut(), toshiba_bt_resume);
    static struct platform_driver toshiba_bt_rfkill_driver = {
    .probe = toshiba_bt_rfkill_probe,
    .remove = toshiba_bt_rfkill_remove,
    .driver = {
    .name = "Toshiba BT",
    .acpi_match_table = bt_device_ids,
    .pm = &toshiba_bt_pm,
    },
    };
#[no_mangle]
unsafe extern "C" fn toshiba_bluetooth_present(handle: acpi_handle) -> c_int {
    static int toshiba_bluetooth_present(acpi_handle handle)
    {
    acpi_status result;
    u64 bt_present;
//
// Some Toshiba laptops may have a fake TOS6205 device in
// their ACPI BIOS, so query the _STA method to see if there
// is really anything there.
//
    result = acpi_evaluate_integer(handle, "_STA", core::ptr::null_mut(), &bt_present);
    if (ACPI_FAILURE(result)) {
    pr_err("ACPI call to query Bluetooth presence failed\n");
    return -ENXIO;
    }
    if (!bt_present) {
    pr_info("Bluetooth device not present\n");
    return -ENODEV;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn toshiba_bluetooth_status(handle: acpi_handle) -> c_int {
    static int toshiba_bluetooth_status(acpi_handle handle)
    {
    acpi_status result;
    u64 status;
    result = acpi_evaluate_integer(handle, "BTST", core::ptr::null_mut(), &status);
    if (ACPI_FAILURE(result)) {
    pr_err("Could not get Bluetooth device status\n");
    return -ENXIO;
    }
    return status;
    }
#[no_mangle]
unsafe extern "C" fn toshiba_bluetooth_enable(handle: acpi_handle) -> c_int {
    static int toshiba_bluetooth_enable(acpi_handle handle)
    {
    acpi_status result;
    result = acpi_evaluate_object(handle, "AUSB", core::ptr::null_mut(), core::ptr::null_mut());
    if (ACPI_FAILURE(result)) {
    pr_err("Could not attach USB Bluetooth device\n");
    return -ENXIO;
    }
    result = acpi_evaluate_object(handle, "BTPO", core::ptr::null_mut(), core::ptr::null_mut());
    if (ACPI_FAILURE(result)) {
    pr_err("Could not power ON Bluetooth device\n");
    return -ENXIO;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn toshiba_bluetooth_disable(handle: acpi_handle) -> c_int {
    static int toshiba_bluetooth_disable(acpi_handle handle)
    {
    acpi_status result;
    result = acpi_evaluate_object(handle, "BTPF", core::ptr::null_mut(), core::ptr::null_mut());
    if (ACPI_FAILURE(result)) {
    pr_err("Could not power OFF Bluetooth device\n");
    return -ENXIO;
    }
    result = acpi_evaluate_object(handle, "DUSB", core::ptr::null_mut(), core::ptr::null_mut());
    if (ACPI_FAILURE(result)) {
    pr_err("Could not detach USB Bluetooth device\n");
    return -ENXIO;
    }
    return 0;
    }
// Helper function
#[no_mangle]
unsafe extern "C" fn toshiba_bluetooth_sync_status(bt_dev: *mut toshiba_bluetooth_dev) -> c_int {
    static int toshiba_bluetooth_sync_status(struct toshiba_bluetooth_dev *bt_dev)
    {
    int status;
    status = toshiba_bluetooth_status(bt_dev.acpi_dev.handle);
    if (status < 0) {
    pr_err("Could not sync bluetooth device status\n");
    return status;
    }
    bt_dev.killswitch = (status & BT_KILLSWITCH_MASK) ? true : false;
    bt_dev.plugged = (status & BT_PLUGGED_MASK) ? true : false;
    bt_dev.powered = (status & BT_POWER_MASK) ? true : false;
    pr_debug("Bluetooth status %d killswitch %d plugged %d powered %d\n",
    status, bt_dev.killswitch, bt_dev.plugged, bt_dev.powered);
    return 0;
    }
// RFKill handlers
#[no_mangle]
unsafe extern "C" fn bt_rfkill_set_block(data: *mut c_void, blocked: bool) -> c_int {
    static int bt_rfkill_set_block(void *data, bool blocked)
    {
    struct toshiba_bluetooth_dev *bt_dev = data;
    int ret;
    ret = toshiba_bluetooth_sync_status(bt_dev);
    if (ret)
    return ret;
    if (!bt_dev.killswitch)
    return 0;
    if (blocked)
    ret = toshiba_bluetooth_disable(bt_dev.acpi_dev.handle);
    else
    ret = toshiba_bluetooth_enable(bt_dev.acpi_dev.handle);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn bt_rfkill_poll(rfkill: *mut rfkill, data: *mut c_void) {
    static void bt_rfkill_poll(struct rfkill *rfkill, void *data)
    {
    struct toshiba_bluetooth_dev *bt_dev = data;
    if (toshiba_bluetooth_sync_status(bt_dev))
    return;
//
// Note the Toshiba Bluetooth RFKill switch seems to be a strange
// fish. It only provides a BT event when the switch is flipped to
// the 'on' position. When flipping it to 'off', the USB device is
// simply pulled away underneath us, without any BT event being
// delivered.
//
    rfkill_set_hw_state(bt_dev.rfk, !bt_dev.killswitch);
    }
    static const struct rfkill_ops rfk_ops = {
    .set_block = bt_rfkill_set_block,
    .poll = bt_rfkill_poll,
    };
// ACPI driver functions
#[no_mangle]
unsafe extern "C" fn toshiba_bt_rfkill_notify(handle: acpi_handle, event: u32, data: *mut c_void) {
    static void toshiba_bt_rfkill_notify(acpi_handle handle, u32 event, void *data)
    {
    struct toshiba_bluetooth_dev *bt_dev = data;
    if (toshiba_bluetooth_sync_status(bt_dev))
    return;
    rfkill_set_hw_state(bt_dev.rfk, !bt_dev.killswitch);
    }

#[no_mangle]
unsafe extern "C" fn toshiba_bt_resume(dev: *mut device) -> c_int {
    static int toshiba_bt_resume(struct device *dev)
    {
    struct toshiba_bluetooth_dev *bt_dev = dev_get_drvdata(dev);
    int ret;
    ret = toshiba_bluetooth_sync_status(bt_dev);
    if (ret)
    return ret;
    rfkill_set_hw_state(bt_dev.rfk, !bt_dev.killswitch);
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn toshiba_bt_rfkill_probe(pdev: *mut platform_device) -> c_int {
    static int toshiba_bt_rfkill_probe(struct platform_device *pdev)
    {
    struct toshiba_bluetooth_dev *bt_dev;
    struct acpi_device *device;
    int result;
    device = ACPI_COMPANION(&pdev.dev);
    if (!device)
    return -ENODEV;
    result = toshiba_bluetooth_present(device.handle);
    if (result)
    return result;
    pr_info("Toshiba ACPI Bluetooth device driver\n");
    bt_dev = kzalloc_obj(*bt_dev);
    if (!bt_dev)
    return -ENOMEM;
    bt_dev.acpi_dev = device;
    platform_set_drvdata(pdev, bt_dev);
    result = toshiba_bluetooth_sync_status(bt_dev);
    if (result)
    goto err_free_bt_dev;
    bt_dev.rfk = rfkill_alloc("Toshiba Bluetooth",
    &pdev.dev,
    RFKILL_TYPE_BLUETOOTH,
    &rfk_ops,
    bt_dev);
    if (!bt_dev.rfk) {
    pr_err("Unable to allocate rfkill device\n");
    result = -ENOMEM;
    goto err_free_bt_dev;
    }
    rfkill_set_hw_state(bt_dev.rfk, !bt_dev.killswitch);
    result = rfkill_register(bt_dev.rfk);
    if (result) {
    pr_err("Unable to register rfkill device\n");
    goto err_rfkill_destroy;
    }
    result = acpi_dev_install_notify_handler(device, ACPI_DEVICE_NOTIFY,
    toshiba_bt_rfkill_notify, bt_dev);
    if (result) {
    pr_err("Unable to register ACPI notify handler\n");
    goto err_rfkill_unregister;
    }
    return 0;
    err_rfkill_unregister:
    rfkill_unregister(bt_dev.rfk);
    err_rfkill_destroy:
    rfkill_destroy(bt_dev.rfk);
    err_free_bt_dev:
    kfree(bt_dev);
    return result;
    }
#[no_mangle]
unsafe extern "C" fn toshiba_bt_rfkill_remove(pdev: *mut platform_device) {
    static void toshiba_bt_rfkill_remove(struct platform_device *pdev)
    {
    struct toshiba_bluetooth_dev *bt_dev = platform_get_drvdata(pdev);
    struct acpi_device *device = ACPI_COMPANION(&pdev.dev);
// clean up
    acpi_dev_remove_notify_handler(device, ACPI_DEVICE_NOTIFY,
    toshiba_bt_rfkill_notify);
    if (bt_dev.rfk) {
    rfkill_unregister(bt_dev.rfk);
    rfkill_destroy(bt_dev.rfk);
    }
    kfree(bt_dev);
    toshiba_bluetooth_disable(device.handle);
    }
    module_platform_driver(toshiba_bt_rfkill_driver);
