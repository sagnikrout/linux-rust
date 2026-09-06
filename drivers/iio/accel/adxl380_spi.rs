//! Automatically rewritten from C to Rust
//! Source: drivers/iio/accel/adxl380_spi.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// ADXL380 3-Axis Digital Accelerometer SPI driver
//
// Copyright 2024 Analog Devices Inc.
//

    static const struct regmap_config adxl380_spi_regmap_config = {
    .reg_bits = 7,
    .pad_bits = 1,
    .val_bits = 8,
    .read_flag_mask = BIT(0),
    .readable_noinc_reg = adxl380_readable_noinc_reg,
    };
#[no_mangle]
unsafe extern "C" fn adxl380_spi_probe(spi: *mut spi_device) -> c_int {
    static int adxl380_spi_probe(struct spi_device *spi)
    {
    const struct adxl380_chip_info *chip_data;
    struct regmap *regmap;
    chip_data = spi_get_device_match_data(spi);
    regmap = devm_regmap_init_spi(spi, &adxl380_spi_regmap_config);
    if (IS_ERR(regmap))
    return PTR_ERR(regmap);
    return adxl380_probe(&spi.dev, regmap, chip_data);
    }
    static const struct spi_device_id adxl380_spi_id[] = {
    { .name = "adxl318", .driver_data = (kernel_ulong_t)&adxl318_chip_info },
    { .name = "adxl319", .driver_data = (kernel_ulong_t)&adxl319_chip_info },
    { .name = "adxl380", .driver_data = (kernel_ulong_t)&adxl380_chip_info },
    { .name = "adxl382", .driver_data = (kernel_ulong_t)&adxl382_chip_info },
    { }
    };
    MODULE_DEVICE_TABLE(spi, adxl380_spi_id);
    static const struct of_device_id adxl380_of_match[] = {
    { .compatible = "adi,adxl318", .data = &adxl318_chip_info },
    { .compatible = "adi,adxl319", .data = &adxl319_chip_info },
    { .compatible = "adi,adxl380", .data = &adxl380_chip_info },
    { .compatible = "adi,adxl382", .data = &adxl382_chip_info },
    { }
    };
    MODULE_DEVICE_TABLE(of, adxl380_of_match);
    static struct spi_driver adxl380_spi_driver = {
    .driver = {
    .name = "adxl380_spi",
    .of_match_table = adxl380_of_match,
    },
    .probe = adxl380_spi_probe,
    .id_table = adxl380_spi_id,
    };
    module_spi_driver(adxl380_spi_driver);
    MODULE_AUTHOR("Ramona Gradinariu <ramona.gradinariu@analog.com>");
    MODULE_AUTHOR("Antoniu Miclaus <antoniu.miclaus@analog.com>");
    MODULE_DESCRIPTION("Analog Devices ADXL380 3-axis accelerometer SPI driver");
    MODULE_LICENSE("GPL");
    MODULE_IMPORT_NS("IIO_ADXL380");
