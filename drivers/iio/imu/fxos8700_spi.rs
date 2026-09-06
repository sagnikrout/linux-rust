//! Automatically rewritten from C to Rust
//! Source: drivers/iio/imu/fxos8700_spi.c
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
// FXOS8700 - NXP IMU, SPI bits
//

#[no_mangle]
unsafe extern "C" fn fxos8700_spi_probe(spi: *mut spi_device) -> c_int {
    static int fxos8700_spi_probe(struct spi_device *spi)
    {
    struct regmap *regmap;
    const struct spi_device_id *id = spi_get_device_id(spi);
    regmap = devm_regmap_init_spi(spi, &fxos8700_regmap_config);
    if (IS_ERR(regmap)) {
    dev_err(&spi.dev, "Failed to register spi regmap %ld\n", PTR_ERR(regmap));
    return PTR_ERR(regmap);
    }
    return fxos8700_core_probe(&spi.dev, regmap, id.name, true);
    }
    static const struct spi_device_id fxos8700_spi_id[] = {
    { .name = "fxos8700" },
    { }
    };
    MODULE_DEVICE_TABLE(spi, fxos8700_spi_id);
    static const struct acpi_device_id fxos8700_acpi_match[] = {
    {"FXOS8700", 0},
    { }
    };
    MODULE_DEVICE_TABLE(acpi, fxos8700_acpi_match);
    static const struct of_device_id fxos8700_of_match[] = {
    { .compatible = "nxp,fxos8700" },
    { }
    };
    MODULE_DEVICE_TABLE(of, fxos8700_of_match);
    static struct spi_driver fxos8700_spi_driver = {
    .probe          = fxos8700_spi_probe,
    .id_table       = fxos8700_spi_id,
    .driver = {
    .acpi_match_table       = fxos8700_acpi_match,
    .of_match_table         = fxos8700_of_match,
    .name                   = "fxos8700_spi",
    },
    };
    module_spi_driver(fxos8700_spi_driver);
    MODULE_AUTHOR("Robert Jones <rjones@gateworks.com>");
    MODULE_DESCRIPTION("FXOS8700 SPI driver");
    MODULE_LICENSE("GPL v2");
