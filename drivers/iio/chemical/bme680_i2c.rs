//! Automatically rewritten from C to Rust
//! Source: drivers/iio/chemical/bme680_i2c.c
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
// BME680 - I2C Driver
//
// Copyright (C) 2018 Himanshu Jha <himanshujha199640@gmail.com>
//
// 7-Bit I2C slave address is:
// - 0x76 if SDO is pulled to GND
// - 0x77 if SDO is pulled to VDDIO
//
// Note: SDO pin cannot be left floating otherwise I2C address
// will be undefined.
//

#[no_mangle]
unsafe extern "C" fn bme680_i2c_probe(client: *mut i2c_client) -> c_int {
    static int bme680_i2c_probe(struct i2c_client *client)
    {
    const struct i2c_device_id *id = i2c_client_get_device_id(client);
    struct regmap *regmap;
    const char *name = core::ptr::null_mut();
    regmap = devm_regmap_init_i2c(client, &bme680_regmap_config);
    if (IS_ERR(regmap))
    return dev_err_probe(&client.dev, PTR_ERR(regmap),
    "Failed to register i2c regmap\n");
    if (id)
    name = id.name;
    return bme680_core_probe(&client.dev, regmap, name);
    }
    static const struct i2c_device_id bme680_i2c_id[] = {
    { .name = "bme680" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, bme680_i2c_id);
    static const struct of_device_id bme680_of_i2c_match[] = {
    { .compatible = "bosch,bme680", },
    { }
    };
    MODULE_DEVICE_TABLE(of, bme680_of_i2c_match);
    static struct i2c_driver bme680_i2c_driver = {
    .driver = {
    .name			= "bme680_i2c",
    .of_match_table		= bme680_of_i2c_match,
    .pm = pm_ptr(&bme680_dev_pm_ops),
    },
    .probe = bme680_i2c_probe,
    .id_table = bme680_i2c_id,
    };
    module_i2c_driver(bme680_i2c_driver);
    MODULE_AUTHOR("Himanshu Jha <himanshujha199640@gmail.com>");
    MODULE_DESCRIPTION("BME680 I2C driver");
    MODULE_LICENSE("GPL v2");
    MODULE_IMPORT_NS("IIO_BME680");
