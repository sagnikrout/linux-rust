//! Automatically rewritten from C to Rust
//! Source: drivers/hwmon/hs3001.c
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
// This is a non-complete driver implementation for the
// HS3001 humidity and temperature sensor and compatibles. It does not include
// the configuration possibilities, where it needs to be set to 'programming mode'
// during power-up.
//
// Copyright (C) 2023 SYS TEC electronic AG
// Author: Andre Werner <andre.werner@systec-electronic.com>
//

// Measurement times

pub const HS3001_RESPONSE_LENGTH: c_int = 4;

// Definitions for Status Bits of A/D Data
pub const HS3001_DATA_VALID: c_uint = 0x00	/* Valid Data */;
pub const HS3001_DATA_STALE: c_uint = 0x01	/* Stale Data */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hs3001_data {
    pub client: *mut i2c_client,
    pub /: *mut *mut u32 wait_time; / in us,
    pub /: *mut *mut int temperature; / in milli degree,
    pub /: *mut *mut u32 humidity; / in milli %,
}

#[no_mangle]
unsafe extern "C" fn hs3001_extract_temperature(raw: u16) -> c_int {
    static int hs3001_extract_temperature(u16 raw)
    {
// fixpoint arithmetic 1 digit
    let mut temp: u32 = (raw >> 2) * HS3001_FIXPOINT_ARITH * 165;
    temp /= (1 << 14) - 1;
    return (int)temp - 40 * HS3001_FIXPOINT_ARITH;
    }
#[no_mangle]
unsafe extern "C" fn hs3001_extract_humidity(raw: u16) -> u32 {
    static u32 hs3001_extract_humidity(u16 raw)
    {
    let mut hum: u32 = (raw & HS3001_MASK_HUMIDITY_0X3FFF) * HS3001_FIXPOINT_ARITH * 100;
    return hum / (1 << 14) - 1;
    }
    static int hs3001_data_fetch_command(struct i2c_client *client,
    struct hs3001_data *data)
    {
    int ret;
    u8 buf[HS3001_RESPONSE_LENGTH];
    u8 hs3001_status;
    ret = i2c_master_recv(client, buf, HS3001_RESPONSE_LENGTH);
    if (ret != HS3001_RESPONSE_LENGTH) {
    ret = ret < 0 ? ret : -EIO;
    dev_dbg(&client.dev,
    "Error in i2c communication. Error code: %d.\n", ret);
    return ret;
    }
    hs3001_status = FIELD_GET(HS3001_MASK_STATUS_0XC0, buf[0]);
    if (hs3001_status == HS3001_DATA_STALE) {
    dev_dbg(&client.dev, "Sensor busy.\n");
    return -EBUSY;
    }
    if (hs3001_status != HS3001_DATA_VALID) {
    dev_dbg(&client.dev, "Data invalid.\n");
    return -EIO;
    }
    data.humidity =
    hs3001_extract_humidity(be16_to_cpup((__be16 *)&buf[0]));
    data.temperature =
    hs3001_extract_temperature(be16_to_cpup((__be16 *)&buf[2]));
    return 0;
    }
    static umode_t hs3001_is_visible(const void *data, enum hwmon_sensor_types type,
    u32 attr, int channel)
    {
// Both, humidity and temperature can only be read.
    return 0444;
    }
    static int hs3001_read(struct device *dev, enum hwmon_sensor_types type,
    u32 attr, int channel, long *val)
    {
    struct hs3001_data *data = dev_get_drvdata(dev);
    struct i2c_client *client = data.client;
    int ret;
    ret = i2c_master_send(client, core::ptr::null_mut(), 0);
    if (ret < 0)
    return ret;
//
// Sensor needs some time to process measurement depending on
// resolution (ref. datasheet)
//
    fsleep(data.wait_time);
    ret = hs3001_data_fetch_command(client, data);
    if (ret < 0)
    return ret;
    switch (type) {
    case hwmon_temp:
    switch (attr) {
    case hwmon_temp_input:
// val = data->temperature;
    break;
    default:
    return -EINVAL;
    }
    break;
    case hwmon_humidity:
    switch (attr) {
    case hwmon_humidity_input:
// val = data->humidity;
    break;
    default:
    return -EINVAL;
    }
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
    static const struct hwmon_channel_info *hs3001_info[] = {
    HWMON_CHANNEL_INFO(temp, HWMON_T_INPUT),
    HWMON_CHANNEL_INFO(humidity, HWMON_H_INPUT),
    core::ptr::null_mut()
    };
    static const struct hwmon_ops hs3001_hwmon_ops = {
    .is_visible = hs3001_is_visible,
    .read = hs3001_read,
    };
    static const struct hwmon_chip_info hs3001_chip_info = {
    .ops = &hs3001_hwmon_ops,
    .info = hs3001_info,
    };
// device ID table
    static const struct i2c_device_id hs3001_ids[] = {
    { .name = "hs3001" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, hs3001_ids);
    static const struct of_device_id hs3001_of_match[] = {
    {.compatible = "renesas,hs3001"},
    { },
    };
    MODULE_DEVICE_TABLE(of, hs3001_of_match);
#[no_mangle]
unsafe extern "C" fn hs3001_probe(client: *mut i2c_client) -> c_int {
    static int hs3001_probe(struct i2c_client *client)
    {
    struct hs3001_data *data;
    struct device *hwmon_dev;
    struct device *dev = &client.dev;
    if (!i2c_check_functionality(client.adapter, I2C_FUNC_I2C))
    return -EOPNOTSUPP;
    data = devm_kzalloc(dev, sizeof(*data), GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    data.client = client;
//
// Measurement time = wake-up time + measurement time temperature
// + measurement time humidity. This is currently static, because
// enabling programming mode is not supported, yet.
//
    data.wait_time = (HS3001_WAKEUP_TIME + HS3001_14BIT_RESOLUTION +
    HS3001_14BIT_RESOLUTION);
    hwmon_dev = devm_hwmon_device_register_with_info(dev,
    client.name,
    data,
    &hs3001_chip_info,
    core::ptr::null_mut());
    if (IS_ERR(hwmon_dev))
    return dev_err_probe(dev, PTR_ERR(hwmon_dev),
    "Unable to register hwmon device.\n");
    return 0;
    }
    static struct i2c_driver hs3001_i2c_driver = {
    .driver = {
    .name = "hs3001",
    .of_match_table = hs3001_of_match,
    },
    .probe = hs3001_probe,
    .id_table = hs3001_ids,
    };
    module_i2c_driver(hs3001_i2c_driver);
    MODULE_AUTHOR("Andre Werner <andre.werner@systec-electronic.com>");
    MODULE_DESCRIPTION("HS3001 humidity and temperature sensor base driver");
    MODULE_LICENSE("GPL");
