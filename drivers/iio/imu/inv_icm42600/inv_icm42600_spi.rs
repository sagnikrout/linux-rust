//! Automatically rewritten from C to Rust
//! Source: drivers/iio/imu/inv_icm42600/inv_icm42600_spi.c
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
// Copyright (C) 2020 InvenSense, Inc.
//

#[no_mangle]
unsafe extern "C" fn inv_icm42600_spi_bus_setup(st: *mut inv_icm42600_state) -> c_int {
    static int inv_icm42600_spi_bus_setup(struct inv_icm42600_state *st)
    {
    unsigned int mask, val;
    int ret;
// setup interface registers
    val = INV_ICM42600_INTF_CONFIG6_I3C_EN |
    INV_ICM42600_INTF_CONFIG6_I3C_SDR_EN |
    INV_ICM42600_INTF_CONFIG6_I3C_DDR_EN;
    ret = regmap_update_bits(st.map, INV_ICM42600_REG_INTF_CONFIG6,
    INV_ICM42600_INTF_CONFIG6_MASK, val);
    if (ret)
    return ret;
    ret = regmap_clear_bits(st.map, INV_ICM42600_REG_INTF_CONFIG4,
    INV_ICM42600_INTF_CONFIG4_I3C_BUS_ONLY);
    if (ret)
    return ret;
// set slew rates for I2C and SPI
    mask = INV_ICM42600_DRIVE_CONFIG_I2C_MASK |
    INV_ICM42600_DRIVE_CONFIG_SPI_MASK;
    val = INV_ICM42600_DRIVE_CONFIG_I2C(INV_ICM42600_SLEW_RATE_20_60NS) |
    INV_ICM42600_DRIVE_CONFIG_SPI(INV_ICM42600_SLEW_RATE_INF_2NS);
    ret = regmap_update_bits(st.map, INV_ICM42600_REG_DRIVE_CONFIG,
    mask, val);
    if (ret)
    return ret;
// disable i2c bus
    return regmap_update_bits(st.map, INV_ICM42600_REG_INTF_CONFIG0,
    INV_ICM42600_INTF_CONFIG0_UI_SIFS_CFG_MASK,
    INV_ICM42600_INTF_CONFIG0_UI_SIFS_CFG_I2C_DIS);
    }
#[no_mangle]
unsafe extern "C" fn inv_icm42600_probe(spi: *mut spi_device) -> c_int {
    static int inv_icm42600_probe(struct spi_device *spi)
    {
    const void *match;
    enum inv_icm42600_chip chip;
    struct regmap *regmap;
    match = device_get_match_data(&spi.dev);
    if (!match)
    return -EINVAL;
    chip = (uintptr_t)match;
// use SPI specific regmap
    regmap = devm_regmap_init_spi(spi, &inv_icm42600_spi_regmap_config);
    if (IS_ERR(regmap))
    return PTR_ERR(regmap);
    return inv_icm42600_core_probe(regmap, chip, inv_icm42600_spi_bus_setup);
    }
//
// device id table is used to identify what device can be
// supported by this driver
//
    static const struct spi_device_id inv_icm42600_id[] = {
    { .name = "icm42600", .driver_data = INV_CHIP_ICM42600 },
    { .name = "icm42602", .driver_data = INV_CHIP_ICM42602 },
    { .name = "icm42605", .driver_data = INV_CHIP_ICM42605 },
    { .name = "icm42686", .driver_data = INV_CHIP_ICM42686 },
    { .name = "icm42622", .driver_data = INV_CHIP_ICM42622 },
    { .name = "icm42688", .driver_data = INV_CHIP_ICM42688 },
    { .name = "icm42631", .driver_data = INV_CHIP_ICM42631 },
    { }
    };
    MODULE_DEVICE_TABLE(spi, inv_icm42600_id);
    static const struct of_device_id inv_icm42600_of_matches[] = {
    {
    .compatible = "invensense,icm42600",
    .data = (void *)INV_CHIP_ICM42600,
    }, {
    .compatible = "invensense,icm42602",
    .data = (void *)INV_CHIP_ICM42602,
    }, {
    .compatible = "invensense,icm42605",
    .data = (void *)INV_CHIP_ICM42605,
    }, {
    .compatible = "invensense,icm42686",
    .data = (void *)INV_CHIP_ICM42686,
    }, {
    .compatible = "invensense,icm42622",
    .data = (void *)INV_CHIP_ICM42622,
    }, {
    .compatible = "invensense,icm42688",
    .data = (void *)INV_CHIP_ICM42688,
    }, {
    .compatible = "invensense,icm42631",
    .data = (void *)INV_CHIP_ICM42631,
    },
    { }
    };
    MODULE_DEVICE_TABLE(of, inv_icm42600_of_matches);
    static struct spi_driver inv_icm42600_driver = {
    .driver = {
    .name = "inv-icm42600-spi",
    .of_match_table = inv_icm42600_of_matches,
    .pm = pm_ptr(&inv_icm42600_pm_ops),
    },
    .id_table = inv_icm42600_id,
    .probe = inv_icm42600_probe,
    };
    module_spi_driver(inv_icm42600_driver);
    MODULE_AUTHOR("InvenSense, Inc.");
    MODULE_DESCRIPTION("InvenSense ICM-426xx SPI driver");
    MODULE_LICENSE("GPL");
    MODULE_IMPORT_NS("IIO_ICM42600");
