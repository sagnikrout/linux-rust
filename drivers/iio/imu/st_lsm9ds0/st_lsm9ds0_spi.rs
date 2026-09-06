//! Automatically rewritten from C to Rust
//! Source: drivers/iio/imu/st_lsm9ds0/st_lsm9ds0_spi.c
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
// STMicroelectronics LSM9DS0 IMU driver
//
// Copyright (C) 2021, Intel Corporation
//
// Author: Andy Shevchenko <andriy.shevchenko@linux.intel.com>
//

    static const struct of_device_id st_lsm9ds0_of_match[] = {
    {
    .compatible = "st,lsm303d-imu",
    .data = LSM303D_IMU_DEV_NAME,
    },
    {
    .compatible = "st,lsm9ds0-imu",
    .data = LSM9DS0_IMU_DEV_NAME,
    },
    { }
    };
    MODULE_DEVICE_TABLE(of, st_lsm9ds0_of_match);
    static const struct spi_device_id st_lsm9ds0_id_table[] = {
    { .name = LSM303D_IMU_DEV_NAME },
    { .name = LSM9DS0_IMU_DEV_NAME },
    { }
    };
    MODULE_DEVICE_TABLE(spi, st_lsm9ds0_id_table);
    static const struct regmap_config st_lsm9ds0_regmap_config = {
    .reg_bits	= 8,
    .val_bits	= 8,
    .read_flag_mask	= 0xc0,
    };
#[no_mangle]
unsafe extern "C" fn st_lsm9ds0_spi_probe(spi: *mut spi_device) -> c_int {
    static int st_lsm9ds0_spi_probe(struct spi_device *spi)
    {
    struct device *dev = &spi.dev;
    struct st_lsm9ds0 *lsm9ds0;
    struct regmap *regmap;
    st_sensors_dev_name_probe(dev, spi.modalias, sizeof(spi.modalias));
    lsm9ds0 = devm_kzalloc(dev, sizeof(*lsm9ds0), GFP_KERNEL);
    if (!lsm9ds0)
    return -ENOMEM;
    lsm9ds0.dev = dev;
    lsm9ds0.name = spi.modalias;
    lsm9ds0.irq = spi.irq;
    regmap = devm_regmap_init_spi(spi, &st_lsm9ds0_regmap_config);
    if (IS_ERR(regmap))
    return PTR_ERR(regmap);
    spi_set_drvdata(spi, lsm9ds0);
    return st_lsm9ds0_probe(lsm9ds0, regmap);
    }
    static struct spi_driver st_lsm9ds0_driver = {
    .driver = {
    .name = "st-lsm9ds0-spi",
    .of_match_table = st_lsm9ds0_of_match,
    },
    .probe = st_lsm9ds0_spi_probe,
    .id_table = st_lsm9ds0_id_table,
    };
    module_spi_driver(st_lsm9ds0_driver);
    MODULE_AUTHOR("Andy Shevchenko <andriy.shevchenko@linux.intel.com>");
    MODULE_DESCRIPTION("STMicroelectronics LSM9DS0 IMU SPI driver");
    MODULE_LICENSE("GPL v2");
    MODULE_IMPORT_NS("IIO_ST_SENSORS");
