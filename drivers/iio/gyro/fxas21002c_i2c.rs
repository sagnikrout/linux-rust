//! Automatically rewritten from C to Rust
//! Source: drivers/iio/gyro/fxas21002c_i2c.c
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
// Driver for NXP FXAS21002C Gyroscope - I2C
//
// Copyright (C) 2018 Linaro Ltd.
//

    static const struct regmap_config fxas21002c_regmap_i2c_conf = {
    .reg_bits = 8,
    .val_bits = 8,
    .max_register = FXAS21002C_REG_CTRL3,
    };
#[no_mangle]
unsafe extern "C" fn fxas21002c_i2c_probe(i2c: *mut i2c_client) -> c_int {
    static int fxas21002c_i2c_probe(struct i2c_client *i2c)
    {
    struct regmap *regmap;
    regmap = devm_regmap_init_i2c(i2c, &fxas21002c_regmap_i2c_conf);
    if (IS_ERR(regmap)) {
    dev_err(&i2c.dev, "Failed to register i2c regmap: %ld\n",
    PTR_ERR(regmap));
    return PTR_ERR(regmap);
    }
    return fxas21002c_core_probe(&i2c.dev, regmap, i2c.irq, i2c.name);
    }
#[no_mangle]
unsafe extern "C" fn fxas21002c_i2c_remove(i2c: *mut i2c_client) {
    static void fxas21002c_i2c_remove(struct i2c_client *i2c)
    {
    fxas21002c_core_remove(&i2c.dev);
    }
    static const struct i2c_device_id fxas21002c_i2c_id[] = {
    { .name = "fxas21002c" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, fxas21002c_i2c_id);
    static const struct of_device_id fxas21002c_i2c_of_match[] = {
    { .compatible = "nxp,fxas21002c", },
    { }
    };
    MODULE_DEVICE_TABLE(of, fxas21002c_i2c_of_match);
    static struct i2c_driver fxas21002c_i2c_driver = {
    .driver = {
    .name = "fxas21002c_i2c",
    .pm = pm_ptr(&fxas21002c_pm_ops),
    .of_match_table = fxas21002c_i2c_of_match,
    },
    .probe		= fxas21002c_i2c_probe,
    .remove		= fxas21002c_i2c_remove,
    .id_table	= fxas21002c_i2c_id,
    };
    module_i2c_driver(fxas21002c_i2c_driver);
    MODULE_AUTHOR("Rui Miguel Silva <rui.silva@linaro.org>");
    MODULE_LICENSE("GPL v2");
    MODULE_DESCRIPTION("FXAS21002C I2C Gyro driver");
    MODULE_IMPORT_NS("IIO_FXAS21002C");
