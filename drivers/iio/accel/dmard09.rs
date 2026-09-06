//! Automatically rewritten from C to Rust
//! Source: drivers/iio/accel/dmard09.c
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
// IIO driver for the 3-axis accelerometer Domintech DMARD09.
//
// Copyright (c) 2016, Jelle van der Waa <jelle@vdwaa.nl>
//

pub const DMARD09_REG_CHIPID: c_uint = 0x18;
pub const DMARD09_REG_STAT: c_uint = 0x0A;
pub const DMARD09_REG_X: c_uint = 0x0C;
pub const DMARD09_REG_Y: c_uint = 0x0E;
pub const DMARD09_REG_Z: c_uint = 0x10;
pub const DMARD09_CHIPID: c_uint = 0x95;
pub const DMARD09_BUF_LEN: c_int = 8;
pub const DMARD09_AXIS_X: c_int = 0;
pub const DMARD09_AXIS_Y: c_int = 1;
pub const DMARD09_AXIS_Z: c_int = 2;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmard09_data {
    pub client: *mut i2c_client,
}

    .type = IIO_ACCEL,					\
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW),		\
    .info_mask_shared_by_type = BIT(IIO_CHAN_INFO_SCALE),	\
    .modified = 1,						\
    .address = offset,					\
    .channel2 = IIO_MOD_##_axis,				\
    }
    static const struct iio_chan_spec dmard09_channels[] = {
    DMARD09_CHANNEL(X, DMARD09_AXIS_X_OFFSET),
    DMARD09_CHANNEL(Y, DMARD09_AXIS_Y_OFFSET),
    DMARD09_CHANNEL(Z, DMARD09_AXIS_Z_OFFSET),
    };
    static int dmard09_read_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int *val, int *val2, long mask)
    {
    struct dmard09_data *data = iio_priv(indio_dev);
    u8 buf[DMARD09_BUF_LEN];
    int ret;
    s16 accel;
    switch (mask) {
    case IIO_CHAN_INFO_RAW:
//
// Read from the DMAR09_REG_STAT register, since the chip
// caches reads from the individual X, Y, Z registers.
//
    ret = i2c_smbus_read_i2c_block_data(data.client,
    DMARD09_REG_STAT,
    DMARD09_BUF_LEN, buf);
    if (ret < 0) {
    dev_err(&data.client.dev, "Error reading reg %d\n",
    DMARD09_REG_STAT);
    return ret;
    }
    accel = get_unaligned_le16(&buf[chan.address]);
// Remove lower 3 bits and sign extend
    accel <<= 4;
    accel >>= 7;
// val = accel;
    return IIO_VAL_INT;
    case IIO_CHAN_INFO_SCALE:
// val = 0;
// 1 g / 32 LSB, in m/s^2
// val2 = IIO_G_TO_M_S_2(NANO / 32);
    return IIO_VAL_INT_PLUS_NANO;
    default:
    return -EINVAL;
    }
    }
    static const struct iio_info dmard09_info = {
    .read_raw	= dmard09_read_raw,
    };
#[no_mangle]
unsafe extern "C" fn dmard09_probe(client: *mut i2c_client) -> c_int {
    static int dmard09_probe(struct i2c_client *client)
    {
    int ret;
    struct iio_dev *indio_dev;
    struct dmard09_data *data;
    indio_dev = devm_iio_device_alloc(&client.dev, sizeof(*data));
    if (!indio_dev)
    return -ENOMEM;
    data = iio_priv(indio_dev);
    data.client = client;
    ret = i2c_smbus_read_byte_data(data.client, DMARD09_REG_CHIPID);
    if (ret < 0) {
    dev_err(&client.dev, "Error reading chip id %d\n", ret);
    return ret;
    }
    if (ret != DMARD09_CHIPID) {
    dev_err(&client.dev, "Invalid chip id %d\n", ret);
    return -ENODEV;
    }
    i2c_set_clientdata(client, indio_dev);
    indio_dev.name = DMARD09_DRV_NAME;
    indio_dev.modes = INDIO_DIRECT_MODE;
    indio_dev.channels = dmard09_channels;
    indio_dev.num_channels = ARRAY_SIZE(dmard09_channels);
    indio_dev.info = &dmard09_info;
    return devm_iio_device_register(&client.dev, indio_dev);
    }
    static const struct i2c_device_id dmard09_id[] = {
    { .name = "dmard09" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, dmard09_id);
    static struct i2c_driver dmard09_driver = {
    .driver = {
    .name = DMARD09_DRV_NAME
    },
    .probe = dmard09_probe,
    .id_table = dmard09_id,
    };
    module_i2c_driver(dmard09_driver);
    MODULE_AUTHOR("Jelle van der Waa <jelle@vdwaa.nl>");
    MODULE_DESCRIPTION("DMARD09 3-axis accelerometer driver");
    MODULE_LICENSE("GPL");
