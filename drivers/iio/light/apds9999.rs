//! Automatically rewritten from C to Rust
//! Source: drivers/iio/light/apds9999.c
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
// IIO driver for Broadcom APDS9999 Lux Light Sensor
//
// Copyright (C) 2026
// Author: Jose A. Perez de Azpillaga <azpijr@gmail.com>
//
// TODO: proximity sensor
//

pub const APDS9999_REG_MAIN_CTRL: c_uint = 0x00;

pub const APDS9999_REG_LS_MEAS_RATE: c_uint = 0x04;

pub const APDS9999_REG_LS_GAIN: c_uint = 0x05;
pub const APDS9999_REG_PART_ID: c_uint = 0x06;
pub const APDS9999_REG_MAIN_STATUS: c_uint = 0x07;

pub const APDS9999_REG_LS_DATA_IR_0: c_uint = 0x0A;
pub const APDS9999_REG_LS_DATA_GREEN_0: c_uint = 0x0D;
pub const APDS9999_REG_LS_DATA_BLUE_0: c_uint = 0x10;
pub const APDS9999_REG_LS_DATA_RED_0: c_uint = 0x13;
pub const APDS9999_PART_ID: c_uint = 0xC2;
pub const APDS9999_GAIN_1X: c_int = 0;
pub const APDS9999_GAIN_3X: c_int = 1;
pub const APDS9999_GAIN_6X: c_int = 2;
pub const APDS9999_GAIN_9X: c_int = 3;
pub const APDS9999_GAIN_18X: c_int = 4;
    static const int apds9999_gains[] = {
    [APDS9999_GAIN_1X]  = 1,
    [APDS9999_GAIN_3X]  = 3,
    [APDS9999_GAIN_6X]  = 6,
    [APDS9999_GAIN_9X]  = 9,
    [APDS9999_GAIN_18X] = 18,
    };
pub const APDS9999_RES_20BIT: c_int = 0;
pub const APDS9999_RES_19BIT: c_int = 1;
pub const APDS9999_RES_18BIT: c_int = 2;
pub const APDS9999_RES_17BIT: c_int = 3;
pub const APDS9999_RES_16BIT: c_int = 4;
pub const APDS9999_RES_13BIT: c_int = 5;
    static const int apds9999_itimes_us[] = {
    [APDS9999_RES_20BIT] = 400 * USEC_PER_MSEC,
    [APDS9999_RES_19BIT] = 200 * USEC_PER_MSEC,
    [APDS9999_RES_18BIT] = 100 * USEC_PER_MSEC,
    [APDS9999_RES_17BIT] =  50 * USEC_PER_MSEC,
    [APDS9999_RES_16BIT] =  25 * USEC_PER_MSEC,
    [APDS9999_RES_13BIT] =   3125,
    };
pub const APDS9999_RATE_25_MS: c_int = 0;
pub const APDS9999_RATE_50_MS: c_int = 1;
pub const APDS9999_RATE_100_MS: c_int = 2;
pub const APDS9999_RATE_200_MS: c_int = 3;
pub const APDS9999_RATE_500_MS: c_int = 4;
pub const APDS9999_RATE_1000_MS: c_int = 5;
pub const APDS9999_RATE_2000_MS: c_int = 6;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct apds9999_data {
    pub client: *mut i2c_client,
// lock: serializes access to device registers and cached values
    pub lock: mutex,
    pub als_gain_idx: c_int,
    pub als_res: c_int,
    pub als_rate: c_int,
}

#[no_mangle]
unsafe extern "C" fn apds9999_standby(client: *mut c_void) {
    static void apds9999_standby(void *client)
    {
    i2c_smbus_write_byte_data(client, APDS9999_REG_MAIN_CTRL, 0);
    }
//
// Apply power-on defaults: 18-bit / 100 ms resolution and rate,
// 3x gain. These match the datasheet reset values.
//
#[no_mangle]
unsafe extern "C" fn apds9999_init(data: *mut apds9999_data) -> c_int {
    static int apds9999_init(struct apds9999_data *data)
    {
    struct device *dev = &data.client.dev;
    struct i2c_client *client = data.client;
    u8 regval;
    int ret;
    ret = devm_add_action_or_reset(dev, apds9999_standby, client);
    if (ret)
    return ret;
    guard(mutex)(&data.lock);
    regval = FIELD_PREP(APDS9999_LS_RES_MASK, APDS9999_RES_18BIT) |
    FIELD_PREP(APDS9999_LS_RATE_MASK, APDS9999_RATE_100_MS);
    ret = i2c_smbus_write_byte_data(client, APDS9999_REG_LS_MEAS_RATE,
    regval);
    if (ret)
    return ret;
    data.als_res = APDS9999_RES_18BIT;
    data.als_rate = APDS9999_RATE_100_MS;
    ret = i2c_smbus_write_byte_data(client, APDS9999_REG_LS_GAIN,
    APDS9999_GAIN_3X);
    if (ret)
    return ret;
    data.als_gain_idx = APDS9999_GAIN_3X;
    return i2c_smbus_write_byte_data(client, APDS9999_REG_MAIN_CTRL,
    APDS9999_MAIN_CTRL_LS_EN);
    }
    static int apds9999_read_channel(struct apds9999_data *data, u8 reg,
    u32 *counts)
    {
    struct i2c_client *client = data.client;
    u8 buf[3];
    int ret, tries;
    guard(mutex)(&data.lock);
//
// Poll MAIN_STATUS for new data.  Timeout: ~2 integration periods
// plus margin.  Each try sleeps 20 ms.
//
    tries = max(2, (apds9999_itimes_us[data.als_res] * 2) / 20000);
    while (tries--) {
    ret = i2c_smbus_read_byte_data(client,
    APDS9999_REG_MAIN_STATUS);
    if (ret < 0)
    return ret;
    if (ret & APDS9999_MAIN_STATUS_LS_DATA)
    break;
    fsleep(20000);
    }
    if (tries < 0)
    return -ETIMEDOUT;
    ret = i2c_smbus_read_i2c_block_data(client, reg, sizeof(buf), buf);
    if (ret < 0)
    return ret;
    if (ret != sizeof(buf))
    return -EIO;
// counts = get_unaligned_le24(buf) & GENMASK(19, 0);
    return 0;
    }
    static int apds9999_read_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int *val, int *val2, long mask)
    {
    struct apds9999_data *data = iio_priv(indio_dev);
    int gain, itime_us;
    u64 scale_nano;
    u32 counts;
    int ret;
    switch (mask) {
    case IIO_CHAN_INFO_RAW:
    ret = apds9999_read_channel(data, chan.address, &counts);
    if (ret)
    return ret;
// val = (int)counts;
    return IIO_VAL_INT;
    case IIO_CHAN_INFO_SCALE: {
    u32 remainder;
//
// Scale (lux per count) = 54 / (gain * integration_time_ms)
//
// The constant 54 is derived from the datasheet table:
// at gain = 3x, itime = 100 ms -> 0.180 lux/count
// -> C = 0.180 * 3 * 100 = 54
//
// Expressed as IIO_VAL_INT_PLUS_NANO.
//
    gain = apds9999_gains[data.als_gain_idx];
    itime_us = apds9999_itimes_us[data.als_res];
// scale_nano = 54 * 1e12 / (gain * itime_us) nano-lux/count
    scale_nano = div_u64(54ULL * NSEC_PER_SEC * USEC_PER_MSEC, (u32)(gain * itime_us));
// val = (int)div_u64_rem(scale_nano, NSEC_PER_SEC, &remainder);
// val2 = (int)remainder;
    return IIO_VAL_INT_PLUS_NANO;
    }
    case IIO_CHAN_INFO_INT_TIME:
// val = 0;
// val2 = apds9999_itimes_us[data->als_res];
    return IIO_VAL_INT_PLUS_MICRO;
    default:
    return -EINVAL;
    }
    }
    static const struct iio_info apds9999_info = {
    .read_raw = apds9999_read_raw,
    };
//
// The green channel uses optical coating to approximate the human eye
// spectral response.  IIO_INTENSITY channels provide raw ADC data for
// red, green, blue, and IR so userspace can compute weighted lux.
//
    static const struct iio_chan_spec apds9999_channels[] = {
    {
    .type = IIO_LIGHT,
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW) |
    BIT(IIO_CHAN_INFO_SCALE),
    .info_mask_shared_by_all = BIT(IIO_CHAN_INFO_INT_TIME),
    .address = APDS9999_REG_LS_DATA_GREEN_0,
    },
    {
    .type = IIO_INTENSITY,
    .modified = 1,
    .channel2 = IIO_MOD_LIGHT_RED,
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW),
    .info_mask_shared_by_all = BIT(IIO_CHAN_INFO_INT_TIME),
    .address = APDS9999_REG_LS_DATA_RED_0,
    },
    {
    .type = IIO_INTENSITY,
    .modified = 1,
    .channel2 = IIO_MOD_LIGHT_GREEN,
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW),
    .info_mask_shared_by_all = BIT(IIO_CHAN_INFO_INT_TIME),
    .address = APDS9999_REG_LS_DATA_GREEN_0,
    },
    {
    .type = IIO_INTENSITY,
    .modified = 1,
    .channel2 = IIO_MOD_LIGHT_BLUE,
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW),
    .info_mask_shared_by_all = BIT(IIO_CHAN_INFO_INT_TIME),
    .address = APDS9999_REG_LS_DATA_BLUE_0,
    },
    {
    .type = IIO_INTENSITY,
    .modified = 1,
    .channel2 = IIO_MOD_LIGHT_IR,
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW),
    .info_mask_shared_by_all = BIT(IIO_CHAN_INFO_INT_TIME),
    .address = APDS9999_REG_LS_DATA_IR_0,
    },
    };
#[no_mangle]
unsafe extern "C" fn apds9999_probe(client: *mut i2c_client) -> c_int {
    static int apds9999_probe(struct i2c_client *client)
    {
    struct device *dev = &client.dev;
    struct apds9999_data *data;
    struct iio_dev *indio_dev;
    int ret, part_id;
    indio_dev = devm_iio_device_alloc(dev, sizeof(*data));
    if (!indio_dev)
    return -ENOMEM;
    data = iio_priv(indio_dev);
    data.client = client;
    ret = devm_mutex_init(dev, &data.lock);
    if (ret)
    return ret;
    part_id = i2c_smbus_read_byte_data(client, APDS9999_REG_PART_ID);
    if (part_id < 0)
    return dev_err_probe(dev, part_id, "failed to read PART_ID\n");
    if (part_id != APDS9999_PART_ID)
    dev_info(dev, "unexpected PART_ID 0x%02x (expected 0x%02x)\n",
    part_id, APDS9999_PART_ID);
    ret = apds9999_init(data);
    if (ret)
    return dev_err_probe(dev, ret, "failed to initialize device\n");
    indio_dev.name = "apds9999";
    indio_dev.info = &apds9999_info;
    indio_dev.channels = apds9999_channels;
    indio_dev.num_channels = ARRAY_SIZE(apds9999_channels);
    indio_dev.modes = INDIO_DIRECT_MODE;
    ret = devm_iio_device_register(dev, indio_dev);
    if (ret)
    return dev_err_probe(dev, ret, "failed to register IIO device\n");
    return 0;
    }
    static const struct i2c_device_id apds9999_id[] = {
    { .name = "apds9999" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, apds9999_id);
    static const struct of_device_id apds9999_of_match[] = {
    { .compatible = "brcm,apds9999" },
    { }
    };
    MODULE_DEVICE_TABLE(of, apds9999_of_match);
    static struct i2c_driver apds9999_driver = {
    .driver = {
    .name = "apds9999",
    .of_match_table = apds9999_of_match,
    },
    .probe = apds9999_probe,
    .id_table = apds9999_id,
    };
    module_i2c_driver(apds9999_driver);
    MODULE_AUTHOR("Jose A. Perez de Azpillaga <azpijr@gmail.com>");
    MODULE_DESCRIPTION("APDS-9999 Lux Light Sensor IIO Driver");
    MODULE_LICENSE("GPL");
