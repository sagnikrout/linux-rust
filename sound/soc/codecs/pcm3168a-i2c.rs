//! Automatically rewritten from C to Rust
//! Source: sound/soc/codecs/pcm3168a-i2c.c
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
// PCM3168A codec i2c driver
//
// Copyright (C) 2015 Imagination Technologies Ltd.
//
// Author: Damien Horsley <Damien.Horsley@imgtec.com>
//

#[no_mangle]
unsafe extern "C" fn pcm3168a_i2c_probe(i2c: *mut i2c_client) -> c_int {
    static int pcm3168a_i2c_probe(struct i2c_client *i2c)
    {
    struct regmap *regmap;
    regmap = devm_regmap_init_i2c(i2c, &pcm3168a_regmap);
    if (IS_ERR(regmap))
    return PTR_ERR(regmap);
    return pcm3168a_probe(&i2c.dev, regmap);
    }
#[no_mangle]
unsafe extern "C" fn pcm3168a_i2c_remove(i2c: *mut i2c_client) {
    static void pcm3168a_i2c_remove(struct i2c_client *i2c)
    {
    pcm3168a_remove(&i2c.dev);
    }
    static const struct i2c_device_id pcm3168a_i2c_id[] = {
    { .name = "pcm3168a" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, pcm3168a_i2c_id);
    static const struct acpi_device_id pcm3168a_acpi_match[] = {
    { "PCM3168A" },
    { "104C3168" },
    {}
    };
    MODULE_DEVICE_TABLE(acpi, pcm3168a_acpi_match);
    static const struct of_device_id pcm3168a_of_match[] = {
    { .compatible = "ti,pcm3168a", },
    { }
    };
    MODULE_DEVICE_TABLE(of, pcm3168a_of_match);
    static struct i2c_driver pcm3168a_i2c_driver = {
    .probe		= pcm3168a_i2c_probe,
    .remove		= pcm3168a_i2c_remove,
    .id_table	= pcm3168a_i2c_id,
    .driver		= {
    .name	= "pcm3168a",
    .acpi_match_table = pcm3168a_acpi_match,
    .of_match_table = pcm3168a_of_match,
    .pm		= pm_ptr(&pcm3168a_pm_ops),
    },
    };
    module_i2c_driver(pcm3168a_i2c_driver);
    MODULE_DESCRIPTION("PCM3168A I2C codec driver");
    MODULE_AUTHOR("Damien Horsley <Damien.Horsley@imgtec.com>");
    MODULE_LICENSE("GPL v2");
