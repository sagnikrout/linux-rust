//! Automatically rewritten from C to Rust
//! Source: sound/soc/codecs/adav803.c
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
// ADAV803 audio driver
//
// Copyright 2014 Analog Devices Inc.
//

    static const struct i2c_device_id adav803_id[] = {
    { .name = "adav803" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, adav803_id);
#[no_mangle]
unsafe extern "C" fn adav803_probe(client: *mut i2c_client) -> c_int {
    static int adav803_probe(struct i2c_client *client)
    {
    return adav80x_bus_probe(&client.dev,
    devm_regmap_init_i2c(client, &adav80x_regmap_config));
    }
    static struct i2c_driver adav803_driver = {
    .driver = {
    .name = "adav803",
    },
    .probe = adav803_probe,
    .id_table = adav803_id,
    };
    module_i2c_driver(adav803_driver);
    MODULE_DESCRIPTION("ASoC ADAV803 driver");
    MODULE_AUTHOR("Lars-Peter Clausen <lars@metafoo.de>");
    MODULE_AUTHOR("Yi Li <yi.li@analog.com>>");
    MODULE_LICENSE("GPL");
