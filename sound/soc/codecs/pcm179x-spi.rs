//! Automatically rewritten from C to Rust
//! Source: sound/soc/codecs/pcm179x-spi.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// PCM179X ASoC SPI driver
//
// Copyright (c) Amarula Solutions B.V. 2013
//
// Michael Trimarchi <michael@amarulasolutions.com>
//

#[no_mangle]
unsafe extern "C" fn pcm179x_spi_probe(spi: *mut spi_device) -> c_int {
    static int pcm179x_spi_probe(struct spi_device *spi)
    {
    struct regmap *regmap;
    int ret;
    regmap = devm_regmap_init_spi(spi, &pcm179x_regmap_config);
    if (IS_ERR(regmap)) {
    ret = PTR_ERR(regmap);
    dev_err(&spi.dev, "Failed to allocate regmap: %d\n", ret);
    return ret;
    }
    return pcm179x_common_init(&spi.dev, regmap);
    }
    static const struct of_device_id pcm179x_of_match[] __maybe_unused = {
    { .compatible = "ti,pcm1792a", },
    { }
    };
    MODULE_DEVICE_TABLE(of, pcm179x_of_match);
    static const struct spi_device_id pcm179x_spi_ids[] = {
    { "pcm1792a", 0 },
    { "pcm179x", 0 },
    { },
    };
    MODULE_DEVICE_TABLE(spi, pcm179x_spi_ids);
    static struct spi_driver pcm179x_spi_driver = {
    .driver = {
    .name = "pcm179x",
    .of_match_table = of_match_ptr(pcm179x_of_match),
    },
    .id_table = pcm179x_spi_ids,
    .probe = pcm179x_spi_probe,
    };
    module_spi_driver(pcm179x_spi_driver);
    MODULE_DESCRIPTION("ASoC PCM179X SPI driver");
    MODULE_AUTHOR("Michael Trimarchi <michael@amarulasolutions.com>");
    MODULE_LICENSE("GPL");
