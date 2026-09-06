//! Automatically rewritten from C to Rust
//! Source: drivers/regulator/qcom-pm8008-regulator.c
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
// Copyright (c) 2019-2020, The Linux Foundation. All rights reserved.
// Copyright (c) 2022 Qualcomm Innovation Center, Inc. All rights reserved.
// Copyright (c) 2024 Linaro Limited
//

pub const DEFAULT_VOLTAGE_STEPPER_RATE: c_int = 38400;
pub const LDO_STEPPER_CTL_REG: c_uint = 0x3b;

pub const LDO_VSET_LB_REG: c_uint = 0x40;
pub const LDO_ENABLE_REG: c_uint = 0x46;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pm8008_regulator {
    pub regmap: *mut regmap,
    pub desc: regulator_desc,
    pub base: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pm8008_regulator_data {
    pub name: *const c_char,
    pub supply_name: *const c_char,
    pub base: c_uint,
    pub min_dropout_uV: c_int,
    pub voltage_range: *const linear_range,
}

    static const struct linear_range nldo_ranges[] = {
    REGULATOR_LINEAR_RANGE(528000, 0, 122, 8000),
    };
    static const struct linear_range pldo_ranges[] = {
    REGULATOR_LINEAR_RANGE(1504000, 0, 237, 8000),
    };
    static const struct pm8008_regulator_data pm8008_reg_data[] = {
    { "ldo1", "vdd-l1-l2", 0x4000, 225000, nldo_ranges, },
    { "ldo2", "vdd-l1-l2", 0x4100, 225000, nldo_ranges, },
    { "ldo3", "vdd-l3-l4", 0x4200, 300000, pldo_ranges, },
    { "ldo4", "vdd-l3-l4", 0x4300, 300000, pldo_ranges, },
    { "ldo5", "vdd-l5",    0x4400, 200000, pldo_ranges, },
    { "ldo6", "vdd-l6",    0x4500, 200000, pldo_ranges, },
    { "ldo7", "vdd-l7",    0x4600, 200000, pldo_ranges, },
    };
#[no_mangle]
unsafe extern "C" fn pm8008_regulator_set_voltage_sel(rdev: *mut regulator_dev, sel: c_uint) -> c_int {
    static int pm8008_regulator_set_voltage_sel(struct regulator_dev *rdev, unsigned int sel)
    {
    struct pm8008_regulator *preg = rdev_get_drvdata(rdev);
    unsigned int mV;
    __le16 val;
    int ret;
    ret = regulator_list_voltage_linear_range(rdev, sel);
    if (ret < 0)
    return ret;
    mV = DIV_ROUND_UP(ret, 1000);
    val = cpu_to_le16(mV);
    ret = regmap_bulk_write(preg.regmap, preg.base + LDO_VSET_LB_REG,
    &val, sizeof(val));
    if (ret < 0)
    return ret;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pm8008_regulator_get_voltage_sel(rdev: *mut regulator_dev) -> c_int {
    static int pm8008_regulator_get_voltage_sel(struct regulator_dev *rdev)
    {
    struct pm8008_regulator *preg = rdev_get_drvdata(rdev);
    unsigned int uV;
    __le16 val;
    int ret;
    ret = regmap_bulk_read(preg.regmap, preg.base + LDO_VSET_LB_REG,
    &val, sizeof(val));
    if (ret < 0)
    return ret;
    uV = le16_to_cpu(val) * 1000;
    return regulator_map_voltage_linear_range(rdev, uV, INT_MAX);
    }
    static const struct regulator_ops pm8008_regulator_ops = {
    .list_voltage		= regulator_list_voltage_linear,
    .set_voltage_sel	= pm8008_regulator_set_voltage_sel,
    .get_voltage_sel	= pm8008_regulator_get_voltage_sel,
    .enable			= regulator_enable_regmap,
    .disable		= regulator_disable_regmap,
    .is_enabled		= regulator_is_enabled_regmap,
    };
#[no_mangle]
unsafe extern "C" fn pm8008_regulator_probe(pdev: *mut platform_device) -> c_int {
    static int pm8008_regulator_probe(struct platform_device *pdev)
    {
    const struct pm8008_regulator_data *data;
    let mut config: regulator_config = {};
    struct device *dev = &pdev.dev;
    struct pm8008_regulator *preg;
    struct regulator_desc *desc;
    struct regulator_dev *rdev;
    struct regmap *regmap;
    unsigned int val;
    int ret, i;
    regmap = dev_get_regmap(dev.parent, "secondary");
    if (!regmap)
    return -EINVAL;
    for (i = 0; i < ARRAY_SIZE(pm8008_reg_data); i++) {
    data = &pm8008_reg_data[i];
    preg = devm_kzalloc(dev, sizeof(*preg), GFP_KERNEL);
    if (!preg)
    return -ENOMEM;
    preg.regmap = regmap;
    preg.base = data.base;
    desc = &preg.desc;
    desc.name = data.name;
    desc.supply_name = data.supply_name;
    desc.of_match = data.name;
    desc.regulators_node = of_match_ptr("regulators");
    desc.ops = &pm8008_regulator_ops;
    desc.type = REGULATOR_VOLTAGE;
    desc.owner = THIS_MODULE;
    desc.linear_ranges = data.voltage_range;
    desc.n_linear_ranges = 1;
    desc.uV_step = desc.linear_ranges[0].step;
    desc.min_uV = desc.linear_ranges[0].min;
    desc.n_voltages = linear_range_values_in_range(&desc.linear_ranges[0]);
    ret = regmap_read(regmap, preg.base + LDO_STEPPER_CTL_REG, &val);
    if (ret < 0) {
    dev_err(dev, "failed to read step rate: %d\n", ret);
    return ret;
    }
    val &= STEP_RATE_MASK;
    desc.ramp_delay = DEFAULT_VOLTAGE_STEPPER_RATE >> val;
    desc.min_dropout_uV = data.min_dropout_uV;
    desc.enable_reg = preg.base + LDO_ENABLE_REG;
    desc.enable_mask = ENABLE_BIT;
    config.dev = dev.parent;
    config.driver_data = preg;
    config.regmap = regmap;
    rdev = devm_regulator_register(dev, desc, &config);
    if (IS_ERR(rdev)) {
    ret = PTR_ERR(rdev);
    dev_err(dev, "failed to register regulator %s: %d\n",
    desc.name, ret);
    return ret;
    }
    }
    return 0;
    }
    static const struct platform_device_id pm8008_regulator_id_table[] = {
    { .name = "pm8008-regulator" },
    { }
    };
    MODULE_DEVICE_TABLE(platform, pm8008_regulator_id_table);
    static struct platform_driver pm8008_regulator_driver = {
    .driver	= {
    .name = "qcom-pm8008-regulator",
    },
    .probe = pm8008_regulator_probe,
    .id_table = pm8008_regulator_id_table,
    };
    module_platform_driver(pm8008_regulator_driver);
    MODULE_DESCRIPTION("Qualcomm PM8008 PMIC regulator driver");
    MODULE_LICENSE("GPL");
