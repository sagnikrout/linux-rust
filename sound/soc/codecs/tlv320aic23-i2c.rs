//! Automatically rewritten from C to Rust
//! Source: sound/soc/codecs/tlv320aic23-i2c.c
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
// ALSA SoC TLV320AIC23 codec driver I2C interface
//
// Author:      Arun KS, <arunks@mistralsolutions.com>
// Copyright:   (C) 2008 Mistral Solutions Pvt Ltd.,
//
// Based on sound/soc/codecs/wm8731.c by Richard Purdie
//

#[no_mangle]
unsafe extern "C" fn tlv320aic23_i2c_probe(i2c: *mut i2c_client) -> c_int {
    static int tlv320aic23_i2c_probe(struct i2c_client *i2c)
    {
    struct regmap *regmap;
    if (!i2c_check_functionality(i2c.adapter, I2C_FUNC_SMBUS_BYTE_DATA))
    return -EINVAL;
    regmap = devm_regmap_init_i2c(i2c, &tlv320aic23_regmap);
    return tlv320aic23_probe(&i2c.dev, regmap);
    }
    static const struct i2c_device_id tlv320aic23_id[] = {
    { .name = "tlv320aic23" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, tlv320aic23_id);

    static const struct of_device_id tlv320aic23_of_match[] = {
    { .compatible = "ti,tlv320aic23", },
    { }
    };
    MODULE_DEVICE_TABLE(of, tlv320aic23_of_match);

    static struct i2c_driver tlv320aic23_i2c_driver = {
    .driver = {
    .name = "tlv320aic23-codec",
    .of_match_table = of_match_ptr(tlv320aic23_of_match),
    },
    .probe = tlv320aic23_i2c_probe,
    .id_table = tlv320aic23_id,
    };
    module_i2c_driver(tlv320aic23_i2c_driver);
    MODULE_DESCRIPTION("ASoC TLV320AIC23 codec driver I2C");
    MODULE_AUTHOR("Arun KS <arunks@mistralsolutions.com>");
    MODULE_LICENSE("GPL");
