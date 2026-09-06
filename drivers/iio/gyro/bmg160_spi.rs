//! Automatically rewritten from C to Rust
//! Source: drivers/iio/gyro/bmg160_spi.c
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

    static const struct regmap_config bmg160_regmap_spi_conf = {
    .reg_bits = 8,
    .val_bits = 8,
    .max_register = 0x3f,
    };
#[no_mangle]
unsafe extern "C" fn bmg160_spi_probe(spi: *mut spi_device) -> c_int {
    static int bmg160_spi_probe(struct spi_device *spi)
    {
    struct regmap *regmap;
    const struct spi_device_id *id = spi_get_device_id(spi);
    regmap = devm_regmap_init_spi(spi, &bmg160_regmap_spi_conf);
    if (IS_ERR(regmap)) {
    dev_err(&spi.dev, "Failed to register spi regmap: %pe\n",
    regmap);
    return PTR_ERR(regmap);
    }
    return bmg160_core_probe(&spi.dev, regmap, spi.irq, id.name);
    }
#[no_mangle]
unsafe extern "C" fn bmg160_spi_remove(spi: *mut spi_device) {
    static void bmg160_spi_remove(struct spi_device *spi)
    {
    bmg160_core_remove(&spi.dev);
    }
    static const struct spi_device_id bmg160_spi_id[] = {
    { .name = "bmg160" },
    { .name = "bmi055_gyro" },
    { .name = "bmi088_gyro" },
    { }
    };
    MODULE_DEVICE_TABLE(spi, bmg160_spi_id);
    static const struct of_device_id bmg160_of_match[] = {
    { .compatible = "bosch,bmg160" },
    { .compatible = "bosch,bmi055_gyro" },
    { .compatible = "bosch,bmi088_gyro" },
    { }
    };
    MODULE_DEVICE_TABLE(of, bmg160_of_match);
    static struct spi_driver bmg160_spi_driver = {
    .driver = {
    .name	= "bmg160_spi",
    .of_match_table = bmg160_of_match,
    .pm	= &bmg160_pm_ops,
    },
    .probe		= bmg160_spi_probe,
    .remove		= bmg160_spi_remove,
    .id_table	= bmg160_spi_id,
    };
    module_spi_driver(bmg160_spi_driver);
    MODULE_AUTHOR("Markus Pargmann <mpa@pengutronix.de>");
    MODULE_LICENSE("GPL v2");
    MODULE_DESCRIPTION("BMG160 SPI Gyro driver");
