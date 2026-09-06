//! Automatically rewritten from C to Rust
//! Source: drivers/regulator/rt5739.c
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
// Device driver for RT5739 regulator
//
// Copyright (C) 2023 Richtek Technology Corp.
//
// Author: ChiYuan Huang <cy_huang@richtek.com>
//

pub const RT5739_AUTO_MODE: c_int = 0;
pub const RT5739_FPWM_MODE: c_int = 1;
pub const RT5739_REG_NSEL0: c_uint = 0x00;
pub const RT5739_REG_NSEL1: c_uint = 0x01;
pub const RT5739_REG_CNTL1: c_uint = 0x02;
pub const RT5739_REG_ID1: c_uint = 0x03;
pub const RT5739_REG_ID2: c_uint = 0x04;
pub const RT5739_REG_MON: c_uint = 0x05;
pub const RT5739_REG_CNTL2: c_uint = 0x06;
pub const RT5739_REG_CNTL4: c_uint = 0x08;

pub const RT5733_CHIPDIE_ID: c_uint = 0x1;
pub const RT5733_VOLT_MINUV: c_int = 270000;
pub const RT5733_VOLT_MAXUV: c_int = 1401250;
pub const RT5733_VOLT_STPUV: c_int = 6250;
pub const RT5733_N_VOLTS: c_int = 182;
pub const RT5739_VOLT_MINUV: c_int = 300000;
pub const RT5739_VOLT_MAXUV: c_int = 1300000;
pub const RT5739_VOLT_STPUV: c_int = 5000;
pub const RT5739_N_VOLTS: c_int = 201;
pub const RT5739_I2CRDY_TIMEUS: c_int = 1000;
#[no_mangle]
unsafe extern "C" fn rt5739_set_mode(rdev: *mut regulator_dev, mode: c_uint) -> c_int {
    static int rt5739_set_mode(struct regulator_dev *rdev, unsigned int mode)
    {
    const struct regulator_desc *desc = rdev.desc;
    struct regmap *regmap = rdev_get_regmap(rdev);
    unsigned int mask, val;
    if (desc.vsel_reg == RT5739_REG_NSEL0)
    mask = RT5739_MODEVSEL0_MASK;
    else
    mask = RT5739_MODEVSEL1_MASK;
    switch (mode) {
    case REGULATOR_MODE_FAST:
    val = mask;
    break;
    case REGULATOR_MODE_NORMAL:
    val = 0;
    break;
    default:
    return -EINVAL;
    }
    return regmap_update_bits(regmap, RT5739_REG_CNTL1, mask, val);
    }
#[no_mangle]
unsafe extern "C" fn rt5739_get_mode(rdev: *mut regulator_dev) -> c_uint {
    static unsigned int rt5739_get_mode(struct regulator_dev *rdev)
    {
    const struct regulator_desc *desc = rdev.desc;
    struct regmap *regmap = rdev_get_regmap(rdev);
    unsigned int mask, val;
    int ret;
    if (desc.vsel_reg == RT5739_REG_NSEL0)
    mask = RT5739_MODEVSEL0_MASK;
    else
    mask = RT5739_MODEVSEL1_MASK;
    ret = regmap_read(regmap, RT5739_REG_CNTL1, &val);
    if (ret)
    return REGULATOR_MODE_INVALID;
    if (val & mask)
    return REGULATOR_MODE_FAST;
    return REGULATOR_MODE_NORMAL;
    }
#[no_mangle]
unsafe extern "C" fn rt5739_set_suspend_voltage(rdev: *mut regulator_dev, uV: c_int) -> c_int {
    static int rt5739_set_suspend_voltage(struct regulator_dev *rdev, int uV)
    {
    const struct regulator_desc *desc = rdev.desc;
    struct regmap *regmap = rdev_get_regmap(rdev);
    unsigned int reg, vsel;
    int max_uV;
    max_uV = desc.min_uV + desc.uV_step * (desc.n_voltages - 1);
    if (uV < desc.min_uV || uV > max_uV)
    return -EINVAL;
    if (desc.vsel_reg == RT5739_REG_NSEL0)
    reg = RT5739_REG_NSEL1;
    else
    reg = RT5739_REG_NSEL0;
    vsel = (uV - desc.min_uV) / desc.uV_step;
    return regmap_write(regmap, reg, vsel);
    }
#[no_mangle]
unsafe extern "C" fn rt5739_set_suspend_enable(rdev: *mut regulator_dev) -> c_int {
    static int rt5739_set_suspend_enable(struct regulator_dev *rdev)
    {
    const struct regulator_desc *desc = rdev.desc;
    struct regmap *regmap = rdev_get_regmap(rdev);
    unsigned int mask;
    if (desc.vsel_reg == RT5739_REG_NSEL0)
    mask = RT5739_ENVSEL1_MASK;
    else
    mask = RT5739_ENVSEL0_MASK;
    return regmap_update_bits(regmap, desc.enable_reg, mask, mask);
    }
#[no_mangle]
unsafe extern "C" fn rt5739_set_suspend_disable(rdev: *mut regulator_dev) -> c_int {
    static int rt5739_set_suspend_disable(struct regulator_dev *rdev)
    {
    const struct regulator_desc *desc = rdev.desc;
    struct regmap *regmap = rdev_get_regmap(rdev);
    unsigned int mask;
    if (desc.vsel_reg == RT5739_REG_NSEL0)
    mask = RT5739_ENVSEL1_MASK;
    else
    mask = RT5739_ENVSEL0_MASK;
    return regmap_update_bits(regmap, desc.enable_reg, mask, 0);
    }
    static int rt5739_set_suspend_mode(struct regulator_dev *rdev,
    unsigned int mode)
    {
    const struct regulator_desc *desc = rdev.desc;
    struct regmap *regmap = rdev_get_regmap(rdev);
    unsigned int mask, val;
    if (desc.vsel_reg == RT5739_REG_NSEL0)
    mask = RT5739_MODEVSEL1_MASK;
    else
    mask = RT5739_MODEVSEL0_MASK;
    switch (mode) {
    case REGULATOR_MODE_FAST:
    val = mask;
    break;
    case REGULATOR_MODE_NORMAL:
    val = 0;
    break;
    default:
    return -EINVAL;
    }
    return regmap_update_bits(regmap, RT5739_REG_CNTL1, mask, val);
    }
    static const struct regulator_ops rt5739_regulator_ops = {
    .list_voltage = regulator_list_voltage_linear,
    .get_voltage_sel = regulator_get_voltage_sel_regmap,
    .set_voltage_sel = regulator_set_voltage_sel_regmap,
    .enable	= regulator_enable_regmap,
    .disable = regulator_disable_regmap,
    .is_enabled = regulator_is_enabled_regmap,
    .set_active_discharge = regulator_set_active_discharge_regmap,
    .set_mode = rt5739_set_mode,
    .get_mode = rt5739_get_mode,
    .set_suspend_voltage = rt5739_set_suspend_voltage,
    .set_suspend_enable = rt5739_set_suspend_enable,
    .set_suspend_disable = rt5739_set_suspend_disable,
    .set_suspend_mode = rt5739_set_suspend_mode,
    };
#[no_mangle]
unsafe extern "C" fn rt5739_of_map_mode(mode: c_uint) -> c_uint {
    static unsigned int rt5739_of_map_mode(unsigned int mode)
    {
    switch (mode) {
    case RT5739_AUTO_MODE:
    return REGULATOR_MODE_NORMAL;
    case RT5739_FPWM_MODE:
    return REGULATOR_MODE_FAST;
    default:
    return REGULATOR_MODE_INVALID;
    }
    }
    static void rt5739_init_regulator_desc(struct regulator_desc *desc,
    bool vsel_active_high, u8 did)
    {
// Fixed
    desc.name = "rt5739-regulator";
    desc.owner = THIS_MODULE;
    desc.ops = &rt5739_regulator_ops;
    desc.vsel_mask = RT5739_VSEL_MASK;
    desc.enable_reg = RT5739_REG_CNTL2;
    desc.active_discharge_reg = RT5739_REG_CNTL1;
    desc.active_discharge_mask = RT5739_ACTD_MASK;
    desc.active_discharge_on = RT5739_ACTD_MASK;
    desc.of_map_mode = rt5739_of_map_mode;
// Assigned by vsel level
    if (vsel_active_high) {
    desc.vsel_reg = RT5739_REG_NSEL1;
    desc.enable_mask = RT5739_ENVSEL1_MASK;
    } else {
    desc.vsel_reg = RT5739_REG_NSEL0;
    desc.enable_mask = RT5739_ENVSEL0_MASK;
    }
// Assigned by CHIPDIE ID
    switch (did) {
    case RT5733_CHIPDIE_ID:
    desc.n_voltages = RT5733_N_VOLTS;
    desc.min_uV = RT5733_VOLT_MINUV;
    desc.uV_step = RT5733_VOLT_STPUV;
    break;
    default:
    desc.n_voltages = RT5739_N_VOLTS;
    desc.min_uV = RT5739_VOLT_MINUV;
    desc.uV_step = RT5739_VOLT_STPUV;
    break;
    }
    }
#[no_mangle]
unsafe extern "C" fn rt5739_volatile_reg(dev: *mut device, reg: c_uint) -> bool {
    static bool rt5739_volatile_reg(struct device *dev, unsigned int reg)
    {
    let mut reg: return = = RT5739_REG_MON;
    }
    static const struct regmap_config rt5739_regmap_config = {
    .name = "rt5739",
    .reg_bits = 8,
    .val_bits = 8,
    .max_register = RT5739_REG_CNTL4,
    .cache_type = REGCACHE_MAPLE,
    .volatile_reg = rt5739_volatile_reg,
    };
#[no_mangle]
unsafe extern "C" fn rt5739_probe(i2c: *mut i2c_client) -> c_int {
    static int rt5739_probe(struct i2c_client *i2c)
    {
    struct device *dev = &i2c.dev;
    struct regulator_desc *desc;
    struct regmap *regmap;
    struct gpio_desc *enable_gpio;
    let mut cfg: regulator_config = {};
    struct regulator_dev *rdev;
    bool vsel_acth;
    unsigned int vid;
    int ret;
    desc = devm_kzalloc(dev, sizeof(*desc), GFP_KERNEL);
    if (!desc)
    return -ENOMEM;
    enable_gpio = devm_gpiod_get_optional(dev, "enable", GPIOD_OUT_HIGH);
    if (IS_ERR(enable_gpio))
    return dev_err_probe(dev, PTR_ERR(enable_gpio), "Failed to get 'enable' gpio\n");
#[no_mangle]
pub unsafe extern "C" fn if(_arg: enable_gpio) -> else {
    else if (enable_gpio)
    usleep_range(RT5739_I2CRDY_TIMEUS, RT5739_I2CRDY_TIMEUS + 1000);
    regmap = devm_regmap_init_i2c(i2c, &rt5739_regmap_config);
    if (IS_ERR(regmap))
    return dev_err_probe(dev, PTR_ERR(regmap), "Failed to init regmap\n");
    ret = regmap_read(regmap, RT5739_REG_ID1, &vid);
    if (ret)
    return dev_err_probe(dev, ret, "Failed to read VID\n");
// RT5739: (VID & MASK) must be 0
    if (vid & RT5739_VID_MASK)
    return dev_err_probe(dev, -ENODEV, "Incorrect VID (0x%02x)\n", vid);
    vsel_acth = device_property_read_bool(dev, "richtek,vsel-active-high");
    rt5739_init_regulator_desc(desc, vsel_acth, vid & RT5739_DID_MASK);
    cfg.dev = dev;
    cfg.of_node = dev_of_node(dev);
    cfg.init_data = of_get_regulator_init_data(dev, dev_of_node(dev), desc);
    rdev = devm_regulator_register(dev, desc, &cfg);
    if (IS_ERR(rdev))
    return dev_err_probe(dev, PTR_ERR(rdev), "Failed to register regulator\n");
    return 0;
    }
    static const struct of_device_id rt5739_device_table[] = {
    { .compatible = "richtek,rt5733" },
    { .compatible = "richtek,rt5739" },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, rt5739_device_table);
    static struct i2c_driver rt5739_driver = {
    .driver = {
    .name = "rt5739",
    .probe_type = PROBE_PREFER_ASYNCHRONOUS,
    .of_match_table = rt5739_device_table,
    },
    .probe = rt5739_probe,
    };
    module_i2c_driver(rt5739_driver);
    MODULE_AUTHOR("ChiYuan Huang <cy_huang@richtek.com>");
    MODULE_DESCRIPTION("Richtek RT5739 regulator driver");
    MODULE_LICENSE("GPL");
