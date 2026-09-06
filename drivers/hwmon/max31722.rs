//! Automatically rewritten from C to Rust
//! Source: drivers/hwmon/max31722.c
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
// max31722 - hwmon driver for Maxim Integrated MAX31722/MAX31723 SPI
// digital thermometer and thermostats.
//
// Copyright (c) 2016, Intel Corporation.
//

pub const MAX31722_REG_CFG: c_uint = 0x00;
pub const MAX31722_REG_TEMP_LSB: c_uint = 0x01;
pub const MAX31722_MODE_CONTINUOUS: c_uint = 0x00;
pub const MAX31722_MODE_STANDBY: c_uint = 0x01;
pub const MAX31722_MODE_MASK: c_uint = 0xFE;
pub const MAX31722_RESOLUTION_12BIT: c_uint = 0x06;
pub const MAX31722_WRITE_MASK: c_uint = 0x80;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct max31722_data {
    pub hwmon_dev: *mut device,
    pub spi_device: *mut spi_device,
    pub mode: u8,
}

#[no_mangle]
unsafe extern "C" fn max31722_set_mode(data: *mut max31722_data, mode: u8) -> c_int {
    static int max31722_set_mode(struct max31722_data *data, u8 mode)
    {
    int ret;
    struct spi_device *spi = data.spi_device;
    u8 buf[2] = {
    MAX31722_REG_CFG | MAX31722_WRITE_MASK,
    (data.mode & MAX31722_MODE_MASK) | mode
    };
    ret = spi_write(spi, &buf, sizeof(buf));
    if (ret < 0) {
    dev_err(&spi.dev, "failed to set sensor mode.\n");
    return ret;
    }
    data.mode = (data.mode & MAX31722_MODE_MASK) | mode;
    return 0;
    }
    static ssize_t max31722_temp_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    ssize_t ret;
    struct max31722_data *data = dev_get_drvdata(dev);
    ret = spi_w8r16(data.spi_device, MAX31722_REG_TEMP_LSB);
    if (ret < 0)
    return ret;
// Keep 12 bits and multiply by the scale of 62.5 millidegrees/bit.
    return sysfs_emit(buf, "%d\n", (s16)le16_to_cpu(ret) * 125 / 32);
    }
    static SENSOR_DEVICE_ATTR_RO(temp1_input, max31722_temp, 0);
    static struct attribute *max31722_attrs[] = {
    &sensor_dev_attr_temp1_input.dev_attr.attr,
    core::ptr::null_mut(),
    };
    ATTRIBUTE_GROUPS(max31722);
#[no_mangle]
unsafe extern "C" fn max31722_probe(spi: *mut spi_device) -> c_int {
    static int max31722_probe(struct spi_device *spi)
    {
    int ret;
    struct max31722_data *data;
    data = devm_kzalloc(&spi.dev, sizeof(*data), GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    spi_set_drvdata(spi, data);
    data.spi_device = spi;
//
// Set SD bit to 0 so we can have continuous measurements.
// Set resolution to 12 bits for maximum precision.
//
    data.mode = MAX31722_MODE_CONTINUOUS | MAX31722_RESOLUTION_12BIT;
    ret = max31722_set_mode(data, MAX31722_MODE_CONTINUOUS);
    if (ret < 0)
    return ret;
    data.hwmon_dev = hwmon_device_register_with_groups(&spi.dev,
    spi.modalias,
    data,
    max31722_groups);
    if (IS_ERR(data.hwmon_dev)) {
    max31722_set_mode(data, MAX31722_MODE_STANDBY);
    return PTR_ERR(data.hwmon_dev);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn max31722_remove(spi: *mut spi_device) {
    static void max31722_remove(struct spi_device *spi)
    {
    struct max31722_data *data = spi_get_drvdata(spi);
    int ret;
    hwmon_device_unregister(data.hwmon_dev);
    ret = max31722_set_mode(data, MAX31722_MODE_STANDBY);
    if (ret)
// There is nothing we can do about this ...
    dev_warn(&spi.dev, "Failed to put device in stand-by mode\n");
    }
#[no_mangle]
unsafe extern "C" fn max31722_suspend(dev: *mut device) -> c_int {
    static int max31722_suspend(struct device *dev)
    {
    struct spi_device *spi_device = to_spi_device(dev);
    struct max31722_data *data = spi_get_drvdata(spi_device);
    return max31722_set_mode(data, MAX31722_MODE_STANDBY);
    }
#[no_mangle]
unsafe extern "C" fn max31722_resume(dev: *mut device) -> c_int {
    static int max31722_resume(struct device *dev)
    {
    struct spi_device *spi_device = to_spi_device(dev);
    struct max31722_data *data = spi_get_drvdata(spi_device);
    return max31722_set_mode(data, MAX31722_MODE_CONTINUOUS);
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(max31722_pm_ops, max31722_suspend, max31722_resume);
    static const struct spi_device_id max31722_spi_id[] = {
    {"max31722", 0},
    {"max31723", 0},
    {}
    };
    MODULE_DEVICE_TABLE(spi, max31722_spi_id);
    static struct spi_driver max31722_driver = {
    .driver = {
    .name = "max31722",
    .pm = pm_sleep_ptr(&max31722_pm_ops),
    },
    .probe =            max31722_probe,
    .remove =           max31722_remove,
    .id_table =         max31722_spi_id,
    };
    module_spi_driver(max31722_driver);
    MODULE_AUTHOR("Tiberiu Breana <tiberiu.a.breana@intel.com>");
    MODULE_DESCRIPTION("max31722 sensor driver");
    MODULE_LICENSE("GPL v2");
