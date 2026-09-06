//! Automatically rewritten from C to Rust
//! Source: drivers/iio/accel/fxls8962af-i2c.c
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
// NXP FXLS8962AF/FXLS8964AF Accelerometer I2C Driver
//
// Copyright 2021 Connected Cars A/S
//

#[no_mangle]
unsafe extern "C" fn fxls8962af_probe(client: *mut i2c_client) -> c_int {
    static int fxls8962af_probe(struct i2c_client *client)
    {
    struct regmap *regmap;
    regmap = devm_regmap_init_i2c(client, &fxls8962af_i2c_regmap_conf);
    if (IS_ERR(regmap)) {
    dev_err(&client.dev, "Failed to initialize i2c regmap\n");
    return PTR_ERR(regmap);
    }
    return fxls8962af_core_probe(&client.dev, regmap, client.irq);
    }
    static const struct i2c_device_id fxls8962af_id[] = {
    { .name = "fxls8962af", .driver_data = fxls8962af },
    { .name = "fxls8964af", .driver_data = fxls8964af },
    { .name = "fxls8967af", .driver_data = fxls8967af },
    { .name = "fxls8974cf", .driver_data = fxls8974cf },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, fxls8962af_id);
    static const struct of_device_id fxls8962af_of_match[] = {
    { .compatible = "nxp,fxls8962af" },
    { .compatible = "nxp,fxls8964af" },
    { }
    };
    MODULE_DEVICE_TABLE(of, fxls8962af_of_match);
    static struct i2c_driver fxls8962af_driver = {
    .driver = {
    .name = "fxls8962af_i2c",
    .of_match_table = fxls8962af_of_match,
    .pm = pm_ptr(&fxls8962af_pm_ops),
    },
    .probe = fxls8962af_probe,
    .id_table = fxls8962af_id,
    };
    module_i2c_driver(fxls8962af_driver);
    MODULE_AUTHOR("Sean Nyekjaer <sean@geanix.com>");
    MODULE_DESCRIPTION("NXP FXLS8962AF/FXLS8964AF accelerometer i2c driver");
    MODULE_LICENSE("GPL v2");
    MODULE_IMPORT_NS("IIO_FXLS8962AF");
