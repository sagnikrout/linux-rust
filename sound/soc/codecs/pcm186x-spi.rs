//! Automatically rewritten from C to Rust
//! Source: sound/soc/codecs/pcm186x-spi.c
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
// Texas Instruments PCM186x Universal Audio ADC - SPI
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
#[no_mangle]
unsafe extern "C" fn pcm186x_spi_probe(spi: *mut spi_device) -> c_int {
    static int pcm186x_spi_probe(struct spi_device *spi)
    {
    const enum pcm186x_type type =
    (enum pcm186x_type)spi_get_device_id(spi).driver_data;
    let mut irq: c_int = spi.irq;
    struct regmap *regmap;
    regmap = devm_regmap_init_spi(spi, &pcm186x_regmap);
    if (IS_ERR(regmap))
    return PTR_ERR(regmap);
    return pcm186x_probe(&spi.dev, type, irq, regmap);
    }
    static const struct spi_device_id pcm186x_spi_id[] = {
    { "pcm1862", PCM1862 },
    { "pcm1863", PCM1863 },
    { "pcm1864", PCM1864 },
    { "pcm1865", PCM1865 },
    { }
    };
    MODULE_DEVICE_TABLE(spi, pcm186x_spi_id);
    static struct spi_driver pcm186x_spi_driver = {
    .probe		= pcm186x_spi_probe,
    .id_table	= pcm186x_spi_id,
    .driver		= {
    .name	= "pcm186x",
    .of_match_table = pcm186x_of_match,
    },
    };
    module_spi_driver(pcm186x_spi_driver);
    MODULE_AUTHOR("Andreas Dannenberg <dannenberg@ti.com>");
    MODULE_AUTHOR("Andrew F. Davis <afd@ti.com>");
    MODULE_DESCRIPTION("PCM186x Universal Audio ADC SPI Interface Driver");
    MODULE_LICENSE("GPL v2");
