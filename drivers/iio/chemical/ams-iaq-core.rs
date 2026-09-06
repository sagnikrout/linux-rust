//! Automatically rewritten from C to Rust
//! Source: drivers/iio/chemical/ams-iaq-core.c
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
// ams-iaq-core.c - Support for AMS iAQ-Core VOC sensors
//
// Copyright (C) 2015, 2018
// Author: Matt Ranostay <matt.ranostay@konsulko.com>
//

pub const AMS_IAQCORE_DATA_SIZE: c_int = 9;
pub const AMS_IAQCORE_VOC_CO2_IDX: c_int = 0;
pub const AMS_IAQCORE_VOC_RESISTANCE_IDX: c_int = 1;
pub const AMS_IAQCORE_VOC_TVOC_IDX: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ams_iaqcore_reading {
    pub co2_ppm: __be16,
    pub status: u8,
    pub resistance: __be32,
    pub voc_ppb: __be16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ams_iaqcore_data {
    pub client: *mut i2c_client,
    pub lock: mutex,
    pub last_update: c_ulong,
    pub buffer: ams_iaqcore_reading,
}

    static const struct iio_chan_spec ams_iaqcore_channels[] = {
    {
    .type = IIO_CONCENTRATION,
    .channel2 = IIO_MOD_CO2,
    .modified = 1,
    .info_mask_separate = BIT(IIO_CHAN_INFO_PROCESSED),
    .address = AMS_IAQCORE_VOC_CO2_IDX,
    },
    {
    .type = IIO_RESISTANCE,
    .info_mask_separate = BIT(IIO_CHAN_INFO_PROCESSED),
    .address = AMS_IAQCORE_VOC_RESISTANCE_IDX,
    },
    {
    .type = IIO_CONCENTRATION,
    .channel2 = IIO_MOD_VOC,
    .modified = 1,
    .info_mask_separate = BIT(IIO_CHAN_INFO_PROCESSED),
    .address = AMS_IAQCORE_VOC_TVOC_IDX,
    },
    };
#[no_mangle]
unsafe extern "C" fn ams_iaqcore_read_measurement(data: *mut ams_iaqcore_data) -> c_int {
    static int ams_iaqcore_read_measurement(struct ams_iaqcore_data *data)
    {
    struct i2c_client *client = data.client;
    int ret;
    struct i2c_msg msg = {
    .addr = client.addr,
    .flags = client.flags | I2C_M_RD,
    .len = AMS_IAQCORE_DATA_SIZE,
    .buf = (char *) &data.buffer,
    };
    ret = i2c_transfer(client.adapter, &msg, 1);
    return (ret == AMS_IAQCORE_DATA_SIZE) ? 0 : ret;
    }
#[no_mangle]
unsafe extern "C" fn ams_iaqcore_get_measurement(data: *mut ams_iaqcore_data) -> c_int {
    static int ams_iaqcore_get_measurement(struct ams_iaqcore_data *data)
    {
    int ret;
// sensor can only be polled once a second max per datasheet
    if (!time_after(jiffies, data.last_update + HZ))
    return 0;
    ret = ams_iaqcore_read_measurement(data);
    if (ret < 0)
    return ret;
    data.last_update = jiffies;
    return 0;
    }
    static int ams_iaqcore_read_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan, int *val,
    int *val2, long mask)
    {
    struct ams_iaqcore_data *data = iio_priv(indio_dev);
    int ret;
    if (mask != IIO_CHAN_INFO_PROCESSED)
    return -EINVAL;
    mutex_lock(&data.lock);
    ret = ams_iaqcore_get_measurement(data);
    if (ret)
    goto err_out;
    switch (chan.address) {
    case AMS_IAQCORE_VOC_CO2_IDX:
// val = 0;
// val2 = be16_to_cpu(data->buffer.co2_ppm);
    ret = IIO_VAL_INT_PLUS_MICRO;
    break;
    case AMS_IAQCORE_VOC_RESISTANCE_IDX:
// val = be32_to_cpu(data->buffer.resistance);
    ret = IIO_VAL_INT;
    break;
    case AMS_IAQCORE_VOC_TVOC_IDX:
// val = 0;
// val2 = be16_to_cpu(data->buffer.voc_ppb);
    ret = IIO_VAL_INT_PLUS_NANO;
    break;
    default:
    ret = -EINVAL;
    }
    err_out:
    mutex_unlock(&data.lock);
    return ret;
    }
    static const struct iio_info ams_iaqcore_info = {
    .read_raw	= ams_iaqcore_read_raw,
    };
#[no_mangle]
unsafe extern "C" fn ams_iaqcore_probe(client: *mut i2c_client) -> c_int {
    static int ams_iaqcore_probe(struct i2c_client *client)
    {
    struct iio_dev *indio_dev;
    struct ams_iaqcore_data *data;
    indio_dev = devm_iio_device_alloc(&client.dev, sizeof(*data));
    if (!indio_dev)
    return -ENOMEM;
    data = iio_priv(indio_dev);
    i2c_set_clientdata(client, indio_dev);
    data.client = client;
// so initial reading will complete
    data.last_update = jiffies - HZ;
    mutex_init(&data.lock);
    indio_dev.info = &ams_iaqcore_info;
    indio_dev.name = dev_name(&client.dev);
    indio_dev.modes = INDIO_DIRECT_MODE;
    indio_dev.channels = ams_iaqcore_channels;
    indio_dev.num_channels = ARRAY_SIZE(ams_iaqcore_channels);
    return devm_iio_device_register(&client.dev, indio_dev);
    }
    static const struct i2c_device_id ams_iaqcore_id[] = {
    { .name = "ams-iaq-core" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, ams_iaqcore_id);
    static const struct of_device_id ams_iaqcore_dt_ids[] = {
    { .compatible = "ams,iaq-core" },
    { }
    };
    MODULE_DEVICE_TABLE(of, ams_iaqcore_dt_ids);
    static struct i2c_driver ams_iaqcore_driver = {
    .driver = {
    .name	= "ams-iaq-core",
    .of_match_table = ams_iaqcore_dt_ids,
    },
    .probe = ams_iaqcore_probe,
    .id_table = ams_iaqcore_id,
    };
    module_i2c_driver(ams_iaqcore_driver);
    MODULE_AUTHOR("Matt Ranostay <matt.ranostay@konsulko.com>");
    MODULE_DESCRIPTION("AMS iAQ-Core VOC sensors");
    MODULE_LICENSE("GPL v2");
