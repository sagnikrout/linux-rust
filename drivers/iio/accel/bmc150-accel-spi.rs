//! Automatically rewritten from C to Rust
//! Source: drivers/iio/accel/bmc150-accel-spi.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// 3-axis accelerometer driver supporting SPI Bosch-Sensortec accelerometer chip
// Copyright © 2015 Pengutronix, Markus Pargmann <mpa@pengutronix.de>
//

#[no_mangle]
unsafe extern "C" fn bmc150_accel_probe(spi: *mut spi_device) -> c_int {
    static int bmc150_accel_probe(struct spi_device *spi)
    {
    struct regmap *regmap;
    const char *name = core::ptr::null_mut();
    let mut type: enum bmc150_type = BOSCH_UNKNOWN;
    const struct spi_device_id *id = spi_get_device_id(spi);
    regmap = devm_regmap_init_spi(spi, &bmc150_regmap_conf);
    if (IS_ERR(regmap)) {
    dev_err(&spi.dev, "Failed to initialize spi regmap\n");
    return PTR_ERR(regmap);
    }
    if (id) {
    name = id.name;
    type = id.driver_data;
    }
    return bmc150_accel_core_probe(&spi.dev, regmap, spi.irq, type, name,
    true);
    }
#[no_mangle]
unsafe extern "C" fn bmc150_accel_remove(spi: *mut spi_device) {
    static void bmc150_accel_remove(struct spi_device *spi)
    {
    bmc150_accel_core_remove(&spi.dev);
    }
    static const struct acpi_device_id bmc150_accel_acpi_match[] = {
    {"BMA0255"},
    {"BMA0280"},
    {"BMA222"},
    {"BMA222E"},
    {"BMA250E"},
    {"BMC150A"},
    {"BMI055A"},
    {"BSBA0150"},
    { }
    };
    MODULE_DEVICE_TABLE(acpi, bmc150_accel_acpi_match);
    static const struct spi_device_id bmc150_accel_id[] = {
    { .name = "bma222",       .driver_data = BOSCH_UNKNOWN },
    { .name = "bma222e",      .driver_data = BOSCH_UNKNOWN },
    { .name = "bma250e",      .driver_data = BOSCH_UNKNOWN },
    { .name = "bma253",       .driver_data = BOSCH_UNKNOWN },
    { .name = "bma255",       .driver_data = BOSCH_UNKNOWN },
    { .name = "bma280",       .driver_data = BOSCH_UNKNOWN },
    { .name = "bmc150_accel", .driver_data = BOSCH_UNKNOWN },
    { .name = "bmc156_accel", .driver_data = BOSCH_BMC156 },
    { .name = "bmi055_accel", .driver_data = BOSCH_UNKNOWN },
    { }
    };
    MODULE_DEVICE_TABLE(spi, bmc150_accel_id);
    static struct spi_driver bmc150_accel_driver = {
    .driver = {
    .name	= "bmc150_accel_spi",
    .acpi_match_table = bmc150_accel_acpi_match,
    .pm	= &bmc150_accel_pm_ops,
    },
    .probe		= bmc150_accel_probe,
    .remove		= bmc150_accel_remove,
    .id_table	= bmc150_accel_id,
    };
    module_spi_driver(bmc150_accel_driver);
    MODULE_AUTHOR("Markus Pargmann <mpa@pengutronix.de>");
    MODULE_LICENSE("GPL v2");
    MODULE_DESCRIPTION("BMC150 SPI accelerometer driver");
    MODULE_IMPORT_NS("IIO_BMC150");
