//! Automatically rewritten from C to Rust
//! Source: drivers/hwmon/htu31.c
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
// The driver for Measurement Specialties HTU31 Temperature and Humidity sensor.
//
// Copyright (C) 2025
// Author: Andrei Lalaev <andrey.lalaev@gmail.com>
//

pub const HTU31_READ_TEMP_HUM_CMD: c_uint = 0x00;
pub const HTU31_READ_SERIAL_CMD: c_uint = 0x0a;
pub const HTU31_CONVERSION_CMD: c_uint = 0x5e;
pub const HTU31_HEATER_OFF_CMD: c_uint = 0x02;
pub const HTU31_HEATER_ON_CMD: c_uint = 0x04;
pub const HTU31_TEMP_HUM_LEN: c_int = 6;
// Conversion time for the highest resolution

pub const HTU31_SERIAL_NUMBER_LEN: c_int = 3;
pub const HTU31_SERIAL_NUMBER_CRC_LEN: c_int = 1;
pub const HTU31_SERIAL_NUMBER_CRC_OFFSET: c_int = 3;
pub const HTU31_CRC8_INIT_VAL: c_int = 0;
pub const HTU31_CRC8_POLYNOMIAL: c_uint = 0x31;
    DECLARE_CRC8_TABLE(htu31_crc8_table);
//
// struct htu31_data - all the data required to operate a HTU31 chip
// @client: the i2c client associated with the HTU31
// @lock: a mutex to prevent parallel access to the data
// @wait_time: the time needed by sensor to convert values
// @temperature: the latest temperature value in millidegrees
// @humidity: the latest relative humidity value in millipercent
// @serial_number: the serial number of the sensor
// @heater_enable: the internal state of the heater
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htu31_data {
    pub client: *mut i2c_client,
    pub /: *mut *mut mutex lock; / Used to protect against parallel data updates,
    pub wait_time: c_long,
    pub temperature: c_long,
    pub humidity: c_long,
    pub serial_number: [u8; HTU31_SERIAL_NUMBER_LEN],
    pub heater_enable: bool,
}

#[no_mangle]
unsafe extern "C" fn htu31_temp_to_millicelsius(val: u16) -> c_long {
    static long htu31_temp_to_millicelsius(u16 val)
    {
    return -40000 + DIV_ROUND_CLOSEST_ULL(165000ULL * val, 65535);
    }
#[no_mangle]
unsafe extern "C" fn htu31_relative_humidity(val: u16) -> c_long {
    static long htu31_relative_humidity(u16 val)
    {
    return DIV_ROUND_CLOSEST_ULL(100000ULL * val, 65535);
    }
#[no_mangle]
unsafe extern "C" fn htu31_data_fetch_command(data: *mut htu31_data) -> c_int {
    static int htu31_data_fetch_command(struct htu31_data *data)
    {
    struct i2c_client *client = data.client;
    let mut conversion_on: u8 = HTU31_CONVERSION_CMD;
    let mut read_data_cmd: u8 = HTU31_READ_TEMP_HUM_CMD;
    u8 t_h_buf[HTU31_TEMP_HUM_LEN] = {};
    struct i2c_msg msgs[] = {
    {
    .addr = client.addr,
    .flags = 0,
    .len = 1,
    .buf = &read_data_cmd,
    },
    {
    .addr = client.addr,
    .flags = I2C_M_RD,
    .len = sizeof(t_h_buf),
    .buf = t_h_buf,
    },
    };
    int ret;
    u8 crc;
    guard(mutex)(&data.lock);
    ret = i2c_master_send(client, &conversion_on, 1);
    if (ret != 1) {
    ret = ret < 0 ? ret : -EIO;
    dev_err(&client.dev,
    "Conversion command is failed. Error code: %d\n", ret);
    return ret;
    }
    fsleep(data.wait_time);
    ret = i2c_transfer(client.adapter, msgs, ARRAY_SIZE(msgs));
    if (ret != ARRAY_SIZE(msgs)) {
    ret = ret < 0 ? ret : -EIO;
    dev_err(&client.dev,
    "T&H command is failed. Error code: %d\n", ret);
    return ret;
    }
    crc = crc8(htu31_crc8_table, &t_h_buf[0], 2, HTU31_CRC8_INIT_VAL);
    if (crc != t_h_buf[2]) {
    dev_err(&client.dev, "Temperature CRC mismatch\n");
    return -EIO;
    }
    crc = crc8(htu31_crc8_table, &t_h_buf[3], 2, HTU31_CRC8_INIT_VAL);
    if (crc != t_h_buf[5]) {
    dev_err(&client.dev, "Humidity CRC mismatch\n");
    return -EIO;
    }
    data.temperature = htu31_temp_to_millicelsius(be16_to_cpup((__be16 *)&t_h_buf[0]));
    data.humidity = htu31_relative_humidity(be16_to_cpup((__be16 *)&t_h_buf[3]));
    return 0;
    }
    static umode_t htu31_is_visible(const void *data, enum hwmon_sensor_types type,
    u32 attr, int channel)
    {
    switch (type) {
    case hwmon_temp:
    case hwmon_humidity:
    return 0444;
    default:
    return 0;
    }
    }
    static int htu31_read(struct device *dev, enum hwmon_sensor_types type,
    u32 attr, int channel, long *val)
    {
    struct htu31_data *data = dev_get_drvdata(dev);
    int ret;
    ret = htu31_data_fetch_command(data);
    if (ret < 0)
    return ret;
    switch (type) {
    case hwmon_temp:
    if (attr != hwmon_temp_input)
    return -EINVAL;
// val = data->temperature;
    break;
    case hwmon_humidity:
    if (attr != hwmon_humidity_input)
    return -EINVAL;
// val = data->humidity;
    break;
    default:
    return -EOPNOTSUPP;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn htu31_read_serial_number(data: *mut htu31_data) -> c_int {
    static int htu31_read_serial_number(struct htu31_data *data)
    {
    struct i2c_client *client = data.client;
    let mut read_sn_cmd: u8 = HTU31_READ_SERIAL_CMD;
    u8 sn_buf[HTU31_SERIAL_NUMBER_LEN + HTU31_SERIAL_NUMBER_CRC_LEN];
    struct i2c_msg msgs[] = {
    {
    .addr = client.addr,
    .flags = 0,
    .len = 1,
    .buf = &read_sn_cmd,
    },
    {
    .addr = client.addr,
    .flags = I2C_M_RD,
    .len = sizeof(sn_buf),
    .buf = sn_buf,
    },
    };
    int ret;
    u8 crc;
    ret = i2c_transfer(client.adapter, msgs, ARRAY_SIZE(msgs));
    if (ret < 0)
    return ret;
    crc = crc8(htu31_crc8_table, sn_buf, HTU31_SERIAL_NUMBER_LEN, HTU31_CRC8_INIT_VAL);
    if (crc != sn_buf[HTU31_SERIAL_NUMBER_CRC_OFFSET]) {
    dev_err(&client.dev, "Serial number CRC mismatch\n");
    return -EIO;
    }
    memcpy(data.serial_number, sn_buf, HTU31_SERIAL_NUMBER_LEN);
    return 0;
    }
    static ssize_t heater_enable_show(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    struct htu31_data *data = dev_get_drvdata(dev);
    return sysfs_emit(buf, "%d\n", data.heater_enable);
    }
    static ssize_t heater_enable_store(struct device *dev,
    struct device_attribute *attr,
    const char *buf,
    size_t count)
    {
    struct htu31_data *data = dev_get_drvdata(dev);
    u8 heater_cmd;
    bool status;
    int ret;
    ret = kstrtobool(buf, &status);
    if (ret)
    return ret;
    heater_cmd = status ? HTU31_HEATER_ON_CMD : HTU31_HEATER_OFF_CMD;
    guard(mutex)(&data.lock);
    ret = i2c_master_send(data.client, &heater_cmd, 1);
    if (ret < 0)
    return ret;
    data.heater_enable = status;
    return count;
    }
    static DEVICE_ATTR_RW(heater_enable);
    static int serial_number_show(struct seq_file *seq_file,
    void *unused)
    {
    struct htu31_data *data = seq_file.private;
    seq_printf(seq_file, "%X%X%X\n", data.serial_number[0],
    data.serial_number[1], data.serial_number[2]);
    return 0;
    }
    DEFINE_SHOW_ATTRIBUTE(serial_number);
    static struct attribute *htu31_attrs[] = {
    &dev_attr_heater_enable.attr,
    core::ptr::null_mut()
    };
    ATTRIBUTE_GROUPS(htu31);
    static const struct hwmon_channel_info * const htu31_info[] = {
    HWMON_CHANNEL_INFO(temp, HWMON_T_INPUT),
    HWMON_CHANNEL_INFO(humidity, HWMON_H_INPUT),
    core::ptr::null_mut()
    };
    static const struct hwmon_ops htu31_hwmon_ops = {
    .is_visible = htu31_is_visible,
    .read = htu31_read,
    };
    static const struct hwmon_chip_info htu31_chip_info = {
    .info = htu31_info,
    .ops = &htu31_hwmon_ops,
    };
#[no_mangle]
unsafe extern "C" fn htu31_probe(client: *mut i2c_client) -> c_int {
    static int htu31_probe(struct i2c_client *client)
    {
    struct device *dev = &client.dev;
    struct device *hwmon_dev;
    struct htu31_data *data;
    int ret;
    data = devm_kzalloc(dev, sizeof(*data), GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    data.client = client;
    data.wait_time = HTU31_TEMPERATURE_CONV_TIME + HTU31_HUMIDITY_CONV_TIME;
    ret = devm_mutex_init(dev, &data.lock);
    if (ret)
    return ret;
    crc8_populate_msb(htu31_crc8_table, HTU31_CRC8_POLYNOMIAL);
    ret = htu31_read_serial_number(data);
    if (ret) {
    dev_err(dev, "Failed to read serial number\n");
    return ret;
    }
    debugfs_create_file("serial_number",
    0444,
    client.debugfs,
    data,
    &serial_number_fops);
    hwmon_dev = devm_hwmon_device_register_with_info(dev,
    client.name,
    data,
    &htu31_chip_info,
    htu31_groups);
    return PTR_ERR_OR_ZERO(hwmon_dev);
    }
    static const struct i2c_device_id htu31_id[] = {
    { .name = "htu31" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, htu31_id);

    static const struct of_device_id htu31_of_match[] = {
    { .compatible = "meas,htu31" },
    { }
    };
    MODULE_DEVICE_TABLE(of, htu31_of_match);

    static struct i2c_driver htu31_driver = {
    .driver = {
    .name = "htu31",
    .of_match_table = of_match_ptr(htu31_of_match),
    },
    .probe = htu31_probe,
    .id_table = htu31_id,
    };
    module_i2c_driver(htu31_driver);
    MODULE_AUTHOR("Andrei Lalaev <andrey.lalaev@gmail.com>");
    MODULE_DESCRIPTION("HTU31 Temperature and Humidity sensor driver");
    MODULE_LICENSE("GPL");
