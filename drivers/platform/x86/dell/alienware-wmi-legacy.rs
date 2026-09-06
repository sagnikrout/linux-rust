//! Automatically rewritten from C to Rust
//! Source: drivers/platform/x86/dell/alienware-wmi-legacy.c
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
// Alienware LEGACY WMI device driver
//
// Copyright (C) 2025 Kurt Borja <kuurtb@gmail.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct legacy_led_args {
    pub colors: color_platform,
    pub brightness: u8,
    pub state: u8,
    pub __packed: },
//
// Legacy WMI driver
//
    static int legacy_wmi_update_led(struct alienfx_priv *priv,
    struct wmi_device *wdev, u8 location)
    {
    struct legacy_led_args legacy_args = {
    .colors = priv.colors[location],
    .brightness = priv.global_brightness,
    .state = 0,
}

    struct acpi_buffer input;
    acpi_status status;
    if (legacy_args.state != LEGACY_RUNNING) {
    legacy_args.state = priv.lighting_control_state;
    input.length = sizeof(legacy_args);
    input.pointer = &legacy_args;
    status = wmi_evaluate_method(LEGACY_POWER_CONTROL_GUID, 0,
    location + 1, &input, core::ptr::null_mut());
    if (ACPI_FAILURE(status))
    return -EIO;
    return 0;
    }
    return alienware_wmi_command(wdev, location + 1, &legacy_args,
    sizeof(legacy_args), core::ptr::null_mut());
    }
    static int legacy_wmi_update_brightness(struct alienfx_priv *priv,
    struct wmi_device *wdev, u8 brightness)
    {
    return legacy_wmi_update_led(priv, wdev, 0);
    }
#[no_mangle]
unsafe extern "C" fn legacy_wmi_probe(wdev: *mut wmi_device, context: *const c_void) -> c_int {
    static int legacy_wmi_probe(struct wmi_device *wdev, const void *context)
    {
    struct alienfx_platdata pdata = {
    .wdev = wdev,
    .ops = {
    .upd_led = legacy_wmi_update_led,
    .upd_brightness = legacy_wmi_update_brightness,
    },
    };
    return alienware_alienfx_setup(&pdata);
    }
    static const struct wmi_device_id alienware_legacy_device_id_table[] = {
    { LEGACY_CONTROL_GUID, core::ptr::null_mut() },
    { },
    };
    MODULE_DEVICE_TABLE(wmi, alienware_legacy_device_id_table);
    static struct wmi_driver alienware_legacy_wmi_driver = {
    .driver = {
    .name = "alienware-wmi-alienfx",
    .probe_type = PROBE_PREFER_ASYNCHRONOUS,
    },
    .id_table = alienware_legacy_device_id_table,
    .probe = legacy_wmi_probe,
    .no_singleton = true,
    };
#[no_mangle]
pub unsafe extern "C" fn alienware_legacy_wmi_init() -> int __init {
    int __init alienware_legacy_wmi_init(void)
    {
    return wmi_driver_register(&alienware_legacy_wmi_driver);
    }
#[no_mangle]
pub unsafe extern "C" fn alienware_legacy_wmi_exit() -> void __exit {
    void __exit alienware_legacy_wmi_exit(void)
    {
    wmi_driver_unregister(&alienware_legacy_wmi_driver);
    }
