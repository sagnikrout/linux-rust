//! Automatically rewritten from C to Rust
//! Source: drivers/iio/imu/inv_icm45600/inv_icm45600_spi.c
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
// Copyright (C) 2025 InvenSense, Inc.

    static const struct regmap_config inv_icm45600_regmap_config = {
    .reg_bits = 8,
    .val_bits = 8,
    };
#[no_mangle]
unsafe extern "C" fn inv_icm45600_spi_bus_setup(st: *mut inv_icm45600_state) -> c_int {
    static int inv_icm45600_spi_bus_setup(struct inv_icm45600_state *st)
    {
// Set slew rates for SPI.
    return regmap_update_bits(st.map, INV_ICM45600_REG_DRIVE_CONFIG0,
    INV_ICM45600_DRIVE_CONFIG0_SPI_MASK,
    FIELD_PREP(INV_ICM45600_DRIVE_CONFIG0_SPI_MASK,
    INV_ICM45600_SPI_SLEW_RATE_5NS));
    }
#[no_mangle]
unsafe extern "C" fn inv_icm45600_probe(spi: *mut spi_device) -> c_int {
    static int inv_icm45600_probe(struct spi_device *spi)
    {
    const struct inv_icm45600_chip_info *chip_info;
    struct regmap *regmap;
    chip_info = spi_get_device_match_data(spi);
    if (!chip_info)
    return -ENODEV;
// Use SPI specific regmap.
    regmap = devm_regmap_init_spi(spi, &inv_icm45600_regmap_config);
    if (IS_ERR(regmap))
    return PTR_ERR(regmap);
    return inv_icm45600_core_probe(regmap, chip_info, true,
    inv_icm45600_spi_bus_setup);
    }
//
// The device id table is used to identify which device is
// supported by this driver.
//
    static const struct spi_device_id inv_icm45600_id[] = {
    { .name = "icm45605", .driver_data = (kernel_ulong_t)&inv_icm45605_chip_info },
    { .name = "icm45606", .driver_data = (kernel_ulong_t)&inv_icm45606_chip_info },
    { .name = "icm45608", .driver_data = (kernel_ulong_t)&inv_icm45608_chip_info },
    { .name = "icm45634", .driver_data = (kernel_ulong_t)&inv_icm45634_chip_info },
    { .name = "icm45686", .driver_data = (kernel_ulong_t)&inv_icm45686_chip_info },
    { .name = "icm45687", .driver_data = (kernel_ulong_t)&inv_icm45687_chip_info },
    { .name = "icm45688p", .driver_data = (kernel_ulong_t)&inv_icm45688p_chip_info },
    { .name = "icm45689", .driver_data = (kernel_ulong_t)&inv_icm45689_chip_info },
    { }
    };
    MODULE_DEVICE_TABLE(spi, inv_icm45600_id);
    static const struct of_device_id inv_icm45600_of_matches[] = {
    {
    .compatible = "invensense,icm45605",
    .data = &inv_icm45605_chip_info,
    }, {
    .compatible = "invensense,icm45606",
    .data = &inv_icm45606_chip_info,
    }, {
    .compatible = "invensense,icm45608",
    .data = &inv_icm45608_chip_info,
    }, {
    .compatible = "invensense,icm45634",
    .data = &inv_icm45634_chip_info,
    }, {
    .compatible = "invensense,icm45686",
    .data = &inv_icm45686_chip_info,
    }, {
    .compatible = "invensense,icm45687",
    .data = &inv_icm45687_chip_info,
    }, {
    .compatible = "invensense,icm45688p",
    .data = &inv_icm45688p_chip_info,
    }, {
    .compatible = "invensense,icm45689",
    .data = &inv_icm45689_chip_info,
    },
    { }
    };
    MODULE_DEVICE_TABLE(of, inv_icm45600_of_matches);
    static struct spi_driver inv_icm45600_driver = {
    .driver = {
    .name = "inv-icm45600-spi",
    .of_match_table = inv_icm45600_of_matches,
    .pm = pm_ptr(&inv_icm45600_pm_ops),
    },
    .id_table = inv_icm45600_id,
    .probe = inv_icm45600_probe,
    };
    module_spi_driver(inv_icm45600_driver);
    MODULE_AUTHOR("InvenSense, Inc.");
    MODULE_DESCRIPTION("InvenSense ICM-456xx SPI driver");
    MODULE_LICENSE("GPL");
    MODULE_IMPORT_NS("IIO_ICM45600");
