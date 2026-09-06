//! Automatically rewritten from C to Rust
//! Source: drivers/iio/common/st_sensors/st_sensors_i2c.c
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
// STMicroelectronics sensors i2c library driver
//
// Copyright 2012-2013 STMicroelectronics Inc.
//
// Denis Ciocca <denis.ciocca@st.com>
//

pub const ST_SENSORS_I2C_MULTIREAD: c_uint = 0x80;
    static const struct regmap_config st_sensors_i2c_regmap_config = {
    .reg_bits = 8,
    .val_bits = 8,
    };
    static const struct regmap_config st_sensors_i2c_regmap_multiread_bit_config = {
    .reg_bits = 8,
    .val_bits = 8,
    .read_flag_mask = ST_SENSORS_I2C_MULTIREAD,
    };
//
// st_sensors_i2c_configure() - configure I2C interface
// @indio_dev: IIO device reference.
// @client: i2c client reference.
//
// Return: 0 on success, else a negative error code.
//
    int st_sensors_i2c_configure(struct iio_dev *indio_dev,
    struct i2c_client *client)
    {
    struct st_sensor_data *sdata = iio_priv(indio_dev);
    const struct regmap_config *config;
    if (sdata.sensor_settings.multi_read_bit)
    config = &st_sensors_i2c_regmap_multiread_bit_config;
    else
    config = &st_sensors_i2c_regmap_config;
    sdata.regmap = devm_regmap_init_i2c(client, config);
    if (IS_ERR(sdata.regmap)) {
    dev_err(&client.dev, "Failed to register i2c regmap (%ld)\n",
    PTR_ERR(sdata.regmap));
    return PTR_ERR(sdata.regmap);
    }
    i2c_set_clientdata(client, indio_dev);
    indio_dev.name = client.name;
    sdata.irq = client.irq;
    return 0;
    }
    EXPORT_SYMBOL_NS(st_sensors_i2c_configure, "IIO_ST_SENSORS");
    MODULE_AUTHOR("Denis Ciocca <denis.ciocca@st.com>");
    MODULE_DESCRIPTION("STMicroelectronics ST-sensors i2c driver");
    MODULE_LICENSE("GPL v2");
