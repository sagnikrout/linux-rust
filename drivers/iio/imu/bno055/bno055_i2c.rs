//! Automatically rewritten from C to Rust
//! Source: drivers/iio/imu/bno055/bno055_i2c.c
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
// Support for I2C-interfaced Bosch BNO055 IMU.
//
// Copyright (C) 2021-2022 Istituto Italiano di Tecnologia
// Electronic Design Laboratory
// Written by Andrea Merello <andrea.merello@iit.it>
//

pub const BNO055_I2C_XFER_BURST_BREAK_THRESHOLD: c_int = 3;
#[no_mangle]
unsafe extern "C" fn bno055_i2c_probe(client: *mut i2c_client) -> c_int {
    static int bno055_i2c_probe(struct i2c_client *client)
    {
    struct regmap *regmap;
    regmap = devm_regmap_init_i2c(client, &bno055_regmap_config);
    if (IS_ERR(regmap))
    return dev_err_probe(&client.dev, PTR_ERR(regmap),
    "Unable to init register map");
    return bno055_probe(&client.dev, regmap,
    BNO055_I2C_XFER_BURST_BREAK_THRESHOLD, true);
    }
    static const struct i2c_device_id bno055_i2c_id[] = {
    { .name = "bno055" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, bno055_i2c_id);
    static const struct of_device_id __maybe_unused bno055_i2c_of_match[] = {
    { .compatible = "bosch,bno055" },
    { }
    };
    MODULE_DEVICE_TABLE(of, bno055_i2c_of_match);
    static struct i2c_driver bno055_driver = {
    .driver = {
    .name = "bno055-i2c",
    .of_match_table = bno055_i2c_of_match,
    },
    .probe = bno055_i2c_probe,
    .id_table = bno055_i2c_id,
    };
    module_i2c_driver(bno055_driver);
    MODULE_AUTHOR("Andrea Merello");
    MODULE_DESCRIPTION("Bosch BNO055 I2C interface");
    MODULE_IMPORT_NS("IIO_BNO055");
    MODULE_LICENSE("GPL");
