//! Automatically rewritten from C to Rust
//! Source: drivers/iio/temperature/max30208.c
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
// Copyright (c) Rajat Khandelwal <rajat.khandelwal@linux.intel.com>
//
// Maxim MAX30208 digital temperature sensor with 0.1°C accuracy
// (7-bit I2C slave address (0x50 - 0x53))
//

pub const MAX30208_STATUS: c_uint = 0x00;

pub const MAX30208_INT_ENABLE: c_uint = 0x01;

pub const MAX30208_FIFO_OVF_CNTR: c_uint = 0x06;
pub const MAX30208_FIFO_DATA_CNTR: c_uint = 0x07;
pub const MAX30208_FIFO_DATA: c_uint = 0x08;
pub const MAX30208_FIFO_CONFIG: c_uint = 0x0a;

pub const MAX30208_SYSTEM_CTRL: c_uint = 0x0c;
pub const MAX30208_SYSTEM_CTRL_RESET: c_uint = 0x01;
pub const MAX30208_TEMP_SENSOR_SETUP: c_uint = 0x14;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct max30208_data {
    pub client: *mut i2c_client,
    pub /: *mut *mut mutex lock; / Lock to prevent concurrent reads of temperature readings,
}

    static const struct iio_chan_spec max30208_channels[] = {
    {
    .type = IIO_TEMP,
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW) | BIT(IIO_CHAN_INFO_SCALE),
    },
    };
//
// max30208_request() - Request a reading
// @data: Struct comprising member elements of the device
//
// Requests a reading from the device and waits until the conversion is ready.
//
#[no_mangle]
unsafe extern "C" fn max30208_request(data: *mut max30208_data) -> c_int {
    static int max30208_request(struct max30208_data *data)
    {
//
// Sensor can take up to 500 ms to respond so execute a total of
// 10 retries to give the device sufficient time.
//
    let mut retries: c_int = 10;
    u8 regval;
    int ret;
    ret = i2c_smbus_read_byte_data(data.client, MAX30208_TEMP_SENSOR_SETUP);
    if (ret < 0)
    return ret;
    regval = ret | MAX30208_TEMP_SENSOR_SETUP_CONV;
    ret = i2c_smbus_write_byte_data(data.client, MAX30208_TEMP_SENSOR_SETUP, regval);
    if (ret)
    return ret;
    while (retries--) {
    ret = i2c_smbus_read_byte_data(data.client, MAX30208_STATUS);
    if (ret < 0)
    return ret;
    if (ret & MAX30208_STATUS_TEMP_RDY)
    return 0;
    msleep(50);
    }
    dev_err(&data.client.dev, "Temperature conversion failed\n");
    return -ETIMEDOUT;
    }
#[no_mangle]
unsafe extern "C" fn max30208_update_temp(data: *mut max30208_data) -> c_int {
    static int max30208_update_temp(struct max30208_data *data)
    {
    u8 data_count;
    int ret;
    mutex_lock(&data.lock);
    ret = max30208_request(data);
    if (ret)
    goto unlock;
    ret = i2c_smbus_read_byte_data(data.client, MAX30208_FIFO_OVF_CNTR);
    if (ret < 0)
    goto unlock;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !ret) -> else {
    ret = i2c_smbus_read_byte_data(data.client, MAX30208_FIFO_DATA_CNTR);
    if (ret < 0)
    goto unlock;
    data_count = ret;
    } else
    data_count = 1;
    while (data_count) {
    ret = i2c_smbus_read_word_swapped(data.client, MAX30208_FIFO_DATA);
    if (ret < 0)
    goto unlock;
    data_count--;
    }
    unlock:
    mutex_unlock(&data.lock);
    return ret;
    }
//
// max30208_config_setup() - Set up FIFO configuration register
// @data: Struct comprising member elements of the device
//
// Sets the rollover bit to '1' to enable overwriting FIFO during overflow.
//
#[no_mangle]
unsafe extern "C" fn max30208_config_setup(data: *mut max30208_data) -> c_int {
    static int max30208_config_setup(struct max30208_data *data)
    {
    u8 regval;
    int ret;
    ret = i2c_smbus_read_byte_data(data.client, MAX30208_FIFO_CONFIG);
    if (ret < 0)
    return ret;
    regval = ret | MAX30208_FIFO_CONFIG_RO;
    ret = i2c_smbus_write_byte_data(data.client, MAX30208_FIFO_CONFIG, regval);
    if (ret)
    return ret;
    return 0;
    }
    static int max30208_read(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int *val, int *val2, long mask)
    {
    struct max30208_data *data = iio_priv(indio_dev);
    int ret;
    switch (mask) {
    case IIO_CHAN_INFO_RAW:
    ret = max30208_update_temp(data);
    if (ret < 0)
    return ret;
// val = sign_extend32(ret, 15);
    return IIO_VAL_INT;
    case IIO_CHAN_INFO_SCALE:
// val = 5;
    return IIO_VAL_INT;
    default:
    return -EINVAL;
    }
    }
    static const struct iio_info max30208_info = {
    .read_raw = max30208_read,
    };
#[no_mangle]
unsafe extern "C" fn max30208_probe(i2c: *mut i2c_client) -> c_int {
    static int max30208_probe(struct i2c_client *i2c)
    {
    struct device *dev = &i2c.dev;
    struct max30208_data *data;
    struct iio_dev *indio_dev;
    int ret;
    indio_dev = devm_iio_device_alloc(dev, sizeof(*data));
    if (!indio_dev)
    return -ENOMEM;
    data = iio_priv(indio_dev);
    data.client = i2c;
    mutex_init(&data.lock);
    indio_dev.name = "max30208";
    indio_dev.channels = max30208_channels;
    indio_dev.num_channels = ARRAY_SIZE(max30208_channels);
    indio_dev.info = &max30208_info;
    indio_dev.modes = INDIO_DIRECT_MODE;
    ret = i2c_smbus_write_byte_data(data.client, MAX30208_SYSTEM_CTRL,
    MAX30208_SYSTEM_CTRL_RESET);
    if (ret) {
    dev_err(dev, "Failure in performing reset\n");
    return ret;
    }
    msleep(50);
    ret = max30208_config_setup(data);
    if (ret)
    return ret;
    ret = devm_iio_device_register(dev, indio_dev);
    if (ret) {
    dev_err(dev, "Failed to register IIO device\n");
    return ret;
    }
    return 0;
    }
    static const struct i2c_device_id max30208_id_table[] = {
    { .name = "max30208" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, max30208_id_table);
    static const struct acpi_device_id max30208_acpi_match[] = {
    { "MAX30208" },
    { }
    };
    MODULE_DEVICE_TABLE(acpi, max30208_acpi_match);
    static const struct of_device_id max30208_of_match[] = {
    { .compatible = "maxim,max30208" },
    { }
    };
    MODULE_DEVICE_TABLE(of, max30208_of_match);
    static struct i2c_driver max30208_driver = {
    .driver = {
    .name = "max30208",
    .of_match_table = max30208_of_match,
    .acpi_match_table = max30208_acpi_match,
    },
    .probe = max30208_probe,
    .id_table = max30208_id_table,
    };
    module_i2c_driver(max30208_driver);
    MODULE_AUTHOR("Rajat Khandelwal <rajat.khandelwal@linux.intel.com>");
    MODULE_DESCRIPTION("Maxim MAX30208 digital temperature sensor");
    MODULE_LICENSE("GPL");
