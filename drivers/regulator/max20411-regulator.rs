//! Automatically rewritten from C to Rust
//! Source: drivers/regulator/max20411-regulator.c
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
// Copyright (c) 2021, The Linux Foundation. All rights reserved.
// Copyright (c) 2022, Linaro Ltd.
//

pub const MAX20411_UV_STEP: c_int = 6250;
pub const MAX20411_BASE_UV: c_int = 243750;

pub const MAX20411_VID_OFFSET: c_uint = 0x7;
pub const MAX20411_VID_MASK: c_uint = 0xff;
pub const MAX20411_SLEW_OFFSET: c_uint = 0x6;
pub const MAX20411_SLEW_DVS_MASK: c_uint = 0xc;
pub const MAX20411_SLEW_SR_MASK: c_uint = 0x3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct max20411 {
    pub dev: *mut device,
    pub of_node: *mut device_node,
    pub desc: regulator_desc,
    pub rdev: *mut regulator_dev,
    pub regmap: *mut regmap,
}

    static const unsigned int max20411_slew_rates[] = { 13100, 6600, 3300, 1600 };
#[no_mangle]
unsafe extern "C" fn max20411_enable_time(rdev: *mut regulator_dev) -> c_int {
    static int max20411_enable_time(struct regulator_dev *rdev)
    {
    int voltage, rate, ret;
    unsigned int val;
// get voltage
    ret = regmap_read(rdev.regmap, rdev.desc.vsel_reg, &val);
    if (ret)
    return ret;
    val &= rdev.desc.vsel_mask;
    voltage = regulator_list_voltage_linear(rdev, val);
// get rate
    ret = regmap_read(rdev.regmap, MAX20411_SLEW_OFFSET, &val);
    if (ret)
    return ret;
    val = FIELD_GET(MAX20411_SLEW_SR_MASK, val);
    rate = max20411_slew_rates[val];
    return DIV_ROUND_UP(voltage, rate);
    }
    static const struct regmap_config max20411_regmap_config = {
    .reg_bits		= 8,
    .val_bits		= 8,
    .max_register		= 0xe,
    };
    static const struct regulator_ops max20411_ops = {
    .get_voltage_sel	= regulator_get_voltage_sel_regmap,
    .set_voltage_sel	= regulator_set_voltage_sel_regmap,
    .list_voltage		= regulator_list_voltage_linear,
    .enable_time		= max20411_enable_time,
    };
    static const struct regulator_desc max20411_desc = {
    .ops = &max20411_ops,
    .owner = THIS_MODULE,
    .type = REGULATOR_VOLTAGE,
    .supply_name = "vin",
    .name = "max20411",
//
// voltage = 0.24375V + selector * 6.25mV
// with valid selector between 41 to 165 (0.5V to 1.275V)
//
    .min_uV = MAX20411_BASE_UV,
    .uV_step = MAX20411_UV_STEP,
    .linear_min_sel = MAX20411_MIN_SEL,
    .n_voltages = MAX20411_MAX_SEL + 1,
    .vsel_reg = MAX20411_VID_OFFSET,
    .vsel_mask = MAX20411_VID_MASK,
    .ramp_reg = MAX20411_SLEW_OFFSET,
    .ramp_mask = MAX20411_SLEW_DVS_MASK,
    .ramp_delay_table = max20411_slew_rates,
    .n_ramp_values = ARRAY_SIZE(max20411_slew_rates),
    };
#[no_mangle]
unsafe extern "C" fn max20411_probe(client: *mut i2c_client) -> c_int {
    static int max20411_probe(struct i2c_client *client)
    {
    struct regulator_init_data *init_data;
    struct device *dev = &client.dev;
    let mut cfg: regulator_config = {};
    struct max20411 *max20411;
    max20411 = devm_kzalloc(dev, sizeof(*max20411), GFP_KERNEL);
    if (!max20411)
    return -ENOMEM;
    max20411.regmap = devm_regmap_init_i2c(client, &max20411_regmap_config);
    if (IS_ERR(max20411.regmap)) {
    dev_err(dev, "Failed to allocate regmap!\n");
    return PTR_ERR(max20411.regmap);
    }
    max20411.dev = dev;
    max20411.of_node = dev.of_node;
    max20411.desc = max20411_desc;
    init_data = of_get_regulator_init_data(max20411.dev, max20411.of_node, &max20411.desc);
    if (!init_data)
    return -ENODATA;
    cfg.dev = max20411.dev;
    cfg.init_data = init_data;
    cfg.of_node = max20411.of_node;
    cfg.driver_data = max20411;
    cfg.ena_gpiod = gpiod_get(max20411.dev, "enable", GPIOD_ASIS);
    if (IS_ERR(cfg.ena_gpiod))
    return dev_err_probe(dev, PTR_ERR(cfg.ena_gpiod),
    "unable to acquire enable gpio\n");
    max20411.rdev = devm_regulator_register(max20411.dev, &max20411.desc, &cfg);
    if (IS_ERR(max20411.rdev))
    dev_err(max20411.dev, "Failed to register regulator: %pe\n", max20411.rdev);
    return PTR_ERR_OR_ZERO(max20411.rdev);
    }
    static const struct of_device_id of_max20411_match_tbl[] = {
    { .compatible = "maxim,max20411", },
    { },
    };
    MODULE_DEVICE_TABLE(of, of_max20411_match_tbl);
    static const struct i2c_device_id max20411_id[] = {
    { .name = "max20411" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, max20411_id);
    static struct i2c_driver max20411_i2c_driver = {
    .driver	= {
    .name = "max20411",
    .probe_type = PROBE_PREFER_ASYNCHRONOUS,
    .of_match_table	= of_max20411_match_tbl,
    },
    .probe = max20411_probe,
    .id_table = max20411_id,
    };
    module_i2c_driver(max20411_i2c_driver);
    MODULE_DESCRIPTION("Maxim MAX20411 High-Efficiency Single Step-Down Converter driver");
    MODULE_LICENSE("GPL");
