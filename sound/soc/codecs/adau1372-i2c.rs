//! Automatically rewritten from C to Rust
//! Source: sound/soc/codecs/adau1372-i2c.c
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
// Driver for ADAU1372 codec
//
// Copyright 2016 Analog Devices Inc.
// Author: Lars-Peter Clausen <lars@metafoo.de>
//

#[no_mangle]
unsafe extern "C" fn adau1372_i2c_probe(client: *mut i2c_client) -> c_int {
    static int adau1372_i2c_probe(struct i2c_client *client)
    {
    return adau1372_probe(&client.dev,
    devm_regmap_init_i2c(client, &adau1372_regmap_config), core::ptr::null_mut());
    }
    static const struct i2c_device_id adau1372_i2c_ids[] = {
    { .name = "adau1372" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, adau1372_i2c_ids);
    static struct i2c_driver adau1372_i2c_driver = {
    .driver = {
    .name = "adau1372",
    .of_match_table = adau1372_of_match,
    },
    .probe = adau1372_i2c_probe,
    .id_table = adau1372_i2c_ids,
    };
    module_i2c_driver(adau1372_i2c_driver);
    MODULE_DESCRIPTION("ASoC ADAU1372 CODEC I2C driver");
    MODULE_AUTHOR("Lars-Peter Clausen <lars@metafoo.de>");
    MODULE_LICENSE("GPL v2");
