//! Automatically rewritten from C to Rust
//! Source: drivers/iio/light/st_uvis25_spi.c
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
// STMicroelectronics uvis25 spi driver
//
// Copyright 2017 STMicroelectronics Inc.
//
// Lorenzo Bianconi <lorenzo.bianconi83@gmail.com>
//

    static const struct regmap_config st_uvis25_spi_regmap_config = {
    .reg_bits = 8,
    .val_bits = 8,
    .read_flag_mask = UVIS25_SENSORS_SPI_READ | UVIS25_SPI_AUTO_INCREMENT,
    .write_flag_mask = UVIS25_SPI_AUTO_INCREMENT,
    };
#[no_mangle]
unsafe extern "C" fn st_uvis25_spi_probe(spi: *mut spi_device) -> c_int {
    static int st_uvis25_spi_probe(struct spi_device *spi)
    {
    struct regmap *regmap;
    regmap = devm_regmap_init_spi(spi, &st_uvis25_spi_regmap_config);
    if (IS_ERR(regmap)) {
    dev_err(&spi.dev, "Failed to register spi regmap %ld\n",
    PTR_ERR(regmap));
    return PTR_ERR(regmap);
    }
    return st_uvis25_probe(&spi.dev, spi.irq, regmap);
    }
    static const struct of_device_id st_uvis25_spi_of_match[] = {
    { .compatible = "st,uvis25", },
    { }
    };
    MODULE_DEVICE_TABLE(of, st_uvis25_spi_of_match);
    static const struct spi_device_id st_uvis25_spi_id_table[] = {
    { .name = ST_UVIS25_DEV_NAME },
    { }
    };
    MODULE_DEVICE_TABLE(spi, st_uvis25_spi_id_table);
    static struct spi_driver st_uvis25_driver = {
    .driver = {
    .name = "st_uvis25_spi",
    .pm = pm_sleep_ptr(&st_uvis25_pm_ops),
    .of_match_table = st_uvis25_spi_of_match,
    },
    .probe = st_uvis25_spi_probe,
    .id_table = st_uvis25_spi_id_table,
    };
    module_spi_driver(st_uvis25_driver);
    MODULE_AUTHOR("Lorenzo Bianconi <lorenzo.bianconi83@gmail.com>");
    MODULE_DESCRIPTION("STMicroelectronics uvis25 spi driver");
    MODULE_LICENSE("GPL v2");
    MODULE_IMPORT_NS("IIO_UVIS25");
