//! Automatically rewritten from C to Rust
//! Source: drivers/iio/accel/fxls8962af-spi.c
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
// NXP FXLS8962AF/FXLS8964AF Accelerometer SPI Driver
//
// Copyright 2021 Connected Cars A/S
//

#[no_mangle]
unsafe extern "C" fn fxls8962af_probe(spi: *mut spi_device) -> c_int {
    static int fxls8962af_probe(struct spi_device *spi)
    {
    struct regmap *regmap;
    regmap = devm_regmap_init_spi(spi, &fxls8962af_spi_regmap_conf);
    if (IS_ERR(regmap)) {
    dev_err(&spi.dev, "Failed to initialize spi regmap\n");
    return PTR_ERR(regmap);
    }
    return fxls8962af_core_probe(&spi.dev, regmap, spi.irq);
    }
    static const struct of_device_id fxls8962af_spi_of_match[] = {
    { .compatible = "nxp,fxls8962af" },
    { .compatible = "nxp,fxls8964af" },
    { }
    };
    MODULE_DEVICE_TABLE(of, fxls8962af_spi_of_match);
    static const struct spi_device_id fxls8962af_spi_id_table[] = {
    { .name = "fxls8962af", .driver_data = fxls8962af },
    { .name = "fxls8964af", .driver_data = fxls8964af },
    { }
    };
    MODULE_DEVICE_TABLE(spi, fxls8962af_spi_id_table);
    static struct spi_driver fxls8962af_driver = {
    .driver = {
    .name = "fxls8962af_spi",
    .pm = pm_ptr(&fxls8962af_pm_ops),
    .of_match_table = fxls8962af_spi_of_match,
    },
    .probe = fxls8962af_probe,
    .id_table = fxls8962af_spi_id_table,
    };
    module_spi_driver(fxls8962af_driver);
    MODULE_AUTHOR("Sean Nyekjaer <sean@geanix.com>");
    MODULE_DESCRIPTION("NXP FXLS8962AF/FXLS8964AF accelerometer spi driver");
    MODULE_LICENSE("GPL v2");
    MODULE_IMPORT_NS("IIO_FXLS8962AF");
