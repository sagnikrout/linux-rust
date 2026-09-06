//! Automatically rewritten from C to Rust
//! Source: sound/soc/codecs/wm8731-i2c.c
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
// wm8731-i2c.c  --  WM8731 ALSA SoC Audio driver I2C code
//
// Copyright 2005 Openedhand Ltd.
// Copyright 2006-12 Wolfson Microelectronics, plc
//
// Author: Richard Purdie <richard@openedhand.com>
//
// Based on wm8753.c by Liam Girdwood
//

    static const struct of_device_id wm8731_of_match[] = {
    { .compatible = "wlf,wm8731", },
    { }
    };
    MODULE_DEVICE_TABLE(of, wm8731_of_match);
#[no_mangle]
unsafe extern "C" fn wm8731_i2c_probe(i2c: *mut i2c_client) -> c_int {
    static int wm8731_i2c_probe(struct i2c_client *i2c)
    {
    struct wm8731_priv *wm8731;
    int ret;
    wm8731 = devm_kzalloc(&i2c.dev, sizeof(struct wm8731_priv),
    GFP_KERNEL);
    if (wm8731 == core::ptr::null_mut())
    return -ENOMEM;
    i2c_set_clientdata(i2c, wm8731);
    wm8731.regmap = devm_regmap_init_i2c(i2c, &wm8731_regmap);
    if (IS_ERR(wm8731.regmap)) {
    ret = PTR_ERR(wm8731.regmap);
    dev_err(&i2c.dev, "Failed to allocate register map: %d\n",
    ret);
    return ret;
    }
    return wm8731_init(&i2c.dev, wm8731);
    }
    static const struct i2c_device_id wm8731_i2c_id[] = {
    { .name = "wm8731" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, wm8731_i2c_id);
    static struct i2c_driver wm8731_i2c_driver = {
    .driver = {
    .name = "wm8731",
    .of_match_table = wm8731_of_match,
    },
    .probe = wm8731_i2c_probe,
    .id_table = wm8731_i2c_id,
    };
    module_i2c_driver(wm8731_i2c_driver);
    MODULE_DESCRIPTION("ASoC WM8731 driver - I2C");
    MODULE_AUTHOR("Richard Purdie");
    MODULE_LICENSE("GPL");
