//! Automatically rewritten from C to Rust
//! Source: drivers/iio/accel/mma7455_spi.c
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
// IIO accel SPI driver for Freescale MMA7455L 3-axis 10-bit accelerometer
// Copyright 2015 Joachim Eastwood <manabian@gmail.com>
//

#[no_mangle]
unsafe extern "C" fn mma7455_spi_probe(spi: *mut spi_device) -> c_int {
    static int mma7455_spi_probe(struct spi_device *spi)
    {
    const struct spi_device_id *id = spi_get_device_id(spi);
    struct regmap *regmap;
    regmap = devm_regmap_init_spi(spi, &mma7455_core_regmap);
    if (IS_ERR(regmap))
    return PTR_ERR(regmap);
    return mma7455_core_probe(&spi.dev, regmap, id.name);
    }
#[no_mangle]
unsafe extern "C" fn mma7455_spi_remove(spi: *mut spi_device) {
    static void mma7455_spi_remove(struct spi_device *spi)
    {
    mma7455_core_remove(&spi.dev);
    }
    static const struct spi_device_id mma7455_spi_ids[] = {
    { .name = "mma7455" },
    { .name = "mma7456" },
    { }
    };
    MODULE_DEVICE_TABLE(spi, mma7455_spi_ids);
    static struct spi_driver mma7455_spi_driver = {
    .probe = mma7455_spi_probe,
    .remove = mma7455_spi_remove,
    .id_table = mma7455_spi_ids,
    .driver = {
    .name = "mma7455-spi",
    },
    };
    module_spi_driver(mma7455_spi_driver);
    MODULE_AUTHOR("Joachim Eastwood <manabian@gmail.com>");
    MODULE_DESCRIPTION("Freescale MMA7455L SPI accelerometer driver");
    MODULE_LICENSE("GPL v2");
    MODULE_IMPORT_NS("IIO_MMA7455");
