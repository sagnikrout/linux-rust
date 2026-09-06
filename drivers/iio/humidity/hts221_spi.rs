//! Automatically rewritten from C to Rust
//! Source: drivers/iio/humidity/hts221_spi.c
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
// STMicroelectronics hts221 spi driver
//
// Copyright 2016 STMicroelectronics Inc.
//
// Lorenzo Bianconi <lorenzo.bianconi@st.com>
//

    static const struct regmap_config hts221_spi_regmap_config = {
    .reg_bits = 8,
    .val_bits = 8,
    .write_flag_mask = HTS221_SPI_AUTO_INCREMENT,
    .read_flag_mask = HTS221_SPI_READ | HTS221_SPI_AUTO_INCREMENT,
    };
#[no_mangle]
unsafe extern "C" fn hts221_spi_probe(spi: *mut spi_device) -> c_int {
    static int hts221_spi_probe(struct spi_device *spi)
    {
    struct regmap *regmap;
    regmap = devm_regmap_init_spi(spi, &hts221_spi_regmap_config);
    if (IS_ERR(regmap)) {
    dev_err(&spi.dev, "Failed to register spi regmap %ld\n",
    PTR_ERR(regmap));
    return PTR_ERR(regmap);
    }
    return hts221_probe(&spi.dev, spi.irq,
    spi.modalias, regmap);
    }
    static const struct of_device_id hts221_spi_of_match[] = {
    { .compatible = "st,hts221", },
    { }
    };
    MODULE_DEVICE_TABLE(of, hts221_spi_of_match);
    static const struct spi_device_id hts221_spi_id_table[] = {
    { .name = HTS221_DEV_NAME },
    { }
    };
    MODULE_DEVICE_TABLE(spi, hts221_spi_id_table);
    static struct spi_driver hts221_driver = {
    .driver = {
    .name = "hts221_spi",
    .pm = pm_sleep_ptr(&hts221_pm_ops),
    .of_match_table = hts221_spi_of_match,
    },
    .probe = hts221_spi_probe,
    .id_table = hts221_spi_id_table,
    };
    module_spi_driver(hts221_driver);
    MODULE_AUTHOR("Lorenzo Bianconi <lorenzo.bianconi@st.com>");
    MODULE_DESCRIPTION("STMicroelectronics hts221 spi driver");
    MODULE_LICENSE("GPL v2");
    MODULE_IMPORT_NS("IIO_HTS221");
