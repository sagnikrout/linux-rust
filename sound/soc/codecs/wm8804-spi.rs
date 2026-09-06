//! Automatically rewritten from C to Rust
//! Source: sound/soc/codecs/wm8804-spi.c
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
// wm8804-spi.c  --  WM8804 S/PDIF transceiver driver - SPI
//
// Copyright 2015 Cirrus Logic Inc
//
// Author: Charles Keepax <ckeepax@opensource.wolfsonmicro.com>
//

#[no_mangle]
unsafe extern "C" fn wm8804_spi_probe(spi: *mut spi_device) -> c_int {
    static int wm8804_spi_probe(struct spi_device *spi)
    {
    struct regmap *regmap;
    regmap = devm_regmap_init_spi(spi, &wm8804_regmap_config);
    if (IS_ERR(regmap))
    return PTR_ERR(regmap);
    return wm8804_probe(&spi.dev, regmap);
    }
#[no_mangle]
unsafe extern "C" fn wm8804_spi_remove(spi: *mut spi_device) {
    static void wm8804_spi_remove(struct spi_device *spi)
    {
    wm8804_remove(&spi.dev);
    }
    static const struct of_device_id wm8804_of_match[] = {
    { .compatible = "wlf,wm8804", },
    { }
    };
    MODULE_DEVICE_TABLE(of, wm8804_of_match);
    static struct spi_driver wm8804_spi_driver = {
    .driver = {
    .name = "wm8804",
    .pm = pm_ptr(&wm8804_pm),
    .of_match_table = wm8804_of_match,
    },
    .probe = wm8804_spi_probe,
    .remove = wm8804_spi_remove
    };
    module_spi_driver(wm8804_spi_driver);
    MODULE_DESCRIPTION("ASoC WM8804 driver - SPI");
    MODULE_AUTHOR("Charles Keepax <ckeepax@opensource.wolfsonmicro.com>");
    MODULE_LICENSE("GPL");
