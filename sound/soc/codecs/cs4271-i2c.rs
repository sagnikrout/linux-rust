//! Automatically rewritten from C to Rust
//! Source: sound/soc/codecs/cs4271-i2c.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// CS4271 I2C audio driver
//
// Copyright (c) 2010 Alexander Sverdlin <subaparts@yandex.ru>
//

#[no_mangle]
unsafe extern "C" fn cs4271_i2c_probe(client: *mut i2c_client) -> c_int {
    static int cs4271_i2c_probe(struct i2c_client *client)
    {
    struct regmap_config config;
    config = cs4271_regmap_config;
    config.reg_bits = 8;
    return cs4271_probe(&client.dev,
    devm_regmap_init_i2c(client, &config));
    }
    static const struct i2c_device_id cs4271_i2c_id[] = {
    { .name = "cs4271" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, cs4271_i2c_id);
    static struct i2c_driver cs4271_i2c_driver = {
    .driver = {
    .name = "cs4271",
    .of_match_table = of_match_ptr(cs4271_dt_ids),
    },
    .probe = cs4271_i2c_probe,
    .id_table = cs4271_i2c_id,
    };
    module_i2c_driver(cs4271_i2c_driver);
    MODULE_DESCRIPTION("ASoC CS4271 I2C Driver");
    MODULE_AUTHOR("Alexander Sverdlin <subaparts@yandex.ru>");
    MODULE_LICENSE("GPL");
