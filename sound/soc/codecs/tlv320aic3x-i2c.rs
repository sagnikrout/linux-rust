//! Automatically rewritten from C to Rust
//! Source: sound/soc/codecs/tlv320aic3x-i2c.c
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
// ALSA SoC TLV320AIC3x codec driver I2C interface
//
// Author:      Arun KS, <arunks@mistralsolutions.com>
// Copyright:   (C) 2008 Mistral Solutions Pvt Ltd.,
//
// Based on sound/soc/codecs/wm8731.c by Richard Purdie
//

    static const struct i2c_device_id aic3x_i2c_id[] = {
    { .name = "tlv320aic3x", .driver_data = AIC3X_MODEL_3X },
    { .name = "tlv320aic33", .driver_data = AIC3X_MODEL_33 },
    { .name = "tlv320aic3007", .driver_data = AIC3X_MODEL_3007 },
    { .name = "tlv320aic3104", .driver_data = AIC3X_MODEL_3104 },
    { .name = "tlv320aic3106", .driver_data = AIC3X_MODEL_3106 },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, aic3x_i2c_id);
#[no_mangle]
unsafe extern "C" fn aic3x_i2c_probe(i2c: *mut i2c_client) -> c_int {
    static int aic3x_i2c_probe(struct i2c_client *i2c)
    {
    struct regmap *regmap;
    struct regmap_config config;
    config = aic3x_regmap;
    config.reg_bits = 8;
    config.val_bits = 8;
    regmap = devm_regmap_init_i2c(i2c, &config);
    return aic3x_probe(&i2c.dev, regmap, (uintptr_t)i2c_get_match_data(i2c));
    }
#[no_mangle]
unsafe extern "C" fn aic3x_i2c_remove(i2c: *mut i2c_client) {
    static void aic3x_i2c_remove(struct i2c_client *i2c)
    {
    aic3x_remove(&i2c.dev);
    }
    static const struct of_device_id aic3x_of_id[] = {
    { .compatible = "ti,tlv320aic3x", },
    { .compatible = "ti,tlv320aic33" },
    { .compatible = "ti,tlv320aic3007" },
    { .compatible = "ti,tlv320aic3104" },
    { .compatible = "ti,tlv320aic3106" },
    {},
    };
    MODULE_DEVICE_TABLE(of, aic3x_of_id);
    static struct i2c_driver aic3x_i2c_driver = {
    .driver = {
    .name = "tlv320aic3x",
    .of_match_table = aic3x_of_id,
    },
    .probe = aic3x_i2c_probe,
    .remove = aic3x_i2c_remove,
    .id_table = aic3x_i2c_id,
    };
    module_i2c_driver(aic3x_i2c_driver);
    MODULE_DESCRIPTION("ASoC TLV320AIC3x codec driver I2C");
    MODULE_AUTHOR("Arun KS <arunks@mistralsolutions.com>");
    MODULE_LICENSE("GPL");
