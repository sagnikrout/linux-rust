//! Automatically rewritten from C to Rust
//! Source: sound/soc/codecs/cs42l51-i2c.c
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
// cs42l56.c -- CS42L51 ALSA SoC I2C audio driver
//
// Copyright 2014 CirrusLogic, Inc.
//
// Author: Brian Austin <brian.austin@cirrus.com>
//

    static const struct i2c_device_id cs42l51_i2c_id[] = {
    { .name = "cs42l51" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, cs42l51_i2c_id);
    static const struct of_device_id cs42l51_of_match[] = {
    { .compatible = "cirrus,cs42l51", },
    { }
    };
    MODULE_DEVICE_TABLE(of, cs42l51_of_match);
#[no_mangle]
unsafe extern "C" fn cs42l51_i2c_probe(i2c: *mut i2c_client) -> c_int {
    static int cs42l51_i2c_probe(struct i2c_client *i2c)
    {
    struct regmap_config config;
    config = cs42l51_regmap;
    return cs42l51_probe(&i2c.dev, devm_regmap_init_i2c(i2c, &config));
    }
#[no_mangle]
unsafe extern "C" fn cs42l51_i2c_remove(i2c: *mut i2c_client) {
    static void cs42l51_i2c_remove(struct i2c_client *i2c)
    {
    cs42l51_remove(&i2c.dev);
    }
    static const struct dev_pm_ops cs42l51_pm_ops = {
    SYSTEM_SLEEP_PM_OPS(cs42l51_suspend, cs42l51_resume)
    };
    static struct i2c_driver cs42l51_i2c_driver = {
    .driver = {
    .name = "cs42l51",
    .of_match_table = cs42l51_of_match,
    .pm = &cs42l51_pm_ops,
    },
    .probe = cs42l51_i2c_probe,
    .remove = cs42l51_i2c_remove,
    .id_table = cs42l51_i2c_id,
    };
    module_i2c_driver(cs42l51_i2c_driver);
    MODULE_DESCRIPTION("ASoC CS42L51 I2C Driver");
    MODULE_AUTHOR("Brian Austin, Cirrus Logic Inc, <brian.austin@cirrus.com>");
    MODULE_LICENSE("GPL");
