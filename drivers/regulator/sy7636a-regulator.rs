//! Automatically rewritten from C to Rust
//! Source: drivers/regulator/sy7636a-regulator.c
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
// Functions to access SY3686A power management chip voltages
//
// Copyright (C) 2019 reMarkable AS - http://www.remarkable.com
//
// Authors: Lars Ivar Miljeteig <lars.ivar.miljeteig@remarkable.com>
// Alistair Francis <alistair@alistair23.me>

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sy7636a_data {
    pub regmap: *mut regmap,
    pub pgood_gpio: *mut gpio_desc,
    pub en_gpio: *mut gpio_desc,
    pub vcom_en_gpio: *mut gpio_desc,
}

#[no_mangle]
unsafe extern "C" fn sy7636a_get_vcom_voltage_op(rdev: *mut regulator_dev) -> c_int {
    static int sy7636a_get_vcom_voltage_op(struct regulator_dev *rdev)
    {
    int ret;
    unsigned int val, val_h;
    ret = regmap_read(rdev.regmap, SY7636A_REG_VCOM_ADJUST_CTRL_L, &val);
    if (ret)
    return ret;
    ret = regmap_read(rdev.regmap, SY7636A_REG_VCOM_ADJUST_CTRL_H, &val_h);
    if (ret)
    return ret;
    val |= (val_h << VCOM_ADJUST_CTRL_SHIFT);
    return (val & VCOM_ADJUST_CTRL_MASK) * VCOM_ADJUST_CTRL_SCAL;
    }
#[no_mangle]
unsafe extern "C" fn sy7636a_get_status(rdev: *mut regulator_dev) -> c_int {
    static int sy7636a_get_status(struct regulator_dev *rdev)
    {
    struct sy7636a_data *data = dev_get_drvdata(rdev.dev.parent);
    let mut ret: c_int = 0;
    ret = gpiod_get_value_cansleep(data.pgood_gpio);
    if (ret < 0)
    dev_err(&rdev.dev, "Failed to read pgood gpio: %d\n", ret);
    return ret;
    }
    static const struct regulator_ops sy7636a_vcom_volt_ops = {
    .get_voltage = sy7636a_get_vcom_voltage_op,
    .enable = regulator_enable_regmap,
    .disable = regulator_disable_regmap,
    .is_enabled = regulator_is_enabled_regmap,
    .get_status = sy7636a_get_status,
    };
    static const struct regulator_desc desc = {
    .name = "vcom",
    .id = 0,
    .ops = &sy7636a_vcom_volt_ops,
    .type = REGULATOR_VOLTAGE,
    .owner = THIS_MODULE,
    .enable_reg = SY7636A_REG_OPERATION_MODE_CRL,
    .enable_mask = SY7636A_OPERATION_MODE_CRL_ONOFF,
    .regulators_node = of_match_ptr("regulators"),
    .of_match = of_match_ptr("vcom"),
    };
#[no_mangle]
unsafe extern "C" fn sy7636a_regulator_probe(pdev: *mut platform_device) -> c_int {
    static int sy7636a_regulator_probe(struct platform_device *pdev)
    {
    struct regmap *regmap = dev_get_regmap(pdev.dev.parent, core::ptr::null_mut());
    let mut config: regulator_config = { };
    struct regulator_dev *rdev;
    struct gpio_desc *gdp;
    struct sy7636a_data *data;
    int ret;
    if (!regmap)
    return -EPROBE_DEFER;
    device_set_of_node_from_dev(&pdev.dev, pdev.dev.parent);
    gdp = devm_gpiod_get(&pdev.dev, "epd-pwr-good", GPIOD_IN);
    if (IS_ERR(gdp)) {
    dev_err(&pdev.dev, "Power good GPIO fault %ld\n", PTR_ERR(gdp));
    return PTR_ERR(gdp);
    }
    data = devm_kzalloc(&pdev.dev, sizeof(struct sy7636a_data), GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    data.regmap = regmap;
    data.pgood_gpio = gdp;
    ret = devm_regulator_get_enable_optional(&pdev.dev, "vin");
    if (ret)
    return dev_err_probe(&pdev.dev, ret,
    "failed to get vin regulator\n");
    data.en_gpio = devm_gpiod_get_optional(&pdev.dev, "enable",
    GPIOD_OUT_HIGH);
    if (IS_ERR(data.en_gpio))
    return dev_err_probe(&pdev.dev,
    PTR_ERR(data.en_gpio),
    "failed to get en gpio\n");
// Let VCOM just follow the default power on sequence
    data.vcom_en_gpio = devm_gpiod_get_optional(&pdev.dev,
    "vcom-en", GPIOD_OUT_LOW);
    if (IS_ERR(data.vcom_en_gpio))
    return dev_err_probe(&pdev.dev,
    PTR_ERR(data.vcom_en_gpio),
    "failed to get vcom-en gpio\n");
// if chip was not enabled, give it time to wake up
    if (data.en_gpio)
    usleep_range(2500, 4000);
    platform_set_drvdata(pdev, data);
    ret = regmap_write(regmap, SY7636A_REG_POWER_ON_DELAY_TIME, 0x0);
    if (ret) {
    dev_err(pdev.dev.parent, "Failed to initialize regulator: %d\n", ret);
    return ret;
    }
    config.dev = &pdev.dev;
    config.regmap = regmap;
    rdev = devm_regulator_register(&pdev.dev, &desc, &config);
    if (IS_ERR(rdev)) {
    dev_err(pdev.dev.parent, "Failed to register %s regulator\n",
    pdev.name);
    return PTR_ERR(rdev);
    }
    return 0;
    }
    static const struct platform_device_id sy7636a_regulator_id_table[] = {
    { .name = "sy7636a-regulator" },
    { }
    };
    MODULE_DEVICE_TABLE(platform, sy7636a_regulator_id_table);
    static struct platform_driver sy7636a_regulator_driver = {
    .driver = {
    .name = "sy7636a-regulator",
    .probe_type = PROBE_PREFER_ASYNCHRONOUS,
    },
    .probe = sy7636a_regulator_probe,
    .id_table = sy7636a_regulator_id_table,
    };
    module_platform_driver(sy7636a_regulator_driver);
    MODULE_AUTHOR("Lars Ivar Miljeteig <lars.ivar.miljeteig@remarkable.com>");
    MODULE_DESCRIPTION("SY7636A voltage regulator driver");
    MODULE_LICENSE("GPL v2");
