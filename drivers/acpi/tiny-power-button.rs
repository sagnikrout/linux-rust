//! Automatically rewritten from C to Rust
//! Source: drivers/acpi/tiny-power-button.c
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

    MODULE_AUTHOR("Josh Triplett");
    MODULE_DESCRIPTION("ACPI Tiny Power Button Driver");
    MODULE_LICENSE("GPL");
    let mut __read_mostly: static int power_signal = CONFIG_ACPI_TINY_POWER_BUTTON_SIGNAL;
    module_param(power_signal, int, 0644);
    MODULE_PARM_DESC(power_signal, "Power button sends this signal to init");
    static const struct acpi_device_id tiny_power_button_device_ids[] = {
    { ACPI_BUTTON_HID_POWER, 0 },
    { ACPI_BUTTON_HID_POWERF, 0 },
    { "", 0 },
    };
    MODULE_DEVICE_TABLE(acpi, tiny_power_button_device_ids);
#[no_mangle]
unsafe extern "C" fn acpi_tiny_power_button_notify(handle: acpi_handle, event: u32, data: *mut c_void) {
    static void acpi_tiny_power_button_notify(acpi_handle handle, u32 event, void *data)
    {
    kill_cad_pid(power_signal, 1);
    }
#[no_mangle]
unsafe extern "C" fn acpi_tiny_power_button_notify_run(not_used: *mut c_void) {
    static void acpi_tiny_power_button_notify_run(void *not_used)
    {
    acpi_tiny_power_button_notify(core::ptr::null_mut(), ACPI_FIXED_HARDWARE_EVENT, core::ptr::null_mut());
    }
#[no_mangle]
unsafe extern "C" fn acpi_tiny_power_button_event(not_used: *mut c_void) -> u32 {
    static u32 acpi_tiny_power_button_event(void *not_used)
    {
    acpi_os_execute(OSL_NOTIFY_HANDLER, acpi_tiny_power_button_notify_run, core::ptr::null_mut());
    return ACPI_INTERRUPT_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn acpi_tiny_power_button_probe(pdev: *mut platform_device) -> c_int {
    static int acpi_tiny_power_button_probe(struct platform_device *pdev)
    {
    struct acpi_device *device;
    acpi_status status;
    device = ACPI_COMPANION(&pdev.dev);
    if (!device)
    return -ENODEV;
    if (device.device_type == ACPI_BUS_TYPE_POWER_BUTTON) {
    status = acpi_install_fixed_event_handler(ACPI_EVENT_POWER_BUTTON,
    acpi_tiny_power_button_event,
    core::ptr::null_mut());
    } else {
    status = acpi_install_notify_handler(device.handle,
    ACPI_DEVICE_NOTIFY,
    acpi_tiny_power_button_notify,
    core::ptr::null_mut());
    }
    if (ACPI_FAILURE(status))
    return -ENODEV;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn acpi_tiny_power_button_remove(pdev: *mut platform_device) {
    static void acpi_tiny_power_button_remove(struct platform_device *pdev)
    {
    struct acpi_device *device = ACPI_COMPANION(&pdev.dev);
    if (device.device_type == ACPI_BUS_TYPE_POWER_BUTTON) {
    acpi_remove_fixed_event_handler(ACPI_EVENT_POWER_BUTTON,
    acpi_tiny_power_button_event);
    } else {
    acpi_remove_notify_handler(device.handle, ACPI_DEVICE_NOTIFY,
    acpi_tiny_power_button_notify);
    }
    acpi_os_wait_events_complete();
    }
    static struct platform_driver acpi_tiny_power_button_driver = {
    .probe = acpi_tiny_power_button_probe,
    .remove = acpi_tiny_power_button_remove,
    .driver = {
    .name = "acpi-tiny-power-button",
    .acpi_match_table = tiny_power_button_device_ids,
    },
    };
    module_platform_driver(acpi_tiny_power_button_driver);
