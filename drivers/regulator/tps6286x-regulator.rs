//! Automatically rewritten from C to Rust
//! Source: drivers/regulator/tps6286x-regulator.c
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
// Copyright Axis Communications AB

pub const TPS6286X_VOUT1: c_uint = 0x01;

pub const TPS6286X_CONTROL: c_uint = 0x03;

pub const TPS6286X_STATUS: c_uint = 0x05;
pub const TPS6286X_MIN_MV: c_int = 400;
pub const TPS6286X_MAX_MV: c_int = 1675;
pub const TPS6286X_STEP_MV: c_int = 5;
#[no_mangle]
unsafe extern "C" fn tps6286x_volatile_reg(dev: *mut device, reg: c_uint) -> bool {
    static bool tps6286x_volatile_reg(struct device *dev, unsigned int reg)
    {
    let mut reg: return = = TPS6286X_STATUS;
    }
    static const struct regmap_config tps6286x_regmap_config = {
    .reg_bits = 8,
    .val_bits = 8,
    .cache_type = REGCACHE_MAPLE,
    .volatile_reg = tps6286x_volatile_reg,
    };
#[no_mangle]
unsafe extern "C" fn tps6286x_set_mode(rdev: *mut regulator_dev, mode: c_uint) -> c_int {
    static int tps6286x_set_mode(struct regulator_dev *rdev, unsigned int mode)
    {
    unsigned int val;
    switch (mode) {
    case REGULATOR_MODE_NORMAL:
    val = 0;
    break;
    case REGULATOR_MODE_FAST:
    val = TPS6286X_CONTROL_FPWM;
    break;
    default:
    return -EINVAL;
    }
    return regmap_update_bits(rdev.regmap, TPS6286X_CONTROL,
    TPS6286X_CONTROL_FPWM, val);
    }
#[no_mangle]
unsafe extern "C" fn tps6286x_get_mode(rdev: *mut regulator_dev) -> c_uint {
    static unsigned int tps6286x_get_mode(struct regulator_dev *rdev)
    {
    unsigned int val;
    int ret;
    ret = regmap_read(rdev.regmap, TPS6286X_CONTROL, &val);
    if (ret < 0)
    return 0;
    return (val & TPS6286X_CONTROL_FPWM) ? REGULATOR_MODE_FAST : REGULATOR_MODE_NORMAL;
    }
    static const struct regulator_ops tps6286x_regulator_ops = {
    .enable = regulator_enable_regmap,
    .disable = regulator_disable_regmap,
    .set_mode = tps6286x_set_mode,
    .get_mode = tps6286x_get_mode,
    .is_enabled = regulator_is_enabled_regmap,
    .set_voltage_sel = regulator_set_voltage_sel_regmap,
    .get_voltage_sel = regulator_get_voltage_sel_regmap,
    .list_voltage = regulator_list_voltage_linear,
    };
#[no_mangle]
unsafe extern "C" fn tps6286x_of_map_mode(mode: c_uint) -> c_uint {
    static unsigned int tps6286x_of_map_mode(unsigned int mode)
    {
    switch (mode) {
    case TPS62864_MODE_NORMAL:
    return REGULATOR_MODE_NORMAL;
    case TPS62864_MODE_FPWM:
    return REGULATOR_MODE_FAST;
    default:
    return REGULATOR_MODE_INVALID;
    }
    }
    static const struct regulator_desc tps6286x_reg = {
    .name = "tps6286x",
    .of_match = "SW",
    .owner = THIS_MODULE,
    .ops = &tps6286x_regulator_ops,
    .of_map_mode = tps6286x_of_map_mode,
    .regulators_node = "regulators",
    .type = REGULATOR_VOLTAGE,
    .n_voltages = ((TPS6286X_MAX_MV - TPS6286X_MIN_MV) / TPS6286X_STEP_MV) + 1,
    .min_uV = TPS6286X_MIN_MV * 1000,
    .uV_step = TPS6286X_STEP_MV * 1000,
    .vsel_reg = TPS6286X_VOUT1,
    .vsel_mask = TPS6286X_VOUT1_VO1_SET,
    .enable_reg = TPS6286X_CONTROL,
    .enable_mask = TPS6286X_CONTROL_SWEN,
    .ramp_delay = 1000,
// tDelay + tRamp, rounded up
    .enable_time = 3000,
    };
    static const struct of_device_id tps6286x_dt_ids[] = {
    { .compatible = "ti,tps62864", },
    { .compatible = "ti,tps62866", },
    { .compatible = "ti,tps62868", },
    { .compatible = "ti,tps62869", },
    { }
    };
    MODULE_DEVICE_TABLE(of, tps6286x_dt_ids);
#[no_mangle]
unsafe extern "C" fn tps6286x_i2c_probe(i2c: *mut i2c_client) -> c_int {
    static int tps6286x_i2c_probe(struct i2c_client *i2c)
    {
    struct device *dev = &i2c.dev;
    let mut config: regulator_config = {};
    struct regulator_dev *rdev;
    struct regmap *regmap;
    regmap = devm_regmap_init_i2c(i2c, &tps6286x_regmap_config);
    if (IS_ERR(regmap))
    return PTR_ERR(regmap);
    config.dev = &i2c.dev;
    config.of_node = dev.of_node;
    config.regmap = regmap;
    rdev = devm_regulator_register(&i2c.dev, &tps6286x_reg, &config);
    if (IS_ERR(rdev)) {
    dev_err(&i2c.dev, "Failed to register tps6286x regulator\n");
    return PTR_ERR(rdev);
    }
    return 0;
    }
    static const struct i2c_device_id tps6286x_i2c_id[] = {
    { .name = "tps62864" },
    { .name = "tps62866" },
    { .name = "tps62868" },
    { .name = "tps62869" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, tps6286x_i2c_id);
    static struct i2c_driver tps6286x_regulator_driver = {
    .driver = {
    .name = "tps6286x",
    .probe_type = PROBE_PREFER_ASYNCHRONOUS,
    .of_match_table = tps6286x_dt_ids,
    },
    .probe = tps6286x_i2c_probe,
    .id_table = tps6286x_i2c_id,
    };
    module_i2c_driver(tps6286x_regulator_driver);
    MODULE_DESCRIPTION("TI TPS6286x Power Regulator driver");
    MODULE_LICENSE("GPL v2");
