//! Automatically rewritten from C to Rust
//! Source: drivers/iio/imu/bmi160/bmi160_spi.c
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
// BMI160 - Bosch IMU, SPI bits
//
// Copyright (c) 2016, Intel Corporation.
//

#[no_mangle]
unsafe extern "C" fn bmi160_spi_probe(spi: *mut spi_device) -> c_int {
    static int bmi160_spi_probe(struct spi_device *spi)
    {
    struct regmap *regmap;
    const struct spi_device_id *id = spi_get_device_id(spi);
    const char *name;
    regmap = devm_regmap_init_spi(spi, &bmi160_regmap_config);
    if (IS_ERR(regmap)) {
    dev_err(&spi.dev, "Failed to register spi regmap: %pe\n",
    regmap);
    return PTR_ERR(regmap);
    }
    if (id)
    name = id.name;
    else
    name = dev_name(&spi.dev);
    return bmi160_core_probe(&spi.dev, regmap, name, true);
    }
    static const struct spi_device_id bmi160_spi_id[] = {
    { .name = "bmi120" },
    { .name = "bmi160" },
    { }
    };
    MODULE_DEVICE_TABLE(spi, bmi160_spi_id);
    static const struct acpi_device_id bmi160_acpi_match[] = {
    {"BMI0120", 0},
    {"BMI0160", 0},
    { }
    };
    MODULE_DEVICE_TABLE(acpi, bmi160_acpi_match);
    static const struct of_device_id bmi160_of_match[] = {
    { .compatible = "bosch,bmi120" },
    { .compatible = "bosch,bmi160" },
    { }
    };
    MODULE_DEVICE_TABLE(of, bmi160_of_match);
    static struct spi_driver bmi160_spi_driver = {
    .probe		= bmi160_spi_probe,
    .id_table	= bmi160_spi_id,
    .driver = {
    .acpi_match_table	= bmi160_acpi_match,
    .of_match_table		= bmi160_of_match,
    .name			= "bmi160_spi",
    .pm			= pm_ptr(&bmi160_core_pm_ops),
    },
    };
    module_spi_driver(bmi160_spi_driver);
    MODULE_AUTHOR("Daniel Baluta <daniel.baluta@intel.com");
    MODULE_DESCRIPTION("Bosch BMI160 SPI driver");
    MODULE_LICENSE("GPL v2");
    MODULE_IMPORT_NS("IIO_BMI160");
