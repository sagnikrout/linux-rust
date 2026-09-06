//! Automatically rewritten from C to Rust
//! Source: drivers/platform/x86/xiaomi-wmi.c
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
// WMI driver for Xiaomi Laptops

    .guid_string = (guid),			\
    .context = &(const unsigned int){key}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xiaomi_wmi {
    pub input_dev: *mut input_dev,
    pub /: *mut *mut mutex key_lock; / Protects the key event sequence,
    pub key_code: c_uint,
}

#[no_mangle]
unsafe extern "C" fn xiaomi_wmi_probe(wdev: *mut wmi_device, context: *const c_void) -> c_int {
    static int xiaomi_wmi_probe(struct wmi_device *wdev, const void *context)
    {
    struct xiaomi_wmi *data;
    int ret;
    if (!context)
    return -EINVAL;
    data = devm_kzalloc(&wdev.dev, sizeof(struct xiaomi_wmi), GFP_KERNEL);
    if (data == core::ptr::null_mut())
    return -ENOMEM;
    dev_set_drvdata(&wdev.dev, data);
    ret = devm_mutex_init(&wdev.dev, &data.key_lock);
    if (ret < 0)
    return ret;
    data.input_dev = devm_input_allocate_device(&wdev.dev);
    if (data.input_dev == core::ptr::null_mut())
    return -ENOMEM;
    data.input_dev.name = "Xiaomi WMI keys";
    data.input_dev.phys = "wmi/input0";
    data.key_code = *((const unsigned int *)context);
    set_bit(EV_KEY, data.input_dev.evbit);
    set_bit(data.key_code, data.input_dev.keybit);
    return input_register_device(data.input_dev);
    }
#[no_mangle]
unsafe extern "C" fn xiaomi_wmi_notify(wdev: *mut wmi_device, dummy: *const wmi_buffer) {
    static void xiaomi_wmi_notify(struct wmi_device *wdev, const struct wmi_buffer *dummy)
    {
    struct xiaomi_wmi *data = dev_get_drvdata(&wdev.dev);
    mutex_lock(&data.key_lock);
    input_report_key(data.input_dev, data.key_code, 1);
    input_sync(data.input_dev);
    input_report_key(data.input_dev, data.key_code, 0);
    input_sync(data.input_dev);
    mutex_unlock(&data.key_lock);
    }
    static const struct wmi_device_id xiaomi_wmi_id_table[] = {
// { XIAOMI_DEVICE(XIAOMI_KEY_FN_ESC_0, KEY_FN_ESC) },
// { XIAOMI_DEVICE(XIAOMI_KEY_FN_ESC_1, KEY_FN_ESC) },
    { XIAOMI_DEVICE(XIAOMI_KEY_FN_FN, KEY_PROG1) },
// { XIAOMI_DEVICE(XIAOMI_KEY_CAPSLOCK, KEY_CAPSLOCK) },
    { XIAOMI_DEVICE(XIAOMI_KEY_FN_F7, KEY_CUT) },
// Terminating entry
    { }
    };
    static struct wmi_driver xiaomi_wmi_driver = {
    .driver = {
    .name = "xiaomi-wmi",
    },
    .id_table = xiaomi_wmi_id_table,
    .min_event_size = 0,
    .probe = xiaomi_wmi_probe,
    .notify_new = xiaomi_wmi_notify,
    .no_singleton = true,
    };
    module_wmi_driver(xiaomi_wmi_driver);
    MODULE_DEVICE_TABLE(wmi, xiaomi_wmi_id_table);
    MODULE_AUTHOR("Mattias Jacobsson");
    MODULE_DESCRIPTION("Xiaomi WMI driver");
    MODULE_LICENSE("GPL v2");
