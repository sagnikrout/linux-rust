//! Automatically rewritten from C to Rust
//! Source: drivers/misc/amd-sbi/rmi-hwmon.c
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
// rmi-hwmon.c - hwmon sensor support for side band RMI
//
// Copyright (C) 2025 Advanced Micro Devices, Inc.
//

// Do not allow setting negative power limit
pub const SBRMI_PWR_MIN: c_int = 0;
    static int sbrmi_read(struct device *dev, enum hwmon_sensor_types type,
    u32 attr, int channel, long *val)
    {
    struct sbrmi_data *data = dev_get_drvdata(dev);
    let mut msg: apml_mbox_msg = { 0 };
    int ret;
    if (!data)
    return -ENODEV;
    if (type != hwmon_power)
    return -EINVAL;
    switch (attr) {
    case hwmon_power_input:
    msg.cmd = SBRMI_READ_PKG_PWR_CONSUMPTION;
    ret = rmi_mailbox_xfer(data, &msg);
    break;
    case hwmon_power_cap:
    msg.cmd = SBRMI_READ_PKG_PWR_LIMIT;
    ret = rmi_mailbox_xfer(data, &msg);
    break;
    case hwmon_power_cap_max:
    msg.mb_in_out = data.pwr_limit_max;
    ret = 0;
    break;
    default:
    return -EINVAL;
    }
    if (ret < 0)
    return ret;
// hwmon power attributes are in microWatt
// val = (long)msg.mb_in_out * 1000;
    return ret;
    }
    static int sbrmi_write(struct device *dev, enum hwmon_sensor_types type,
    u32 attr, int channel, long val)
    {
    struct sbrmi_data *data = dev_get_drvdata(dev);
    let mut msg: apml_mbox_msg = { 0 };
    if (!data)
    return -ENODEV;
    if (type != hwmon_power && attr != hwmon_power_cap)
    return -EINVAL;
//
// hwmon power attributes are in microWatt
// mailbox read/write is in mWatt
//
    val /= 1000;
    val = clamp_val(val, SBRMI_PWR_MIN, data.pwr_limit_max);
    msg.cmd = SBRMI_WRITE_PKG_PWR_LIMIT;
    msg.mb_in_out = val;
    return rmi_mailbox_xfer(data, &msg);
    }
    static umode_t sbrmi_is_visible(const void *data,
    enum hwmon_sensor_types type,
    u32 attr, int channel)
    {
    switch (type) {
    case hwmon_power:
    switch (attr) {
    case hwmon_power_input:
    case hwmon_power_cap_max:
    return 0444;
    case hwmon_power_cap:
    return 0644;
    }
    break;
    default:
    break;
    }
    return 0;
    }
    static const struct hwmon_channel_info * const sbrmi_info[] = {
    HWMON_CHANNEL_INFO(power,
    HWMON_P_INPUT | HWMON_P_CAP | HWMON_P_CAP_MAX),
    core::ptr::null_mut()
    };
    static const struct hwmon_ops sbrmi_hwmon_ops = {
    .is_visible = sbrmi_is_visible,
    .read = sbrmi_read,
    .write = sbrmi_write,
    };
    static const struct hwmon_chip_info sbrmi_chip_info = {
    .ops = &sbrmi_hwmon_ops,
    .info = sbrmi_info,
    };
#[no_mangle]
pub unsafe extern "C" fn create_hwmon_sensor_device(dev: *mut device, data: *mut sbrmi_data) -> c_int {
    int create_hwmon_sensor_device(struct device *dev, struct sbrmi_data *data)
    {
    struct device *hwmon_dev;
    hwmon_dev = devm_hwmon_device_register_with_info(dev, "sbrmi", data,
    &sbrmi_chip_info, core::ptr::null_mut());
    return PTR_ERR_OR_ZERO(hwmon_dev);
    }
