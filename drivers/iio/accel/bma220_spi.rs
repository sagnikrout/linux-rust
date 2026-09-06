//! Automatically rewritten from C to Rust
//! Source: drivers/iio/accel/bma220_spi.c
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
// BMA220 Digital triaxial acceleration sensor driver
//
// Copyright (c) 2016,2020 Intel Corporation.
//

#[no_mangle]
unsafe extern "C" fn bma220_spi_probe(spi: *mut spi_device) -> c_int {
    static int bma220_spi_probe(struct spi_device *spi)
    {
    struct regmap *regmap;
    regmap = devm_regmap_init_spi(spi, &bma220_spi_regmap_config);
    if (IS_ERR(regmap))
    return dev_err_probe(&spi.dev, PTR_ERR(regmap),
    "failed to create regmap\n");
    return bma220_common_probe(&spi.dev, regmap, spi.irq);
    }
    static const struct spi_device_id bma220_spi_id[] = {
    { .name = "bma220" },
    { }
    };
    static const struct acpi_device_id bma220_acpi_id[] = {
    { "BMA0220", 0 },
    { }
    };
    MODULE_DEVICE_TABLE(spi, bma220_spi_id);
    static const struct of_device_id bma220_of_spi_match[] = {
    { .compatible = "bosch,bma220" },
    { }
    };
    MODULE_DEVICE_TABLE(of, bma220_of_spi_match);
    static struct spi_driver bma220_spi_driver = {
    .driver = {
    .name = "bma220_spi",
    .pm = pm_sleep_ptr(&bma220_pm_ops),
    .of_match_table = bma220_of_spi_match,
    .acpi_match_table = bma220_acpi_id,
    },
    .probe =            bma220_spi_probe,
    .id_table =         bma220_spi_id,
    };
    module_spi_driver(bma220_spi_driver);
    MODULE_AUTHOR("Tiberiu Breana <tiberiu.a.breana@intel.com>");
    MODULE_DESCRIPTION("BMA220 triaxial acceleration sensor spi driver");
    MODULE_LICENSE("GPL");
    MODULE_IMPORT_NS("IIO_BOSCH_BMA220");
