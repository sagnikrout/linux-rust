//! Automatically rewritten from C to Rust
//! Source: sound/soc/codecs/adau1781-spi.c
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
// Driver for ADAU1381/ADAU1781 CODEC
//
// Copyright 2014 Analog Devices Inc.
// Author: Lars-Peter Clausen <lars@metafoo.de>
//

#[no_mangle]
unsafe extern "C" fn adau1781_spi_switch_mode(dev: *mut device) {
    static void adau1781_spi_switch_mode(struct device *dev)
    {
    struct spi_device *spi = to_spi_device(dev);
//
// To get the device into SPI mode CLATCH has to be pulled low three
// times.  Do this by issuing three dummy reads.
//
    spi_w8r8(spi, 0x00);
    spi_w8r8(spi, 0x00);
    spi_w8r8(spi, 0x00);
    }
#[no_mangle]
unsafe extern "C" fn adau1781_spi_probe(spi: *mut spi_device) -> c_int {
    static int adau1781_spi_probe(struct spi_device *spi)
    {
    const struct spi_device_id *id = spi_get_device_id(spi);
    struct regmap_config config;
    if (!id)
    return -EINVAL;
    config = adau1781_regmap_config;
    config.val_bits = 8;
    config.reg_bits = 24;
    config.read_flag_mask = 0x1;
    return adau1781_probe(&spi.dev,
    devm_regmap_init_spi(spi, &config),
    id.driver_data, adau1781_spi_switch_mode);
    }
#[no_mangle]
unsafe extern "C" fn adau1781_spi_remove(spi: *mut spi_device) {
    static void adau1781_spi_remove(struct spi_device *spi)
    {
    adau17x1_remove(&spi.dev);
    }
    static const struct spi_device_id adau1781_spi_id[] = {
    { "adau1381", ADAU1381 },
    { "adau1781", ADAU1781 },
    { }
    };
    MODULE_DEVICE_TABLE(spi, adau1781_spi_id);

    static const struct of_device_id adau1781_spi_dt_ids[] = {
    { .compatible = "adi,adau1381", },
    { .compatible = "adi,adau1781", },
    { },
    };
    MODULE_DEVICE_TABLE(of, adau1781_spi_dt_ids);

    static struct spi_driver adau1781_spi_driver = {
    .driver = {
    .name = "adau1781",
    .of_match_table = of_match_ptr(adau1781_spi_dt_ids),
    },
    .probe = adau1781_spi_probe,
    .remove = adau1781_spi_remove,
    .id_table = adau1781_spi_id,
    };
    module_spi_driver(adau1781_spi_driver);
    MODULE_DESCRIPTION("ASoC ADAU1381/ADAU1781 CODEC SPI driver");
    MODULE_AUTHOR("Lars-Peter Clausen <lars@metafoo.de>");
    MODULE_LICENSE("GPL");
