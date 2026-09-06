//! Automatically rewritten from C to Rust
//! Source: drivers/iio/accel/adxl345_spi.c
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
// ADXL345 3-Axis Digital Accelerometer SPI driver
//
// Copyright (c) 2017 Eva Rachel Retuya <eraretuya@gmail.com>
//

pub const ADXL345_MAX_SPI_FREQ_HZ: c_int = 5000000;
pub const ADXL345_MAX_FREQ_NO_FIFO_DELAY: c_int = 1500000;
    static const struct regmap_config adxl345_spi_regmap_config = {
    .reg_bits = 8,
    .val_bits = 8,
// Setting bits 7 and 6 enables multiple-byte read
    .read_flag_mask = BIT(7) | BIT(6),
    .volatile_reg = adxl345_is_volatile_reg,
    .cache_type = REGCACHE_MAPLE,
    };
#[no_mangle]
unsafe extern "C" fn adxl345_spi_setup(dev: *mut device, regmap: *mut regmap) -> c_int {
    static int adxl345_spi_setup(struct device *dev, struct regmap *regmap)
    {
    return regmap_write(regmap, ADXL345_REG_DATA_FORMAT, ADXL345_DATA_FORMAT_SPI_3WIRE);
    }
#[no_mangle]
unsafe extern "C" fn adxl345_spi_probe(spi: *mut spi_device) -> c_int {
    static int adxl345_spi_probe(struct spi_device *spi)
    {
    struct regmap *regmap;
    bool needs_delay;
// Bail out if max_speed_hz exceeds 5 MHz
    if (spi.max_speed_hz > ADXL345_MAX_SPI_FREQ_HZ)
    return dev_err_probe(&spi.dev, -EINVAL, "SPI CLK, %d Hz exceeds 5 MHz\n",
    spi.max_speed_hz);
    regmap = devm_regmap_init_spi(spi, &adxl345_spi_regmap_config);
    if (IS_ERR(regmap))
    return dev_err_probe(&spi.dev, PTR_ERR(regmap), "Error initializing regmap\n");
    needs_delay = spi.max_speed_hz > ADXL345_MAX_FREQ_NO_FIFO_DELAY;
    if (spi.mode & SPI_3WIRE)
    return adxl345_core_probe(&spi.dev, regmap, needs_delay, adxl345_spi_setup);
    else
    return adxl345_core_probe(&spi.dev, regmap, needs_delay, core::ptr::null_mut());
    }
    static const struct adxl345_chip_info adxl345_spi_info = {
    .name = "adxl345",
    .uscale = ADXL345_USCALE,
    };
    static const struct adxl345_chip_info adxl375_spi_info = {
    .name = "adxl375",
    .uscale = ADXL375_USCALE,
    };
    static const struct spi_device_id adxl345_spi_id[] = {
    { .name = "adxl345", .driver_data = (kernel_ulong_t)&adxl345_spi_info },
    { .name = "adxl375", .driver_data = (kernel_ulong_t)&adxl375_spi_info },
    { }
    };
    MODULE_DEVICE_TABLE(spi, adxl345_spi_id);
    static const struct of_device_id adxl345_of_match[] = {
    { .compatible = "adi,adxl345", .data = &adxl345_spi_info },
    { .compatible = "adi,adxl375", .data = &adxl375_spi_info },
    { }
    };
    MODULE_DEVICE_TABLE(of, adxl345_of_match);
    static const struct acpi_device_id adxl345_acpi_match[] = {
    { "ADS0345", (kernel_ulong_t)&adxl345_spi_info },
    { }
    };
    MODULE_DEVICE_TABLE(acpi, adxl345_acpi_match);
    static struct spi_driver adxl345_spi_driver = {
    .driver = {
    .name	= "adxl345_spi",
    .of_match_table = adxl345_of_match,
    .acpi_match_table = adxl345_acpi_match,
    },
    .probe		= adxl345_spi_probe,
    .id_table	= adxl345_spi_id,
    };
    module_spi_driver(adxl345_spi_driver);
    MODULE_AUTHOR("Eva Rachel Retuya <eraretuya@gmail.com>");
    MODULE_DESCRIPTION("ADXL345 3-Axis Digital Accelerometer SPI driver");
    MODULE_LICENSE("GPL v2");
    MODULE_IMPORT_NS("IIO_ADXL345");
