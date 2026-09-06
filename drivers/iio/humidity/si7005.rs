//! Automatically rewritten from C to Rust
//! Source: drivers/iio/humidity/si7005.c
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
// si7005.c - Support for Silabs Si7005 humidity and temperature sensor
//
// Copyright (c) 2014 Peter Meerwald <pmeerw@pmeerw.net>
//
// (7-bit I2C slave address 0x40)
//
// TODO: heater, fast mode, processed mode (temp. / linearity compensation)
//

pub const SI7005_STATUS: c_uint = 0x00;
pub const SI7005_DATA: c_uint = 0x01 /* 16-bit, MSB */;
pub const SI7005_CONFIG: c_uint = 0x03;
pub const SI7005_ID: c_uint = 0x11;

pub const SI7005_ID_7005: c_uint = 0x50;
pub const SI7005_ID_7015: c_uint = 0xf0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct si7005_data {
    pub client: *mut i2c_client,
    pub lock: mutex,
    pub config: u8,
}

#[no_mangle]
unsafe extern "C" fn si7005_read_measurement(data: *mut si7005_data, temp: bool) -> c_int {
    static int si7005_read_measurement(struct si7005_data *data, bool temp)
    {
    let mut tries: c_int = 50;
    int ret;
    mutex_lock(&data.lock);
    ret = i2c_smbus_write_byte_data(data.client, SI7005_CONFIG,
    data.config | SI7005_CONFIG_START |
    (temp ? SI7005_CONFIG_TEMP : 0));
    if (ret < 0)
    goto done;
    while (tries-- > 0) {
    msleep(20);
    ret = i2c_smbus_read_byte_data(data.client, SI7005_STATUS);
    if (ret < 0)
    goto done;
    if (!(ret & SI7005_STATUS_NRDY))
    break;
    }
    if (tries < 0) {
    ret = -EIO;
    goto done;
    }
    ret = i2c_smbus_read_word_swapped(data.client, SI7005_DATA);
    done:
    mutex_unlock(&data.lock);
    return ret;
    }
    static int si7005_read_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan, int *val,
    int *val2, long mask)
    {
    struct si7005_data *data = iio_priv(indio_dev);
    int ret;
    switch (mask) {
    case IIO_CHAN_INFO_RAW:
    ret = si7005_read_measurement(data, chan.type == IIO_TEMP);
    if (ret < 0)
    return ret;
// val = ret;
    return IIO_VAL_INT;
    case IIO_CHAN_INFO_SCALE:
    if (chan.type == IIO_TEMP) {
// val = 7;
// val2 = 812500;
    } else {
// val = 3;
// val2 = 906250;
    }
    return IIO_VAL_INT_PLUS_MICRO;
    case IIO_CHAN_INFO_OFFSET:
    if (chan.type == IIO_TEMP)
// val = -50 * 32 * 4;
    else
// val = -24 * 16 * 16;
    return IIO_VAL_INT;
    default:
    break;
    }
    return -EINVAL;
    }
    static const struct iio_chan_spec si7005_channels[] = {
    {
    .type = IIO_HUMIDITYRELATIVE,
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW) |
    BIT(IIO_CHAN_INFO_SCALE) | BIT(IIO_CHAN_INFO_OFFSET),
    },
    {
    .type = IIO_TEMP,
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW) |
    BIT(IIO_CHAN_INFO_SCALE) | BIT(IIO_CHAN_INFO_OFFSET),
    }
    };
    static const struct iio_info si7005_info = {
    .read_raw = si7005_read_raw,
    };
#[no_mangle]
unsafe extern "C" fn si7005_probe(client: *mut i2c_client) -> c_int {
    static int si7005_probe(struct i2c_client *client)
    {
    struct iio_dev *indio_dev;
    struct si7005_data *data;
    int ret;
    if (!i2c_check_functionality(client.adapter, I2C_FUNC_SMBUS_WORD_DATA))
    return -EOPNOTSUPP;
    indio_dev = devm_iio_device_alloc(&client.dev, sizeof(*data));
    if (!indio_dev)
    return -ENOMEM;
    data = iio_priv(indio_dev);
    i2c_set_clientdata(client, indio_dev);
    data.client = client;
    mutex_init(&data.lock);
    indio_dev.name = dev_name(&client.dev);
    indio_dev.modes = INDIO_DIRECT_MODE;
    indio_dev.info = &si7005_info;
    indio_dev.channels = si7005_channels;
    indio_dev.num_channels = ARRAY_SIZE(si7005_channels);
    ret = i2c_smbus_read_byte_data(client, SI7005_ID);
    if (ret < 0)
    return ret;
    if (ret != SI7005_ID_7005 && ret != SI7005_ID_7015)
    return -ENODEV;
    ret = i2c_smbus_read_byte_data(client, SI7005_CONFIG);
    if (ret < 0)
    return ret;
    data.config = ret;
    return devm_iio_device_register(&client.dev, indio_dev);
    }
    static const struct i2c_device_id si7005_id[] = {
    { .name = "si7005" },
    { .name = "th02" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, si7005_id);
    static const struct of_device_id si7005_dt_ids[] = {
    { .compatible = "silabs,si7005" },
    { }
    };
    MODULE_DEVICE_TABLE(of, si7005_dt_ids);
    static struct i2c_driver si7005_driver = {
    .driver = {
    .name	= "si7005",
    .of_match_table = si7005_dt_ids,
    },
    .probe = si7005_probe,
    .id_table = si7005_id,
    };
    module_i2c_driver(si7005_driver);
    MODULE_AUTHOR("Peter Meerwald <pmeerw@pmeerw.net>");
    MODULE_DESCRIPTION("Silabs Si7005 humidity and temperature sensor driver");
    MODULE_LICENSE("GPL");
