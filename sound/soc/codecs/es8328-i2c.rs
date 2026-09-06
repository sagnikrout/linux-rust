//! Automatically rewritten from C to Rust
//! Source: sound/soc/codecs/es8328-i2c.c
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
// es8328-i2c.c  --  ES8328 ALSA SoC I2C Audio driver
//
// Copyright 2014 Sutajio Ko-Usagi PTE LTD
//
// Author: Sean Cross <xobs@kosagi.com>
//

    static const struct i2c_device_id es8328_id[] = {
    { .name = "es8328" },
    { .name = "es8388" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, es8328_id);
    static const struct of_device_id es8328_of_match[] = {
    { .compatible = "everest,es8328", },
    { .compatible = "everest,es8388", },
    { }
    };
    MODULE_DEVICE_TABLE(of, es8328_of_match);
#[no_mangle]
unsafe extern "C" fn es8328_i2c_probe(i2c: *mut i2c_client) -> c_int {
    static int es8328_i2c_probe(struct i2c_client *i2c)
    {
    return es8328_probe(&i2c.dev,
    devm_regmap_init_i2c(i2c, &es8328_regmap_config));
    }
    static struct i2c_driver es8328_i2c_driver = {
    .driver = {
    .name		= "es8328",
    .of_match_table = es8328_of_match,
    },
    .probe = es8328_i2c_probe,
    .id_table = es8328_id,
    };
    module_i2c_driver(es8328_i2c_driver);
    MODULE_DESCRIPTION("ASoC ES8328 audio CODEC I2C driver");
    MODULE_AUTHOR("Sean Cross <xobs@kosagi.com>");
    MODULE_LICENSE("GPL");
