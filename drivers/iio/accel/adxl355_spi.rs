//! Automatically rewritten from C to Rust
//! Source: drivers/iio/accel/adxl355_spi.c
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
// ADXL355 3-Axis Digital Accelerometer SPI driver
//
// Copyright (c) 2021 Puranjay Mohan <puranjay12@gmail.com>
//

    static const struct regmap_config adxl355_spi_regmap_config = {
    .reg_bits = 7,
    .pad_bits = 1,
    .val_bits = 8,
    .read_flag_mask = BIT(0),
    .max_register = 0x2F,
    .rd_table = &adxl355_readable_regs_tbl,
    .wr_table = &adxl355_writeable_regs_tbl,
    };
#[no_mangle]
unsafe extern "C" fn adxl355_spi_probe(spi: *mut spi_device) -> c_int {
    static int adxl355_spi_probe(struct spi_device *spi)
    {
    const struct adxl355_chip_info *chip_data;
    struct device *dev = &spi.dev;
    struct regmap *regmap;
    chip_data = spi_get_device_match_data(spi);
    if (!chip_data)
    return -EINVAL;
    regmap = devm_regmap_init_spi(spi, &adxl355_spi_regmap_config);
    if (IS_ERR(regmap))
    return dev_err_probe(dev, PTR_ERR(regmap), "Error initializing spi regmap\n");
    return adxl355_core_probe(dev, regmap, chip_data);
    }
    static const struct spi_device_id adxl355_spi_id[] = {
    { .name = "adxl355", .driver_data = (kernel_ulong_t)&adxl35x_chip_info[ADXL355] },
    { .name = "adxl359", .driver_data = (kernel_ulong_t)&adxl35x_chip_info[ADXL359] },
    { }
    };
    MODULE_DEVICE_TABLE(spi, adxl355_spi_id);
    static const struct of_device_id adxl355_of_match[] = {
    { .compatible = "adi,adxl355", .data = &adxl35x_chip_info[ADXL355] },
    { .compatible = "adi,adxl359", .data = &adxl35x_chip_info[ADXL359] },
    { }
    };
    MODULE_DEVICE_TABLE(of, adxl355_of_match);
    static struct spi_driver adxl355_spi_driver = {
    .driver = {
    .name	= "adxl355_spi",
    .of_match_table = adxl355_of_match,
    },
    .probe		= adxl355_spi_probe,
    .id_table	= adxl355_spi_id,
    };
    module_spi_driver(adxl355_spi_driver);
    MODULE_AUTHOR("Puranjay Mohan <puranjay12@gmail.com>");
    MODULE_DESCRIPTION("ADXL355 3-Axis Digital Accelerometer SPI driver");
    MODULE_LICENSE("GPL v2");
    MODULE_IMPORT_NS("IIO_ADXL355");
