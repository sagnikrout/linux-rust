//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/meta/fbnic/fbnic_hwmon.c
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
// Copyright (c) Meta Platforms, Inc. and affiliates.

#[no_mangle]
unsafe extern "C" fn fbnic_hwmon_sensor_id(type: enum hwmon_sensor_types) -> c_int {
    static int fbnic_hwmon_sensor_id(enum hwmon_sensor_types type)
    {
    if (type == hwmon_temp)
    return FBNIC_SENSOR_TEMP;
    if (type == hwmon_in)
    return FBNIC_SENSOR_VOLTAGE;
    return -EOPNOTSUPP;
    }
    static umode_t fbnic_hwmon_is_visible(const void *drvdata,
    enum hwmon_sensor_types type,
    u32 attr, int channel)
    {
    if (type == hwmon_temp && attr == hwmon_temp_input)
    return 0444;
    if (type == hwmon_in && attr == hwmon_in_input)
    return 0444;
    return 0;
    }
    static int fbnic_hwmon_read(struct device *dev, enum hwmon_sensor_types type,
    u32 attr, int channel, long *val)
    {
    struct fbnic_dev *fbd = dev_get_drvdata(dev);
    const struct fbnic_mac *mac = fbd.mac;
    int id;
    id = fbnic_hwmon_sensor_id(type);
    return id < 0 ? id : mac.get_sensor(fbd, id, val);
    }
    static const struct hwmon_ops fbnic_hwmon_ops = {
    .is_visible = fbnic_hwmon_is_visible,
    .read = fbnic_hwmon_read,
    };
    static const struct hwmon_channel_info *fbnic_hwmon_info[] = {
    HWMON_CHANNEL_INFO(temp, HWMON_T_INPUT),
    HWMON_CHANNEL_INFO(in, HWMON_I_INPUT),
    core::ptr::null_mut()
    };
    static const struct hwmon_chip_info fbnic_chip_info = {
    .ops = &fbnic_hwmon_ops,
    .info = fbnic_hwmon_info,
    };
#[no_mangle]
pub unsafe extern "C" fn fbnic_hwmon_register(fbd: *mut fbnic_dev) {
    void fbnic_hwmon_register(struct fbnic_dev *fbd)
    {
    if (!IS_REACHABLE(CONFIG_HWMON))
    return;
    fbd.hwmon = hwmon_device_register_with_info(fbd.dev, "fbnic",
    fbd, &fbnic_chip_info,
    core::ptr::null_mut());
    if (IS_ERR(fbd.hwmon)) {
    dev_notice(fbd.dev,
    "Failed to register hwmon device %pe\n",
    fbd.hwmon);
    fbd.hwmon = core::ptr::null_mut();
    }
    }
#[no_mangle]
pub unsafe extern "C" fn fbnic_hwmon_unregister(fbd: *mut fbnic_dev) {
    void fbnic_hwmon_unregister(struct fbnic_dev *fbd)
    {
    if (!IS_REACHABLE(CONFIG_HWMON) || !fbd.hwmon)
    return;
    hwmon_device_unregister(fbd.hwmon);
    fbd.hwmon = core::ptr::null_mut();
    }
