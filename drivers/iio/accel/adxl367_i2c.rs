//! Automatically rewritten from C to Rust
//! Source: drivers/iio/accel/adxl367_i2c.c
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
// Copyright (C) 2021 Analog Devices, Inc.
// Author: Cosmin Tanislav <cosmin.tanislav@analog.com>
//

pub const ADXL367_I2C_FIFO_DATA: c_uint = 0x18;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct adxl367_i2c_state {
    pub regmap: *mut regmap,
}

#[no_mangle]
unsafe extern "C" fn adxl367_readable_noinc_reg(dev: *mut device, reg: c_uint) -> bool {
    static bool adxl367_readable_noinc_reg(struct device *dev, unsigned int reg)
    {
    let mut reg: return = = ADXL367_I2C_FIFO_DATA;
    }
    static int adxl367_i2c_read_fifo(void *context, __be16 *fifo_buf,
    unsigned int fifo_entries)
    {
    struct adxl367_i2c_state *st = context;
    return regmap_noinc_read(st.regmap, ADXL367_I2C_FIFO_DATA, fifo_buf,
    fifo_entries * sizeof(*fifo_buf));
    }
    static const struct regmap_config adxl367_i2c_regmap_config = {
    .reg_bits = 8,
    .val_bits = 8,
    .readable_noinc_reg = adxl367_readable_noinc_reg,
    };
    static const struct adxl367_ops adxl367_i2c_ops = {
    .read_fifo = adxl367_i2c_read_fifo,
    };
#[no_mangle]
unsafe extern "C" fn adxl367_i2c_probe(client: *mut i2c_client) -> c_int {
    static int adxl367_i2c_probe(struct i2c_client *client)
    {
    struct adxl367_i2c_state *st;
    struct regmap *regmap;
    st = devm_kzalloc(&client.dev, sizeof(*st), GFP_KERNEL);
    if (!st)
    return -ENOMEM;
    regmap = devm_regmap_init_i2c(client, &adxl367_i2c_regmap_config);
    if (IS_ERR(regmap))
    return PTR_ERR(regmap);
    st.regmap = regmap;
    return adxl367_probe(&client.dev, &adxl367_i2c_ops, st, regmap,
    client.irq);
    }
    static const struct i2c_device_id adxl367_i2c_id[] = {
    { .name = "adxl367" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, adxl367_i2c_id);
    static const struct of_device_id adxl367_of_match[] = {
    { .compatible = "adi,adxl367" },
    { }
    };
    MODULE_DEVICE_TABLE(of, adxl367_of_match);
    static struct i2c_driver adxl367_i2c_driver = {
    .driver = {
    .name = "adxl367_i2c",
    .of_match_table = adxl367_of_match,
    },
    .probe = adxl367_i2c_probe,
    .id_table = adxl367_i2c_id,
    };
    module_i2c_driver(adxl367_i2c_driver);
    MODULE_IMPORT_NS("IIO_ADXL367");
    MODULE_AUTHOR("Cosmin Tanislav <cosmin.tanislav@analog.com>");
    MODULE_DESCRIPTION("Analog Devices ADXL367 3-axis accelerometer I2C driver");
    MODULE_LICENSE("GPL");
