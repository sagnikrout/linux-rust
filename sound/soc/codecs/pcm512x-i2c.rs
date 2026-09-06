//! Automatically rewritten from C to Rust
//! Source: sound/soc/codecs/pcm512x-i2c.c
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
// Driver for the PCM512x CODECs
//
// Author:	Mark Brown <broonie@kernel.org>
// Copyright 2014 Linaro Ltd
//

#[no_mangle]
unsafe extern "C" fn pcm512x_i2c_probe(i2c: *mut i2c_client) -> c_int {
    static int pcm512x_i2c_probe(struct i2c_client *i2c)
    {
    struct regmap *regmap;
    let mut config: regmap_config = pcm512x_regmap;
// msb needs to be set to enable auto-increment of addresses
    config.read_flag_mask = 0x80;
    config.write_flag_mask = 0x80;
    regmap = devm_regmap_init_i2c(i2c, &config);
    if (IS_ERR(regmap))
    return PTR_ERR(regmap);
    return pcm512x_probe(&i2c.dev, regmap);
    }
#[no_mangle]
unsafe extern "C" fn pcm512x_i2c_remove(i2c: *mut i2c_client) {
    static void pcm512x_i2c_remove(struct i2c_client *i2c)
    {
    pcm512x_remove(&i2c.dev);
    }
    static const struct i2c_device_id pcm512x_i2c_id[] = {
    { .name = "pcm5121" },
    { .name = "pcm5122" },
    { .name = "pcm5141" },
    { .name = "pcm5142" },
    { .name = "pcm5242" },
    { .name = "tas5754" },
    { .name = "tas5756" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, pcm512x_i2c_id);

    static const struct of_device_id pcm512x_of_match[] = {
    { .compatible = "ti,pcm5121", },
    { .compatible = "ti,pcm5122", },
    { .compatible = "ti,pcm5141", },
    { .compatible = "ti,pcm5142", },
    { .compatible = "ti,pcm5242", },
    { .compatible = "ti,tas5754", },
    { .compatible = "ti,tas5756", },
    { }
    };
    MODULE_DEVICE_TABLE(of, pcm512x_of_match);

    static const struct acpi_device_id pcm512x_acpi_match[] = {
    { "104C5121", 0 },
    { "104C5122", 0 },
    { "104C5141", 0 },
    { "104C5142", 0 },
    { },
    };
    MODULE_DEVICE_TABLE(acpi, pcm512x_acpi_match);

    static struct i2c_driver pcm512x_i2c_driver = {
    .probe		= pcm512x_i2c_probe,
    .remove 	= pcm512x_i2c_remove,
    .id_table	= pcm512x_i2c_id,
    .driver		= {
    .name	= "pcm512x",
    .of_match_table = of_match_ptr(pcm512x_of_match),
    .acpi_match_table = ACPI_PTR(pcm512x_acpi_match),
    .pm     = pm_ptr(&pcm512x_pm_ops),
    },
    };
    module_i2c_driver(pcm512x_i2c_driver);
    MODULE_DESCRIPTION("ASoC PCM512x codec driver - I2C");
    MODULE_AUTHOR("Mark Brown <broonie@kernel.org>");
    MODULE_LICENSE("GPL v2");
