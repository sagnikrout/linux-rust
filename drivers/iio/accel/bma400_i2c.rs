//! Automatically rewritten from C to Rust
//! Source: drivers/iio/accel/bma400_i2c.c
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
// I2C IIO driver for Bosch BMA400 triaxial acceleration sensor.
//
// Copyright 2019 Dan Robertson <dan@dlrobertson.com>
//
// I2C address is either 0x14 or 0x15 depending on SDO
//

#[no_mangle]
unsafe extern "C" fn bma400_i2c_probe(client: *mut i2c_client) -> c_int {
    static int bma400_i2c_probe(struct i2c_client *client)
    {
    const struct i2c_device_id *id = i2c_client_get_device_id(client);
    struct regmap *regmap;
    regmap = devm_regmap_init_i2c(client, &bma400_regmap_config);
    if (IS_ERR(regmap)) {
    dev_err(&client.dev, "failed to create regmap\n");
    return PTR_ERR(regmap);
    }
    return bma400_probe(&client.dev, regmap, client.irq, id.name);
    }
    static const struct i2c_device_id bma400_i2c_ids[] = {
    { .name = "bma400" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, bma400_i2c_ids);
    static const struct of_device_id bma400_of_i2c_match[] = {
    { .compatible = "bosch,bma400" },
    { }
    };
    MODULE_DEVICE_TABLE(of, bma400_of_i2c_match);
    static struct i2c_driver bma400_i2c_driver = {
    .driver = {
    .name = "bma400",
    .of_match_table = bma400_of_i2c_match,
    },
    .probe = bma400_i2c_probe,
    .id_table = bma400_i2c_ids,
    };
    module_i2c_driver(bma400_i2c_driver);
    MODULE_AUTHOR("Dan Robertson <dan@dlrobertson.com>");
    MODULE_DESCRIPTION("Bosch BMA400 triaxial acceleration sensor (I2C)");
    MODULE_LICENSE("GPL");
    MODULE_IMPORT_NS("IIO_BMA400");
