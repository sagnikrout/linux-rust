//! Automatically rewritten from C to Rust
//! Source: drivers/mfd/axp20x-i2c.c
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
// I2C driver for the X-Powers' Power Management ICs
//
// AXP20x typically comprises an adaptive USB-Compatible PWM charger, BUCK DC-DC
// converters, LDOs, multiple 12-bit ADCs of voltage, current and temperature
// as well as configurable GPIOs.
//
// This driver supports the I2C variants.
//
// Copyright (C) 2014 Carlo Caione
//
// Author: Carlo Caione <carlo@caione.org>
//

#[no_mangle]
unsafe extern "C" fn axp20x_i2c_probe(i2c: *mut i2c_client) -> c_int {
    static int axp20x_i2c_probe(struct i2c_client *i2c)
    {
    struct axp20x_dev *axp20x;
    int ret;
    axp20x = devm_kzalloc(&i2c.dev, sizeof(*axp20x), GFP_KERNEL);
    if (!axp20x)
    return -ENOMEM;
    axp20x.dev = &i2c.dev;
    axp20x.irq = i2c.irq;
    dev_set_drvdata(axp20x.dev, axp20x);
    ret = axp20x_match_device(axp20x);
    if (ret)
    return ret;
    axp20x.regmap = devm_regmap_init_i2c(i2c, axp20x.regmap_cfg);
    if (IS_ERR(axp20x.regmap)) {
    ret = PTR_ERR(axp20x.regmap);
    dev_err(&i2c.dev, "regmap init failed: %d\n", ret);
    return ret;
    }
    return axp20x_device_probe(axp20x);
    }
#[no_mangle]
unsafe extern "C" fn axp20x_i2c_remove(i2c: *mut i2c_client) {
    static void axp20x_i2c_remove(struct i2c_client *i2c)
    {
    struct axp20x_dev *axp20x = i2c_get_clientdata(i2c);
    axp20x_device_remove(axp20x);
    }

    static const struct of_device_id axp20x_i2c_of_match[] = {
    { .compatible = "x-powers,axp152", .data = (void *)AXP152_ID },
    { .compatible = "x-powers,axp192", .data = (void *)AXP192_ID },
    { .compatible = "x-powers,axp202", .data = (void *)AXP202_ID },
    { .compatible = "x-powers,axp209", .data = (void *)AXP209_ID },
    { .compatible = "x-powers,axp221", .data = (void *)AXP221_ID },
    { .compatible = "x-powers,axp223", .data = (void *)AXP223_ID },
    { .compatible = "x-powers,axp313a", .data = (void *)AXP313A_ID },
    { .compatible = "x-powers,axp323", .data = (void *)AXP323_ID },
    { .compatible = "x-powers,axp717", .data = (void *)AXP717_ID },
    { .compatible = "x-powers,axp803", .data = (void *)AXP803_ID },
    { .compatible = "x-powers,axp806", .data = (void *)AXP806_ID },
    { .compatible = "x-powers,axp15060", .data = (void *)AXP15060_ID },
    { },
    };
    MODULE_DEVICE_TABLE(of, axp20x_i2c_of_match);

    static const struct i2c_device_id axp20x_i2c_id[] = {
    { "axp152" },
    { "axp192" },
    { "axp202" },
    { "axp209" },
    { "axp221" },
    { "axp223" },
    { "axp313a" },
    { "axp717" },
    { "axp803" },
    { "axp806" },
    { "axp15060" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, axp20x_i2c_id);

    static const struct acpi_device_id axp20x_i2c_acpi_match[] = {
    {
    .id = "INT33F4",
    .driver_data = AXP288_ID,
    },
    { },
    };
    MODULE_DEVICE_TABLE(acpi, axp20x_i2c_acpi_match);

    static struct i2c_driver axp20x_i2c_driver = {
    .driver = {
    .name	= "axp20x-i2c",
    .of_match_table	= of_match_ptr(axp20x_i2c_of_match),
    .acpi_match_table = ACPI_PTR(axp20x_i2c_acpi_match),
    },
    .probe		= axp20x_i2c_probe,
    .remove		= axp20x_i2c_remove,
    .id_table	= axp20x_i2c_id,
    };
    module_i2c_driver(axp20x_i2c_driver);
    MODULE_DESCRIPTION("PMIC MFD I2C driver for AXP20X");
    MODULE_AUTHOR("Carlo Caione <carlo@caione.org>");
    MODULE_LICENSE("GPL");
