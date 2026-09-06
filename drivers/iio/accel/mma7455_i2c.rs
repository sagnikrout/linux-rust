//! Automatically rewritten from C to Rust
//! Source: drivers/iio/accel/mma7455_i2c.c
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
// IIO accel I2C driver for Freescale MMA7455L 3-axis 10-bit accelerometer
// Copyright 2015 Joachim Eastwood <manabian@gmail.com>
//

#[no_mangle]
unsafe extern "C" fn mma7455_i2c_probe(i2c: *mut i2c_client) -> c_int {
    static int mma7455_i2c_probe(struct i2c_client *i2c)
    {
    const struct i2c_device_id *id = i2c_client_get_device_id(i2c);
    struct regmap *regmap;
    const char *name = core::ptr::null_mut();
    regmap = devm_regmap_init_i2c(i2c, &mma7455_core_regmap);
    if (IS_ERR(regmap))
    return PTR_ERR(regmap);
    if (id)
    name = id.name;
    return mma7455_core_probe(&i2c.dev, regmap, name);
    }
#[no_mangle]
unsafe extern "C" fn mma7455_i2c_remove(i2c: *mut i2c_client) {
    static void mma7455_i2c_remove(struct i2c_client *i2c)
    {
    mma7455_core_remove(&i2c.dev);
    }
    static const struct i2c_device_id mma7455_i2c_ids[] = {
    { .name = "mma7455" },
    { .name = "mma7456" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, mma7455_i2c_ids);
    static const struct of_device_id mma7455_of_match[] = {
    { .compatible = "fsl,mma7455" },
    { .compatible = "fsl,mma7456" },
    { }
    };
    MODULE_DEVICE_TABLE(of, mma7455_of_match);
    static struct i2c_driver mma7455_i2c_driver = {
    .probe = mma7455_i2c_probe,
    .remove = mma7455_i2c_remove,
    .id_table = mma7455_i2c_ids,
    .driver = {
    .name	= "mma7455-i2c",
    .of_match_table = mma7455_of_match,
    },
    };
    module_i2c_driver(mma7455_i2c_driver);
    MODULE_AUTHOR("Joachim Eastwood <manabian@gmail.com>");
    MODULE_DESCRIPTION("Freescale MMA7455L I2C accelerometer driver");
    MODULE_LICENSE("GPL v2");
    MODULE_IMPORT_NS("IIO_MMA7455");
