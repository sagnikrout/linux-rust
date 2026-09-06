//! Automatically rewritten from C to Rust
//! Source: sound/soc/codecs/pcm1789-i2c.c
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
// Audio driver for PCM1789 I2C
// Copyright (C) 2018 Bootlin
// Mylène Josserand <mylene.josserand@bootlin.com>

#[no_mangle]
unsafe extern "C" fn pcm1789_i2c_probe(client: *mut i2c_client) -> c_int {
    static int pcm1789_i2c_probe(struct i2c_client *client)
    {
    struct regmap *regmap;
    int ret;
    regmap = devm_regmap_init_i2c(client, &pcm1789_regmap_config);
    if (IS_ERR(regmap)) {
    ret = PTR_ERR(regmap);
    dev_err(&client.dev, "Failed to allocate regmap: %d\n", ret);
    return ret;
    }
    return pcm1789_common_init(&client.dev, regmap);
    }
#[no_mangle]
unsafe extern "C" fn pcm1789_i2c_remove(client: *mut i2c_client) {
    static void pcm1789_i2c_remove(struct i2c_client *client)
    {
    pcm1789_common_exit(&client.dev);
    }

    static const struct of_device_id pcm1789_of_match[] = {
    { .compatible = "ti,pcm1789", },
    { }
    };
    MODULE_DEVICE_TABLE(of, pcm1789_of_match);

    static const struct i2c_device_id pcm1789_i2c_ids[] = {
    { .name = "pcm1789" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, pcm1789_i2c_ids);
    static struct i2c_driver pcm1789_i2c_driver = {
    .driver = {
    .name	= "pcm1789",
    .of_match_table = of_match_ptr(pcm1789_of_match),
    },
    .id_table	= pcm1789_i2c_ids,
    .probe		= pcm1789_i2c_probe,
    .remove	= pcm1789_i2c_remove,
    };
    module_i2c_driver(pcm1789_i2c_driver);
    MODULE_DESCRIPTION("ASoC PCM1789 I2C driver");
    MODULE_AUTHOR("Mylène Josserand <mylene.josserand@bootlin.com>");
    MODULE_LICENSE("GPL");
