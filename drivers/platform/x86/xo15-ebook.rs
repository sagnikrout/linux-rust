//! Automatically rewritten from C to Rust
//! Source: drivers/platform/x86/xo15-ebook.c
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
// OLPC XO-1.5 ebook switch driver
// (based on generic ACPI button driver)
//
// Copyright (C) 2009 Paul Fox <pgf@laptop.org>
// Copyright (C) 2010 One Laptop per Child
//

pub const XO15_EBOOK_TYPE_UNKNOWN: c_uint = 0x00;
pub const XO15_EBOOK_NOTIFY_STATUS: c_uint = 0x80;

    MODULE_DESCRIPTION("OLPC XO-1.5 ebook switch driver");
    MODULE_LICENSE("GPL");
    static const struct acpi_device_id ebook_device_ids[] = {
    { XO15_EBOOK_HID, 0 },
    { "", 0 },
    };
    MODULE_DEVICE_TABLE(acpi, ebook_device_ids);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ebook_switch {
    pub input: *mut input_dev,
    pub /: *mut *mut char phys[32]; / for input device,
    pub gpe_enabled: bool,
}

#[no_mangle]
unsafe extern "C" fn ebook_send_state(dev: *mut device) -> c_int {
    static int ebook_send_state(struct device *dev)
    {
    struct ebook_switch *button = dev_get_drvdata(dev);
    unsigned long long state;
    acpi_status status;
    status = acpi_evaluate_integer(ACPI_HANDLE(dev), "EBK", core::ptr::null_mut(), &state);
    if (ACPI_FAILURE(status))
    return -EIO;
// input layer checks if event is redundant
    input_report_switch(button.input, SW_TABLET_MODE, !state);
    input_sync(button.input);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ebook_switch_notify(handle: acpi_handle, event: u32, data: *mut c_void) {
    static void ebook_switch_notify(acpi_handle handle, u32 event, void *data)
    {
    switch (event) {
    case ACPI_FIXED_HARDWARE_EVENT:
    case XO15_EBOOK_NOTIFY_STATUS:
    ebook_send_state(data);
    break;
    default:
    acpi_handle_debug(handle, "Unsupported event [0x%x]\n", event);
    break;
    }
    }

#[no_mangle]
unsafe extern "C" fn ebook_switch_resume(dev: *mut device) -> c_int {
    static int ebook_switch_resume(struct device *dev)
    {
    return ebook_send_state(dev);
    }

    static SIMPLE_DEV_PM_OPS(ebook_switch_pm, core::ptr::null_mut(), ebook_switch_resume);
#[no_mangle]
unsafe extern "C" fn ebook_switch_probe(pdev: *mut platform_device) -> c_int {
    static int ebook_switch_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct acpi_device *device = ACPI_COMPANION(dev);
    const struct acpi_device_id *id;
    struct ebook_switch *button;
    struct input_dev *input;
    int error;
    button = devm_kzalloc(dev, sizeof(*button), GFP_KERNEL);
    if (!button)
    return -ENOMEM;
    platform_set_drvdata(pdev, button);
    input = devm_input_allocate_device(dev);
    if (!input)
    return -ENOMEM;
    button.input = input;
    id = acpi_match_acpi_device(ebook_device_ids, device);
    if (!id)
    return dev_err_probe(dev, -ENODEV, "Unsupported hid\n");
    snprintf(button.phys, sizeof(button.phys), "%s/button/input0", id.id);
    input.name = "EBook Switch";
    input.phys = button.phys;
    input.id.bustype = BUS_HOST;
    input.evbit[0] = BIT_MASK(EV_SW);
    set_bit(SW_TABLET_MODE, input.swbit);
    error = input_register_device(input);
    if (error)
    return error;
    error = acpi_dev_install_notify_handler(device, ACPI_DEVICE_NOTIFY,
    ebook_switch_notify, dev);
    if (error)
    return error;
    ebook_send_state(dev);
    if (device.wakeup.flags.valid) {
// Button's GPE is run-wake GPE
    acpi_enable_gpe(device.wakeup.gpe_device,
    device.wakeup.gpe_number);
    button.gpe_enabled = true;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ebook_switch_remove(pdev: *mut platform_device) {
    static void ebook_switch_remove(struct platform_device *pdev)
    {
    struct ebook_switch *button = platform_get_drvdata(pdev);
    struct acpi_device *device = ACPI_COMPANION(&pdev.dev);
    if (button.gpe_enabled)
    acpi_disable_gpe(device.wakeup.gpe_device,
    device.wakeup.gpe_number);
    acpi_dev_remove_notify_handler(device, ACPI_DEVICE_NOTIFY,
    ebook_switch_notify);
    }
    static struct platform_driver xo15_ebook_driver = {
    .probe = ebook_switch_probe,
    .remove = ebook_switch_remove,
    .driver = {
    .name = MODULE_NAME,
    .acpi_match_table = ebook_device_ids,
    .pm = &ebook_switch_pm,
    },
    };
    module_platform_driver(xo15_ebook_driver);
