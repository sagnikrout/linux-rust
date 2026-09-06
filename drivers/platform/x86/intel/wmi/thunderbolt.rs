//! Automatically rewritten from C to Rust
//! Source: drivers/platform/x86/intel/wmi/thunderbolt.c
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
// WMI Thunderbolt driver
//
// Copyright (C) 2017 Dell Inc. All Rights Reserved.
//

    static ssize_t force_power_store(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t count)
    {
    struct wmi_buffer buffer;
    int ret;
    u8 mode;
    buffer.length = sizeof(mode);
    buffer.data = &mode;
    mode = hex_to_bin(buf[0]);
    if (mode > 1)
    return -EINVAL;
    ret = wmidev_invoke_procedure(to_wmi_device(dev), 0, 1, &buffer);
    if (ret < 0)
    return ret;
    return count;
    }
    static DEVICE_ATTR_WO(force_power);
    static struct attribute *tbt_attrs[] = {
    &dev_attr_force_power.attr,
    core::ptr::null_mut()
    };
    ATTRIBUTE_GROUPS(tbt);
    static const struct wmi_device_id intel_wmi_thunderbolt_id_table[] = {
    { .guid_string = INTEL_WMI_THUNDERBOLT_GUID },
    { },
    };
    static struct wmi_driver intel_wmi_thunderbolt_driver = {
    .driver = {
    .name = "intel-wmi-thunderbolt",
    .dev_groups = tbt_groups,
    },
    .id_table = intel_wmi_thunderbolt_id_table,
    .no_singleton = true,
    };
    module_wmi_driver(intel_wmi_thunderbolt_driver);
    MODULE_DEVICE_TABLE(wmi, intel_wmi_thunderbolt_id_table);
    MODULE_AUTHOR("Mario Limonciello <mario.limonciello@dell.com>");
    MODULE_DESCRIPTION("Intel WMI Thunderbolt force power driver");
    MODULE_LICENSE("GPL v2");
