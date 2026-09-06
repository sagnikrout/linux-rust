//! Automatically rewritten from C to Rust
//! Source: drivers/iio/imu/st_lsm9ds0/st_lsm9ds0_core.c
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
// STMicroelectronics LSM9DS0 IMU driver
//
// Copyright (C) 2021, Intel Corporation
//
// Author: Andy Shevchenko <andriy.shevchenko@linux.intel.com>
//

#[no_mangle]
unsafe extern "C" fn st_lsm9ds0_probe_accel(lsm9ds0: *mut st_lsm9ds0, regmap: *mut regmap) -> c_int {
    static int st_lsm9ds0_probe_accel(struct st_lsm9ds0 *lsm9ds0, struct regmap *regmap)
    {
    const struct st_sensor_settings *settings;
    struct device *dev = lsm9ds0.dev;
    struct st_sensor_data *data;
    settings = st_accel_get_settings(lsm9ds0.name);
    if (!settings)
    return dev_err_probe(dev, -ENODEV, "device name %s not recognized.\n",
    lsm9ds0.name);
    lsm9ds0.accel = devm_iio_device_alloc(dev, sizeof(*data));
    if (!lsm9ds0.accel)
    return -ENOMEM;
    lsm9ds0.accel.name = lsm9ds0.name;
    data = iio_priv(lsm9ds0.accel);
    data.sensor_settings = (struct st_sensor_settings *)settings;
    data.irq = lsm9ds0.irq;
    data.regmap = regmap;
    return st_accel_common_probe(lsm9ds0.accel);
    }
#[no_mangle]
unsafe extern "C" fn st_lsm9ds0_probe_magn(lsm9ds0: *mut st_lsm9ds0, regmap: *mut regmap) -> c_int {
    static int st_lsm9ds0_probe_magn(struct st_lsm9ds0 *lsm9ds0, struct regmap *regmap)
    {
    const struct st_sensor_settings *settings;
    struct device *dev = lsm9ds0.dev;
    struct st_sensor_data *data;
    settings = st_magn_get_settings(lsm9ds0.name);
    if (!settings)
    return dev_err_probe(dev, -ENODEV, "device name %s not recognized.\n",
    lsm9ds0.name);
    lsm9ds0.magn = devm_iio_device_alloc(dev, sizeof(*data));
    if (!lsm9ds0.magn)
    return -ENOMEM;
    lsm9ds0.magn.name = lsm9ds0.name;
    data = iio_priv(lsm9ds0.magn);
    data.sensor_settings = (struct st_sensor_settings *)settings;
    data.irq = lsm9ds0.irq;
    data.regmap = regmap;
    return st_magn_common_probe(lsm9ds0.magn);
    }
#[no_mangle]
pub unsafe extern "C" fn st_lsm9ds0_probe(lsm9ds0: *mut st_lsm9ds0, regmap: *mut regmap) -> c_int {
    int st_lsm9ds0_probe(struct st_lsm9ds0 *lsm9ds0, struct regmap *regmap)
    {
    struct device *dev = lsm9ds0.dev;
    static const char * const regulator_names[] = { "vdd", "vddio" };
    int ret;
// Regulators not mandatory, but if requested we should enable them.
    ret = devm_regulator_bulk_get_enable(dev, ARRAY_SIZE(regulator_names),
    regulator_names);
    if (ret)
    return dev_err_probe(dev, ret, "unable to enable Vdd supply\n");
// Setup accelerometer device
    ret = st_lsm9ds0_probe_accel(lsm9ds0, regmap);
    if (ret)
    return ret;
// Setup magnetometer device
    return st_lsm9ds0_probe_magn(lsm9ds0, regmap);
    }
    EXPORT_SYMBOL_NS_GPL(st_lsm9ds0_probe, "IIO_ST_SENSORS");
    MODULE_AUTHOR("Andy Shevchenko <andriy.shevchenko@linux.intel.com>");
    MODULE_DESCRIPTION("STMicroelectronics LSM9DS0 IMU core driver");
    MODULE_LICENSE("GPL v2");
    MODULE_IMPORT_NS("IIO_ST_SENSORS");
