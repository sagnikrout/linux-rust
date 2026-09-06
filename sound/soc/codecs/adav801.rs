//! Automatically rewritten from C to Rust
//! Source: sound/soc/codecs/adav801.c
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
// ADAV801 audio driver
//
// Copyright 2014 Analog Devices Inc.
//

    static const struct spi_device_id adav80x_spi_id[] = {
    { "adav801", 0 },
    { }
    };
    MODULE_DEVICE_TABLE(spi, adav80x_spi_id);
#[no_mangle]
unsafe extern "C" fn adav80x_spi_probe(spi: *mut spi_device) -> c_int {
    static int adav80x_spi_probe(struct spi_device *spi)
    {
    struct regmap_config config;
    config = adav80x_regmap_config;
    config.read_flag_mask = 0x01;
    return adav80x_bus_probe(&spi.dev, devm_regmap_init_spi(spi, &config));
    }
    static struct spi_driver adav80x_spi_driver = {
    .driver = {
    .name	= "adav801",
    },
    .probe		= adav80x_spi_probe,
    .id_table	= adav80x_spi_id,
    };
    module_spi_driver(adav80x_spi_driver);
    MODULE_DESCRIPTION("ASoC ADAV801 driver");
    MODULE_AUTHOR("Lars-Peter Clausen <lars@metafoo.de>");
    MODULE_AUTHOR("Yi Li <yi.li@analog.com>>");
    MODULE_LICENSE("GPL");
