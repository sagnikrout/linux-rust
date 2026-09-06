//! Automatically rewritten from C to Rust
//! Source: sound/soc/codecs/pcm3168a-spi.c
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
// PCM3168A codec spi driver
//
// Copyright (C) 2015 Imagination Technologies Ltd.
//
// Author: Damien Horsley <Damien.Horsley@imgtec.com>
//

#[no_mangle]
unsafe extern "C" fn pcm3168a_spi_probe(spi: *mut spi_device) -> c_int {
    static int pcm3168a_spi_probe(struct spi_device *spi)
    {
    struct regmap *regmap;
    regmap = devm_regmap_init_spi(spi, &pcm3168a_regmap);
    if (IS_ERR(regmap))
    return PTR_ERR(regmap);
    return pcm3168a_probe(&spi.dev, regmap);
    }
#[no_mangle]
unsafe extern "C" fn pcm3168a_spi_remove(spi: *mut spi_device) {
    static void pcm3168a_spi_remove(struct spi_device *spi)
    {
    pcm3168a_remove(&spi.dev);
    }
    static const struct spi_device_id pcm3168a_spi_id[] = {
    { "pcm3168a", },
    { },
    };
    MODULE_DEVICE_TABLE(spi, pcm3168a_spi_id);
    static const struct of_device_id pcm3168a_of_match[] = {
    { .compatible = "ti,pcm3168a", },
    { }
    };
    MODULE_DEVICE_TABLE(of, pcm3168a_of_match);
    static struct spi_driver pcm3168a_spi_driver = {
    .probe		= pcm3168a_spi_probe,
    .remove		= pcm3168a_spi_remove,
    .id_table	= pcm3168a_spi_id,
    .driver = {
    .name	= "pcm3168a",
    .of_match_table = pcm3168a_of_match,
    .pm		= pm_ptr(&pcm3168a_pm_ops),
    },
    };
    module_spi_driver(pcm3168a_spi_driver);
    MODULE_DESCRIPTION("PCM3168A SPI codec driver");
    MODULE_AUTHOR("Damien Horsley <Damien.Horsley@imgtec.com>");
    MODULE_LICENSE("GPL v2");
