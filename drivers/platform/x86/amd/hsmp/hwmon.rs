//! Automatically rewritten from C to Rust
//! Source: drivers/platform/x86/amd/hsmp/hwmon.c
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
// AMD HSMP hwmon support
// Copyright (c) 2025, AMD.
// All Rights Reserved.
//
// This file provides hwmon implementation for HSMP interface.
//

    static int hsmp_hwmon_write(struct device *dev, enum hwmon_sensor_types type,
    u32 attr, int channel, long val)
    {
    let mut sock_ind: u16 = (uintptr_t)dev_get_drvdata(dev);
    let mut msg: hsmp_message = {};
    if (type != hwmon_power)
    return -EOPNOTSUPP;
    if (attr != hwmon_power_cap)
    return -EOPNOTSUPP;
    if (val < 0)
    return -EINVAL;
    msg.num_args = 1;
    msg.args[0] = val / MICROWATT_PER_MILLIWATT;
    msg.msg_id = HSMP_SET_SOCKET_POWER_LIMIT;
    msg.sock_ind = sock_ind;
    return hsmp_send_message(&msg);
    }
    static int hsmp_hwmon_read(struct device *dev,
    enum hwmon_sensor_types type,
    u32 attr, int channel, long *val)
    {
    let mut sock_ind: u16 = (uintptr_t)dev_get_drvdata(dev);
    let mut msg: hsmp_message = {};
    int ret;
    if (type != hwmon_power)
    return -EOPNOTSUPP;
    msg.sock_ind = sock_ind;
    msg.response_sz = 1;
    switch (attr) {
    case hwmon_power_input:
    msg.msg_id = HSMP_GET_SOCKET_POWER;
    break;
    case hwmon_power_cap:
    msg.msg_id = HSMP_GET_SOCKET_POWER_LIMIT;
    break;
    case hwmon_power_cap_max:
    msg.msg_id = HSMP_GET_SOCKET_POWER_LIMIT_MAX;
    break;
    default:
    return -EOPNOTSUPP;
    }
    ret = hsmp_send_message(&msg);
    if (!ret)
// val = msg.args[0] * MICROWATT_PER_MILLIWATT;
    return ret;
    }
    static umode_t hsmp_hwmon_is_visble(const void *data,
    enum hwmon_sensor_types type,
    u32 attr, int channel)
    {
    if (type != hwmon_power)
    return 0;
    switch (attr) {
    case hwmon_power_input:
    return 0444;
    case hwmon_power_cap:
    return 0644;
    case hwmon_power_cap_max:
    return 0444;
    default:
    return 0;
    }
    }
    static const struct hwmon_ops hsmp_hwmon_ops = {
    .read = hsmp_hwmon_read,
    .is_visible = hsmp_hwmon_is_visble,
    .write	= hsmp_hwmon_write,
    };
    static const struct hwmon_channel_info * const hsmp_info[] = {
    HWMON_CHANNEL_INFO(power, HWMON_P_INPUT | HWMON_P_CAP | HWMON_P_CAP_MAX),
    core::ptr::null_mut()
    };
    static const struct hwmon_chip_info hsmp_chip_info = {
    .ops = &hsmp_hwmon_ops,
    .info = hsmp_info,
    };
#[no_mangle]
pub unsafe extern "C" fn hsmp_create_sensor(dev: *mut device, sock_ind: u16) -> c_int {
    int hsmp_create_sensor(struct device *dev, u16 sock_ind)
    {
    struct device *hwmon_dev;
    hwmon_dev = devm_hwmon_device_register_with_info(dev, HSMP_HWMON_NAME,
    (void *)(uintptr_t)sock_ind,
    &hsmp_chip_info,
    core::ptr::null_mut());
    return PTR_ERR_OR_ZERO(hwmon_dev);
    }
    EXPORT_SYMBOL_NS(hsmp_create_sensor, "AMD_HSMP");
