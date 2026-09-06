//! Automatically rewritten from C to Rust
//! Source: drivers/platform/x86/wmi-bmof.c
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
// WMI embedded Binary MOF driver
//
// Copyright (c) 2015 Andrew Lutomirski
// Copyright (C) 2017 VMware, Inc. All Rights Reserved.
//

    static ssize_t bmof_read(struct file *filp, struct kobject *kobj, const struct bin_attribute *attr,
    char *buf, loff_t off, size_t count)
    {
    struct device *dev = kobj_to_dev(kobj);
    struct wmi_buffer *buffer = dev_get_drvdata(dev);
    return memory_read_from_buffer(buf, count, &off, buffer.data, buffer.length);
    }
    static const BIN_ATTR_ADMIN_RO(bmof, 0);
    static const struct bin_attribute * const bmof_attrs[] = {
    &bin_attr_bmof,
    core::ptr::null_mut()
    };
#[no_mangle]
unsafe extern "C" fn bmof_bin_size(kobj: *mut kobject, attr: *const bin_attribute, n: c_int) -> usize {
    static size_t bmof_bin_size(struct kobject *kobj, const struct bin_attribute *attr, int n)
    {
    struct device *dev = kobj_to_dev(kobj);
    struct wmi_buffer *buffer = dev_get_drvdata(dev);
    return buffer.length;
    }
    static const struct attribute_group bmof_group = {
    .bin_size = bmof_bin_size,
    .bin_attrs = bmof_attrs,
    };
    static const struct attribute_group *bmof_groups[] = {
    &bmof_group,
    core::ptr::null_mut()
    };
#[no_mangle]
unsafe extern "C" fn wmi_bmof_probe(wdev: *mut wmi_device, context: *const c_void) -> c_int {
    static int wmi_bmof_probe(struct wmi_device *wdev, const void *context)
    {
    struct wmi_buffer *buffer;
    int ret;
    buffer = devm_kzalloc(&wdev.dev, sizeof(*buffer), GFP_KERNEL);
    if (!buffer)
    return -ENOMEM;
    ret = wmidev_query_block(wdev, 0, buffer, 0);
    if (ret < 0)
    return ret;
    dev_set_drvdata(&wdev.dev, buffer);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn wmi_bmof_remove(wdev: *mut wmi_device) {
    static void wmi_bmof_remove(struct wmi_device *wdev)
    {
    struct wmi_buffer *buffer = dev_get_drvdata(&wdev.dev);
    kfree(buffer.data);
    }
    static const struct wmi_device_id wmi_bmof_id_table[] = {
    { .guid_string = WMI_BMOF_GUID },
    { },
    };
    static struct wmi_driver wmi_bmof_driver = {
    .driver = {
    .name = "wmi-bmof",
    .dev_groups = bmof_groups,
    },
    .probe = wmi_bmof_probe,
    .remove = wmi_bmof_remove,
    .id_table = wmi_bmof_id_table,
    .no_singleton = true,
    };
    module_wmi_driver(wmi_bmof_driver);
    MODULE_DEVICE_TABLE(wmi, wmi_bmof_id_table);
    MODULE_AUTHOR("Andrew Lutomirski <luto@kernel.org>");
    MODULE_DESCRIPTION("WMI embedded Binary MOF driver");
    MODULE_LICENSE("GPL");
