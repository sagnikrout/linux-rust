//! Automatically rewritten from C to Rust
//! Source: drivers/platform/x86/lenovo/wmi-camera.c
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
// Lenovo WMI Camera Button Driver
//
// Author: Ai Chao <aichao@kylinos.cn>
// Copyright (C) 2024 KylinSoft Corporation.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lenovo_wmi_priv {
    pub idev: *mut input_dev,
    pub /: *mut *mut mutex notify_lock; / lenovo WMI camera button notify lock,
}

    enum {
    SW_CAMERA_OFF	= 0,
    SW_CAMERA_ON	= 1,
    };
#[no_mangle]
unsafe extern "C" fn camera_shutter_input_setup(wdev: *mut wmi_device, camera_mode: u8) -> c_int {
    static int camera_shutter_input_setup(struct wmi_device *wdev, u8 camera_mode)
    {
    struct lenovo_wmi_priv *priv = dev_get_drvdata(&wdev.dev);
    int err;
    priv.idev = input_allocate_device();
    if (!priv.idev)
    return -ENOMEM;
    priv.idev.name = "Lenovo WMI Camera Button";
    priv.idev.phys = "wmi/input0";
    priv.idev.id.bustype = BUS_HOST;
    priv.idev.dev.parent = &wdev.dev;
    input_set_capability(priv.idev, EV_SW, SW_CAMERA_LENS_COVER);
    input_report_switch(priv.idev, SW_CAMERA_LENS_COVER,
    camera_mode == SW_CAMERA_ON ? 0 : 1);
    input_sync(priv.idev);
    err = input_register_device(priv.idev);
    if (err) {
    input_free_device(priv.idev);
    priv.idev = core::ptr::null_mut();
    }
    return err;
    }
#[no_mangle]
unsafe extern "C" fn lenovo_wmi_notify(wdev: *mut wmi_device, obj: *mut union acpi_object) {
    static void lenovo_wmi_notify(struct wmi_device *wdev, union acpi_object *obj)
    {
    struct lenovo_wmi_priv *priv = dev_get_drvdata(&wdev.dev);
    u8 camera_mode;
    if (obj.type != ACPI_TYPE_BUFFER) {
    dev_err(&wdev.dev, "Bad response type %u\n", obj.type);
    return;
    }
    if (obj.buffer.length != 1) {
    dev_err(&wdev.dev, "Invalid buffer length %u\n", obj.buffer.length);
    return;
    }
//
// obj->buffer.pointer[0] is camera mode:
// 0 camera close
// 1 camera open
//
    camera_mode = obj.buffer.pointer[0];
    if (camera_mode > SW_CAMERA_ON) {
    dev_err(&wdev.dev, "Unknown camera mode %u\n", camera_mode);
    return;
    }
    guard(mutex)(&priv.notify_lock);
    if (!priv.idev) {
    if (camera_shutter_input_setup(wdev, camera_mode))
    dev_warn(&wdev.dev, "Failed to register input device\n");
    return;
    }
    if (camera_mode == SW_CAMERA_ON)
    input_report_switch(priv.idev, SW_CAMERA_LENS_COVER, 0);
    else
    input_report_switch(priv.idev, SW_CAMERA_LENS_COVER, 1);
    input_sync(priv.idev);
    }
#[no_mangle]
unsafe extern "C" fn lenovo_wmi_probe(wdev: *mut wmi_device, context: *const c_void) -> c_int {
    static int lenovo_wmi_probe(struct wmi_device *wdev, const void *context)
    {
    struct lenovo_wmi_priv *priv;
    priv = devm_kzalloc(&wdev.dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    dev_set_drvdata(&wdev.dev, priv);
    mutex_init(&priv.notify_lock);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lenovo_wmi_remove(wdev: *mut wmi_device) {
    static void lenovo_wmi_remove(struct wmi_device *wdev)
    {
    struct lenovo_wmi_priv *priv = dev_get_drvdata(&wdev.dev);
    if (priv.idev)
    input_unregister_device(priv.idev);
    mutex_destroy(&priv.notify_lock);
    }
    static const struct wmi_device_id lenovo_wmi_id_table[] = {
    { .guid_string = WMI_LENOVO_CAMERABUTTON_EVENT_GUID },
    {  }
    };
    MODULE_DEVICE_TABLE(wmi, lenovo_wmi_id_table);
    static struct wmi_driver lenovo_wmi_driver = {
    .driver = {
    .name = "lenovo-wmi-camera",
    .probe_type = PROBE_PREFER_ASYNCHRONOUS,
    },
    .id_table = lenovo_wmi_id_table,
    .min_event_size = sizeof(u8),
    .no_singleton = true,
    .probe = lenovo_wmi_probe,
    .notify = lenovo_wmi_notify,
    .remove = lenovo_wmi_remove,
    };
    module_wmi_driver(lenovo_wmi_driver);
    MODULE_AUTHOR("Ai Chao <aichao@kylinos.cn>");
    MODULE_DESCRIPTION("Lenovo WMI Camera Button Driver");
    MODULE_LICENSE("GPL");
