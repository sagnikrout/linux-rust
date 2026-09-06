//! Automatically rewritten from C to Rust
//! Source: sound/soc/codecs/pcm3060-spi.c
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
// PCM3060 SPI driver
//
// Copyright (C) 2018 Kirill Marinushkin <k.marinushkin@gmail.com>

#[no_mangle]
unsafe extern "C" fn pcm3060_spi_probe(spi: *mut spi_device) -> c_int {
    static int pcm3060_spi_probe(struct spi_device *spi)
    {
    struct pcm3060_priv *priv;
    priv = devm_kzalloc(&spi.dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    spi_set_drvdata(spi, priv);
    priv.regmap = devm_regmap_init_spi(spi, &pcm3060_regmap);
    if (IS_ERR(priv.regmap))
    return PTR_ERR(priv.regmap);
    return pcm3060_probe(&spi.dev);
    }
    static const struct spi_device_id pcm3060_spi_id[] = {
    { .name = "pcm3060" },
    { },
    };
    MODULE_DEVICE_TABLE(spi, pcm3060_spi_id);

    static const struct of_device_id pcm3060_of_match[] = {
    { .compatible = "ti,pcm3060" },
    { },
    };
    MODULE_DEVICE_TABLE(of, pcm3060_of_match);

    static struct spi_driver pcm3060_spi_driver = {
    .driver = {
    .name = "pcm3060",

    .of_match_table = pcm3060_of_match,

    },
    .id_table = pcm3060_spi_id,
    .probe = pcm3060_spi_probe,
    };
    module_spi_driver(pcm3060_spi_driver);
    MODULE_DESCRIPTION("PCM3060 SPI driver");
    MODULE_AUTHOR("Kirill Marinushkin <k.marinushkin@gmail.com>");
    MODULE_LICENSE("GPL v2");
