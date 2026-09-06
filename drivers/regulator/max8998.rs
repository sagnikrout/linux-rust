//! Automatically rewritten from C to Rust
//! Source: drivers/regulator/max8998.c
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
// max8998.c - Voltage regulator driver for the Maxim 8998
//
// Copyright (C) 2009-2010 Samsung Electronics
// Kyungmin Park <kyungmin.park@samsung.com>
// Marek Szyprowski <m.szyprowski@samsung.com>

#[repr(C)]
#[derive(Copy, Clone)]
pub struct max8998_data {
    pub dev: *mut device,
    pub iodev: *mut max8998_dev,
    pub num_regulators: c_int,
    pub /: *mut *mut u8 buck1_vol[4]; / voltages for selection,
    pub buck2_vol: [u8; 2],
    pub /: *mut *mut unsigned int buck1_idx; / index to last changed voltage,
// value in a set
    pub buck2_idx: c_uint,
    pub buck1_gpio1: *mut gpio_desc,
    pub buck1_gpio2: *mut gpio_desc,
    pub buck2_gpio: *mut gpio_desc,
}

    static const unsigned int charger_current_table[] = {
    90000, 380000, 475000, 550000, 570000, 600000, 700000, 800000,
    };
    static int max8998_get_enable_register(struct regulator_dev *rdev,
    int *reg, int *shift)
    {
    let mut ldo: c_int = rdev_get_id(rdev);
    switch (ldo) {
    case MAX8998_LDO2 ... MAX8998_LDO5:
// reg = MAX8998_REG_ONOFF1;
// shift = 3 - (ldo - MAX8998_LDO2);
    break;
    case MAX8998_LDO6 ... MAX8998_LDO13:
// reg = MAX8998_REG_ONOFF2;
// shift = 7 - (ldo - MAX8998_LDO6);
    break;
    case MAX8998_LDO14 ... MAX8998_LDO17:
// reg = MAX8998_REG_ONOFF3;
// shift = 7 - (ldo - MAX8998_LDO14);
    break;
    case MAX8998_BUCK1 ... MAX8998_BUCK4:
// reg = MAX8998_REG_ONOFF1;
// shift = 7 - (ldo - MAX8998_BUCK1);
    break;
    case MAX8998_EN32KHZ_AP ... MAX8998_ENVICHG:
// reg = MAX8998_REG_ONOFF4;
// shift = 7 - (ldo - MAX8998_EN32KHZ_AP);
    break;
    case MAX8998_ESAFEOUT1 ... MAX8998_ESAFEOUT2:
// reg = MAX8998_REG_CHGR2;
// shift = 7 - (ldo - MAX8998_ESAFEOUT1);
    break;
    case MAX8998_CHARGER:
// reg = MAX8998_REG_CHGR2;
// shift = 0;
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn max8998_ldo_is_enabled(rdev: *mut regulator_dev) -> c_int {
    static int max8998_ldo_is_enabled(struct regulator_dev *rdev)
    {
    struct max8998_data *max8998 = rdev_get_drvdata(rdev);
    struct i2c_client *i2c = max8998.iodev.i2c;
    int ret, reg, shift = 8;
    u8 val;
    ret = max8998_get_enable_register(rdev, &reg, &shift);
    if (ret)
    return ret;
    ret = max8998_read_reg(i2c, reg, &val);
    if (ret)
    return ret;
    return val & (1 << shift);
    }
#[no_mangle]
unsafe extern "C" fn max8998_ldo_is_enabled_inverted(rdev: *mut regulator_dev) -> c_int {
    static int max8998_ldo_is_enabled_inverted(struct regulator_dev *rdev)
    {
    return (!max8998_ldo_is_enabled(rdev));
    }
#[no_mangle]
unsafe extern "C" fn max8998_ldo_enable(rdev: *mut regulator_dev) -> c_int {
    static int max8998_ldo_enable(struct regulator_dev *rdev)
    {
    struct max8998_data *max8998 = rdev_get_drvdata(rdev);
    struct i2c_client *i2c = max8998.iodev.i2c;
    int reg, shift = 8, ret;
    ret = max8998_get_enable_register(rdev, &reg, &shift);
    if (ret)
    return ret;
    return max8998_update_reg(i2c, reg, 1<<shift, 1<<shift);
    }
#[no_mangle]
unsafe extern "C" fn max8998_ldo_disable(rdev: *mut regulator_dev) -> c_int {
    static int max8998_ldo_disable(struct regulator_dev *rdev)
    {
    struct max8998_data *max8998 = rdev_get_drvdata(rdev);
    struct i2c_client *i2c = max8998.iodev.i2c;
    int reg, shift = 8, ret;
    ret = max8998_get_enable_register(rdev, &reg, &shift);
    if (ret)
    return ret;
    return max8998_update_reg(i2c, reg, 0, 1<<shift);
    }
    static int max8998_get_voltage_register(struct regulator_dev *rdev,
    int *_reg, int *_shift, int *_mask)
    {
    let mut ldo: c_int = rdev_get_id(rdev);
    struct max8998_data *max8998 = rdev_get_drvdata(rdev);
    int reg, shift = 0, mask = 0xff;
    switch (ldo) {
    case MAX8998_LDO2 ... MAX8998_LDO3:
    reg = MAX8998_REG_LDO2_LDO3;
    mask = 0xf;
    if (ldo == MAX8998_LDO2)
    shift = 4;
    else
    shift = 0;
    break;
    case MAX8998_LDO4 ... MAX8998_LDO7:
    reg = MAX8998_REG_LDO4 + (ldo - MAX8998_LDO4);
    break;
    case MAX8998_LDO8 ... MAX8998_LDO9:
    reg = MAX8998_REG_LDO8_LDO9;
    mask = 0xf;
    if (ldo == MAX8998_LDO8)
    shift = 4;
    else
    shift = 0;
    break;
    case MAX8998_LDO10 ... MAX8998_LDO11:
    reg = MAX8998_REG_LDO10_LDO11;
    if (ldo == MAX8998_LDO10) {
    shift = 5;
    mask = 0x7;
    } else {
    shift = 0;
    mask = 0x1f;
    }
    break;
    case MAX8998_LDO12 ... MAX8998_LDO17:
    reg = MAX8998_REG_LDO12 + (ldo - MAX8998_LDO12);
    break;
    case MAX8998_BUCK1:
    reg = MAX8998_REG_BUCK1_VOLTAGE1 + max8998.buck1_idx;
    break;
    case MAX8998_BUCK2:
    reg = MAX8998_REG_BUCK2_VOLTAGE1 + max8998.buck2_idx;
    break;
    case MAX8998_BUCK3:
    reg = MAX8998_REG_BUCK3;
    break;
    case MAX8998_BUCK4:
    reg = MAX8998_REG_BUCK4;
    break;
    default:
    return -EINVAL;
    }
// _reg = reg;
// _shift = shift;
// _mask = mask;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn max8998_get_voltage_sel(rdev: *mut regulator_dev) -> c_int {
    static int max8998_get_voltage_sel(struct regulator_dev *rdev)
    {
    struct max8998_data *max8998 = rdev_get_drvdata(rdev);
    struct i2c_client *i2c = max8998.iodev.i2c;
    int reg, shift = 0, mask, ret;
    u8 val;
    ret = max8998_get_voltage_register(rdev, &reg, &shift, &mask);
    if (ret)
    return ret;
    ret = max8998_read_reg(i2c, reg, &val);
    if (ret)
    return ret;
    val >>= shift;
    val &= mask;
    return val;
    }
    static int max8998_set_voltage_ldo_sel(struct regulator_dev *rdev,
    unsigned selector)
    {
    struct max8998_data *max8998 = rdev_get_drvdata(rdev);
    struct i2c_client *i2c = max8998.iodev.i2c;
    int reg, shift = 0, mask, ret;
    ret = max8998_get_voltage_register(rdev, &reg, &shift, &mask);
    if (ret)
    return ret;
    ret = max8998_update_reg(i2c, reg, selector<<shift, mask<<shift);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn buck1_gpio_set(gpio1: *mut gpio_desc, gpio2: *mut gpio_desc, v: c_int) {
    static inline void buck1_gpio_set(struct gpio_desc *gpio1, struct gpio_desc *gpio2, int v)
    {
    gpiod_set_value(gpio1, v & 0x1);
    gpiod_set_value(gpio2, (v >> 1) & 0x1);
    }
#[no_mangle]
pub unsafe extern "C" fn buck2_gpio_set(gpio: *mut gpio_desc, v: c_int) {
    static inline void buck2_gpio_set(struct gpio_desc *gpio, int v)
    {
    gpiod_set_value(gpio, v & 0x1);
    }
    static int max8998_set_voltage_buck_sel(struct regulator_dev *rdev,
    unsigned selector)
    {
    struct max8998_data *max8998 = rdev_get_drvdata(rdev);
    struct max8998_platform_data *pdata = max8998.iodev.pdata;
    struct i2c_client *i2c = max8998.iodev.i2c;
    let mut buck: c_int = rdev_get_id(rdev);
    int reg, shift = 0, mask, ret, j;
    static u8 buck1_last_val;
    ret = max8998_get_voltage_register(rdev, &reg, &shift, &mask);
    if (ret)
    return ret;
    switch (buck) {
    case MAX8998_BUCK1:
    dev_dbg(max8998.dev,
    "BUCK1, selector:%d, buck1_vol1:%d, buck1_vol2:%d\n"
    "buck1_vol3:%d, buck1_vol4:%d\n",
    selector, max8998.buck1_vol[0], max8998.buck1_vol[1],
    max8998.buck1_vol[2], max8998.buck1_vol[3]);
    if (max8998.buck1_gpio1 && max8998.buck1_gpio2) {
// check if requested voltage
// value is already defined
    for (j = 0; j < ARRAY_SIZE(max8998.buck1_vol); j++) {
    if (max8998.buck1_vol[j] == selector) {
    max8998.buck1_idx = j;
    buck1_gpio_set(max8998.buck1_gpio1,
    max8998.buck1_gpio2, j);
    goto buck1_exit;
    }
    }
    if (pdata.buck_voltage_lock)
    return -EINVAL;
// no predefine regulator found
    max8998.buck1_idx = (buck1_last_val % 2) + 2;
    dev_dbg(max8998.dev, "max8998.buck1_idx:%d\n",
    max8998.buck1_idx);
    max8998.buck1_vol[max8998.buck1_idx] = selector;
    ret = max8998_get_voltage_register(rdev, &reg,
    &shift,
    &mask);
    ret = max8998_write_reg(i2c, reg, selector);
    buck1_gpio_set(max8998.buck1_gpio1,
    max8998.buck1_gpio2, max8998.buck1_idx);
    buck1_last_val++;
    buck1_exit:
    dev_dbg(max8998.dev, "%s: SET1:%d, SET2:%d\n",
    i2c.name, gpiod_get_value(max8998.buck1_gpio1),
    gpiod_get_value(max8998.buck1_gpio2));
    break;
    } else {
    ret = max8998_write_reg(i2c, reg, selector);
    }
    break;
    case MAX8998_BUCK2:
    dev_dbg(max8998.dev,
    "BUCK2, selector:%d buck2_vol1:%d, buck2_vol2:%d\n",
    selector, max8998.buck2_vol[0], max8998.buck2_vol[1]);
    if (max8998.buck2_gpio) {
// check if requested voltage
// value is already defined
    for (j = 0; j < ARRAY_SIZE(max8998.buck2_vol); j++) {
    if (max8998.buck2_vol[j] == selector) {
    max8998.buck2_idx = j;
    buck2_gpio_set(max8998.buck2_gpio, j);
    goto buck2_exit;
    }
    }
    if (pdata.buck_voltage_lock)
    return -EINVAL;
    max8998_get_voltage_register(rdev,
    &reg, &shift, &mask);
    ret = max8998_write_reg(i2c, reg, selector);
    max8998.buck2_vol[max8998.buck2_idx] = selector;
    buck2_gpio_set(max8998.buck2_gpio, max8998.buck2_idx);
    buck2_exit:
    dev_dbg(max8998.dev, "%s: SET3:%d\n", i2c.name,
    gpiod_get_value(max8998.buck2_gpio));
    } else {
    ret = max8998_write_reg(i2c, reg, selector);
    }
    break;
    case MAX8998_BUCK3:
    case MAX8998_BUCK4:
    ret = max8998_update_reg(i2c, reg, selector<<shift,
    mask<<shift);
    break;
    }
    return ret;
    }
    static int max8998_set_voltage_buck_time_sel(struct regulator_dev *rdev,
    unsigned int old_selector,
    unsigned int new_selector)
    {
    struct max8998_data *max8998 = rdev_get_drvdata(rdev);
    struct i2c_client *i2c = max8998.iodev.i2c;
    let mut buck: c_int = rdev_get_id(rdev);
    let mut val: u8 = 0;
    int difference, ret;
    if (buck < MAX8998_BUCK1 || buck > MAX8998_BUCK4)
    return -EINVAL;
// Voltage stabilization
    ret = max8998_read_reg(i2c, MAX8998_REG_ONOFF4, &val);
    if (ret)
    return ret;
// lp3974 hasn't got ENRAMP bit - ramp is assumed as true
// MAX8998 has ENRAMP bit implemented, so test it
    if (max8998.iodev.type == TYPE_MAX8998 && !(val & MAX8998_ENRAMP))
    return 0;
    difference = (new_selector - old_selector) * rdev.desc.uV_step / 1000;
    if (difference > 0)
    return DIV_ROUND_UP(difference, (val & 0x0f) + 1);
    return 0;
    }
    static int max8998_set_current_limit(struct regulator_dev *rdev,
    int min_uA, int max_uA)
    {
    struct max8998_data *max8998 = rdev_get_drvdata(rdev);
    struct i2c_client *i2c = max8998.iodev.i2c;
    let mut n_currents: c_uint = rdev.desc.n_current_limits;
    int i, sel = -1;
    if (n_currents == 0)
    return -EINVAL;
    if (rdev.desc.curr_table) {
    const unsigned int *curr_table = rdev.desc.curr_table;
    let mut ascend: bool = curr_table[n_currents - 1] > curr_table[0];
// search for closest to maximum
    if (ascend) {
    for (i = n_currents - 1; i >= 0; i--) {
    if (min_uA <= curr_table[i] &&
    curr_table[i] <= max_uA) {
    sel = i;
    break;
    }
    }
    } else {
    for (i = 0; i < n_currents; i++) {
    if (min_uA <= curr_table[i] &&
    curr_table[i] <= max_uA) {
    sel = i;
    break;
    }
    }
    }
    }
    if (sel < 0)
    return -EINVAL;
    sel <<= ffs(rdev.desc.csel_mask) - 1;
    return max8998_update_reg(i2c, rdev.desc.csel_reg,
    sel, rdev.desc.csel_mask);
    }
#[no_mangle]
unsafe extern "C" fn max8998_get_current_limit(rdev: *mut regulator_dev) -> c_int {
    static int max8998_get_current_limit(struct regulator_dev *rdev)
    {
    struct max8998_data *max8998 = rdev_get_drvdata(rdev);
    struct i2c_client *i2c = max8998.iodev.i2c;
    u8 val;
    int ret;
    ret = max8998_read_reg(i2c, rdev.desc.csel_reg, &val);
    if (ret != 0)
    return ret;
    val &= rdev.desc.csel_mask;
    val >>= ffs(rdev.desc.csel_mask) - 1;
    if (rdev.desc.curr_table) {
    if (val >= rdev.desc.n_current_limits)
    return -EINVAL;
    return rdev.desc.curr_table[val];
    }
    return -EINVAL;
    }
    static const struct regulator_ops max8998_ldo_ops = {
    .list_voltage		= regulator_list_voltage_linear,
    .map_voltage		= regulator_map_voltage_linear,
    .is_enabled		= max8998_ldo_is_enabled,
    .enable			= max8998_ldo_enable,
    .disable		= max8998_ldo_disable,
    .get_voltage_sel	= max8998_get_voltage_sel,
    .set_voltage_sel	= max8998_set_voltage_ldo_sel,
    };
    static const struct regulator_ops max8998_buck_ops = {
    .list_voltage		= regulator_list_voltage_linear,
    .map_voltage		= regulator_map_voltage_linear,
    .is_enabled		= max8998_ldo_is_enabled,
    .enable			= max8998_ldo_enable,
    .disable		= max8998_ldo_disable,
    .get_voltage_sel	= max8998_get_voltage_sel,
    .set_voltage_sel	= max8998_set_voltage_buck_sel,
    .set_voltage_time_sel	= max8998_set_voltage_buck_time_sel,
    };
    static const struct regulator_ops max8998_charger_ops = {
    .set_current_limit	= max8998_set_current_limit,
    .get_current_limit	= max8998_get_current_limit,
    .is_enabled		= max8998_ldo_is_enabled_inverted,
// Swapped as register is inverted
    .enable			= max8998_ldo_disable,
    .disable		= max8998_ldo_enable,
    };
    static const struct regulator_ops max8998_others_ops = {
    .is_enabled		= max8998_ldo_is_enabled,
    .enable			= max8998_ldo_enable,
    .disable		= max8998_ldo_disable,
    };

    { \
    .name = #_name, \
    .id = MAX8998_##_name, \
    .ops = _ops, \
    .min_uV = (_min), \
    .uV_step = (_step), \
    .n_voltages = ((_max) - (_min)) / (_step) + 1, \
    .type = REGULATOR_VOLTAGE, \
    .owner = THIS_MODULE, \
    }

    { \
    .name = #_name, \
    .id = MAX8998_##_name, \
    .ops = _ops, \
    .curr_table = _table, \
    .n_current_limits = ARRAY_SIZE(_table), \
    .csel_reg = _reg, \
    .csel_mask = _mask, \
    .type = REGULATOR_CURRENT, \
    .owner = THIS_MODULE, \
    }

    { \
    .name = #_name, \
    .id = _id, \
    .ops = &max8998_others_ops, \
    .type = REGULATOR_VOLTAGE, \
    .owner = THIS_MODULE, \
    }
    static const struct regulator_desc regulators[] = {
    MAX8998_LINEAR_REG(LDO2, &max8998_ldo_ops, 800000, 50000, 1300000),
    MAX8998_LINEAR_REG(LDO3, &max8998_ldo_ops, 800000, 50000, 1300000),
    MAX8998_LINEAR_REG(LDO4, &max8998_ldo_ops, 1600000, 100000, 3600000),
    MAX8998_LINEAR_REG(LDO5, &max8998_ldo_ops, 1600000, 100000, 3600000),
    MAX8998_LINEAR_REG(LDO6, &max8998_ldo_ops, 1600000, 100000, 3600000),
    MAX8998_LINEAR_REG(LDO7, &max8998_ldo_ops, 1600000, 100000, 3600000),
    MAX8998_LINEAR_REG(LDO8, &max8998_ldo_ops, 3000000, 100000, 3600000),
    MAX8998_LINEAR_REG(LDO9, &max8998_ldo_ops, 2800000, 100000, 3100000),
    MAX8998_LINEAR_REG(LDO10, &max8998_ldo_ops, 950000, 50000, 1300000),
    MAX8998_LINEAR_REG(LDO11, &max8998_ldo_ops, 1600000, 100000, 3600000),
    MAX8998_LINEAR_REG(LDO12, &max8998_ldo_ops, 800000, 100000, 3300000),
    MAX8998_LINEAR_REG(LDO13, &max8998_ldo_ops, 800000, 100000, 3300000),
    MAX8998_LINEAR_REG(LDO14, &max8998_ldo_ops, 1200000, 100000, 3300000),
    MAX8998_LINEAR_REG(LDO15, &max8998_ldo_ops, 1200000, 100000, 3300000),
    MAX8998_LINEAR_REG(LDO16, &max8998_ldo_ops, 1600000, 100000, 3600000),
    MAX8998_LINEAR_REG(LDO17, &max8998_ldo_ops, 1600000, 100000, 3600000),
    MAX8998_LINEAR_REG(BUCK1, &max8998_buck_ops, 750000, 25000, 1525000),
    MAX8998_LINEAR_REG(BUCK2, &max8998_buck_ops, 750000, 25000, 1525000),
    MAX8998_LINEAR_REG(BUCK3, &max8998_buck_ops, 1600000, 100000, 3600000),
    MAX8998_LINEAR_REG(BUCK4, &max8998_buck_ops, 800000, 100000, 2300000),
    MAX8998_OTHERS_REG(EN32KHz-AP, MAX8998_EN32KHZ_AP),
    MAX8998_OTHERS_REG(EN32KHz-CP, MAX8998_EN32KHZ_CP),
    MAX8998_OTHERS_REG(ENVICHG, MAX8998_ENVICHG),
    MAX8998_OTHERS_REG(ESAFEOUT1, MAX8998_ESAFEOUT1),
    MAX8998_OTHERS_REG(ESAFEOUT2, MAX8998_ESAFEOUT2),
    MAX8998_CURRENT_REG(CHARGER, &max8998_charger_ops,
    charger_current_table, MAX8998_REG_CHGR1, 0x7),
    };
    static int max8998_pmic_dt_parse_pdata(struct max8998_dev *iodev,
    struct max8998_platform_data *pdata)
    {
    struct device_node *pmic_np = iodev.dev.of_node;
    struct device_node *regulators_np, *reg_np;
    struct max8998_regulator_data *rdata;
    unsigned int i;
    int ret;
    regulators_np = of_get_child_by_name(pmic_np, "regulators");
    if (!regulators_np) {
    dev_err(iodev.dev, "could not find regulators sub-node\n");
    return -EINVAL;
    }
// count the number of regulators to be supported in pmic
    pdata.num_regulators = of_get_child_count(regulators_np);
    rdata = devm_kcalloc(iodev.dev,
    pdata.num_regulators, sizeof(*rdata),
    GFP_KERNEL);
    if (!rdata) {
    of_node_put(regulators_np);
    return -ENOMEM;
    }
    pdata.regulators = rdata;
    for (i = 0; i < ARRAY_SIZE(regulators); ++i) {
    reg_np = of_get_child_by_name(regulators_np,
    regulators[i].name);
    if (!reg_np)
    continue;
    rdata.id = regulators[i].id;
    rdata.initdata = of_get_regulator_init_data(iodev.dev,
    reg_np,
    &regulators[i]);
    rdata.reg_node = reg_np;
    ++rdata;
    }
    pdata.num_regulators = rdata - pdata.regulators;
    of_node_put(regulators_np);
    pdata.buck_voltage_lock = of_property_read_bool(pmic_np, "max8998,pmic-buck-voltage-lock");
    ret = of_property_read_u32(pmic_np,
    "max8998,pmic-buck1-default-dvs-idx",
    &pdata.buck1_default_idx);
    if (!ret && pdata.buck1_default_idx >= 4) {
    pdata.buck1_default_idx = 0;
    dev_warn(iodev.dev, "invalid value for default dvs index, using 0 instead\n");
    }
    ret = of_property_read_u32(pmic_np,
    "max8998,pmic-buck2-default-dvs-idx",
    &pdata.buck2_default_idx);
    if (!ret && pdata.buck2_default_idx >= 2) {
    pdata.buck2_default_idx = 0;
    dev_warn(iodev.dev, "invalid value for default dvs index, using 0 instead\n");
    }
    ret = of_property_read_u32_array(pmic_np,
    "max8998,pmic-buck1-dvs-voltage",
    pdata.buck1_voltage,
    ARRAY_SIZE(pdata.buck1_voltage));
    if (ret) {
    dev_err(iodev.dev, "buck1 voltages not specified\n");
    return -EINVAL;
    }
    ret = of_property_read_u32_array(pmic_np,
    "max8998,pmic-buck2-dvs-voltage",
    pdata.buck2_voltage,
    ARRAY_SIZE(pdata.buck2_voltage));
    if (ret) {
    dev_err(iodev.dev, "buck2 voltages not specified\n");
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn max8998_pmic_probe(pdev: *mut platform_device) -> c_int {
    static int max8998_pmic_probe(struct platform_device *pdev)
    {
    struct max8998_dev *iodev = dev_get_drvdata(pdev.dev.parent);
    struct max8998_platform_data *pdata = iodev.pdata;
    let mut config: regulator_config = { };
    struct regulator_dev *rdev;
    struct max8998_data *max8998;
    struct i2c_client *i2c;
    enum gpiod_flags flags;
    int i, ret;
    unsigned int v;
    if (!pdata) {
    dev_err(pdev.dev.parent, "No platform init data supplied\n");
    return -ENODEV;
    }
    if (IS_ENABLED(CONFIG_OF) && iodev.dev.of_node) {
    ret = max8998_pmic_dt_parse_pdata(iodev, pdata);
    if (ret)
    return ret;
    }
    max8998 = devm_kzalloc(&pdev.dev, sizeof(struct max8998_data),
    GFP_KERNEL);
    if (!max8998)
    return -ENOMEM;
    max8998.dev = &pdev.dev;
    max8998.iodev = iodev;
    max8998.num_regulators = pdata.num_regulators;
    platform_set_drvdata(pdev, max8998);
    i2c = max8998.iodev.i2c;
    max8998.buck1_idx = pdata.buck1_default_idx;
    max8998.buck2_idx = pdata.buck2_default_idx;
// Check if MAX8998 voltage selection GPIOs are defined
    flags = (max8998.buck1_idx & BIT(0)) ? GPIOD_OUT_HIGH : GPIOD_OUT_LOW;
    max8998.buck1_gpio1 = devm_gpiod_get_index_optional(iodev.dev,
    "max8998,pmic-buck1-dvs",
    0,
    flags);
    if (IS_ERR(max8998.buck1_gpio1))
    return dev_err_probe(&pdev.dev, PTR_ERR(max8998.buck1_gpio1),
    "could not get BUCK1 GPIO1\n");
    gpiod_set_consumer_name(max8998.buck1_gpio1, "MAX8998 BUCK1_SET1");
    flags = (max8998.buck1_idx & BIT(1)) ? GPIOD_OUT_HIGH : GPIOD_OUT_LOW;
    max8998.buck1_gpio2 = devm_gpiod_get_index_optional(iodev.dev,
    "max8998,pmic-buck1-dvs",
    1,
    flags);
    if (IS_ERR(max8998.buck1_gpio2))
    return dev_err_probe(&pdev.dev, PTR_ERR(max8998.buck1_gpio2),
    "could not get BUCK1 GPIO2\n");
    gpiod_set_consumer_name(max8998.buck1_gpio1, "MAX8998 BUCK1_SET2");
    flags = (max8998.buck2_idx & BIT(0)) ? GPIOD_OUT_HIGH : GPIOD_OUT_LOW;
    max8998.buck2_gpio = devm_gpiod_get_index_optional(iodev.dev,
    "max8998,pmic-buck2-dvs",
    0,
    flags);
    if (IS_ERR(max8998.buck2_gpio))
    return dev_err_probe(&pdev.dev, PTR_ERR(max8998.buck2_gpio),
    "could not get BUCK2 GPIO\n");
    gpiod_set_consumer_name(max8998.buck1_gpio1, "MAX8998 BUCK2_SET3");
    if (max8998.buck1_gpio1 && max8998.buck1_gpio2) {
// Set predefined values for BUCK1 registers
    for (v = 0; v < ARRAY_SIZE(pdata.buck1_voltage); ++v) {
    let mut index: c_int = MAX8998_BUCK1 - MAX8998_LDO2;
    i = 0;
    while (regulators[index].min_uV +
    regulators[index].uV_step * i
    < pdata.buck1_voltage[v])
    i++;
    max8998.buck1_vol[v] = i;
    ret = max8998_write_reg(i2c,
    MAX8998_REG_BUCK1_VOLTAGE1 + v, i);
    if (ret)
    return ret;
    }
    }
    if (max8998.buck2_gpio) {
// Set predefined values for BUCK2 registers
    for (v = 0; v < ARRAY_SIZE(pdata.buck2_voltage); ++v) {
    let mut index: c_int = MAX8998_BUCK2 - MAX8998_LDO2;
    i = 0;
    while (regulators[index].min_uV +
    regulators[index].uV_step * i
    < pdata.buck2_voltage[v])
    i++;
    max8998.buck2_vol[v] = i;
    ret = max8998_write_reg(i2c,
    MAX8998_REG_BUCK2_VOLTAGE1 + v, i);
    if (ret)
    return ret;
    }
    }
    for (i = 0; i < pdata.num_regulators; i++) {
    let mut index: c_int = pdata.regulators[i].id - MAX8998_LDO2;
    config.dev = max8998.dev;
    config.of_node = pdata.regulators[i].reg_node;
    config.init_data = pdata.regulators[i].initdata;
    config.driver_data = max8998;
    rdev = devm_regulator_register(&pdev.dev, &regulators[index],
    &config);
    if (IS_ERR(rdev)) {
    ret = PTR_ERR(rdev);
    dev_err(max8998.dev, "regulator %s init failed (%d)\n",
    regulators[index].name, ret);
    return ret;
    }
    }
    return 0;
    }
    static const struct platform_device_id max8998_pmic_id[] = {
    { .name = "max8998-pmic", .driver_data = TYPE_MAX8998 },
    { .name = "lp3974-pmic", .driver_data = TYPE_LP3974 },
    { }
    };
    MODULE_DEVICE_TABLE(platform, max8998_pmic_id);
    static struct platform_driver max8998_pmic_driver = {
    .driver = {
    .name = "max8998-pmic",
    .probe_type = PROBE_PREFER_ASYNCHRONOUS,
    },
    .probe = max8998_pmic_probe,
    .id_table = max8998_pmic_id,
    };
#[no_mangle]
unsafe extern "C" fn max8998_pmic_init() -> int __init {
    static int __init max8998_pmic_init(void)
    {
    return platform_driver_register(&max8998_pmic_driver);
    }
    subsys_initcall(max8998_pmic_init);
#[no_mangle]
unsafe extern "C" fn max8998_pmic_cleanup() -> void __exit {
    static void __exit max8998_pmic_cleanup(void)
    {
    platform_driver_unregister(&max8998_pmic_driver);
    }
    module_exit(max8998_pmic_cleanup);
    MODULE_DESCRIPTION("MAXIM 8998 voltage regulator driver");
    MODULE_AUTHOR("Kyungmin Park <kyungmin.park@samsung.com>");
    MODULE_LICENSE("GPL");
