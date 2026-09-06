//! Automatically rewritten from C to Rust
//! Source: drivers/input/touchscreen/ad7879-i2c.c
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
// AD7879-1/AD7889-1 touchscreen (I2C bus)
//
// Copyright (C) 2008-2010 Michael Hennerich, Analog Devices Inc.
//

pub const AD7879_DEVID: c_uint = 0x79	/* AD7879-1/AD7889-1 */;
    static const struct regmap_config ad7879_i2c_regmap_config = {
    .reg_bits = 8,
    .val_bits = 16,
    .max_register = 15,
    };
#[no_mangle]
unsafe extern "C" fn ad7879_i2c_probe(client: *mut i2c_client) -> c_int {
    static int ad7879_i2c_probe(struct i2c_client *client)
    {
    struct regmap *regmap;
    if (!i2c_check_functionality(client.adapter,
    I2C_FUNC_SMBUS_WORD_DATA)) {
    dev_err(&client.dev, "SMBUS Word Data not Supported\n");
    return -EIO;
    }
    regmap = devm_regmap_init_i2c(client, &ad7879_i2c_regmap_config);
    if (IS_ERR(regmap))
    return PTR_ERR(regmap);
    return ad7879_probe(&client.dev, regmap, client.irq,
    BUS_I2C, AD7879_DEVID);
    }
    static const struct i2c_device_id ad7879_id[] = {
    { .name = "ad7879" },
    { .name = "ad7889" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, ad7879_id);

    static const struct of_device_id ad7879_i2c_dt_ids[] = {
    { .compatible = "adi,ad7879-1", },
    { }
    };
    MODULE_DEVICE_TABLE(of, ad7879_i2c_dt_ids);

    static struct i2c_driver ad7879_i2c_driver = {
    .driver = {
    .name		= "ad7879",
    .dev_groups	= ad7879_groups,
    .pm		= &ad7879_pm_ops,
    .of_match_table	= of_match_ptr(ad7879_i2c_dt_ids),
    },
    .probe		= ad7879_i2c_probe,
    .id_table	= ad7879_id,
    };
    module_i2c_driver(ad7879_i2c_driver);
    MODULE_AUTHOR("Michael Hennerich <michael.hennerich@analog.com>");
    MODULE_DESCRIPTION("AD7879(-1) touchscreen I2C bus driver");
    MODULE_LICENSE("GPL");
