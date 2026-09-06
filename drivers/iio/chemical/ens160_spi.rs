//! Automatically rewritten from C to Rust
//! Source: drivers/iio/chemical/ens160_spi.c
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
// ScioSense ENS160 multi-gas sensor SPI driver
//
// Copyright (c) 2024 Gustavo Silva <gustavograzs@gmail.com>
//

    static const struct regmap_config ens160_regmap_spi_conf = {
    .reg_bits = 8,
    .val_bits = 8,
    .reg_shift = -1,
    .read_flag_mask = ENS160_SPI_READ,
    };
#[no_mangle]
unsafe extern "C" fn ens160_spi_probe(spi: *mut spi_device) -> c_int {
    static int ens160_spi_probe(struct spi_device *spi)
    {
    struct regmap *regmap;
    regmap = devm_regmap_init_spi(spi, &ens160_regmap_spi_conf);
    if (IS_ERR(regmap))
    return dev_err_probe(&spi.dev, PTR_ERR(regmap),
    "Failed to register spi regmap\n");
    return devm_ens160_core_probe(&spi.dev, regmap, spi.irq, "ens160");
    }
    static const struct of_device_id ens160_spi_of_match[] = {
    { .compatible = "sciosense,ens160" },
    { }
    };
    MODULE_DEVICE_TABLE(of, ens160_spi_of_match);
    static const struct spi_device_id ens160_spi_id[] = {
    { .name = "ens160" },
    { }
    };
    MODULE_DEVICE_TABLE(spi, ens160_spi_id);
    static struct spi_driver ens160_spi_driver = {
    .driver = {
    .name	= "ens160",
    .of_match_table = ens160_spi_of_match,
    .pm = pm_sleep_ptr(&ens160_pm_ops),
    },
    .probe		= ens160_spi_probe,
    .id_table	= ens160_spi_id,
    };
    module_spi_driver(ens160_spi_driver);
    MODULE_AUTHOR("Gustavo Silva <gustavograzs@gmail.com>");
    MODULE_DESCRIPTION("ScioSense ENS160 SPI driver");
    MODULE_LICENSE("GPL v2");
    MODULE_IMPORT_NS("IIO_ENS160");
