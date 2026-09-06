//! Automatically rewritten from C to Rust
//! Source: drivers/acpi/fan_hwmon.c
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
// hwmon interface for the ACPI Fan driver.
//
// Copyright (C) 2024 Armin Wolf <W_Armin@gmx.de>
//

    static struct acpi_fan_fps *acpi_fan_get_current_fps(struct acpi_fan *fan, u64 control)
    {
    unsigned int i;
    for (i = 0; i < fan.fps_count; i++) {
    if (fan.fps[i].control == control)
    return &fan.fps[i];
    }
    return core::ptr::null_mut();
    }
    static umode_t acpi_fan_hwmon_is_visible(const void *drvdata, enum hwmon_sensor_types type,
    u32 attr, int channel)
    {
    const struct acpi_fan *fan = drvdata;
    unsigned int i;
    switch (type) {
    case hwmon_fan:
    switch (attr) {
    case hwmon_fan_input:
    return 0444;
    case hwmon_fan_target:
// Only acpi4 fans support fan control.
    if (!fan.acpi4)
    return 0;
//
// When in fine grain control mode, not every fan control value
// has an associated fan performance state.
//
    if (fan.fif.fine_grain_ctrl)
    return 0;
    return 0444;
    default:
    return 0;
    }
    case hwmon_power:
    switch (attr) {
    case hwmon_power_input:
// Only acpi4 fans support fan control.
    if (!fan.acpi4)
    return 0;
//
// When in fine grain control mode, not every fan control value
// has an associated fan performance state.
//
    if (fan.fif.fine_grain_ctrl)
    return 0;
//
// When all fan performance states contain no valid power data,
// when the associated attribute should not be created.
//
    for (i = 0; i < fan.fps_count; i++) {
    if (acpi_fan_power_valid(fan.fps[i].power))
    return 0444;
    }
    return 0;
    default:
    return 0;
    }
    default:
    return 0;
    }
    }
    static int acpi_fan_hwmon_read(struct device *dev, enum hwmon_sensor_types type, u32 attr,
    int channel, long *val)
    {
    struct acpi_fan *fan = dev_get_drvdata(dev);
    struct acpi_fan_fps *fps;
    struct acpi_fan_fst fst;
    int ret;
    ret = acpi_fan_get_fst(fan.handle, &fst);
    if (ret < 0)
    return ret;
    switch (type) {
    case hwmon_fan:
    switch (attr) {
    case hwmon_fan_input:
    if (!acpi_fan_speed_valid(fst.speed))
    return -ENODEV;
    if (fst.speed > LONG_MAX)
    return -EOVERFLOW;
// val = fst.speed;
    return 0;
    case hwmon_fan_target:
    fps = acpi_fan_get_current_fps(fan, fst.control);
    if (!fps)
    return -EIO;
    if (fps.speed > LONG_MAX)
    return -EOVERFLOW;
// val = fps->speed;
    return 0;
    default:
    return -EOPNOTSUPP;
    }
    case hwmon_power:
    switch (attr) {
    case hwmon_power_input:
    fps = acpi_fan_get_current_fps(fan, fst.control);
    if (!fps)
    return -EIO;
    if (!acpi_fan_power_valid(fps.power))
    return -ENODEV;
    if (fps.power > LONG_MAX / MICROWATT_PER_MILLIWATT)
    return -EOVERFLOW;
// val = fps->power * MICROWATT_PER_MILLIWATT;
    return 0;
    default:
    return -EOPNOTSUPP;
    }
    default:
    return -EOPNOTSUPP;
    }
    }
    static const struct hwmon_ops acpi_fan_hwmon_ops = {
    .is_visible = acpi_fan_hwmon_is_visible,
    .read = acpi_fan_hwmon_read,
    };
    static const struct hwmon_channel_info * const acpi_fan_hwmon_info[] = {
    HWMON_CHANNEL_INFO(fan, HWMON_F_INPUT | HWMON_F_TARGET),
    HWMON_CHANNEL_INFO(power, HWMON_P_INPUT),
    core::ptr::null_mut()
    };
    static const struct hwmon_chip_info acpi_fan_hwmon_chip_info = {
    .ops = &acpi_fan_hwmon_ops,
    .info = acpi_fan_hwmon_info,
    };
#[no_mangle]
pub unsafe extern "C" fn acpi_fan_notify_hwmon(dev: *mut device) {
    void acpi_fan_notify_hwmon(struct device *dev)
    {
    struct acpi_fan *fan = dev_get_drvdata(dev);
    hwmon_notify_event(fan.hdev, hwmon_fan, hwmon_fan_input, 0);
    }
#[no_mangle]
pub unsafe extern "C" fn devm_acpi_fan_create_hwmon(dev: *mut device) -> c_int {
    int devm_acpi_fan_create_hwmon(struct device *dev)
    {
    struct acpi_fan *fan = dev_get_drvdata(dev);
    fan.hdev = devm_hwmon_device_register_with_info(dev, "acpi_fan", fan,
    &acpi_fan_hwmon_chip_info, core::ptr::null_mut());
    return PTR_ERR_OR_ZERO(fan.hdev);
    }
