//! Automatically rewritten from C to Rust
//! Source: sound/soc/codecs/pcm186x-i2c.c
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
// Texas Instruments PCM186x Universal Audio ADC - I2C
//
// Copyright (C) 2015-2017 Texas Instruments Incorporated - https://www.ti.com
// Andreas Dannenberg <dannenberg@ti.com>
// Andrew F. Davis <afd@ti.com>
//

    static const struct of_device_id pcm186x_of_match[] = {
    { .compatible = "ti,pcm1862", .data = (void *)PCM1862 },
    { .compatible = "ti,pcm1863", .data = (void *)PCM1863 },
    { .compatible = "ti,pcm1864", .data = (void *)PCM1864 },
    { .compatible = "ti,pcm1865", .data = (void *)PCM1865 },
    { }
    };
    MODULE_DEVICE_TABLE(of, pcm186x_of_match);
    static const struct i2c_device_id pcm186x_i2c_id[] = {
    { .name = "pcm1862", .driver_data = PCM1862 },
    { .name = "pcm1863", .driver_data = PCM1863 },
    { .name = "pcm1864", .driver_data = PCM1864 },
    { .name = "pcm1865", .driver_data = PCM1865 },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, pcm186x_i2c_id);
#[no_mangle]
unsafe extern "C" fn pcm186x_i2c_probe(i2c: *mut i2c_client) -> c_int {
    static int pcm186x_i2c_probe(struct i2c_client *i2c)
    {
    let mut type: enum pcm186x_type = (uintptr_t)i2c_get_match_data(i2c);
    let mut irq: c_int = i2c.irq;
    struct regmap *regmap;
    regmap = devm_regmap_init_i2c(i2c, &pcm186x_regmap);
    if (IS_ERR(regmap))
    return PTR_ERR(regmap);
    return pcm186x_probe(&i2c.dev, type, irq, regmap);
    }
    static struct i2c_driver pcm186x_i2c_driver = {
    .probe		= pcm186x_i2c_probe,
    .id_table	= pcm186x_i2c_id,
    .driver		= {
    .name	= "pcm186x",
    .of_match_table = pcm186x_of_match,
    },
    };
    module_i2c_driver(pcm186x_i2c_driver);
    MODULE_AUTHOR("Andreas Dannenberg <dannenberg@ti.com>");
    MODULE_AUTHOR("Andrew F. Davis <afd@ti.com>");
    MODULE_DESCRIPTION("PCM186x Universal Audio ADC I2C Interface Driver");
    MODULE_LICENSE("GPL v2");
