//! Automatically rewritten from C to Rust
//! Source: drivers/regulator/rtq6752-regulator.c
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

    enum {
    RTQ6752_IDX_PAVDD = 0,
    RTQ6752_IDX_NAVDD = 1,
    RTQ6752_IDX_MAX
    };
pub const RTQ6752_REG_PAVDD: c_uint = 0x00;
pub const RTQ6752_REG_NAVDD: c_uint = 0x01;
pub const RTQ6752_REG_PAVDDONDLY: c_uint = 0x07;
pub const RTQ6752_REG_PAVDDSSTIME: c_uint = 0x08;
pub const RTQ6752_REG_NAVDDONDLY: c_uint = 0x0D;
pub const RTQ6752_REG_NAVDDSSTIME: c_uint = 0x0E;
pub const RTQ6752_REG_OPTION1: c_uint = 0x12;
pub const RTQ6752_REG_CHSWITCH: c_uint = 0x16;
pub const RTQ6752_REG_FAULT: c_uint = 0x1D;

pub const RTQ6752_VOUT_MINUV: c_int = 5000000;
pub const RTQ6752_VOUT_STEPUV: c_int = 50000;
pub const RTQ6752_VOUT_NUM: c_int = 47;
pub const RTQ6752_I2CRDY_TIMEUS: c_int = 1000;
pub const RTQ6752_MINSS_TIMEUS: c_int = 5000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtq6752_priv {
    pub regmap: *mut regmap,
    pub enable_gpio: *mut gpio_desc,
    pub lock: mutex,
    pub enable_flag: c_uchar,
}

#[no_mangle]
unsafe extern "C" fn rtq6752_set_vdd_enable(rdev: *mut regulator_dev) -> c_int {
    static int rtq6752_set_vdd_enable(struct regulator_dev *rdev)
    {
    struct rtq6752_priv *priv = rdev_get_drvdata(rdev);
    let mut rid: c_int = rdev_get_id(rdev), ret;
    mutex_lock(&priv.lock);
    if (!priv.enable_flag) {
    if (priv.enable_gpio) {
    gpiod_set_value(priv.enable_gpio, 1);
    usleep_range(RTQ6752_I2CRDY_TIMEUS,
    RTQ6752_I2CRDY_TIMEUS + 100);
    }
    regcache_cache_only(priv.regmap, false);
    ret = regcache_sync(priv.regmap);
    if (ret) {
    mutex_unlock(&priv.lock);
    return ret;
    }
    }
    priv.enable_flag |= BIT(rid);
    mutex_unlock(&priv.lock);
    return regulator_enable_regmap(rdev);
    }
#[no_mangle]
unsafe extern "C" fn rtq6752_set_vdd_disable(rdev: *mut regulator_dev) -> c_int {
    static int rtq6752_set_vdd_disable(struct regulator_dev *rdev)
    {
    struct rtq6752_priv *priv = rdev_get_drvdata(rdev);
    let mut rid: c_int = rdev_get_id(rdev), ret;
    ret = regulator_disable_regmap(rdev);
    if (ret)
    return ret;
    mutex_lock(&priv.lock);
    priv.enable_flag &= ~BIT(rid);
    if (!priv.enable_flag) {
    regcache_cache_only(priv.regmap, true);
    regcache_mark_dirty(priv.regmap);
    if (priv.enable_gpio)
    gpiod_set_value(priv.enable_gpio, 0);
    }
    mutex_unlock(&priv.lock);
    return 0;
    }
    static int rtq6752_get_error_flags(struct regulator_dev *rdev,
    unsigned int *flags)
    {
    unsigned int val, events = 0;
    static const unsigned int fault_mask[] = {
    RTQ6752_PAVDDF_MASK, RTQ6752_NAVDDF_MASK };
    let mut rid: c_int = rdev_get_id(rdev), ret;
    ret = regmap_read(rdev.regmap, RTQ6752_REG_FAULT, &val);
    if (ret)
    return ret;
    if (val & fault_mask[rid])
    events = REGULATOR_ERROR_REGULATION_OUT;
// flags = events;
    return 0;
    }
    static const struct regulator_ops rtq6752_regulator_ops = {
    .list_voltage = regulator_list_voltage_linear,
    .set_voltage_sel = regulator_set_voltage_sel_regmap,
    .get_voltage_sel = regulator_get_voltage_sel_regmap,
    .enable = rtq6752_set_vdd_enable,
    .disable = rtq6752_set_vdd_disable,
    .is_enabled = regulator_is_enabled_regmap,
    .set_active_discharge = regulator_set_active_discharge_regmap,
    .get_error_flags = rtq6752_get_error_flags,
    };
    static const struct regulator_desc rtq6752_regulator_descs[] = {
    {
    .name = "rtq6752-pavdd",
    .of_match = of_match_ptr("pavdd"),
    .regulators_node = of_match_ptr("regulators"),
    .id = RTQ6752_IDX_PAVDD,
    .n_voltages = RTQ6752_VOUT_NUM,
    .ops = &rtq6752_regulator_ops,
    .owner = THIS_MODULE,
    .min_uV = RTQ6752_VOUT_MINUV,
    .uV_step = RTQ6752_VOUT_STEPUV,
    .enable_time = RTQ6752_MINSS_TIMEUS,
    .vsel_reg = RTQ6752_REG_PAVDD,
    .vsel_mask = RTQ6752_VOUT_MASK,
    .enable_reg = RTQ6752_REG_CHSWITCH,
    .enable_mask = RTQ6752_PAVDDEN_MASK,
    .active_discharge_reg = RTQ6752_REG_OPTION1,
    .active_discharge_mask = RTQ6752_PAVDDAD_MASK,
    .active_discharge_off = RTQ6752_PAVDDAD_MASK,
    },
    {
    .name = "rtq6752-navdd",
    .of_match = of_match_ptr("navdd"),
    .regulators_node = of_match_ptr("regulators"),
    .id = RTQ6752_IDX_NAVDD,
    .n_voltages = RTQ6752_VOUT_NUM,
    .ops = &rtq6752_regulator_ops,
    .owner = THIS_MODULE,
    .min_uV = RTQ6752_VOUT_MINUV,
    .uV_step = RTQ6752_VOUT_STEPUV,
    .enable_time = RTQ6752_MINSS_TIMEUS,
    .vsel_reg = RTQ6752_REG_NAVDD,
    .vsel_mask = RTQ6752_VOUT_MASK,
    .enable_reg = RTQ6752_REG_CHSWITCH,
    .enable_mask = RTQ6752_NAVDDEN_MASK,
    .active_discharge_reg = RTQ6752_REG_OPTION1,
    .active_discharge_mask = RTQ6752_NAVDDAD_MASK,
    .active_discharge_off = RTQ6752_NAVDDAD_MASK,
    }
    };
#[no_mangle]
unsafe extern "C" fn rtq6752_init_device_properties(priv: *mut rtq6752_priv) -> c_int {
    static int rtq6752_init_device_properties(struct rtq6752_priv *priv)
    {
    u8 raw_vals[] = { 0, 0 };
    int ret;
// Configure PAVDD on and softstart delay time to the minimum
    ret = regmap_raw_write(priv.regmap, RTQ6752_REG_PAVDDONDLY, raw_vals,
    ARRAY_SIZE(raw_vals));
    if (ret)
    return ret;
// Configure NAVDD on and softstart delay time to the minimum
    return regmap_raw_write(priv.regmap, RTQ6752_REG_NAVDDONDLY, raw_vals,
    ARRAY_SIZE(raw_vals));
    }
#[no_mangle]
unsafe extern "C" fn rtq6752_is_volatile_reg(dev: *mut device, reg: c_uint) -> bool {
    static bool rtq6752_is_volatile_reg(struct device *dev, unsigned int reg)
    {
    if (reg == RTQ6752_REG_FAULT)
    return true;
    return false;
    }
    static const struct reg_default rtq6752_reg_defaults[] = {
    { RTQ6752_REG_PAVDD, 0x14 },
    { RTQ6752_REG_NAVDD, 0x14 },
    { RTQ6752_REG_PAVDDONDLY, 0x01 },
    { RTQ6752_REG_PAVDDSSTIME, 0x01 },
    { RTQ6752_REG_NAVDDONDLY, 0x01 },
    { RTQ6752_REG_NAVDDSSTIME, 0x01 },
    { RTQ6752_REG_OPTION1, 0x07 },
    { RTQ6752_REG_CHSWITCH, 0x29 },
    };
    static const struct regmap_config rtq6752_regmap_config = {
    .reg_bits = 8,
    .val_bits = 8,
    .cache_type = REGCACHE_MAPLE,
    .max_register = RTQ6752_REG_FAULT,
    .reg_defaults = rtq6752_reg_defaults,
    .num_reg_defaults = ARRAY_SIZE(rtq6752_reg_defaults),
    .volatile_reg = rtq6752_is_volatile_reg,
    };
#[no_mangle]
unsafe extern "C" fn rtq6752_probe(i2c: *mut i2c_client) -> c_int {
    static int rtq6752_probe(struct i2c_client *i2c)
    {
    struct rtq6752_priv *priv;
    let mut reg_cfg: regulator_config = {};
    struct regulator_dev *rdev;
    int i, ret;
    priv = devm_kzalloc(&i2c.dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    mutex_init(&priv.lock);
    priv.enable_gpio = devm_gpiod_get_optional(&i2c.dev, "enable",
    GPIOD_OUT_HIGH);
    if (IS_ERR(priv.enable_gpio)) {
    dev_err(&i2c.dev, "Failed to get 'enable' gpio\n");
    return PTR_ERR(priv.enable_gpio);
    }
    usleep_range(RTQ6752_I2CRDY_TIMEUS, RTQ6752_I2CRDY_TIMEUS + 100);
// Default EN pin to high, PAVDD and NAVDD will be on
    priv.enable_flag = RTQ6752_ENABLE_MASK;
    priv.regmap = devm_regmap_init_i2c(i2c, &rtq6752_regmap_config);
    if (IS_ERR(priv.regmap)) {
    dev_err(&i2c.dev, "Failed to init regmap\n");
    return PTR_ERR(priv.regmap);
    }
    ret = rtq6752_init_device_properties(priv);
    if (ret) {
    dev_err(&i2c.dev, "Failed to init device properties\n");
    return ret;
    }
    reg_cfg.dev = &i2c.dev;
    reg_cfg.regmap = priv.regmap;
    reg_cfg.driver_data = priv;
    for (i = 0; i < ARRAY_SIZE(rtq6752_regulator_descs); i++) {
    rdev = devm_regulator_register(&i2c.dev,
    rtq6752_regulator_descs + i,
    &reg_cfg);
    if (IS_ERR(rdev)) {
    dev_err(&i2c.dev, "Failed to init %d regulator\n", i);
    return PTR_ERR(rdev);
    }
    }
    return 0;
    }
    static const struct of_device_id __maybe_unused rtq6752_device_table[] = {
    { .compatible = "richtek,rtq6752", },
    {}
    };
    MODULE_DEVICE_TABLE(of, rtq6752_device_table);
    static struct i2c_driver rtq6752_driver = {
    .driver = {
    .name = "rtq6752",
    .probe_type = PROBE_PREFER_ASYNCHRONOUS,
    .of_match_table = rtq6752_device_table,
    },
    .probe = rtq6752_probe,
    };
    module_i2c_driver(rtq6752_driver);
    MODULE_AUTHOR("ChiYuan Huang <cy_huang@richtek.com>");
    MODULE_DESCRIPTION("Richtek RTQ6752 Regulator Driver");
    MODULE_LICENSE("GPL v2");
