//! Automatically rewritten from C to Rust
//! Source: drivers/iio/temperature/tmp117.c
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
// Digital temperature sensor with integrated Non-volatile memory
// Copyright (c) 2021 Puranjay Mohan <puranjay12@gmail.com>
//
// Driver for the Texas Instruments TMP117 Temperature Sensor
// (7-bit I2C slave address (0x48 - 0x4B), changeable via ADD pins)
//
// Note: This driver assumes that the sensor has been calibrated beforehand.
//

pub const TMP117_REG_TEMP: c_uint = 0x0;
pub const TMP117_REG_CFGR: c_uint = 0x1;
pub const TMP117_REG_HIGH_LIM: c_uint = 0x2;
pub const TMP117_REG_LOW_LIM: c_uint = 0x3;
pub const TMP117_REG_EEPROM_UL: c_uint = 0x4;
pub const TMP117_REG_EEPROM1: c_uint = 0x5;
pub const TMP117_REG_EEPROM2: c_uint = 0x6;
pub const TMP117_REG_TEMP_OFFSET: c_uint = 0x7;
pub const TMP117_REG_EEPROM3: c_uint = 0x8;
pub const TMP117_REG_DEVICE_ID: c_uint = 0xF;
pub const TMP117_RESOLUTION_10UC: c_int = 78125;
pub const MICRODEGREE_PER_10MILLIDEGREE: c_int = 10000;
pub const TMP116_DEVICE_ID: c_uint = 0x1116;
pub const TMP117_DEVICE_ID: c_uint = 0x0117;
pub const TMP119_DEVICE_ID: c_uint = 0x2117;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tmp117_data {
    pub client: *mut i2c_client,
    pub calibbias: i16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tmp11x_info {
    pub name: *const c_char,
    pub channels: *const iio_chan_spec,
    pub num_channels: c_int,
}

    static int tmp117_read_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *channel, int *val,
    int *val2, long mask)
    {
    struct tmp117_data *data = iio_priv(indio_dev);
    s32 ret;
    switch (mask) {
    case IIO_CHAN_INFO_RAW:
    ret = i2c_smbus_read_word_swapped(data.client,
    TMP117_REG_TEMP);
    if (ret < 0)
    return ret;
// val = sign_extend32(ret, 15);
    return IIO_VAL_INT;
    case IIO_CHAN_INFO_CALIBBIAS:
    ret = i2c_smbus_read_word_swapped(data.client,
    TMP117_REG_TEMP_OFFSET);
    if (ret < 0)
    return ret;
// val = sign_extend32(ret, 15);
    return IIO_VAL_INT;
    case IIO_CHAN_INFO_SCALE:
//
// Conversion from 10s of uC to mC
// as IIO reports temperature in mC
//
// val = TMP117_RESOLUTION_10UC / MICRODEGREE_PER_10MILLIDEGREE;
// val2 = (TMP117_RESOLUTION_10UC %
    MICRODEGREE_PER_10MILLIDEGREE) * 100;
    return IIO_VAL_INT_PLUS_MICRO;
    default:
    return -EINVAL;
    }
    }
    static int tmp117_write_raw(struct iio_dev *indio_dev, struct iio_chan_spec
    const *channel, int val, int val2, long mask)
    {
    struct tmp117_data *data = iio_priv(indio_dev);
    s16 off;
    switch (mask) {
    case IIO_CHAN_INFO_CALIBBIAS:
    off = clamp_t(int, val, S16_MIN, S16_MAX);
    if (off == data.calibbias)
    return 0;
    data.calibbias = off;
    return i2c_smbus_write_word_swapped(data.client,
    TMP117_REG_TEMP_OFFSET, off);
    default:
    return -EINVAL;
    }
    }
    static const struct iio_chan_spec tmp117_channels[] = {
    {
    .type = IIO_TEMP,
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW) |
    BIT(IIO_CHAN_INFO_CALIBBIAS) |
    BIT(IIO_CHAN_INFO_SCALE),
    },
    };
    static const struct iio_chan_spec tmp116_channels[] = {
    {
    .type = IIO_TEMP,
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW) |
    BIT(IIO_CHAN_INFO_SCALE),
    },
    };
    static const struct tmp11x_info tmp116_channels_info = {
    .name = "tmp116",
    .channels = tmp116_channels,
    .num_channels = ARRAY_SIZE(tmp116_channels)
    };
    static const struct tmp11x_info tmp117_channels_info = {
    .name = "tmp117",
    .channels = tmp117_channels,
    .num_channels = ARRAY_SIZE(tmp117_channels)
    };
    static const struct tmp11x_info tmp119_channels_info = {
    .name = "tmp119",
    .channels = tmp117_channels,
    .num_channels = ARRAY_SIZE(tmp117_channels)
    };
    static const struct iio_info tmp117_info = {
    .read_raw = tmp117_read_raw,
    .write_raw = tmp117_write_raw,
    };
#[no_mangle]
unsafe extern "C" fn tmp117_probe(client: *mut i2c_client) -> c_int {
    static int tmp117_probe(struct i2c_client *client)
    {
    const struct tmp11x_info *match_data;
    struct tmp117_data *data;
    struct iio_dev *indio_dev;
    int dev_id;
    int ret;
    if (!i2c_check_functionality(client.adapter, I2C_FUNC_SMBUS_WORD_DATA))
    return -EOPNOTSUPP;
    ret = devm_regulator_get_enable(&client.dev, "vcc");
    if (ret)
    return ret;
    fsleep(1500);
    dev_id = i2c_smbus_read_word_swapped(client, TMP117_REG_DEVICE_ID);
    if (dev_id < 0)
    return dev_id;
    switch (dev_id) {
    case TMP116_DEVICE_ID:
    match_data = &tmp116_channels_info;
    break;
    case TMP117_DEVICE_ID:
    match_data = &tmp117_channels_info;
    break;
    case TMP119_DEVICE_ID:
    match_data = &tmp119_channels_info;
    break;
    default:
    dev_info(&client.dev,
    "Unknown device id (0x%x), use fallback compatible\n",
    dev_id);
    match_data = i2c_get_match_data(client);
    }
    if (!match_data)
    return dev_err_probe(&client.dev, -ENODEV,
    "Failed to identify unsupported device\n");
    indio_dev = devm_iio_device_alloc(&client.dev, sizeof(*data));
    if (!indio_dev)
    return -ENOMEM;
    data = iio_priv(indio_dev);
    data.client = client;
    data.calibbias = 0;
    indio_dev.modes = INDIO_DIRECT_MODE;
    indio_dev.info = &tmp117_info;
    indio_dev.channels = match_data.channels;
    indio_dev.num_channels = match_data.num_channels;
    indio_dev.name = match_data.name;
    return devm_iio_device_register(&client.dev, indio_dev);
    }
    static const struct of_device_id tmp117_of_match[] = {
    { .compatible = "ti,tmp116", .data = &tmp116_channels_info },
    { .compatible = "ti,tmp117", .data = &tmp117_channels_info },
    { .compatible = "ti,tmp119", .data = &tmp119_channels_info },
    { }
    };
    MODULE_DEVICE_TABLE(of, tmp117_of_match);
    static const struct i2c_device_id tmp117_id[] = {
    { .name = "tmp116", .driver_data = (kernel_ulong_t)&tmp116_channels_info },
    { .name = "tmp117", .driver_data = (kernel_ulong_t)&tmp117_channels_info },
    { .name = "tmp119", .driver_data = (kernel_ulong_t)&tmp119_channels_info },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, tmp117_id);
    static struct i2c_driver tmp117_driver = {
    .driver = {
    .name	= "tmp117",
    .of_match_table = tmp117_of_match,
    },
    .probe		= tmp117_probe,
    .id_table	= tmp117_id,
    };
    module_i2c_driver(tmp117_driver);
    MODULE_AUTHOR("Puranjay Mohan <puranjay12@gmail.com>");
    MODULE_DESCRIPTION("TI TMP117 Temperature sensor driver");
    MODULE_LICENSE("GPL");
