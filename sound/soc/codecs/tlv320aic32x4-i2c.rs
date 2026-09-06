//! Automatically rewritten from C to Rust
//! Source: sound/soc/codecs/tlv320aic32x4-i2c.c
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
// Copyright 2011-2019 NW Digital Radio
//
// Author: Annaliese McDermond <nh6z@nh6z.net>
//
// Based on sound/soc/codecs/wm8974 and TI driver for kernel 2.6.27.
//

    static const struct regmap_config aic32x4_i2c_regmap_config = {
    .reg_bits = 8,
    .val_bits = 8,
    .max_register = AIC32X4_REFPOWERUP,
    .ranges = aic32x4_regmap_pages,
    .num_ranges = 1,
    };
#[no_mangle]
unsafe extern "C" fn aic32x4_i2c_probe(i2c: *mut i2c_client) -> c_int {
    static int aic32x4_i2c_probe(struct i2c_client *i2c)
    {
    struct regmap *regmap;
    enum aic32x4_type type;
    regmap = devm_regmap_init_i2c(i2c, &aic32x4_i2c_regmap_config);
    if (IS_ERR(regmap))
    return PTR_ERR(regmap);
    type = (uintptr_t)i2c_get_match_data(i2c);
    return aic32x4_probe(&i2c.dev, regmap, type);
    }
#[no_mangle]
unsafe extern "C" fn aic32x4_i2c_remove(i2c: *mut i2c_client) {
    static void aic32x4_i2c_remove(struct i2c_client *i2c)
    {
    aic32x4_remove(&i2c.dev);
    }
    static const struct i2c_device_id aic32x4_i2c_id[] = {
    { .name = "tlv320aic32x4", .driver_data = (kernel_ulong_t)AIC32X4_TYPE_AIC32X4 },
    { .name = "tlv320aic32x6", .driver_data = (kernel_ulong_t)AIC32X4_TYPE_AIC32X6 },
    { .name = "tas2505", .driver_data = (kernel_ulong_t)AIC32X4_TYPE_TAS2505 },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(i2c, aic32x4_i2c_id);
    static const struct of_device_id aic32x4_of_id[] = {
    { .compatible = "ti,tlv320aic32x4", .data = (void *)AIC32X4_TYPE_AIC32X4 },
    { .compatible = "ti,tlv320aic32x6", .data = (void *)AIC32X4_TYPE_AIC32X6 },
    { .compatible = "ti,tas2505", .data = (void *)AIC32X4_TYPE_TAS2505 },
    { /* senitel */ }
    };
    MODULE_DEVICE_TABLE(of, aic32x4_of_id);
    static struct i2c_driver aic32x4_i2c_driver = {
    .driver = {
    .name = "tlv320aic32x4",
    .of_match_table = aic32x4_of_id,
    },
    .probe =    aic32x4_i2c_probe,
    .remove =   aic32x4_i2c_remove,
    .id_table = aic32x4_i2c_id,
    };
    module_i2c_driver(aic32x4_i2c_driver);
    MODULE_DESCRIPTION("ASoC TLV320AIC32x4 codec driver I2C");
    MODULE_AUTHOR("Annaliese McDermond <nh6z@nh6z.net>");
    MODULE_LICENSE("GPL");
