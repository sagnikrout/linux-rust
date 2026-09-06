//! Automatically rewritten from C to Rust
//! Source: drivers/platform/x86/gigabyte-wmi.c
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
// Copyright (C) 2021 Thomas Weißschuh <linux@weissschuh.net>
//

pub const NUM_TEMPERATURE_SENSORS: c_int = 6;
    static u8 usable_sensors_mask;
    enum gigabyte_wmi_commandtype {
    GIGABYTE_WMI_BUILD_DATE_QUERY       =   0x1,
    GIGABYTE_WMI_MAINBOARD_TYPE_QUERY   =   0x2,
    GIGABYTE_WMI_FIRMWARE_VERSION_QUERY =   0x4,
    GIGABYTE_WMI_MAINBOARD_NAME_QUERY   =   0x5,
    GIGABYTE_WMI_TEMPERATURE_QUERY      = 0x125,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gigabyte_wmi_args {
    pub arg1: u32,
}

    static int gigabyte_wmi_perform_query(struct wmi_device *wdev,
    enum gigabyte_wmi_commandtype command,
    struct gigabyte_wmi_args *args, struct acpi_buffer *out)
    {
    const struct acpi_buffer in = {
    .length = sizeof(*args),
    .pointer = args,
    };
    let mut ret: acpi_status = wmidev_evaluate_method(wdev, 0x0, command, &in, out);
    if (ACPI_FAILURE(ret))
    return -EIO;
    return 0;
    }
    static int gigabyte_wmi_query_integer(struct wmi_device *wdev,
    enum gigabyte_wmi_commandtype command,
    struct gigabyte_wmi_args *args, u64 *res)
    {
    union acpi_object *obj;
    let mut result: acpi_buffer = { ACPI_ALLOCATE_BUFFER, core::ptr::null_mut() };
    int ret;
    ret = gigabyte_wmi_perform_query(wdev, command, args, &result);
    if (ret)
    return ret;
    obj = result.pointer;
    if (obj && obj.type == ACPI_TYPE_INTEGER)
// res = obj->integer.value;
    else
    ret = -EIO;
    kfree(result.pointer);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn gigabyte_wmi_temperature(wdev: *mut wmi_device, sensor: u8, res: *mut c_long) -> c_int {
    static int gigabyte_wmi_temperature(struct wmi_device *wdev, u8 sensor, long *res)
    {
    struct gigabyte_wmi_args args = {
    .arg1 = sensor,
    };
    u64 temp;
    acpi_status ret;
    ret = gigabyte_wmi_query_integer(wdev, GIGABYTE_WMI_TEMPERATURE_QUERY, &args, &temp);
    if (ret == 0) {
    if (temp == 0)
    return -ENODEV;
// res = (s8)temp * 1000; // value is a signed 8-bit integer
    }
    return ret;
    }
    static int gigabyte_wmi_hwmon_read(struct device *dev, enum hwmon_sensor_types type,
    u32 attr, int channel, long *val)
    {
    struct wmi_device *wdev = dev_get_drvdata(dev);
    return gigabyte_wmi_temperature(wdev, channel, val);
    }
    static umode_t gigabyte_wmi_hwmon_is_visible(const void *data, enum hwmon_sensor_types type,
    u32 attr, int channel)
    {
    return usable_sensors_mask & BIT(channel) ? 0444  : 0;
    }
    static const struct hwmon_channel_info * const gigabyte_wmi_hwmon_info[] = {
    HWMON_CHANNEL_INFO(temp,
    HWMON_T_INPUT,
    HWMON_T_INPUT,
    HWMON_T_INPUT,
    HWMON_T_INPUT,
    HWMON_T_INPUT,
    HWMON_T_INPUT),
    core::ptr::null_mut()
    };
    static const struct hwmon_ops gigabyte_wmi_hwmon_ops = {
    .read = gigabyte_wmi_hwmon_read,
    .is_visible = gigabyte_wmi_hwmon_is_visible,
    };
    static const struct hwmon_chip_info gigabyte_wmi_hwmon_chip_info = {
    .ops = &gigabyte_wmi_hwmon_ops,
    .info = gigabyte_wmi_hwmon_info,
    };
#[no_mangle]
unsafe extern "C" fn gigabyte_wmi_detect_sensor_usability(wdev: *mut wmi_device) -> u8 {
    static u8 gigabyte_wmi_detect_sensor_usability(struct wmi_device *wdev)
    {
    int i;
    long temp;
    let mut r: u8 = 0;
    for (i = 0; i < NUM_TEMPERATURE_SENSORS; i++) {
    if (!gigabyte_wmi_temperature(wdev, i, &temp))
    r |= BIT(i);
    }
    return r;
    }
#[no_mangle]
unsafe extern "C" fn gigabyte_wmi_probe(wdev: *mut wmi_device, context: *const c_void) -> c_int {
    static int gigabyte_wmi_probe(struct wmi_device *wdev, const void *context)
    {
    struct device *hwmon_dev;
    usable_sensors_mask = gigabyte_wmi_detect_sensor_usability(wdev);
    if (!usable_sensors_mask) {
    dev_info(&wdev.dev, "No temperature sensors usable");
    return -ENODEV;
    }
    hwmon_dev = devm_hwmon_device_register_with_info(&wdev.dev, "gigabyte_wmi", wdev,
    &gigabyte_wmi_hwmon_chip_info, core::ptr::null_mut());
    return PTR_ERR_OR_ZERO(hwmon_dev);
    }
    static const struct wmi_device_id gigabyte_wmi_id_table[] = {
    { GIGABYTE_WMI_GUID, core::ptr::null_mut() },
    { }
    };
    static struct wmi_driver gigabyte_wmi_driver = {
    .driver = {
    .name = "gigabyte-wmi",
    },
    .id_table = gigabyte_wmi_id_table,
    .probe = gigabyte_wmi_probe,
    };
    module_wmi_driver(gigabyte_wmi_driver);
    MODULE_DEVICE_TABLE(wmi, gigabyte_wmi_id_table);
    MODULE_AUTHOR("Thomas Weißschuh <linux@weissschuh.net>");
    MODULE_DESCRIPTION("Gigabyte WMI temperature driver");
    MODULE_LICENSE("GPL");
