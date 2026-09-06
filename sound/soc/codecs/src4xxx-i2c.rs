//! Automatically rewritten from C to Rust
//! Source: sound/soc/codecs/src4xxx-i2c.c
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
// Driver for SRC4XXX codecs
//
// Copyright 2021-2022 Deqx Pty Ltd
// Author: Matt Flax <flatmax@flatmax.com>

#[no_mangle]
unsafe extern "C" fn src4xxx_i2c_probe(i2c: *mut i2c_client) -> c_int {
    static int src4xxx_i2c_probe(struct i2c_client *i2c)
    {
    return src4xxx_probe(&i2c.dev,
    devm_regmap_init_i2c(i2c, &src4xxx_regmap_config), core::ptr::null_mut());
    }
    static const struct i2c_device_id src4xxx_i2c_ids[] = {
    { .name = "src4392" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, src4xxx_i2c_ids);
    static const struct of_device_id src4xxx_of_match[] __maybe_unused = {
    { .compatible = "ti,src4392", },
    { }
    };
    MODULE_DEVICE_TABLE(of, src4xxx_of_match);
    static struct i2c_driver src4xxx_i2c_driver = {
    .driver = {
    .name = "src4xxx",
    .of_match_table = of_match_ptr(src4xxx_of_match),
    },
    .probe = src4xxx_i2c_probe,
    .id_table = src4xxx_i2c_ids,
    };
    module_i2c_driver(src4xxx_i2c_driver);
    MODULE_DESCRIPTION("ASoC SRC4392 CODEC I2C driver");
    MODULE_AUTHOR("Matt Flax <flatmax@flatmax.com>");
    MODULE_LICENSE("GPL");
