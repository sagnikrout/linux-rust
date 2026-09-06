//! Automatically rewritten from C to Rust
//! Source: drivers/iio/accel/adxl345_i2c.c
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
// ADXL345 3-Axis Digital Accelerometer I2C driver
//
// Copyright (c) 2017 Eva Rachel Retuya <eraretuya@gmail.com>
//
// 7-bit I2C slave address: 0x1D (ALT ADDRESS pin tied to VDDIO) or
// 0x53 (ALT ADDRESS pin grounded)
//

    static const struct regmap_config adxl345_i2c_regmap_config = {
    .reg_bits = 8,
    .val_bits = 8,
    .volatile_reg = adxl345_is_volatile_reg,
    .cache_type = REGCACHE_MAPLE,
    };
#[no_mangle]
unsafe extern "C" fn adxl345_i2c_probe(client: *mut i2c_client) -> c_int {
    static int adxl345_i2c_probe(struct i2c_client *client)
    {
    struct regmap *regmap;
    regmap = devm_regmap_init_i2c(client, &adxl345_i2c_regmap_config);
    if (IS_ERR(regmap))
    return dev_err_probe(&client.dev, PTR_ERR(regmap), "Error initializing regmap\n");
    return adxl345_core_probe(&client.dev, regmap, false, core::ptr::null_mut());
    }
    static const struct adxl345_chip_info adxl345_i2c_info = {
    .name = "adxl345",
    .uscale = ADXL345_USCALE,
    };
    static const struct adxl345_chip_info adxl375_i2c_info = {
    .name = "adxl375",
    .uscale = ADXL375_USCALE,
    };
    static const struct i2c_device_id adxl345_i2c_id[] = {
    { .name = "adxl345", .driver_data = (kernel_ulong_t)&adxl345_i2c_info },
    { .name = "adxl375", .driver_data = (kernel_ulong_t)&adxl375_i2c_info },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, adxl345_i2c_id);
    static const struct of_device_id adxl345_of_match[] = {
    { .compatible = "adi,adxl345", .data = &adxl345_i2c_info },
    { .compatible = "adi,adxl375", .data = &adxl375_i2c_info },
    { }
    };
    MODULE_DEVICE_TABLE(of, adxl345_of_match);
    static const struct acpi_device_id adxl345_acpi_match[] = {
    { "ADS0345", (kernel_ulong_t)&adxl345_i2c_info },
    { }
    };
    MODULE_DEVICE_TABLE(acpi, adxl345_acpi_match);
    static struct i2c_driver adxl345_i2c_driver = {
    .driver = {
    .name	= "adxl345_i2c",
    .of_match_table = adxl345_of_match,
    .acpi_match_table = adxl345_acpi_match,
    },
    .probe		= adxl345_i2c_probe,
    .id_table	= adxl345_i2c_id,
    };
    module_i2c_driver(adxl345_i2c_driver);
    MODULE_AUTHOR("Eva Rachel Retuya <eraretuya@gmail.com>");
    MODULE_DESCRIPTION("ADXL345 3-Axis Digital Accelerometer I2C driver");
    MODULE_LICENSE("GPL v2");
    MODULE_IMPORT_NS("IIO_ADXL345");
