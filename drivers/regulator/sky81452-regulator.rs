//! Automatically rewritten from C to Rust
//! Source: drivers/regulator/sky81452-regulator.c
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
// sky81452-regulator.c	SKY81452 regulator driver
//
// Copyright 2014 Skyworks Solutions Inc.
// Author : Gyungoh Yoo <jack.yoo@skyworksinc.com>

// registers
pub const SKY81452_REG1: c_uint = 0x01;
pub const SKY81452_REG3: c_uint = 0x03;
// bit mask
pub const SKY81452_LEN: c_uint = 0x40;
pub const SKY81452_LOUT: c_uint = 0x1F;
    static const struct regulator_ops sky81452_reg_ops = {
    .list_voltage = regulator_list_voltage_linear_range,
    .map_voltage = regulator_map_voltage_linear_range,
    .get_voltage_sel = regulator_get_voltage_sel_regmap,
    .set_voltage_sel = regulator_set_voltage_sel_regmap,
    .enable = regulator_enable_regmap,
    .disable = regulator_disable_regmap,
    .is_enabled = regulator_is_enabled_regmap,
    };
    static const struct linear_range sky81452_reg_ranges[] = {
    REGULATOR_LINEAR_RANGE(4500000, 0, 14, 250000),
    REGULATOR_LINEAR_RANGE(9000000, 15, 31, 1000000),
    };
    static const struct regulator_desc sky81452_reg = {
    .name = "LOUT",
    .of_match = of_match_ptr("lout"),
    .regulators_node = of_match_ptr("regulator"),
    .ops = &sky81452_reg_ops,
    .type = REGULATOR_VOLTAGE,
    .owner = THIS_MODULE,
    .n_voltages = SKY81452_LOUT + 1,
    .linear_ranges = sky81452_reg_ranges,
    .n_linear_ranges = ARRAY_SIZE(sky81452_reg_ranges),
    .vsel_reg = SKY81452_REG3,
    .vsel_mask = SKY81452_LOUT,
    .enable_reg = SKY81452_REG1,
    .enable_mask = SKY81452_LEN,
    };
#[no_mangle]
unsafe extern "C" fn sky81452_reg_probe(pdev: *mut platform_device) -> c_int {
    static int sky81452_reg_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    const struct regulator_init_data *init_data = dev_get_platdata(dev);
    let mut config: regulator_config = { };
    struct regulator_dev *rdev;
    config.dev = dev.parent;
    config.init_data = init_data;
    config.of_node = dev.of_node;
    config.regmap = dev_get_drvdata(dev.parent);
    rdev = devm_regulator_register(dev, &sky81452_reg, &config);
    if (IS_ERR(rdev)) {
    dev_err(dev, "failed to register. err=%ld\n", PTR_ERR(rdev));
    return PTR_ERR(rdev);
    }
    platform_set_drvdata(pdev, rdev);
    return 0;
    }
    static struct platform_driver sky81452_reg_driver = {
    .driver = {
    .name = "sky81452-regulator",
    .probe_type = PROBE_PREFER_ASYNCHRONOUS,
    },
    .probe = sky81452_reg_probe,
    };
    module_platform_driver(sky81452_reg_driver);
    MODULE_DESCRIPTION("Skyworks SKY81452 Regulator driver");
    MODULE_AUTHOR("Gyungoh Yoo <jack.yoo@skyworksinc.com>");
    MODULE_LICENSE("GPL v2");
