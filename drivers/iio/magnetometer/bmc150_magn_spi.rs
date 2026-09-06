//! Automatically rewritten from C to Rust
//! Source: drivers/iio/magnetometer/bmc150_magn_spi.c
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
// 3-axis magnetometer driver support following SPI Bosch-Sensortec chips:
// - BMC150
// - BMC156
// - BMM150
//
// Copyright (c) 2016, Intel Corporation.
//

#[no_mangle]
unsafe extern "C" fn bmc150_magn_spi_probe(spi: *mut spi_device) -> c_int {
    static int bmc150_magn_spi_probe(struct spi_device *spi)
    {
    struct regmap *regmap;
    const struct spi_device_id *id = spi_get_device_id(spi);
    regmap = devm_regmap_init_spi(spi, &bmc150_magn_regmap_config);
    if (IS_ERR(regmap)) {
    dev_err(&spi.dev, "Failed to register spi regmap: %pe\n",
    regmap);
    return PTR_ERR(regmap);
    }
    return bmc150_magn_probe(&spi.dev, regmap, spi.irq, id.name);
    }
#[no_mangle]
unsafe extern "C" fn bmc150_magn_spi_remove(spi: *mut spi_device) {
    static void bmc150_magn_spi_remove(struct spi_device *spi)
    {
    bmc150_magn_remove(&spi.dev);
    }
    static const struct spi_device_id bmc150_magn_spi_id[] = {
    { .name = "bmc150_magn" },
    { .name = "bmc156_magn" },
    { .name = "bmm150_magn" },
    { }
    };
    MODULE_DEVICE_TABLE(spi, bmc150_magn_spi_id);
    static struct spi_driver bmc150_magn_spi_driver = {
    .probe		= bmc150_magn_spi_probe,
    .remove		= bmc150_magn_spi_remove,
    .id_table	= bmc150_magn_spi_id,
    .driver = {
    .name	= "bmc150_magn_spi",
    },
    };
    module_spi_driver(bmc150_magn_spi_driver);
    MODULE_AUTHOR("Daniel Baluta <daniel.baluta@intel.com");
    MODULE_DESCRIPTION("BMC150 magnetometer SPI driver");
    MODULE_LICENSE("GPL v2");
    MODULE_IMPORT_NS("IIO_BMC150_MAGN");
