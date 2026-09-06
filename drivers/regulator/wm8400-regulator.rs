//! Automatically rewritten from C to Rust
//! Source: drivers/regulator/wm8400-regulator.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// Regulator support for WM8400
//
// Copyright 2008 Wolfson Microelectronics PLC.
//
// Author: Mark Brown <broonie@opensource.wolfsonmicro.com>

    static const struct linear_range wm8400_ldo_ranges[] = {
    REGULATOR_LINEAR_RANGE(900000, 0, 14, 50000),
    REGULATOR_LINEAR_RANGE(1700000, 15, 31, 100000),
    };
    static const struct regulator_ops wm8400_ldo_ops = {
    .is_enabled = regulator_is_enabled_regmap,
    .enable = regulator_enable_regmap,
    .disable = regulator_disable_regmap,
    .list_voltage = regulator_list_voltage_linear_range,
    .get_voltage_sel = regulator_get_voltage_sel_regmap,
    .set_voltage_sel = regulator_set_voltage_sel_regmap,
    .map_voltage = regulator_map_voltage_linear_range,
    };
#[no_mangle]
unsafe extern "C" fn wm8400_dcdc_get_mode(dev: *mut regulator_dev) -> c_uint {
    static unsigned int wm8400_dcdc_get_mode(struct regulator_dev *dev)
    {
    struct regmap *rmap = rdev_get_regmap(dev);
    let mut offset: c_int = (rdev_get_id(dev) - WM8400_DCDC1) * 2;
    u16 data[2];
    int ret;
    ret = regmap_bulk_read(rmap, WM8400_DCDC1_CONTROL_1 + offset, data, 2);
    if (ret != 0)
    return 0;
// Datasheet: hibernate
    if (data[0] & WM8400_DC1_SLEEP)
    return REGULATOR_MODE_STANDBY;
// Datasheet: standby
    if (!(data[0] & WM8400_DC1_ACTIVE))
    return REGULATOR_MODE_IDLE;
// Datasheet: active with or without force PWM
    if (data[1] & WM8400_DC1_FRC_PWM)
    return REGULATOR_MODE_FAST;
    else
    return REGULATOR_MODE_NORMAL;
    }
#[no_mangle]
unsafe extern "C" fn wm8400_dcdc_set_mode(dev: *mut regulator_dev, mode: c_uint) -> c_int {
    static int wm8400_dcdc_set_mode(struct regulator_dev *dev, unsigned int mode)
    {
    struct regmap *rmap = rdev_get_regmap(dev);
    let mut offset: c_int = (rdev_get_id(dev) - WM8400_DCDC1) * 2;
    int ret;
    switch (mode) {
    case REGULATOR_MODE_FAST:
// Datasheet: active with force PWM
    ret = regmap_update_bits(rmap, WM8400_DCDC1_CONTROL_2 + offset,
    WM8400_DC1_FRC_PWM, WM8400_DC1_FRC_PWM);
    if (ret != 0)
    return ret;
    return regmap_update_bits(rmap, WM8400_DCDC1_CONTROL_1 + offset,
    WM8400_DC1_ACTIVE | WM8400_DC1_SLEEP,
    WM8400_DC1_ACTIVE);
    case REGULATOR_MODE_NORMAL:
// Datasheet: active
    ret = regmap_update_bits(rmap, WM8400_DCDC1_CONTROL_2 + offset,
    WM8400_DC1_FRC_PWM, 0);
    if (ret != 0)
    return ret;
    return regmap_update_bits(rmap, WM8400_DCDC1_CONTROL_1 + offset,
    WM8400_DC1_ACTIVE | WM8400_DC1_SLEEP,
    WM8400_DC1_ACTIVE);
    case REGULATOR_MODE_IDLE:
// Datasheet: standby
    return regmap_update_bits(rmap, WM8400_DCDC1_CONTROL_1 + offset,
    WM8400_DC1_ACTIVE | WM8400_DC1_SLEEP, 0);
    default:
    return -EINVAL;
    }
    }
    static unsigned int wm8400_dcdc_get_optimum_mode(struct regulator_dev *dev,
    int input_uV, int output_uV,
    int load_uA)
    {
    return REGULATOR_MODE_NORMAL;
    }
    static const struct regulator_ops wm8400_dcdc_ops = {
    .is_enabled = regulator_is_enabled_regmap,
    .enable = regulator_enable_regmap,
    .disable = regulator_disable_regmap,
    .list_voltage = regulator_list_voltage_linear,
    .map_voltage = regulator_map_voltage_linear,
    .get_voltage_sel = regulator_get_voltage_sel_regmap,
    .set_voltage_sel = regulator_set_voltage_sel_regmap,
    .get_mode = wm8400_dcdc_get_mode,
    .set_mode = wm8400_dcdc_set_mode,
    .get_optimum_mode = wm8400_dcdc_get_optimum_mode,
    };
    static const struct regulator_desc regulators[] = {
    {
    .name = "LDO1",
    .id = WM8400_LDO1,
    .ops = &wm8400_ldo_ops,
    .enable_reg = WM8400_LDO1_CONTROL,
    .enable_mask = WM8400_LDO1_ENA,
    .n_voltages = WM8400_LDO1_VSEL_MASK + 1,
    .linear_ranges = wm8400_ldo_ranges,
    .n_linear_ranges = ARRAY_SIZE(wm8400_ldo_ranges),
    .vsel_reg = WM8400_LDO1_CONTROL,
    .vsel_mask = WM8400_LDO1_VSEL_MASK,
    .type = REGULATOR_VOLTAGE,
    .owner = THIS_MODULE,
    },
    {
    .name = "LDO2",
    .id = WM8400_LDO2,
    .ops = &wm8400_ldo_ops,
    .enable_reg = WM8400_LDO2_CONTROL,
    .enable_mask = WM8400_LDO2_ENA,
    .n_voltages = WM8400_LDO2_VSEL_MASK + 1,
    .linear_ranges = wm8400_ldo_ranges,
    .n_linear_ranges = ARRAY_SIZE(wm8400_ldo_ranges),
    .type = REGULATOR_VOLTAGE,
    .vsel_reg = WM8400_LDO2_CONTROL,
    .vsel_mask = WM8400_LDO2_VSEL_MASK,
    .owner = THIS_MODULE,
    },
    {
    .name = "LDO3",
    .id = WM8400_LDO3,
    .ops = &wm8400_ldo_ops,
    .enable_reg = WM8400_LDO3_CONTROL,
    .enable_mask = WM8400_LDO3_ENA,
    .n_voltages = WM8400_LDO3_VSEL_MASK + 1,
    .linear_ranges = wm8400_ldo_ranges,
    .n_linear_ranges = ARRAY_SIZE(wm8400_ldo_ranges),
    .vsel_reg = WM8400_LDO3_CONTROL,
    .vsel_mask = WM8400_LDO3_VSEL_MASK,
    .type = REGULATOR_VOLTAGE,
    .owner = THIS_MODULE,
    },
    {
    .name = "LDO4",
    .id = WM8400_LDO4,
    .ops = &wm8400_ldo_ops,
    .enable_reg = WM8400_LDO4_CONTROL,
    .enable_mask = WM8400_LDO4_ENA,
    .n_voltages = WM8400_LDO4_VSEL_MASK + 1,
    .linear_ranges = wm8400_ldo_ranges,
    .n_linear_ranges = ARRAY_SIZE(wm8400_ldo_ranges),
    .vsel_reg = WM8400_LDO4_CONTROL,
    .vsel_mask = WM8400_LDO4_VSEL_MASK,
    .type = REGULATOR_VOLTAGE,
    .owner = THIS_MODULE,
    },
    {
    .name = "DCDC1",
    .id = WM8400_DCDC1,
    .ops = &wm8400_dcdc_ops,
    .enable_reg = WM8400_DCDC1_CONTROL_1,
    .enable_mask = WM8400_DC1_ENA_MASK,
    .n_voltages = WM8400_DC1_VSEL_MASK + 1,
    .vsel_reg = WM8400_DCDC1_CONTROL_1,
    .vsel_mask = WM8400_DC1_VSEL_MASK,
    .min_uV = 850000,
    .uV_step = 25000,
    .type = REGULATOR_VOLTAGE,
    .owner = THIS_MODULE,
    },
    {
    .name = "DCDC2",
    .id = WM8400_DCDC2,
    .ops = &wm8400_dcdc_ops,
    .enable_reg = WM8400_DCDC2_CONTROL_1,
    .enable_mask = WM8400_DC2_ENA_MASK,
    .n_voltages = WM8400_DC2_VSEL_MASK + 1,
    .vsel_reg = WM8400_DCDC2_CONTROL_1,
    .vsel_mask = WM8400_DC2_VSEL_MASK,
    .min_uV = 850000,
    .uV_step = 25000,
    .type = REGULATOR_VOLTAGE,
    .owner = THIS_MODULE,
    },
    };
#[no_mangle]
unsafe extern "C" fn wm8400_regulator_probe(pdev: *mut platform_device) -> c_int {
    static int wm8400_regulator_probe(struct platform_device *pdev)
    {
    struct wm8400 *wm8400 = container_of(pdev, struct wm8400, regulators[pdev.id]);
    let mut config: regulator_config = { };
    struct regulator_dev *rdev;
    config.dev = &pdev.dev;
    config.init_data = dev_get_platdata(&pdev.dev);
    config.driver_data = wm8400;
    config.regmap = wm8400.regmap;
    rdev = devm_regulator_register(&pdev.dev, &regulators[pdev.id],
    &config);
    if (IS_ERR(rdev))
    return PTR_ERR(rdev);
    platform_set_drvdata(pdev, rdev);
    return 0;
    }
    static struct platform_driver wm8400_regulator_driver = {
    .driver = {
    .name = "wm8400-regulator",
    .probe_type = PROBE_PREFER_ASYNCHRONOUS,
    },
    .probe = wm8400_regulator_probe,
    };
//
// wm8400_register_regulator - enable software control of a WM8400 regulator
//
// This function enables software control of a WM8400 regulator via
// the regulator API.  It is intended to be called from the
// platform_init() callback of the WM8400 MFD driver.
//
// @dev:      The WM8400 device to operate on.
// @reg:      The regulator to control.
// @initdata: Regulator initdata for the regulator.
//
    int wm8400_register_regulator(struct device *dev, int reg,
    struct regulator_init_data *initdata)
    {
    struct wm8400 *wm8400 = dev_get_drvdata(dev);
    if (wm8400.regulators[reg].name)
    return -EBUSY;
    initdata.driver_data = wm8400;
    wm8400.regulators[reg].name = "wm8400-regulator";
    wm8400.regulators[reg].id = reg;
    wm8400.regulators[reg].dev.parent = dev;
    wm8400.regulators[reg].dev.platform_data = initdata;
    return platform_device_register(&wm8400.regulators[reg]);
    }
    EXPORT_SYMBOL_GPL(wm8400_register_regulator);
#[no_mangle]
unsafe extern "C" fn wm8400_regulator_init() -> int __init {
    static int __init wm8400_regulator_init(void)
    {
    return platform_driver_register(&wm8400_regulator_driver);
    }
    subsys_initcall(wm8400_regulator_init);
#[no_mangle]
unsafe extern "C" fn wm8400_regulator_exit() -> void __exit {
    static void __exit wm8400_regulator_exit(void)
    {
    platform_driver_unregister(&wm8400_regulator_driver);
    }
    module_exit(wm8400_regulator_exit);
    MODULE_AUTHOR("Mark Brown <broonie@opensource.wolfsonmicro.com>");
    MODULE_DESCRIPTION("WM8400 regulator driver");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:wm8400-regulator");
