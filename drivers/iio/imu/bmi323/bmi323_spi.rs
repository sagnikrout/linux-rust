//! Automatically rewritten from C to Rust
//! Source: drivers/iio/imu/bmi323/bmi323_spi.c
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


// SPDX-License-Identifier: GPL-2.0
//
// SPI driver for Bosch BMI323 6-Axis IMU.
//
// Copyright (C) 2023, Jagath Jog J <jagathjog1996@gmail.com>
//

//
// From BMI323 datasheet section 4: Notes on the Serial Interface Support.
// Each SPI register read operation requires to read one dummy byte before
// the actual payload.
//
    static int bmi323_regmap_spi_read(void *context, const void *reg_buf,
    size_t reg_size, void *val_buf,
    size_t val_size)
    {
    struct spi_device *spi = context;
    return spi_write_then_read(spi, reg_buf, reg_size, val_buf, val_size);
    }
    static int bmi323_regmap_spi_write(void *context, const void *data,
    size_t count)
    {
    struct spi_device *spi = context;
    u8 *data_buff = (u8 *)data;
    data_buff[1] = data_buff[0];
    return spi_write(spi, data_buff + 1, count - 1);
    }
    static const struct regmap_bus bmi323_regmap_bus = {
    .read = bmi323_regmap_spi_read,
    .write = bmi323_regmap_spi_write,
    };
    static const struct regmap_config bmi323_spi_regmap_config = {
    .reg_bits = 8,
    .val_bits = 16,
    .pad_bits = 8,
    .read_flag_mask = BIT(7),
    .max_register = BMI323_CFG_RES_REG,
    .val_format_endian = REGMAP_ENDIAN_LITTLE,
    };
#[no_mangle]
unsafe extern "C" fn bmi323_spi_probe(spi: *mut spi_device) -> c_int {
    static int bmi323_spi_probe(struct spi_device *spi)
    {
    struct device *dev = &spi.dev;
    struct regmap *regmap;
    regmap = devm_regmap_init(dev, &bmi323_regmap_bus, dev,
    &bmi323_spi_regmap_config);
    if (IS_ERR(regmap))
    return dev_err_probe(dev, PTR_ERR(regmap),
    "Failed to initialize SPI Regmap\n");
    return bmi323_core_probe(dev);
    }
    static const struct spi_device_id bmi323_spi_ids[] = {
    { .name = "bmi323" },
    { }
    };
    MODULE_DEVICE_TABLE(spi, bmi323_spi_ids);
    static const struct of_device_id bmi323_of_spi_match[] = {
    { .compatible = "bosch,bmi323" },
    { }
    };
    MODULE_DEVICE_TABLE(of, bmi323_of_spi_match);
    static struct spi_driver bmi323_spi_driver = {
    .driver = {
    .name = "bmi323",
    .pm = pm_ptr(&bmi323_core_pm_ops),
    .of_match_table = bmi323_of_spi_match,
    },
    .probe = bmi323_spi_probe,
    .id_table = bmi323_spi_ids,
    };
    module_spi_driver(bmi323_spi_driver);
    MODULE_DESCRIPTION("Bosch BMI323 IMU driver");
    MODULE_AUTHOR("Jagath Jog J <jagathjog1996@gmail.com>");
    MODULE_LICENSE("GPL");
    MODULE_IMPORT_NS("IIO_BMI323");
