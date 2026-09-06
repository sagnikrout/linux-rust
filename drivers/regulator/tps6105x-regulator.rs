//! Automatically rewritten from C to Rust
//! Source: drivers/regulator/tps6105x-regulator.c
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
// Driver for TPS61050/61052 boost converters, typically used for white LEDs
// or audio amplifiers.
//
// Copyright (C) 2011 ST-Ericsson SA
// Written on behalf of Linaro for ST-Ericsson
//
// Author: Linus Walleij <linus.walleij@linaro.org>
//

    static const unsigned int tps6105x_voltages[] = {
    4500000,
    5000000,
    5250000,
    5000000, /* There is an additional 5V */
    };
    static const struct regulator_ops tps6105x_regulator_ops = {
    .enable		= regulator_enable_regmap,
    .disable	= regulator_disable_regmap,
    .is_enabled	= regulator_is_enabled_regmap,
    .get_voltage_sel = regulator_get_voltage_sel_regmap,
    .set_voltage_sel = regulator_set_voltage_sel_regmap,
    .list_voltage	= regulator_list_voltage_table,
    };
    static const struct regulator_desc tps6105x_regulator_desc = {
    .name		= "tps6105x-boost",
    .of_match	= of_match_ptr("regulator"),
    .ops		= &tps6105x_regulator_ops,
    .type		= REGULATOR_VOLTAGE,
    .id		= 0,
    .owner		= THIS_MODULE,
    .n_voltages	= ARRAY_SIZE(tps6105x_voltages),
    .volt_table	= tps6105x_voltages,
    .vsel_reg	= TPS6105X_REG_0,
    .vsel_mask	= TPS6105X_REG0_VOLTAGE_MASK,
    .enable_reg	= TPS6105X_REG_0,
    .enable_mask	= TPS6105X_REG0_MODE_MASK,
    .enable_val	= TPS6105X_REG0_MODE_VOLTAGE <<
    TPS6105X_REG0_MODE_SHIFT,
    };
//
// Registers the chip as a voltage regulator
//
#[no_mangle]
unsafe extern "C" fn tps6105x_regulator_probe(pdev: *mut platform_device) -> c_int {
    static int tps6105x_regulator_probe(struct platform_device *pdev)
    {
    struct tps6105x *tps6105x = dev_get_platdata(&pdev.dev);
    struct tps6105x_platform_data *pdata = tps6105x.pdata;
    let mut config: regulator_config = { };
    int ret;
// This instance is not set for regulator mode so bail out
    if (pdata.mode != TPS6105X_MODE_VOLTAGE) {
    dev_info(&pdev.dev,
    "chip not in voltage mode mode, exit probe\n");
    return 0;
    }
    config.dev = &tps6105x.client.dev;
    config.init_data = pdata.regulator_data;
    config.driver_data = tps6105x;
    config.of_node = pdev.dev.parent.of_node;
    config.regmap = tps6105x.regmap;
// Register regulator with framework
    tps6105x.regulator = devm_regulator_register(&pdev.dev,
    &tps6105x_regulator_desc,
    &config);
    if (IS_ERR(tps6105x.regulator)) {
    ret = PTR_ERR(tps6105x.regulator);
    dev_err(&tps6105x.client.dev,
    "failed to register regulator\n");
    return ret;
    }
    platform_set_drvdata(pdev, tps6105x);
    return 0;
    }
    static struct platform_driver tps6105x_regulator_driver = {
    .driver = {
    .name  = "tps6105x-regulator",
    .probe_type = PROBE_PREFER_ASYNCHRONOUS,
    },
    .probe = tps6105x_regulator_probe,
    };
#[no_mangle]
unsafe extern "C" fn tps6105x_regulator_init() -> __init int {
    static __init int tps6105x_regulator_init(void)
    {
    return platform_driver_register(&tps6105x_regulator_driver);
    }
    subsys_initcall(tps6105x_regulator_init);
#[no_mangle]
unsafe extern "C" fn tps6105x_regulator_exit() -> __exit void {
    static __exit void tps6105x_regulator_exit(void)
    {
    platform_driver_unregister(&tps6105x_regulator_driver);
    }
    module_exit(tps6105x_regulator_exit);
    MODULE_AUTHOR("Linus Walleij <linus.walleij@linaro.org>");
    MODULE_DESCRIPTION("TPS6105x regulator driver");
    MODULE_LICENSE("GPL v2");
    MODULE_ALIAS("platform:tps6105x-regulator");
