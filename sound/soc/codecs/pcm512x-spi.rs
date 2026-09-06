//! Automatically rewritten from C to Rust
//! Source: sound/soc/codecs/pcm512x-spi.c
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
unsafe extern "C" fn pcm512x_spi_probe(spi: *mut spi_device) -> c_int {
    static int pcm512x_spi_probe(struct spi_device *spi)
    {
    struct regmap *regmap;
    int ret;
    regmap = devm_regmap_init_spi(spi, &pcm512x_regmap);
    if (IS_ERR(regmap)) {
    ret = PTR_ERR(regmap);
    return ret;
    }
    return pcm512x_probe(&spi.dev, regmap);
    }
#[no_mangle]
unsafe extern "C" fn pcm512x_spi_remove(spi: *mut spi_device) {
    static void pcm512x_spi_remove(struct spi_device *spi)
    {
    pcm512x_remove(&spi.dev);
    }
    static const struct spi_device_id pcm512x_spi_id[] = {
    { "pcm5121", },
    { "pcm5122", },
    { "pcm5141", },
    { "pcm5142", },
    { "pcm5242", },
    { },
    };
    MODULE_DEVICE_TABLE(spi, pcm512x_spi_id);
    static const struct of_device_id pcm512x_of_match[] = {
    { .compatible = "ti,pcm5121", },
    { .compatible = "ti,pcm5122", },
    { .compatible = "ti,pcm5141", },
    { .compatible = "ti,pcm5142", },
    { .compatible = "ti,pcm5242", },
    { }
    };
    MODULE_DEVICE_TABLE(of, pcm512x_of_match);
    static struct spi_driver pcm512x_spi_driver = {
    .probe		= pcm512x_spi_probe,
    .remove		= pcm512x_spi_remove,
    .id_table	= pcm512x_spi_id,
    .driver = {
    .name	= "pcm512x",
    .of_match_table = pcm512x_of_match,
    .pm     = pm_ptr(&pcm512x_pm_ops),
    },
    };
    module_spi_driver(pcm512x_spi_driver);
    MODULE_DESCRIPTION("ASoC PCM512x codec driver - SPI");
    MODULE_AUTHOR("Mark Brown <broonie@kernel.org>");
    MODULE_LICENSE("GPL v2");
