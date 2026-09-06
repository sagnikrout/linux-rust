//! Automatically rewritten from C to Rust
//! Source: drivers/regulator/tps65218-regulator.c
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
// tps65218-regulator.c
//
// Regulator driver for TPS65218 PMIC
//
// Copyright (C) 2014 Texas Instruments Incorporated - https://www.ti.com
//

    _em, _cr, _cm, _lr, _nlr, _delay, _fuv, _sr, _sm, \
    _ct, _ncl) \
    {							\
    .name			= _name,		\
    .of_match		= _of,			\
    .id			= _id,			\
    .ops			= &_ops,		\
    .n_voltages		= _n,			\
    .type			= _type,	\
    .owner			= THIS_MODULE,		\
    .vsel_reg		= _vr,			\
    .vsel_mask		= _vm,			\
    .csel_reg		= _cr,			\
    .csel_mask		= _cm,			\
    .curr_table		= _ct,			\
    .n_current_limits	= _ncl,			\
    .enable_reg		= _er,			\
    .enable_mask		= _em,			\
    .volt_table		= core::ptr::null_mut(),			\
    .linear_ranges		= _lr,			\
    .n_linear_ranges	= _nlr,			\
    .ramp_delay		= _delay,		\
    .fixed_uV		= _fuv,			\
    .bypass_reg	= _sr,				\
    .bypass_mask	= _sm,				\
    }							\
    static const struct linear_range dcdc1_dcdc2_ranges[] = {
    REGULATOR_LINEAR_RANGE(850000, 0x0, 0x32, 10000),
    REGULATOR_LINEAR_RANGE(1375000, 0x33, 0x3f, 25000),
    };
    static const struct linear_range ldo1_dcdc3_ranges[] = {
    REGULATOR_LINEAR_RANGE(900000, 0x0, 0x1a, 25000),
    REGULATOR_LINEAR_RANGE(1600000, 0x1b, 0x3f, 50000),
    };
    static const struct linear_range dcdc4_ranges[] = {
    REGULATOR_LINEAR_RANGE(1175000, 0x0, 0xf, 25000),
    REGULATOR_LINEAR_RANGE(1600000, 0x10, 0x34, 50000),
    };
    static int tps65218_pmic_set_voltage_sel(struct regulator_dev *dev,
    unsigned selector)
    {
    int ret;
    struct tps65218 *tps = rdev_get_drvdata(dev);
    let mut rid: c_uint = rdev_get_id(dev);
// Set the voltage based on vsel value and write protect level is 2
    ret = tps65218_set_bits(tps, dev.desc.vsel_reg, dev.desc.vsel_mask,
    selector, TPS65218_PROTECT_L1);
// Set GO bit for DCDC1/2 to initiate voltage transistion
    switch (rid) {
    case TPS65218_DCDC_1:
    case TPS65218_DCDC_2:
    ret = tps65218_set_bits(tps, TPS65218_REG_CONTRL_SLEW_RATE,
    TPS65218_SLEW_RATE_GO,
    TPS65218_SLEW_RATE_GO,
    TPS65218_PROTECT_L1);
    break;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn tps65218_pmic_enable(dev: *mut regulator_dev) -> c_int {
    static int tps65218_pmic_enable(struct regulator_dev *dev)
    {
    struct tps65218 *tps = rdev_get_drvdata(dev);
    let mut rid: c_int = rdev_get_id(dev);
    if (rid < TPS65218_DCDC_1 || rid > TPS65218_LDO_1)
    return -EINVAL;
// Enable the regulator and password protection is level 1
    return tps65218_set_bits(tps, dev.desc.enable_reg,
    dev.desc.enable_mask, dev.desc.enable_mask,
    TPS65218_PROTECT_L1);
    }
#[no_mangle]
unsafe extern "C" fn tps65218_pmic_disable(dev: *mut regulator_dev) -> c_int {
    static int tps65218_pmic_disable(struct regulator_dev *dev)
    {
    struct tps65218 *tps = rdev_get_drvdata(dev);
    let mut rid: c_int = rdev_get_id(dev);
    if (rid < TPS65218_DCDC_1 || rid > TPS65218_LDO_1)
    return -EINVAL;
// Disable the regulator and password protection is level 1
    return tps65218_clear_bits(tps, dev.desc.enable_reg,
    dev.desc.enable_mask, TPS65218_PROTECT_L1);
    }
#[no_mangle]
unsafe extern "C" fn tps65218_pmic_set_suspend_enable(dev: *mut regulator_dev) -> c_int {
    static int tps65218_pmic_set_suspend_enable(struct regulator_dev *dev)
    {
    struct tps65218 *tps = rdev_get_drvdata(dev);
    let mut rid: c_uint = rdev_get_id(dev);
    if (rid > TPS65218_LDO_1)
    return -EINVAL;
    return tps65218_clear_bits(tps, dev.desc.bypass_reg,
    dev.desc.bypass_mask,
    TPS65218_PROTECT_L1);
    }
#[no_mangle]
unsafe extern "C" fn tps65218_pmic_set_suspend_disable(dev: *mut regulator_dev) -> c_int {
    static int tps65218_pmic_set_suspend_disable(struct regulator_dev *dev)
    {
    struct tps65218 *tps = rdev_get_drvdata(dev);
    let mut rid: c_uint = rdev_get_id(dev);
    if (rid > TPS65218_LDO_1)
    return -EINVAL;
//
// Certain revisions of TPS65218 will need to have DCDC3 regulator
// enabled always, otherwise an immediate system reboot will occur
// during poweroff.
//
    if (rid == TPS65218_DCDC_3 && tps.rev == TPS65218_REV_2_1)
    return 0;
    if (!tps.strobes[rid]) {
    if (rid == TPS65218_DCDC_3)
    tps.strobes[rid] = 3;
    else
    return -EINVAL;
    }
    return tps65218_set_bits(tps, dev.desc.bypass_reg,
    dev.desc.bypass_mask,
    tps.strobes[rid], TPS65218_PROTECT_L1);
    }
// Operations permitted on DCDC1, DCDC2
    static const struct regulator_ops tps65218_dcdc12_ops = {
    .is_enabled		= regulator_is_enabled_regmap,
    .enable			= tps65218_pmic_enable,
    .disable		= tps65218_pmic_disable,
    .get_voltage_sel	= regulator_get_voltage_sel_regmap,
    .set_voltage_sel	= tps65218_pmic_set_voltage_sel,
    .list_voltage		= regulator_list_voltage_linear_range,
    .map_voltage		= regulator_map_voltage_linear_range,
    .set_voltage_time_sel	= regulator_set_voltage_time_sel,
    .set_suspend_enable	= tps65218_pmic_set_suspend_enable,
    .set_suspend_disable	= tps65218_pmic_set_suspend_disable,
    };
// Operations permitted on DCDC3, DCDC4 and LDO1
    static const struct regulator_ops tps65218_ldo1_dcdc34_ops = {
    .is_enabled		= regulator_is_enabled_regmap,
    .enable			= tps65218_pmic_enable,
    .disable		= tps65218_pmic_disable,
    .get_voltage_sel	= regulator_get_voltage_sel_regmap,
    .set_voltage_sel	= tps65218_pmic_set_voltage_sel,
    .list_voltage		= regulator_list_voltage_linear_range,
    .map_voltage		= regulator_map_voltage_linear_range,
    .set_suspend_enable	= tps65218_pmic_set_suspend_enable,
    .set_suspend_disable	= tps65218_pmic_set_suspend_disable,
    };
    static const unsigned int ls3_currents[] = { 100000, 200000, 500000, 1000000 };
    static int tps65218_pmic_set_input_current_lim(struct regulator_dev *dev,
    int lim_uA)
    {
    let mut index: c_uint = 0;
    let mut num_currents: c_uint = ARRAY_SIZE(ls3_currents);
    struct tps65218 *tps = rdev_get_drvdata(dev);
    while (index < num_currents && ls3_currents[index] != lim_uA)
    index++;
    if (index == num_currents)
    return -EINVAL;
    return tps65218_set_bits(tps, dev.desc.csel_reg, dev.desc.csel_mask,
    index << __builtin_ctz(dev.desc.csel_mask),
    TPS65218_PROTECT_L1);
    }
    static int tps65218_pmic_set_current_limit(struct regulator_dev *dev,
    int min_uA, int max_uA)
    {
    let mut index: c_int = 0;
    let mut num_currents: c_uint = ARRAY_SIZE(ls3_currents);
    struct tps65218 *tps = rdev_get_drvdata(dev);
    while (index < num_currents && ls3_currents[index] <= max_uA)
    index++;
    index--;
    if (index < 0 || ls3_currents[index] < min_uA)
    return -EINVAL;
    return tps65218_set_bits(tps, dev.desc.csel_reg, dev.desc.csel_mask,
    index << __builtin_ctz(dev.desc.csel_mask),
    TPS65218_PROTECT_L1);
    }
    static const struct regulator_ops tps65218_ls23_ops = {
    .is_enabled		= regulator_is_enabled_regmap,
    .enable			= tps65218_pmic_enable,
    .disable		= tps65218_pmic_disable,
    .set_input_current_limit = tps65218_pmic_set_input_current_lim,
    .set_current_limit	= tps65218_pmic_set_current_limit,
    .get_current_limit	= regulator_get_current_limit_regmap,
    };
// Operations permitted on DCDC5, DCDC6
    static const struct regulator_ops tps65218_dcdc56_pmic_ops = {
    .is_enabled		= regulator_is_enabled_regmap,
    .enable			= tps65218_pmic_enable,
    .disable		= tps65218_pmic_disable,
    .set_suspend_enable	= tps65218_pmic_set_suspend_enable,
    .set_suspend_disable	= tps65218_pmic_set_suspend_disable,
    };
    static const struct regulator_desc regulators[] = {
    TPS65218_REGULATOR("DCDC1", "regulator-dcdc1", TPS65218_DCDC_1,
    REGULATOR_VOLTAGE, tps65218_dcdc12_ops, 64,
    TPS65218_REG_CONTROL_DCDC1,
    TPS65218_CONTROL_DCDC1_MASK, TPS65218_REG_ENABLE1,
    TPS65218_ENABLE1_DC1_EN, 0, 0, dcdc1_dcdc2_ranges,
    2, 4000, 0, TPS65218_REG_SEQ3,
    TPS65218_SEQ3_DC1_SEQ_MASK, core::ptr::null_mut(), 0),
    TPS65218_REGULATOR("DCDC2", "regulator-dcdc2", TPS65218_DCDC_2,
    REGULATOR_VOLTAGE, tps65218_dcdc12_ops, 64,
    TPS65218_REG_CONTROL_DCDC2,
    TPS65218_CONTROL_DCDC2_MASK, TPS65218_REG_ENABLE1,
    TPS65218_ENABLE1_DC2_EN, 0, 0, dcdc1_dcdc2_ranges,
    2, 4000, 0, TPS65218_REG_SEQ3,
    TPS65218_SEQ3_DC2_SEQ_MASK, core::ptr::null_mut(), 0),
    TPS65218_REGULATOR("DCDC3", "regulator-dcdc3", TPS65218_DCDC_3,
    REGULATOR_VOLTAGE, tps65218_ldo1_dcdc34_ops, 64,
    TPS65218_REG_CONTROL_DCDC3,
    TPS65218_CONTROL_DCDC3_MASK, TPS65218_REG_ENABLE1,
    TPS65218_ENABLE1_DC3_EN, 0, 0, ldo1_dcdc3_ranges, 2,
    0, 0, TPS65218_REG_SEQ4, TPS65218_SEQ4_DC3_SEQ_MASK,
    core::ptr::null_mut(), 0),
    TPS65218_REGULATOR("DCDC4", "regulator-dcdc4", TPS65218_DCDC_4,
    REGULATOR_VOLTAGE, tps65218_ldo1_dcdc34_ops, 53,
    TPS65218_REG_CONTROL_DCDC4,
    TPS65218_CONTROL_DCDC4_MASK, TPS65218_REG_ENABLE1,
    TPS65218_ENABLE1_DC4_EN, 0, 0, dcdc4_ranges, 2,
    0, 0, TPS65218_REG_SEQ4, TPS65218_SEQ4_DC4_SEQ_MASK,
    core::ptr::null_mut(), 0),
    TPS65218_REGULATOR("DCDC5", "regulator-dcdc5", TPS65218_DCDC_5,
    REGULATOR_VOLTAGE, tps65218_dcdc56_pmic_ops, 1, -1,
    -1, TPS65218_REG_ENABLE1, TPS65218_ENABLE1_DC5_EN, 0,
    0, core::ptr::null_mut(), 0, 0, 1000000, TPS65218_REG_SEQ5,
    TPS65218_SEQ5_DC5_SEQ_MASK, core::ptr::null_mut(), 0),
    TPS65218_REGULATOR("DCDC6", "regulator-dcdc6", TPS65218_DCDC_6,
    REGULATOR_VOLTAGE, tps65218_dcdc56_pmic_ops, 1, -1,
    -1, TPS65218_REG_ENABLE1, TPS65218_ENABLE1_DC6_EN, 0,
    0, core::ptr::null_mut(), 0, 0, 1800000, TPS65218_REG_SEQ5,
    TPS65218_SEQ5_DC6_SEQ_MASK, core::ptr::null_mut(), 0),
    TPS65218_REGULATOR("LDO1", "regulator-ldo1", TPS65218_LDO_1,
    REGULATOR_VOLTAGE, tps65218_ldo1_dcdc34_ops, 64,
    TPS65218_REG_CONTROL_LDO1,
    TPS65218_CONTROL_LDO1_MASK, TPS65218_REG_ENABLE2,
    TPS65218_ENABLE2_LDO1_EN, 0, 0, ldo1_dcdc3_ranges,
    2, 0, 0, TPS65218_REG_SEQ6,
    TPS65218_SEQ6_LDO1_SEQ_MASK, core::ptr::null_mut(), 0),
    TPS65218_REGULATOR("LS2", "regulator-ls2", TPS65218_LS_2,
    REGULATOR_CURRENT, tps65218_ls23_ops, 0, 0, 0,
    TPS65218_REG_ENABLE2, TPS65218_ENABLE2_LS2_EN,
    TPS65218_REG_CONFIG2, TPS65218_CONFIG2_LS2ILIM_MASK,
    core::ptr::null_mut(), 0, 0, 0, 0, 0, ls3_currents,
    ARRAY_SIZE(ls3_currents)),
    TPS65218_REGULATOR("LS3", "regulator-ls3", TPS65218_LS_3,
    REGULATOR_CURRENT, tps65218_ls23_ops, 0, 0, 0,
    TPS65218_REG_ENABLE2, TPS65218_ENABLE2_LS3_EN,
    TPS65218_REG_CONFIG2, TPS65218_CONFIG2_LS3ILIM_MASK,
    core::ptr::null_mut(), 0, 0, 0, 0, 0, ls3_currents,
    ARRAY_SIZE(ls3_currents)),
    };
#[no_mangle]
unsafe extern "C" fn tps65218_regulator_probe(pdev: *mut platform_device) -> c_int {
    static int tps65218_regulator_probe(struct platform_device *pdev)
    {
    struct tps65218 *tps = dev_get_drvdata(pdev.dev.parent);
    struct regulator_dev *rdev;
    let mut config: regulator_config = { };
    int i, ret;
    unsigned int val;
    config.dev = &pdev.dev;
    config.dev.of_node = tps.dev.of_node;
    config.driver_data = tps;
    config.regmap = tps.regmap;
// Allocate memory for strobes
    tps.strobes = devm_kcalloc(&pdev.dev,
    TPS65218_NUM_REGULATOR, sizeof(u8),
    GFP_KERNEL);
    if (!tps.strobes)
    return -ENOMEM;
    for (i = 0; i < ARRAY_SIZE(regulators); i++) {
    rdev = devm_regulator_register(&pdev.dev, &regulators[i],
    &config);
    if (IS_ERR(rdev)) {
    dev_err(tps.dev, "failed to register %s regulator\n",
    pdev.name);
    return PTR_ERR(rdev);
    }
    ret = regmap_read(tps.regmap, regulators[i].bypass_reg, &val);
    if (ret)
    return ret;
    tps.strobes[i] = val & regulators[i].bypass_mask;
    }
    return 0;
    }
    static const struct platform_device_id tps65218_regulator_id_table[] = {
    { .name = "tps65218-regulator" },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(platform, tps65218_regulator_id_table);
    static struct platform_driver tps65218_regulator_driver = {
    .driver = {
    .name = "tps65218-pmic",
    .probe_type = PROBE_PREFER_ASYNCHRONOUS,
    },
    .probe = tps65218_regulator_probe,
    .id_table = tps65218_regulator_id_table,
    };
    module_platform_driver(tps65218_regulator_driver);
    MODULE_AUTHOR("J Keerthy <j-keerthy@ti.com>");
    MODULE_DESCRIPTION("TPS65218 voltage regulator driver");
    MODULE_ALIAS("platform:tps65218-pmic");
    MODULE_LICENSE("GPL v2");
