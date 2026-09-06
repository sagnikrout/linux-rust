//! Automatically rewritten from C to Rust
//! Source: sound/soc/codecs/cs42xx8-i2c.c
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
// Cirrus Logic CS42448/CS42888 Audio CODEC DAI I2C driver
//
// Copyright (C) 2014 Freescale Semiconductor, Inc.
//
// Author: Nicolin Chen <Guangyu.Chen@freescale.com>
//

#[no_mangle]
unsafe extern "C" fn cs42xx8_i2c_probe(i2c: *mut i2c_client) -> c_int {
    static int cs42xx8_i2c_probe(struct i2c_client *i2c)
    {
    int ret;
    struct cs42xx8_driver_data *drvdata;
    drvdata = (struct cs42xx8_driver_data *)i2c_get_match_data(i2c);
    if (!drvdata)
    return dev_err_probe(&i2c.dev, -EINVAL,
    "failed to find driver data\n");
    ret = cs42xx8_probe(&i2c.dev,
    devm_regmap_init_i2c(i2c, &cs42xx8_regmap_config), drvdata);
    if (ret)
    return ret;
    pm_runtime_enable(&i2c.dev);
    pm_request_idle(&i2c.dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cs42xx8_i2c_remove(i2c: *mut i2c_client) {
    static void cs42xx8_i2c_remove(struct i2c_client *i2c)
    {
    pm_runtime_disable(&i2c.dev);
    }
    static const struct of_device_id cs42xx8_of_match[] = {
    { .compatible = "cirrus,cs42448", .data = &cs42448_data, },
    { .compatible = "cirrus,cs42888", .data = &cs42888_data, },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, cs42xx8_of_match);
    static const struct i2c_device_id cs42xx8_i2c_id[] = {
    { .name = "cs42448", .driver_data = (kernel_ulong_t)&cs42448_data },
    { .name = "cs42888", .driver_data = (kernel_ulong_t)&cs42888_data },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, cs42xx8_i2c_id);
    static struct i2c_driver cs42xx8_i2c_driver = {
    .driver = {
    .name = "cs42xx8",
    .pm = pm_ptr(&cs42xx8_pm),
    .of_match_table = cs42xx8_of_match,
    },
    .probe = cs42xx8_i2c_probe,
    .remove = cs42xx8_i2c_remove,
    .id_table = cs42xx8_i2c_id,
    };
    module_i2c_driver(cs42xx8_i2c_driver);
    MODULE_DESCRIPTION("Cirrus Logic CS42448/CS42888 ALSA SoC Codec I2C Driver");
    MODULE_AUTHOR("Freescale Semiconductor, Inc.");
    MODULE_LICENSE("GPL");
