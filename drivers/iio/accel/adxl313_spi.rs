//! Automatically rewritten from C to Rust
//! Source: drivers/iio/accel/adxl313_spi.c
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
// ADXL313 3-Axis Digital Accelerometer
//
// Copyright (c) 2021 Lucas Stankus <lucas.p.stankus@gmail.com>
//
// Datasheet: https://www.analog.com/media/en/technical-documentation/data-sheets/ADXL313.pdf
//

    static const struct regmap_config adxl31x_spi_regmap_config[] = {
    [ADXL312] = {
    .reg_bits	= 8,
    .val_bits	= 8,
    .rd_table	= &adxl312_readable_regs_table,
    .wr_table	= &adxl312_writable_regs_table,
    .max_register	= 0x39,
// Setting bits 7 and 6 enables multiple-byte read
    .read_flag_mask	= BIT(7) | BIT(6),
    .volatile_reg	= adxl313_is_volatile_reg,
    .cache_type	= REGCACHE_MAPLE,
    },
    [ADXL313] = {
    .reg_bits	= 8,
    .val_bits	= 8,
    .rd_table	= &adxl313_readable_regs_table,
    .wr_table	= &adxl313_writable_regs_table,
    .max_register	= 0x39,
// Setting bits 7 and 6 enables multiple-byte read
    .read_flag_mask	= BIT(7) | BIT(6),
    .volatile_reg	= adxl313_is_volatile_reg,
    .cache_type	= REGCACHE_MAPLE,
    },
    [ADXL314] = {
    .reg_bits	= 8,
    .val_bits	= 8,
    .rd_table	= &adxl314_readable_regs_table,
    .wr_table	= &adxl314_writable_regs_table,
    .max_register	= 0x39,
// Setting bits 7 and 6 enables multiple-byte read
    .read_flag_mask	= BIT(7) | BIT(6),
    .volatile_reg	= adxl313_is_volatile_reg,
    .cache_type	= REGCACHE_MAPLE,
    },
    };
#[no_mangle]
unsafe extern "C" fn adxl313_spi_setup(dev: *mut device, regmap: *mut regmap) -> c_int {
    static int adxl313_spi_setup(struct device *dev, struct regmap *regmap)
    {
    struct spi_device *spi = container_of(dev, struct spi_device, dev);
    int ret;
    if (spi.mode & SPI_3WIRE) {
    ret = regmap_write(regmap, ADXL313_REG_DATA_FORMAT,
    ADXL313_SPI_3WIRE);
    if (ret)
    return ret;
    }
    return regmap_update_bits(regmap, ADXL313_REG_POWER_CTL,
    ADXL313_I2C_DISABLE, ADXL313_I2C_DISABLE);
    }
#[no_mangle]
unsafe extern "C" fn adxl313_spi_probe(spi: *mut spi_device) -> c_int {
    static int adxl313_spi_probe(struct spi_device *spi)
    {
    const struct adxl313_chip_info *chip_data;
    struct device *dev = &spi.dev;
    struct regmap *regmap;
    int ret;
    spi.mode |= SPI_MODE_3;
    ret = spi_setup(spi);
    if (ret)
    return ret;
    chip_data = spi_get_device_match_data(spi);
    regmap = devm_regmap_init_spi(spi,
    &adxl31x_spi_regmap_config[chip_data.type]);
    if (IS_ERR(regmap))
    return dev_err_probe(dev, PTR_ERR(regmap), "Error initializing spi regmap\n");
    return adxl313_core_probe(dev, regmap, chip_data, &adxl313_spi_setup);
    }
    static const struct spi_device_id adxl313_spi_id[] = {
    { .name = "adxl312", .driver_data = (kernel_ulong_t)&adxl31x_chip_info[ADXL312] },
    { .name = "adxl313", .driver_data = (kernel_ulong_t)&adxl31x_chip_info[ADXL313] },
    { .name = "adxl314", .driver_data = (kernel_ulong_t)&adxl31x_chip_info[ADXL314] },
    { }
    };
    MODULE_DEVICE_TABLE(spi, adxl313_spi_id);
    static const struct of_device_id adxl313_of_match[] = {
    { .compatible = "adi,adxl312", .data = &adxl31x_chip_info[ADXL312] },
    { .compatible = "adi,adxl313", .data = &adxl31x_chip_info[ADXL313] },
    { .compatible = "adi,adxl314", .data = &adxl31x_chip_info[ADXL314] },
    { }
    };
    MODULE_DEVICE_TABLE(of, adxl313_of_match);
    static struct spi_driver adxl313_spi_driver = {
    .driver = {
    .name	= "adxl313_spi",
    .of_match_table = adxl313_of_match,
    },
    .probe		= adxl313_spi_probe,
    .id_table	= adxl313_spi_id,
    };
    module_spi_driver(adxl313_spi_driver);
    MODULE_AUTHOR("Lucas Stankus <lucas.p.stankus@gmail.com>");
    MODULE_DESCRIPTION("ADXL313 3-Axis Digital Accelerometer SPI driver");
    MODULE_LICENSE("GPL v2");
    MODULE_IMPORT_NS("IIO_ADXL313");
