//! Automatically rewritten from C to Rust
//! Source: sound/soc/codecs/ad193x-i2c.c
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
// AD1936/AD1937 audio driver
//
// Copyright 2014 Analog Devices Inc.
//

    static const struct i2c_device_id ad193x_id[] = {
    { .name = "ad1936", .driver_data = AD193X },
    { .name = "ad1937", .driver_data = AD193X },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, ad193x_id);
#[no_mangle]
unsafe extern "C" fn ad193x_i2c_probe(client: *mut i2c_client) -> c_int {
    static int ad193x_i2c_probe(struct i2c_client *client)
    {
    struct regmap_config config;
    config = ad193x_regmap_config;
    config.val_bits = 8;
    config.reg_bits = 8;
    return ad193x_probe(&client.dev,
    devm_regmap_init_i2c(client, &config),
    (uintptr_t)i2c_get_match_data(client));
    }
    static struct i2c_driver ad193x_i2c_driver = {
    .driver = {
    .name = "ad193x",
    },
    .probe = ad193x_i2c_probe,
    .id_table = ad193x_id,
    };
    module_i2c_driver(ad193x_i2c_driver);
    MODULE_DESCRIPTION("ASoC AD1936/AD1937 audio CODEC driver");
    MODULE_AUTHOR("Barry Song <21cnbao@gmail.com>");
    MODULE_LICENSE("GPL");
