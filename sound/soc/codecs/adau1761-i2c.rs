//! Automatically rewritten from C to Rust
//! Source: sound/soc/codecs/adau1761-i2c.c
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
// Driver for ADAU1361/ADAU1461/ADAU1761/ADAU1961 codec
//
// Copyright 2014 Analog Devices Inc.
// Author: Lars-Peter Clausen <lars@metafoo.de>
//

#[no_mangle]
unsafe extern "C" fn adau1761_i2c_probe(client: *mut i2c_client) -> c_int {
    static int adau1761_i2c_probe(struct i2c_client *client)
    {
    struct regmap_config config;
    config = adau1761_regmap_config;
    config.val_bits = 8;
    config.reg_bits = 16;
    return adau1761_probe(&client.dev,
    devm_regmap_init_i2c(client, &config),
    (uintptr_t)i2c_get_match_data(client), core::ptr::null_mut());
    }
#[no_mangle]
unsafe extern "C" fn adau1761_i2c_remove(client: *mut i2c_client) {
    static void adau1761_i2c_remove(struct i2c_client *client)
    {
    adau17x1_remove(&client.dev);
    }
    static const struct i2c_device_id adau1761_i2c_ids[] = {
    { .name = "adau1361", .driver_data = ADAU1361 },
    { .name = "adau1461", .driver_data = ADAU1761 },
    { .name = "adau1761", .driver_data = ADAU1761 },
    { .name = "adau1961", .driver_data = ADAU1361 },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, adau1761_i2c_ids);

    static const struct of_device_id adau1761_i2c_dt_ids[] = {
    { .compatible = "adi,adau1361", },
    { .compatible = "adi,adau1461", },
    { .compatible = "adi,adau1761", },
    { .compatible = "adi,adau1961", },
    { },
    };
    MODULE_DEVICE_TABLE(of, adau1761_i2c_dt_ids);

    static struct i2c_driver adau1761_i2c_driver = {
    .driver = {
    .name = "adau1761",
    .of_match_table = of_match_ptr(adau1761_i2c_dt_ids),
    },
    .probe = adau1761_i2c_probe,
    .remove = adau1761_i2c_remove,
    .id_table = adau1761_i2c_ids,
    };
    module_i2c_driver(adau1761_i2c_driver);
    MODULE_DESCRIPTION("ASoC ADAU1361/ADAU1461/ADAU1761/ADAU1961 CODEC I2C driver");
    MODULE_AUTHOR("Lars-Peter Clausen <lars@metafoo.de>");
    MODULE_LICENSE("GPL");
