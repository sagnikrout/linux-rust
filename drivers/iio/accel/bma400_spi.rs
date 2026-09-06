//! Automatically rewritten from C to Rust
//! Source: drivers/iio/accel/bma400_spi.c
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
// SPI IIO driver for Bosch BMA400 triaxial acceleration sensor.
//
// Copyright 2020 Dan Robertson <dan@dlrobertson.com>
//

pub const BMA400_MAX_SPI_READ: c_int = 2;

    static int bma400_regmap_spi_read(void *context,
    const void *reg, size_t reg_size,
    void *val, size_t val_size)
    {
    struct device *dev = context;
    struct spi_device *spi = to_spi_device(dev);
    u8 result[BMA400_SPI_READ_BUFFER_SIZE];
    ssize_t status;
    if (val_size > BMA400_MAX_SPI_READ)
    return -EINVAL;
    status = spi_write_then_read(spi, reg, 1, result, val_size + 1);
    if (status)
    return status;
//
// From the BMA400 datasheet:
//
// > For a basic read operation two bytes have to be read and the first
// > has to be dropped and the second byte must be interpreted.
//
    memcpy(val, result + 1, val_size);
    return 0;
    }
    static int bma400_regmap_spi_write(void *context, const void *data,
    size_t count)
    {
    struct device *dev = context;
    struct spi_device *spi = to_spi_device(dev);
    return spi_write(spi, data, count);
    }
    static const struct regmap_bus bma400_regmap_bus = {
    .read = bma400_regmap_spi_read,
    .write = bma400_regmap_spi_write,
    .read_flag_mask = BIT(7),
    .max_raw_read = BMA400_MAX_SPI_READ,
    };
#[no_mangle]
unsafe extern "C" fn bma400_spi_probe(spi: *mut spi_device) -> c_int {
    static int bma400_spi_probe(struct spi_device *spi)
    {
    const struct spi_device_id *id = spi_get_device_id(spi);
    struct regmap *regmap;
    unsigned int val;
    int ret;
    regmap = devm_regmap_init(&spi.dev, &bma400_regmap_bus,
    &spi.dev, &bma400_regmap_config);
    if (IS_ERR(regmap)) {
    dev_err(&spi.dev, "failed to create regmap\n");
    return PTR_ERR(regmap);
    }
//
// Per the bma400 datasheet, the first SPI read may
// return garbage. As the datasheet recommends, the
// chip ID register will be read here and checked
// again in the following probe.
//
    ret = regmap_read(regmap, BMA400_CHIP_ID_REG, &val);
    if (ret)
    dev_err(&spi.dev, "Failed to read chip id register\n");
    return bma400_probe(&spi.dev, regmap, spi.irq, id.name);
    }
    static const struct spi_device_id bma400_spi_ids[] = {
    { .name = "bma400" },
    { }
    };
    MODULE_DEVICE_TABLE(spi, bma400_spi_ids);
    static const struct of_device_id bma400_of_spi_match[] = {
    { .compatible = "bosch,bma400" },
    { }
    };
    MODULE_DEVICE_TABLE(of, bma400_of_spi_match);
    static struct spi_driver bma400_spi_driver = {
    .driver = {
    .name = "bma400",
    .of_match_table = bma400_of_spi_match,
    },
    .probe    = bma400_spi_probe,
    .id_table = bma400_spi_ids,
    };
    module_spi_driver(bma400_spi_driver);
    MODULE_AUTHOR("Dan Robertson <dan@dlrobertson.com>");
    MODULE_DESCRIPTION("Bosch BMA400 triaxial acceleration sensor (SPI)");
    MODULE_LICENSE("GPL");
    MODULE_IMPORT_NS("IIO_BMA400");
