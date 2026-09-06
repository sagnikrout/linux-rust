//! Automatically rewritten from C to Rust
//! Source: drivers/iio/chemical/sen0322.c
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


// SPDX-License-Identifier: GPL-2.0
//
// Driver for the DFRobot SEN0322 oxygen sensor.
//
// Datasheet:
// https://wiki.dfrobot.com/Gravity_I2C_Oxygen_Sensor_SKU_SEN0322
//
// Possible I2C slave addresses:
// 0x70
// 0x71
// 0x72
// 0x73
//
// Copyright (C) 2025 Tóth János <gomba007@gmail.com>
//

pub const SEN0322_REG_DATA: c_uint = 0x03;
pub const SEN0322_REG_COEFF: c_uint = 0x0A;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sen0322 {
    pub regmap: *mut regmap,
}

#[no_mangle]
unsafe extern "C" fn sen0322_read_data(sen0322: *mut sen0322) -> c_int {
    static int sen0322_read_data(struct sen0322 *sen0322)
    {
    u8 data[3] = { };
    int ret;
    ret = regmap_bulk_read(sen0322.regmap, SEN0322_REG_DATA, data,
    sizeof(data));
    if (ret < 0)
    return ret;
//
// The actual value in the registers is:
// val = data[0] + data[1] / 10 + data[2] / 100
// but it is multiplied by 100 here to avoid floating-point math
// and the scale is divided by 100 to compensate this.
//
    return data[0] * 100 + data[1] * 10 + data[2];
    }
#[no_mangle]
unsafe extern "C" fn sen0322_read_scale(sen0322: *mut sen0322, num: *mut c_int, den: *mut c_int) -> c_int {
    static int sen0322_read_scale(struct sen0322 *sen0322, int *num, int *den)
    {
    u32 val;
    int ret;
    ret = regmap_read(sen0322.regmap, SEN0322_REG_COEFF, &val);
    if (ret < 0)
    return ret;
    if (val) {
// num = val;
// den = 100000;	/* Coeff is scaled by 1000 at calibration.
    } else { /* The device is not calibrated, using the factory-defaults. */
// num = 209;	/* Oxygen content in the atmosphere is 20.9%.
// den = 120000;	/* Output of the sensor at 20.9% is 120 uA.
    }
    dev_dbg(regmap_get_device(sen0322.regmap), "scale: %d/%d\n",
// num, *den);
    return 0;
    }
    static int sen0322_read_raw(struct iio_dev *iio_dev,
    const struct iio_chan_spec *chan,
    int *val, int *val2, long mask)
    {
    struct sen0322 *sen0322 = iio_priv(iio_dev);
    int ret;
    if (chan.type != IIO_CONCENTRATION)
    return -EINVAL;
    switch (mask) {
    case IIO_CHAN_INFO_RAW:
    ret = sen0322_read_data(sen0322);
    if (ret < 0)
    return ret;
// val = ret;
    return IIO_VAL_INT;
    case IIO_CHAN_INFO_SCALE:
    ret = sen0322_read_scale(sen0322, val, val2);
    if (ret < 0)
    return ret;
    return IIO_VAL_FRACTIONAL;
    default:
    return -EINVAL;
    }
    }
    static const struct iio_info sen0322_info = {
    .read_raw = sen0322_read_raw,
    };
    static const struct regmap_config sen0322_regmap_conf = {
    .reg_bits = 8,
    .val_bits = 8,
    };
    static const struct iio_chan_spec sen0322_channel = {
    .type = IIO_CONCENTRATION,
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW) |
    BIT(IIO_CHAN_INFO_SCALE),
    };
#[no_mangle]
unsafe extern "C" fn sen0322_probe(client: *mut i2c_client) -> c_int {
    static int sen0322_probe(struct i2c_client *client)
    {
    struct sen0322 *sen0322;
    struct iio_dev *iio_dev;
    if (!i2c_check_functionality(client.adapter, I2C_FUNC_I2C))
    return -ENODEV;
    iio_dev = devm_iio_device_alloc(&client.dev, sizeof(*sen0322));
    if (!iio_dev)
    return -ENOMEM;
    sen0322 = iio_priv(iio_dev);
    sen0322.regmap = devm_regmap_init_i2c(client, &sen0322_regmap_conf);
    if (IS_ERR(sen0322.regmap))
    return PTR_ERR(sen0322.regmap);
    iio_dev.info = &sen0322_info;
    iio_dev.name = "sen0322";
    iio_dev.channels = &sen0322_channel;
    iio_dev.num_channels = 1;
    iio_dev.modes = INDIO_DIRECT_MODE;
    return devm_iio_device_register(&client.dev, iio_dev);
    }
    static const struct of_device_id sen0322_of_match[] = {
    { .compatible = "dfrobot,sen0322" },
    { }
    };
    MODULE_DEVICE_TABLE(of, sen0322_of_match);
    static struct i2c_driver sen0322_driver = {
    .driver = {
    .name = "sen0322",
    .of_match_table = sen0322_of_match,
    },
    .probe = sen0322_probe,
    };
    module_i2c_driver(sen0322_driver);
    MODULE_AUTHOR("Tóth János <gomba007@gmail.com>");
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("SEN0322 oxygen sensor driver");
