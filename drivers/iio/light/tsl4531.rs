//! Automatically rewritten from C to Rust
//! Source: drivers/iio/light/tsl4531.c
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
// tsl4531.c - Support for TAOS TSL4531 ambient light sensor
//
// Copyright 2013 Peter Meerwald <pmeerw@pmeerw.net>
//
// IIO driver for the TSL4531x family
// TSL45311/TSL45313: 7-bit I2C slave address 0x39
// TSL45315/TSL45317: 7-bit I2C slave address 0x29
//
// TODO: single cycle measurement
//

// operating modes in control register
pub const TSL4531_MODE_POWERDOWN: c_uint = 0x00;
pub const TSL4531_MODE_SINGLE_ADC: c_uint = 0x02;
pub const TSL4531_MODE_NORMAL: c_uint = 0x03;
// integration time control in config register
pub const TSL4531_TCNTRL_400MS: c_uint = 0x00;
pub const TSL4531_TCNTRL_200MS: c_uint = 0x01;
pub const TSL4531_TCNTRL_100MS: c_uint = 0x02;
// part number in id register
pub const TSL45311_ID: c_uint = 0x8;
pub const TSL45313_ID: c_uint = 0x9;
pub const TSL45315_ID: c_uint = 0xa;
pub const TSL45317_ID: c_uint = 0xb;
pub const TSL4531_ID_SHIFT: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tsl4531_data {
    pub client: *mut i2c_client,
    pub lock: mutex,
    pub int_time: c_int,
}

    static IIO_CONST_ATTR_INT_TIME_AVAIL("0.1 0.2 0.4");
    static struct attribute *tsl4531_attributes[] = {
    &iio_const_attr_integration_time_available.dev_attr.attr,
    core::ptr::null_mut()
    };
    static const struct attribute_group tsl4531_attribute_group = {
    .attrs = tsl4531_attributes,
    };
    static const struct iio_chan_spec tsl4531_channels[] = {
    {
    .type = IIO_LIGHT,
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW) |
    BIT(IIO_CHAN_INFO_SCALE) |
    BIT(IIO_CHAN_INFO_INT_TIME)
    }
    };
    static int tsl4531_read_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int *val, int *val2, long mask)
    {
    struct tsl4531_data *data = iio_priv(indio_dev);
    int ret;
    switch (mask) {
    case IIO_CHAN_INFO_RAW:
    ret = i2c_smbus_read_word_data(data.client,
    TSL4531_DATA);
    if (ret < 0)
    return ret;
// val = ret;
    return IIO_VAL_INT;
    case IIO_CHAN_INFO_SCALE:
// 0.. 1x, 1 .. 2x, 2 .. 4x
// val = 1 << data->int_time;
    return IIO_VAL_INT;
    case IIO_CHAN_INFO_INT_TIME:
    if (data.int_time == 0)
// val2 = 400000;
#[no_mangle]
pub unsafe extern "C" fn if(1: data->int_time ==) -> else {
    else if (data.int_time == 1)
// val2 = 200000;
#[no_mangle]
pub unsafe extern "C" fn if(2: data->int_time ==) -> else {
    else if (data.int_time == 2)
// val2 = 100000;
    else
    return -EINVAL;
// val = 0;
    return IIO_VAL_INT_PLUS_MICRO;
    default:
    return -EINVAL;
    }
    }
    static int tsl4531_write_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int val, int val2, long mask)
    {
    struct tsl4531_data *data = iio_priv(indio_dev);
    int int_time, ret;
    switch (mask) {
    case IIO_CHAN_INFO_INT_TIME:
    if (val != 0)
    return -EINVAL;
    if (val2 == 400000)
    int_time = 0;
#[no_mangle]
pub unsafe extern "C" fn if(200000: val2 ==) -> else {
    else if (val2 == 200000)
    int_time = 1;
#[no_mangle]
pub unsafe extern "C" fn if(100000: val2 ==) -> else {
    else if (val2 == 100000)
    int_time = 2;
    else
    return -EINVAL;
    mutex_lock(&data.lock);
    ret = i2c_smbus_write_byte_data(data.client,
    TSL4531_CONFIG, int_time);
    if (ret >= 0)
    data.int_time = int_time;
    mutex_unlock(&data.lock);
    return ret;
    default:
    return -EINVAL;
    }
    }
    static const struct iio_info tsl4531_info = {
    .read_raw = tsl4531_read_raw,
    .write_raw = tsl4531_write_raw,
    .attrs = &tsl4531_attribute_group,
    };
#[no_mangle]
unsafe extern "C" fn tsl4531_check_id(client: *mut i2c_client) -> c_int {
    static int tsl4531_check_id(struct i2c_client *client)
    {
    let mut ret: c_int = i2c_smbus_read_byte_data(client, TSL4531_ID);
    if (ret < 0)
    return ret;
    switch (ret >> TSL4531_ID_SHIFT) {
    case TSL45311_ID:
    case TSL45313_ID:
    case TSL45315_ID:
    case TSL45317_ID:
    return 0;
    default:
    return -ENODEV;
    }
    }
#[no_mangle]
unsafe extern "C" fn tsl4531_probe(client: *mut i2c_client) -> c_int {
    static int tsl4531_probe(struct i2c_client *client)
    {
    struct tsl4531_data *data;
    struct iio_dev *indio_dev;
    int ret;
    indio_dev = devm_iio_device_alloc(&client.dev, sizeof(*data));
    if (!indio_dev)
    return -ENOMEM;
    data = iio_priv(indio_dev);
    i2c_set_clientdata(client, indio_dev);
    data.client = client;
    mutex_init(&data.lock);
    ret = tsl4531_check_id(client);
    if (ret) {
    dev_err(&client.dev, "no TSL4531 sensor\n");
    return ret;
    }
    ret = i2c_smbus_write_byte_data(data.client, TSL4531_CONTROL,
    TSL4531_MODE_NORMAL);
    if (ret < 0)
    return ret;
    ret = i2c_smbus_write_byte_data(data.client, TSL4531_CONFIG,
    TSL4531_TCNTRL_400MS);
    if (ret < 0)
    return ret;
    indio_dev.info = &tsl4531_info;
    indio_dev.channels = tsl4531_channels;
    indio_dev.num_channels = ARRAY_SIZE(tsl4531_channels);
    indio_dev.name = TSL4531_DRV_NAME;
    indio_dev.modes = INDIO_DIRECT_MODE;
    return iio_device_register(indio_dev);
    }
#[no_mangle]
unsafe extern "C" fn tsl4531_powerdown(client: *mut i2c_client) -> c_int {
    static int tsl4531_powerdown(struct i2c_client *client)
    {
    return i2c_smbus_write_byte_data(client, TSL4531_CONTROL,
    TSL4531_MODE_POWERDOWN);
    }
#[no_mangle]
unsafe extern "C" fn tsl4531_remove(client: *mut i2c_client) {
    static void tsl4531_remove(struct i2c_client *client)
    {
    iio_device_unregister(i2c_get_clientdata(client));
    tsl4531_powerdown(client);
    }
#[no_mangle]
unsafe extern "C" fn tsl4531_suspend(dev: *mut device) -> c_int {
    static int tsl4531_suspend(struct device *dev)
    {
    return tsl4531_powerdown(to_i2c_client(dev));
    }
#[no_mangle]
unsafe extern "C" fn tsl4531_resume(dev: *mut device) -> c_int {
    static int tsl4531_resume(struct device *dev)
    {
    return i2c_smbus_write_byte_data(to_i2c_client(dev), TSL4531_CONTROL,
    TSL4531_MODE_NORMAL);
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(tsl4531_pm_ops, tsl4531_suspend,
    tsl4531_resume);
    static const struct i2c_device_id tsl4531_id[] = {
    { .name = "tsl4531" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, tsl4531_id);
    static struct i2c_driver tsl4531_driver = {
    .driver = {
    .name   = TSL4531_DRV_NAME,
    .pm	= pm_sleep_ptr(&tsl4531_pm_ops),
    },
    .probe = tsl4531_probe,
    .remove = tsl4531_remove,
    .id_table = tsl4531_id,
    };
    module_i2c_driver(tsl4531_driver);
    MODULE_AUTHOR("Peter Meerwald <pmeerw@pmeerw.net>");
    MODULE_DESCRIPTION("TAOS TSL4531 ambient light sensors driver");
    MODULE_LICENSE("GPL");
