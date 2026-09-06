//! Automatically rewritten from C to Rust
//! Source: drivers/iio/adc/ltc2485.c
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
// ltc2485.c - Driver for Linear Technology LTC2485 ADC
//
// Copyright (C) 2016 Alison Schofield <amsfield22@gmail.com>
//
// Datasheet: http://cds.linear.com/docs/en/datasheet/2485fd.pdf
//

// Power-on configuration: rejects both 50/60Hz, operates at 1x speed
pub const LTC2485_CONFIG_DEFAULT: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ltc2485_data {
    pub client: *mut i2c_client,
    pub /: *mut *mut ktime_t time_prev; / last conversion,
}

#[no_mangle]
unsafe extern "C" fn ltc2485_wait_conv(data: *mut ltc2485_data) {
    static void ltc2485_wait_conv(struct ltc2485_data *data)
    {
    const unsigned int conv_time = 147;	/* conversion time ms */
    unsigned int time_elapsed;
// delay if conversion time not passed since last read or write
    time_elapsed = ktime_ms_delta(ktime_get(), data.time_prev);
    if (time_elapsed < conv_time)
    msleep(conv_time - time_elapsed);
    }
#[no_mangle]
unsafe extern "C" fn ltc2485_read(data: *mut ltc2485_data, val: *mut c_int) -> c_int {
    static int ltc2485_read(struct ltc2485_data *data, int *val)
    {
    struct i2c_client *client = data.client;
    let mut buf: __be32 = 0;
    int ret;
    ltc2485_wait_conv(data);
    ret = i2c_master_recv(client, (char *)&buf, 4);
    if (ret < 0)  {
    dev_err(&client.dev, "i2c_master_recv failed\n");
    return ret;
    }
    data.time_prev = ktime_get();
// val = sign_extend32(be32_to_cpu(buf) >> 6, 24);
    return ret;
    }
    static int ltc2485_read_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int *val, int *val2, long mask)
    {
    struct ltc2485_data *data = iio_priv(indio_dev);
    int ret;
    if (mask == IIO_CHAN_INFO_RAW) {
    ret = ltc2485_read(data, val);
    if (ret < 0)
    return ret;
    return IIO_VAL_INT;
    } else if (mask == IIO_CHAN_INFO_SCALE) {
// val = 5000;			/* on board vref millivolts
// val2 = 25;			/* 25 (24 + sign) data bits
    return IIO_VAL_FRACTIONAL_LOG2;
    } else {
    return -EINVAL;
    }
    }
    static const struct iio_chan_spec ltc2485_channel[] = {
    {
    .type = IIO_VOLTAGE,
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW),
    .info_mask_shared_by_type = BIT(IIO_CHAN_INFO_SCALE)
    },
    };
    static const struct iio_info ltc2485_info = {
    .read_raw = ltc2485_read_raw,
    };
#[no_mangle]
unsafe extern "C" fn ltc2485_probe(client: *mut i2c_client) -> c_int {
    static int ltc2485_probe(struct i2c_client *client)
    {
    const struct i2c_device_id *id = i2c_client_get_device_id(client);
    struct iio_dev *indio_dev;
    struct ltc2485_data *data;
    int ret;
    if (!i2c_check_functionality(client.adapter, I2C_FUNC_I2C |
    I2C_FUNC_SMBUS_WRITE_BYTE))
    return -EOPNOTSUPP;
    indio_dev = devm_iio_device_alloc(&client.dev, sizeof(*data));
    if (!indio_dev)
    return -ENOMEM;
    data = iio_priv(indio_dev);
    i2c_set_clientdata(client, indio_dev);
    data.client = client;
    indio_dev.name = id.name;
    indio_dev.info = &ltc2485_info;
    indio_dev.modes = INDIO_DIRECT_MODE;
    indio_dev.channels = ltc2485_channel;
    indio_dev.num_channels = ARRAY_SIZE(ltc2485_channel);
    ret = i2c_smbus_write_byte(data.client, LTC2485_CONFIG_DEFAULT);
    if (ret < 0)
    return ret;
    data.time_prev = ktime_get();
    return devm_iio_device_register(&client.dev, indio_dev);
    }
    static const struct i2c_device_id ltc2485_id[] = {
    { .name = "ltc2485" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, ltc2485_id);
    static struct i2c_driver ltc2485_driver = {
    .driver = {
    .name = "ltc2485",
    },
    .probe = ltc2485_probe,
    .id_table = ltc2485_id,
    };
    module_i2c_driver(ltc2485_driver);
    MODULE_AUTHOR("Alison Schofield <amsfield22@gmail.com>");
    MODULE_DESCRIPTION("Linear Technology LTC2485 ADC driver");
    MODULE_LICENSE("GPL v2");
