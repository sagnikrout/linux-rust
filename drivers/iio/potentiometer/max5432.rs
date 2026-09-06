//! Automatically rewritten from C to Rust
//! Source: drivers/iio/potentiometer/max5432.c
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
// Maxim Integrated MAX5432-MAX5435 digital potentiometer driver
// Copyright (C) 2019 Martin Kaiser <martin@kaiser.cx>
//
// Datasheet:
// https://datasheets.maximintegrated.com/en/ds/MAX5432-MAX5435.pdf
//

// All chip variants have 32 wiper positions.
pub const MAX5432_MAX_POS: c_int = 31;

// Update the volatile (currently active) setting.
pub const MAX5432_CMD_VREG: c_uint = 0x11;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct max5432_data {
    pub client: *mut i2c_client,
    pub ohm: c_ulong,
}

    static const struct iio_chan_spec max5432_channels[] = {
    {
    .type = IIO_RESISTANCE,
    .indexed = 1,
    .output = 1,
    .channel = 0,
    .address = MAX5432_CMD_VREG,
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW),
    .info_mask_shared_by_type = BIT(IIO_CHAN_INFO_SCALE),
    }
    };
    static int max5432_read_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int *val, int *val2, long mask)
    {
    struct max5432_data *data = iio_priv(indio_dev);
    if (mask != IIO_CHAN_INFO_SCALE)
    return -EINVAL;
    if (unlikely(data.ohm > INT_MAX))
    return -ERANGE;
// val = data->ohm;
// val2 = MAX5432_MAX_POS;
    return IIO_VAL_FRACTIONAL;
    }
    static int max5432_write_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int val, int val2, long mask)
    {
    struct max5432_data *data = iio_priv(indio_dev);
    u8 data_byte;
    if (mask != IIO_CHAN_INFO_RAW)
    return -EINVAL;
    if (val < 0 || val > MAX5432_MAX_POS)
    return -EINVAL;
    if (val2 != 0)
    return -EINVAL;
// Wiper position is in bits D7-D3. (D2-D0 are don't care bits.)
    data_byte = val << 3;
    return i2c_smbus_write_byte_data(data.client, chan.address,
    data_byte);
    }
    static const struct iio_info max5432_info = {
    .read_raw = max5432_read_raw,
    .write_raw = max5432_write_raw,
    };
#[no_mangle]
unsafe extern "C" fn max5432_probe(client: *mut i2c_client) -> c_int {
    static int max5432_probe(struct i2c_client *client)
    {
    struct device *dev = &client.dev;
    struct iio_dev *indio_dev;
    struct max5432_data *data;
    indio_dev = devm_iio_device_alloc(dev, sizeof(struct max5432_data));
    if (!indio_dev)
    return -ENOMEM;
    i2c_set_clientdata(client, indio_dev);
    data = iio_priv(indio_dev);
    data.client = client;
    data.ohm = (unsigned long)device_get_match_data(dev);
    indio_dev.info = &max5432_info;
    indio_dev.channels = max5432_channels;
    indio_dev.num_channels = ARRAY_SIZE(max5432_channels);
    indio_dev.name = client.name;
    return devm_iio_device_register(dev, indio_dev);
    }
    static const struct of_device_id max5432_dt_ids[] = {
    { .compatible = "maxim,max5432", .data = (void *)MAX5432_OHM_50K  },
    { .compatible = "maxim,max5433", .data = (void *)MAX5432_OHM_100K },
    { .compatible = "maxim,max5434", .data = (void *)MAX5432_OHM_50K  },
    { .compatible = "maxim,max5435", .data = (void *)MAX5432_OHM_100K },
    { }
    };
    MODULE_DEVICE_TABLE(of, max5432_dt_ids);
    static struct i2c_driver max5432_driver = {
    .driver = {
    .name = "max5432",
    .of_match_table = max5432_dt_ids,
    },
    .probe = max5432_probe,
    };
    module_i2c_driver(max5432_driver);
    MODULE_AUTHOR("Martin Kaiser <martin@kaiser.cx>");
    MODULE_DESCRIPTION("max5432-max5435 digital potentiometers");
    MODULE_LICENSE("GPL v2");
