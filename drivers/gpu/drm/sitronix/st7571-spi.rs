//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/sitronix/st7571-spi.c
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
// Driver for Sitronix ST7571 connected via SPI bus.
//
// Copyright (C) 2025 Marcus Folkesson <marcus.folkesson@gmail.com>
//

    static const struct regmap_config st7571_spi_regmap_config = {
    .reg_bits = 8,
    .val_bits = 8,
    .can_multi_write = true,
    };
#[no_mangle]
unsafe extern "C" fn st7571_spi_probe(spi: *mut spi_device) -> c_int {
    static int st7571_spi_probe(struct spi_device *spi)
    {
    struct st7571_device *st7571;
    struct regmap *regmap;
    regmap = devm_regmap_init_spi(spi, &st7571_spi_regmap_config);
    if (IS_ERR(regmap)) {
    return dev_err_probe(&spi.dev, PTR_ERR(regmap),
    "Failed to initialize regmap\n");
    }
    st7571 = st7571_probe(&spi.dev, regmap);
    if (IS_ERR(st7571))
    return dev_err_probe(&spi.dev, PTR_ERR(st7571),
    "Failed to initialize regmap\n");
    spi_set_drvdata(spi, st7571);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn st7571_spi_remove(spi: *mut spi_device) {
    static void st7571_spi_remove(struct spi_device *spi)
    {
    struct st7571_device *st7571 = spi_get_drvdata(spi);
    st7571_remove(st7571);
    }
    static const struct of_device_id st7571_of_match[] = {
    { .compatible = "sitronix,st7567", .data = &st7567_config },
    { .compatible = "sitronix,st7571", .data = &st7571_config },
    {},
    };
    MODULE_DEVICE_TABLE(of, st7571_of_match);
    static const struct spi_device_id st7571_spi_id[] = {
    { "st7567", 0 },
    { "st7571", 0 },
    { }
    };
    MODULE_DEVICE_TABLE(spi, st7571_spi_id);
    static struct spi_driver st7571_spi_driver = {
    .driver = {
    .name = "st7571-spi",
    .of_match_table = st7571_of_match,
    },
    .probe = st7571_spi_probe,
    .remove = st7571_spi_remove,
    .id_table = st7571_spi_id,
    };
    module_spi_driver(st7571_spi_driver);
    MODULE_AUTHOR("Marcus Folkesson <marcus.folkesson@gmail.com>");
    MODULE_DESCRIPTION("DRM Driver for Sitronix ST7571 LCD controller (SPI)");
    MODULE_LICENSE("GPL");
    MODULE_IMPORT_NS("DRM_ST7571");
