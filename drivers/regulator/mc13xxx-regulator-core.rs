//! Automatically rewritten from C to Rust
//! Source: drivers/regulator/mc13xxx-regulator-core.c
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
// Regulator Driver for Freescale MC13xxx PMIC
//
// Copyright 2010 Yong Shen <yong.shen@linaro.org>
//
// Based on mc13783 regulator driver :
// Copyright (C) 2008 Sascha Hauer, Pengutronix <s.hauer@pengutronix.de>
// Copyright 2009 Alberto Panizzo <maramaopercheseimorto@gmail.com>
//
// Regs infos taken from mc13xxx drivers from freescale and mc13xxx.pdf file
// from freescale

#[no_mangle]
unsafe extern "C" fn mc13xxx_regulator_enable(rdev: *mut regulator_dev) -> c_int {
    static int mc13xxx_regulator_enable(struct regulator_dev *rdev)
    {
    struct mc13xxx_regulator_priv *priv = rdev_get_drvdata(rdev);
    struct mc13xxx_regulator *mc13xxx_regulators = priv.mc13xxx_regulators;
    let mut id: c_int = rdev_get_id(rdev);
    dev_dbg(rdev_get_dev(rdev), "%s id: %d\n", __func__, id);
    return mc13xxx_reg_rmw(priv.mc13xxx, mc13xxx_regulators[id].reg,
    mc13xxx_regulators[id].enable_bit,
    mc13xxx_regulators[id].enable_bit);
    }
#[no_mangle]
unsafe extern "C" fn mc13xxx_regulator_disable(rdev: *mut regulator_dev) -> c_int {
    static int mc13xxx_regulator_disable(struct regulator_dev *rdev)
    {
    struct mc13xxx_regulator_priv *priv = rdev_get_drvdata(rdev);
    struct mc13xxx_regulator *mc13xxx_regulators = priv.mc13xxx_regulators;
    let mut id: c_int = rdev_get_id(rdev);
    dev_dbg(rdev_get_dev(rdev), "%s id: %d\n", __func__, id);
    return mc13xxx_reg_rmw(priv.mc13xxx, mc13xxx_regulators[id].reg,
    mc13xxx_regulators[id].enable_bit, 0);
    }
#[no_mangle]
unsafe extern "C" fn mc13xxx_regulator_is_enabled(rdev: *mut regulator_dev) -> c_int {
    static int mc13xxx_regulator_is_enabled(struct regulator_dev *rdev)
    {
    struct mc13xxx_regulator_priv *priv = rdev_get_drvdata(rdev);
    struct mc13xxx_regulator *mc13xxx_regulators = priv.mc13xxx_regulators;
    int ret, id = rdev_get_id(rdev);
    unsigned int val;
    ret = mc13xxx_reg_read(priv.mc13xxx, mc13xxx_regulators[id].reg, &val);
    if (ret)
    return ret;
    return (val & mc13xxx_regulators[id].enable_bit) != 0;
    }
    static int mc13xxx_regulator_set_voltage_sel(struct regulator_dev *rdev,
    unsigned selector)
    {
    struct mc13xxx_regulator_priv *priv = rdev_get_drvdata(rdev);
    struct mc13xxx_regulator *mc13xxx_regulators = priv.mc13xxx_regulators;
    let mut id: c_int = rdev_get_id(rdev);
    return mc13xxx_reg_rmw(priv.mc13xxx, mc13xxx_regulators[id].vsel_reg,
    mc13xxx_regulators[id].vsel_mask,
    selector << mc13xxx_regulators[id].vsel_shift);
    }
#[no_mangle]
unsafe extern "C" fn mc13xxx_regulator_get_voltage(rdev: *mut regulator_dev) -> c_int {
    static int mc13xxx_regulator_get_voltage(struct regulator_dev *rdev)
    {
    struct mc13xxx_regulator_priv *priv = rdev_get_drvdata(rdev);
    struct mc13xxx_regulator *mc13xxx_regulators = priv.mc13xxx_regulators;
    int ret, id = rdev_get_id(rdev);
    unsigned int val;
    dev_dbg(rdev_get_dev(rdev), "%s id: %d\n", __func__, id);
    ret = mc13xxx_reg_read(priv.mc13xxx,
    mc13xxx_regulators[id].vsel_reg, &val);
    if (ret)
    return ret;
    val = (val & mc13xxx_regulators[id].vsel_mask)
    >> mc13xxx_regulators[id].vsel_shift;
    dev_dbg(rdev_get_dev(rdev), "%s id: %d val: %d\n", __func__, id, val);
    BUG_ON(val >= mc13xxx_regulators[id].desc.n_voltages);
    return rdev.desc.volt_table[val];
    }
    const struct regulator_ops mc13xxx_regulator_ops = {
    .enable = mc13xxx_regulator_enable,
    .disable = mc13xxx_regulator_disable,
    .is_enabled = mc13xxx_regulator_is_enabled,
    .list_voltage = regulator_list_voltage_table,
    .set_voltage_sel = mc13xxx_regulator_set_voltage_sel,
    .get_voltage = mc13xxx_regulator_get_voltage,
    };
    EXPORT_SYMBOL_GPL(mc13xxx_regulator_ops);
    int mc13xxx_fixed_regulator_set_voltage(struct regulator_dev *rdev, int min_uV,
    int max_uV, unsigned *selector)
    {
    let mut id: c_int = rdev_get_id(rdev);
    dev_dbg(rdev_get_dev(rdev), "%s id: %d min_uV: %d max_uV: %d\n",
    __func__, id, min_uV, max_uV);
    if (min_uV <= rdev.desc.volt_table[0] &&
    rdev.desc.volt_table[0] <= max_uV) {
// selector = 0;
    return 0;
    } else {
    return -EINVAL;
    }
    }
    EXPORT_SYMBOL_GPL(mc13xxx_fixed_regulator_set_voltage);
    const struct regulator_ops mc13xxx_fixed_regulator_ops = {
    .enable = mc13xxx_regulator_enable,
    .disable = mc13xxx_regulator_disable,
    .is_enabled = mc13xxx_regulator_is_enabled,
    .list_voltage = regulator_list_voltage_table,
    .set_voltage = mc13xxx_fixed_regulator_set_voltage,
    };
    EXPORT_SYMBOL_GPL(mc13xxx_fixed_regulator_ops);

#[no_mangle]
pub unsafe extern "C" fn mc13xxx_get_num_regulators_dt(pdev: *mut platform_device) -> c_int {
    int mc13xxx_get_num_regulators_dt(struct platform_device *pdev)
    {
    struct device_node *parent;
    int num;
    if (!pdev.dev.parent.of_node)
    return -ENODEV;
    parent = of_get_child_by_name(pdev.dev.parent.of_node, "regulators");
    if (!parent)
    return -ENODEV;
    num = of_get_child_count(parent);
    of_node_put(parent);
    return num;
    }
    EXPORT_SYMBOL_GPL(mc13xxx_get_num_regulators_dt);
    struct mc13xxx_regulator_init_data *mc13xxx_parse_regulators_dt(
    struct platform_device *pdev, struct mc13xxx_regulator *regulators,
    int num_regulators)
    {
    struct mc13xxx_regulator_priv *priv = platform_get_drvdata(pdev);
    struct mc13xxx_regulator_init_data *data, *p;
    struct device_node *parent, *child;
    int i, parsed = 0;
    if (!pdev.dev.parent.of_node)
    return core::ptr::null_mut();
    parent = of_get_child_by_name(pdev.dev.parent.of_node, "regulators");
    if (!parent)
    return core::ptr::null_mut();
    data = devm_kcalloc(&pdev.dev, priv.num_regulators, sizeof(*data),
    GFP_KERNEL);
    if (!data) {
    of_node_put(parent);
    return core::ptr::null_mut();
    }
    p = data;
    for_each_child_of_node(parent, child) {
    let mut found: c_int = 0;
    for (i = 0; i < num_regulators; i++) {
    if (!regulators[i].desc.name)
    continue;
    if (of_node_name_eq(child,
    regulators[i].desc.name)) {
    p.id = i;
    p.init_data = of_get_regulator_init_data(
    &pdev.dev, child,
    &regulators[i].desc);
    p.node = child;
    p++;
    parsed++;
    found = 1;
    break;
    }
    }
    if (!found)
    dev_warn(&pdev.dev,
    "Unknown regulator: %pOFn\n", child);
    }
    of_node_put(parent);
    priv.num_regulators = parsed;
    return data;
    }
    EXPORT_SYMBOL_GPL(mc13xxx_parse_regulators_dt);

    MODULE_LICENSE("GPL v2");
    MODULE_AUTHOR("Yong Shen <yong.shen@linaro.org>");
    MODULE_DESCRIPTION("Regulator Driver for Freescale MC13xxx PMIC");
    MODULE_ALIAS("mc13xxx-regulator-core");
