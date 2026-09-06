//! Automatically rewritten from C to Rust
//! Source: drivers/hwmon/sy7636a-hwmon.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// Functions to access SY3686A power management chip temperature
//
// Copyright (C) 2021 reMarkable AS - http://www.remarkable.com
//
// Authors: Lars Ivar Miljeteig <lars.ivar.miljeteig@remarkable.com>
// Alistair Francis <alistair@alistair23.me>
//

    static int sy7636a_read(struct device *dev, enum hwmon_sensor_types type,
    u32 attr, int channel, long *temp)
    {
    struct regmap *regmap = dev_get_drvdata(dev);
    int ret, reg_val;
    ret = regmap_read(regmap,
    SY7636A_REG_TERMISTOR_READOUT, &reg_val);
    if (ret)
    return ret;
// temp = reg_val * 1000;
    return 0;
    }
    static umode_t sy7636a_is_visible(const void *data,
    enum hwmon_sensor_types type,
    u32 attr, int channel)
    {
    if (type != hwmon_temp)
    return 0;
    if (attr != hwmon_temp_input)
    return 0;
    return 0444;
    }
    static const struct hwmon_ops sy7636a_hwmon_ops = {
    .is_visible = sy7636a_is_visible,
    .read = sy7636a_read,
    };
    static const struct hwmon_channel_info * const sy7636a_info[] = {
    HWMON_CHANNEL_INFO(chip, HWMON_C_REGISTER_TZ),
    HWMON_CHANNEL_INFO(temp, HWMON_T_INPUT),
    core::ptr::null_mut()
    };
    static const struct hwmon_chip_info sy7636a_chip_info = {
    .ops = &sy7636a_hwmon_ops,
    .info = sy7636a_info,
    };
#[no_mangle]
unsafe extern "C" fn sy7636a_sensor_probe(pdev: *mut platform_device) -> c_int {
    static int sy7636a_sensor_probe(struct platform_device *pdev)
    {
    struct regmap *regmap = dev_get_regmap(pdev.dev.parent, core::ptr::null_mut());
    struct device *hwmon_dev;
    int err;
    if (!regmap)
    return -EPROBE_DEFER;
    err = devm_regulator_get_enable(&pdev.dev, "vcom");
    if (err)
    return err;
    hwmon_dev = devm_hwmon_device_register_with_info(&pdev.dev,
    "sy7636a_temperature", regmap,
    &sy7636a_chip_info, core::ptr::null_mut());
    if (IS_ERR(hwmon_dev)) {
    err = PTR_ERR(hwmon_dev);
    dev_err(&pdev.dev, "Unable to register hwmon device, returned %d\n", err);
    return err;
    }
    return 0;
    }
    static struct platform_driver sy7636a_sensor_driver = {
    .probe = sy7636a_sensor_probe,
    .driver = {
    .name = "sy7636a-temperature",
    },
    };
    module_platform_driver(sy7636a_sensor_driver);
    MODULE_DESCRIPTION("SY7636A sensor driver");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:sy7636a-temperature");
