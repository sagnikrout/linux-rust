//! Automatically rewritten from C to Rust
//! Source: sound/soc/codecs/ad193x-spi.c
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
// AD1938/AD1939 audio driver
//
// Copyright 2014 Analog Devices Inc.
//

#[no_mangle]
unsafe extern "C" fn ad193x_spi_probe(spi: *mut spi_device) -> c_int {
    static int ad193x_spi_probe(struct spi_device *spi)
    {
    const struct spi_device_id *id = spi_get_device_id(spi);
    struct regmap_config config;
    config = ad193x_regmap_config;
    config.val_bits = 8;
    config.reg_bits = 16;
    config.read_flag_mask = 0x09;
    config.write_flag_mask = 0x08;
    return ad193x_probe(&spi.dev, devm_regmap_init_spi(spi, &config),
    (enum ad193x_type)id.driver_data);
    }
    static const struct spi_device_id ad193x_spi_id[] = {
    { "ad193x", AD193X },
    { "ad1933", AD1933 },
    { "ad1934", AD1934 },
    { "ad1938", AD193X },
    { "ad1939", AD193X },
    { "adau1328", AD193X },
    { }
    };
    MODULE_DEVICE_TABLE(spi, ad193x_spi_id);
    static struct spi_driver ad193x_spi_driver = {
    .driver = {
    .name	= "ad193x",
    },
    .probe		= ad193x_spi_probe,
    .id_table	= ad193x_spi_id,
    };
    module_spi_driver(ad193x_spi_driver);
    MODULE_DESCRIPTION("ASoC AD1938/AD1939 audio CODEC driver");
    MODULE_AUTHOR("Barry Song <21cnbao@gmail.com>");
    MODULE_LICENSE("GPL");
