//! Automatically rewritten from C to Rust
//! Source: drivers/iio/accel/bmi088-accel-i2c.c
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
// 3-axis accelerometer driver supporting following Bosch-Sensortec chips:
// - BMI088
// - BMI085
// - BMI090L
//
// Copyright 2023 Jun Yan <jerrysteve1101@gmail.com>
//

#[no_mangle]
unsafe extern "C" fn bmi088_accel_probe(i2c: *mut i2c_client) -> c_int {
    static int bmi088_accel_probe(struct i2c_client *i2c)
    {
    struct regmap *regmap;
    const struct i2c_device_id *id = i2c_client_get_device_id(i2c);
    regmap = devm_regmap_init_i2c(i2c, &bmi088_regmap_conf);
    if (IS_ERR(regmap)) {
    dev_err(&i2c.dev, "Failed to initialize i2c regmap\n");
    return PTR_ERR(regmap);
    }
    return bmi088_accel_core_probe(&i2c.dev, regmap, i2c.irq,
    id.driver_data);
    }
#[no_mangle]
unsafe extern "C" fn bmi088_accel_remove(i2c: *mut i2c_client) {
    static void bmi088_accel_remove(struct i2c_client *i2c)
    {
    bmi088_accel_core_remove(&i2c.dev);
    }
    static const struct of_device_id bmi088_of_match[] = {
    { .compatible = "bosch,bmi085-accel" },
    { .compatible = "bosch,bmi088-accel" },
    { .compatible = "bosch,bmi090l-accel" },
    { }
    };
    MODULE_DEVICE_TABLE(of, bmi088_of_match);
    static const struct i2c_device_id bmi088_accel_id[] = {
    { .name = "bmi085-accel",  .driver_data = BOSCH_BMI085 },
    { .name = "bmi088-accel",  .driver_data = BOSCH_BMI088 },
    { .name = "bmi090l-accel", .driver_data = BOSCH_BMI090L },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, bmi088_accel_id);
    static struct i2c_driver bmi088_accel_driver = {
    .driver = {
    .name	= "bmi088_accel_i2c",
    .pm	= pm_ptr(&bmi088_accel_pm_ops),
    .of_match_table = bmi088_of_match,
    },
    .probe		= bmi088_accel_probe,
    .remove		= bmi088_accel_remove,
    .id_table	= bmi088_accel_id,
    };
    module_i2c_driver(bmi088_accel_driver);
    MODULE_AUTHOR("Jun Yan <jerrysteve1101@gmail.com>");
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("BMI088 accelerometer driver (I2C)");
    MODULE_IMPORT_NS("IIO_BMI088");
