//! Automatically rewritten from C to Rust
//! Source: drivers/iio/accel/bmi088-accel-spi.c
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
// 3-axis accelerometer driver supporting following Bosch-Sensortec chips:
// - BMI088
// - BMI085
// - BMI090L
//
// Copyright (c) 2018-2020, Topic Embedded Products
//

#[no_mangle]
unsafe extern "C" fn bmi088_regmap_spi_write(context: *mut c_void, data: *const c_void, count: usize) -> c_int {
    static int bmi088_regmap_spi_write(void *context, const void *data, size_t count)
    {
    struct spi_device *spi = context;
// Write register is same as generic SPI
    return spi_write(spi, data, count);
    }
    static int bmi088_regmap_spi_read(void *context, const void *reg,
    size_t reg_size, void *val, size_t val_size)
    {
    struct spi_device *spi = context;
    u8 addr[2];
    addr[0] = *(u8 *)reg;
    addr[0] |= BIT(7); /* Set RW = '1' */
    addr[1] = 0; /* Read requires a dummy byte transfer */
    return spi_write_then_read(spi, addr, sizeof(addr), val, val_size);
    }
    static const struct regmap_bus bmi088_regmap_bus = {
    .write = bmi088_regmap_spi_write,
    .read = bmi088_regmap_spi_read,
    };
#[no_mangle]
unsafe extern "C" fn bmi088_accel_probe(spi: *mut spi_device) -> c_int {
    static int bmi088_accel_probe(struct spi_device *spi)
    {
    struct regmap *regmap;
    const struct spi_device_id *id = spi_get_device_id(spi);
    regmap = devm_regmap_init(&spi.dev, &bmi088_regmap_bus,
    spi, &bmi088_regmap_conf);
    if (IS_ERR(regmap)) {
    dev_err(&spi.dev, "Failed to initialize spi regmap\n");
    return PTR_ERR(regmap);
    }
    return bmi088_accel_core_probe(&spi.dev, regmap, spi.irq,
    id.driver_data);
    }
#[no_mangle]
unsafe extern "C" fn bmi088_accel_remove(spi: *mut spi_device) {
    static void bmi088_accel_remove(struct spi_device *spi)
    {
    bmi088_accel_core_remove(&spi.dev);
    }
    static const struct of_device_id bmi088_of_match[] = {
    { .compatible = "bosch,bmi085-accel" },
    { .compatible = "bosch,bmi088-accel" },
    { .compatible = "bosch,bmi090l-accel" },
    { }
    };
    MODULE_DEVICE_TABLE(of, bmi088_of_match);
    static const struct spi_device_id bmi088_accel_id[] = {
    { .name = "bmi085-accel", .driver_data = BOSCH_BMI085 },
    { .name = "bmi088-accel", .driver_data = BOSCH_BMI088 },
    { .name = "bmi090l-accel", .driver_data = BOSCH_BMI090L },
    { }
    };
    MODULE_DEVICE_TABLE(spi, bmi088_accel_id);
    static struct spi_driver bmi088_accel_driver = {
    .driver = {
    .name	= "bmi088_accel_spi",
    .pm	= pm_ptr(&bmi088_accel_pm_ops),
    .of_match_table = bmi088_of_match,
    },
    .probe		= bmi088_accel_probe,
    .remove		= bmi088_accel_remove,
    .id_table	= bmi088_accel_id,
    };
    module_spi_driver(bmi088_accel_driver);
    MODULE_AUTHOR("Niek van Agt <niek.van.agt@topicproducts.com>");
    MODULE_LICENSE("GPL v2");
    MODULE_DESCRIPTION("BMI088 accelerometer driver (SPI)");
    MODULE_IMPORT_NS("IIO_BMI088");
