//! Automatically rewritten from C to Rust
//! Source: drivers/platform/x86/adv_swbutton.c
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
//
// adv_swbutton.c - Software Button Interface Driver.
//
// (C) Copyright 2020 Advantech Corporation, Inc
//

pub const ACPI_BUTTON_NOTIFY_SWBTN_RELEASE: c_uint = 0x86;
pub const ACPI_BUTTON_NOTIFY_SWBTN_PRESSED: c_uint = 0x85;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct adv_swbutton {
    pub input: *mut input_dev,
    pub phys: [c_char; 32],
}

// -------------------------------------------------------------------------
// Driver Interface
// --------------------------------------------------------------------------
//
#[no_mangle]
unsafe extern "C" fn adv_swbutton_notify(handle: acpi_handle, event: u32, context: *mut c_void) {
    static void adv_swbutton_notify(acpi_handle handle, u32 event, void *context)
    {
    struct platform_device *device = context;
    struct adv_swbutton *button = dev_get_drvdata(&device.dev);
    switch (event) {
    case ACPI_BUTTON_NOTIFY_SWBTN_RELEASE:
    input_report_key(button.input, KEY_PROG1, 0);
    input_sync(button.input);
    break;
    case ACPI_BUTTON_NOTIFY_SWBTN_PRESSED:
    input_report_key(button.input, KEY_PROG1, 1);
    input_sync(button.input);
    break;
    default:
    dev_dbg(&device.dev, "Unsupported event [0x%x]\n", event);
    }
    }
#[no_mangle]
unsafe extern "C" fn adv_swbutton_probe(device: *mut platform_device) -> c_int {
    static int adv_swbutton_probe(struct platform_device *device)
    {
    struct adv_swbutton *button;
    struct input_dev *input;
    acpi_handle handle;
    acpi_status status;
    int error;
    handle = ACPI_HANDLE(&device.dev);
    if (!handle)
    return -ENODEV;
    button = devm_kzalloc(&device.dev, sizeof(*button), GFP_KERNEL);
    if (!button)
    return -ENOMEM;
    dev_set_drvdata(&device.dev, button);
    input = devm_input_allocate_device(&device.dev);
    if (!input)
    return -ENOMEM;
    button.input = input;
    snprintf(button.phys, sizeof(button.phys), "%s/button/input0", ACPI_BUTTON_HID_SWBTN);
    input.name = "Advantech Software Button";
    input.phys = button.phys;
    input.id.bustype = BUS_HOST;
    input.dev.parent = &device.dev;
    set_bit(EV_REP, input.evbit);
    input_set_capability(input, EV_KEY, KEY_PROG1);
    error = input_register_device(input);
    if (error)
    return error;
    device_init_wakeup(&device.dev, true);
    status = acpi_install_notify_handler(handle,
    ACPI_DEVICE_NOTIFY,
    adv_swbutton_notify,
    device);
    if (ACPI_FAILURE(status)) {
    dev_err(&device.dev, "Error installing notify handler\n");
    return -EIO;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn adv_swbutton_remove(device: *mut platform_device) {
    static void adv_swbutton_remove(struct platform_device *device)
    {
    let mut handle: acpi_handle = ACPI_HANDLE(&device.dev);
    acpi_remove_notify_handler(handle, ACPI_DEVICE_NOTIFY,
    adv_swbutton_notify);
    }
    static const struct acpi_device_id button_device_ids[] = {
    {ACPI_BUTTON_HID_SWBTN, 0},
    {"", 0},
    };
    MODULE_DEVICE_TABLE(acpi, button_device_ids);
    static struct platform_driver adv_swbutton_driver = {
    .driver = {
    .name = "adv_swbutton",
    .acpi_match_table = button_device_ids,
    },
    .probe = adv_swbutton_probe,
    .remove = adv_swbutton_remove,
    };
    module_platform_driver(adv_swbutton_driver);
    MODULE_AUTHOR("Andrea Ho");
    MODULE_DESCRIPTION("Advantech ACPI SW Button Driver");
    MODULE_LICENSE("GPL v2");
