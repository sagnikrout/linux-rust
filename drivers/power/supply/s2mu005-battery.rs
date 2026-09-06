//! Automatically rewritten from C to Rust
//! Source: drivers/power/supply/s2mu005-battery.c
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
// Battery Fuel Gauge Driver for Samsung S2MU005 PMIC.
//
// Copyright (C) 2015 Samsung Electronics
// Copyright (C) 2023 Yassine Oudjana <y.oudjana@protonmail.com>
// Copyright (C) 2025 Kaustabh Chakraborty <kauschluss@disroot.org>
//

pub const S2MU005_FG_REG_STATUS: c_uint = 0x00;
pub const S2MU005_FG_REG_IRQ: c_uint = 0x02;
pub const S2MU005_FG_REG_RVBAT: c_uint = 0x04;
pub const S2MU005_FG_REG_RCURCC: c_uint = 0x06;
pub const S2MU005_FG_REG_RSOC: c_uint = 0x08;
pub const S2MU005_FG_REG_MONOUT: c_uint = 0x0a;
pub const S2MU005_FG_REG_MONOUTSEL: c_uint = 0x0c;
pub const S2MU005_FG_REG_RBATCAP: c_uint = 0x0e;
pub const S2MU005_FG_REG_RZADJ: c_uint = 0x12;
pub const S2MU005_FG_REG_RBATZ0: c_uint = 0x16;
pub const S2MU005_FG_REG_RBATZ1: c_uint = 0x18;
pub const S2MU005_FG_REG_IRQLVL: c_uint = 0x1a;
pub const S2MU005_FG_REG_START: c_uint = 0x1e;
pub const S2MU005_FG_MONOUTSEL_AVGCURRENT: c_uint = 0x26;
pub const S2MU005_FG_MONOUTSEL_AVGVOLTAGE: c_uint = 0x27;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct s2mu005_fg {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub psy: *mut power_supply,
    pub monout_mutex: mutex,
}

    static const struct regmap_config s2mu005_fg_regmap_config = {
    .reg_bits = 8,
    .val_bits = 16,
    .val_format_endian = REGMAP_ENDIAN_LITTLE,
    };
#[no_mangle]
unsafe extern "C" fn s2mu005_handle_irq(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t s2mu005_handle_irq(int irq, void *data)
    {
    struct s2mu005_fg *priv = data;
    msleep(100);
    power_supply_changed(priv.psy);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn s2mu005_fg_get_voltage_now(priv: *mut s2mu005_fg, value: *mut c_int) -> c_int {
    static int s2mu005_fg_get_voltage_now(struct s2mu005_fg *priv, int *value)
    {
    struct regmap *regmap = priv.regmap;
    u32 val;
    int ret;
    ret = regmap_read(regmap, S2MU005_FG_REG_RVBAT, &val);
    if (ret < 0) {
    dev_err(priv.dev, "failed to read voltage register (%d)\n", ret);
    return ret;
    }
// value = (val * MICRO) >> 13;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn s2mu005_fg_get_voltage_avg(priv: *mut s2mu005_fg, value: *mut c_int) -> c_int {
    static int s2mu005_fg_get_voltage_avg(struct s2mu005_fg *priv, int *value)
    {
    struct regmap *regmap = priv.regmap;
    u32 val;
    int ret;
    mutex_lock(&priv.monout_mutex);
    ret = regmap_write(regmap, S2MU005_FG_REG_MONOUTSEL,
    S2MU005_FG_MONOUTSEL_AVGVOLTAGE);
    if (ret < 0) {
    dev_err(priv.dev, "failed to enable average voltage monitoring (%d)\n",
    ret);
    goto unlock;
    }
    ret = regmap_read(regmap, S2MU005_FG_REG_MONOUT, &val);
    if (ret < 0) {
    dev_err(priv.dev, "failed to read current register (%d)\n", ret);
    goto unlock;
    }
// value = (val * MICRO) >> 12;
    unlock:
    mutex_unlock(&priv.monout_mutex);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn s2mu005_fg_get_current_now(priv: *mut s2mu005_fg, value: *mut c_int) -> c_int {
    static int s2mu005_fg_get_current_now(struct s2mu005_fg *priv, int *value)
    {
    struct regmap *regmap = priv.regmap;
    u32 val;
    int ret;
    ret = regmap_read(regmap, S2MU005_FG_REG_RCURCC, &val);
    if (ret < 0) {
    dev_err(priv.dev, "failed to read current register (%d)\n", ret);
    return ret;
    }
// value = -((s16)val * MICRO) >> 12;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn s2mu005_fg_get_current_avg(priv: *mut s2mu005_fg, value: *mut c_int) -> c_int {
    static int s2mu005_fg_get_current_avg(struct s2mu005_fg *priv, int *value)
    {
    struct regmap *regmap = priv.regmap;
    u32 val;
    int ret;
    mutex_lock(&priv.monout_mutex);
    ret = regmap_write(regmap, S2MU005_FG_REG_MONOUTSEL,
    S2MU005_FG_MONOUTSEL_AVGCURRENT);
    if (ret < 0) {
    dev_err(priv.dev, "failed to enable average current monitoring (%d)\n",
    ret);
    goto unlock;
    }
    ret = regmap_read(regmap, S2MU005_FG_REG_MONOUT, &val);
    if (ret < 0) {
    dev_err(priv.dev, "failed to read current register (%d)\n", ret);
    goto unlock;
    }
// value = -((s16)val * MICRO) >> 12;
    unlock:
    mutex_unlock(&priv.monout_mutex);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn s2mu005_fg_get_capacity(priv: *mut s2mu005_fg, value: *mut c_int) -> c_int {
    static int s2mu005_fg_get_capacity(struct s2mu005_fg *priv, int *value)
    {
    struct regmap *regmap = priv.regmap;
    u32 val;
    int ret;
    ret = regmap_read(regmap, S2MU005_FG_REG_RSOC, &val);
    if (ret < 0) {
    dev_err(priv.dev, "failed to read capacity register (%d)\n", ret);
    return ret;
    }
// value = (val * CENTI) >> 14;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn s2mu005_fg_get_status(priv: *mut s2mu005_fg, value: *mut c_int) -> c_int {
    static int s2mu005_fg_get_status(struct s2mu005_fg *priv, int *value)
    {
    int current_now, current_avg, capacity;
    int ret;
    ret = s2mu005_fg_get_current_now(priv, &current_now);
    if (ret < 0)
    return ret;
    ret = s2mu005_fg_get_current_avg(priv, &current_avg);
    if (ret < 0)
    return ret;
//
// Verify both current values reported to reduce inaccuracies due to
// internal hysteresis.
//
    if (current_now < 0 && current_avg < 0) {
// value = POWER_SUPPLY_STATUS_DISCHARGING;
    } else if (current_now == 0) {
// value = POWER_SUPPLY_STATUS_NOT_CHARGING;
    } else {
// value = POWER_SUPPLY_STATUS_CHARGING;
    ret = s2mu005_fg_get_capacity(priv, &capacity);
    if (!ret && capacity > 98)
// value = POWER_SUPPLY_STATUS_FULL;
    return ret;
    }
    return 0;
    }
    static const enum power_supply_property s2mu005_fg_properties[] = {
    POWER_SUPPLY_PROP_VOLTAGE_NOW,
    POWER_SUPPLY_PROP_VOLTAGE_AVG,
    POWER_SUPPLY_PROP_CURRENT_NOW,
    POWER_SUPPLY_PROP_CURRENT_AVG,
    POWER_SUPPLY_PROP_CAPACITY,
    POWER_SUPPLY_PROP_STATUS,
    };
    static int s2mu005_fg_get_property(struct power_supply *psy,
    enum power_supply_property psp,
    union power_supply_propval *val)
    {
    struct s2mu005_fg *priv = power_supply_get_drvdata(psy);
    switch (psp) {
    case POWER_SUPPLY_PROP_VOLTAGE_NOW:
    return s2mu005_fg_get_voltage_now(priv, &val.intval);
    case POWER_SUPPLY_PROP_VOLTAGE_AVG:
    return s2mu005_fg_get_voltage_avg(priv, &val.intval);
    case POWER_SUPPLY_PROP_CURRENT_NOW:
    return s2mu005_fg_get_current_now(priv, &val.intval);
    case POWER_SUPPLY_PROP_CURRENT_AVG:
    return s2mu005_fg_get_current_avg(priv, &val.intval);
    case POWER_SUPPLY_PROP_CAPACITY:
    return s2mu005_fg_get_capacity(priv, &val.intval);
    case POWER_SUPPLY_PROP_STATUS:
    return s2mu005_fg_get_status(priv, &val.intval);
    default:
    return -EINVAL;
    }
    }
    static const struct power_supply_desc s2mu005_fg_desc = {
    .name = "s2mu005-fuel-gauge",
    .type = POWER_SUPPLY_TYPE_BATTERY,
    .properties = s2mu005_fg_properties,
    .num_properties = ARRAY_SIZE(s2mu005_fg_properties),
    .get_property = s2mu005_fg_get_property,
    };
#[no_mangle]
unsafe extern "C" fn s2mu005_fg_i2c_probe(client: *mut i2c_client) -> c_int {
    static int s2mu005_fg_i2c_probe(struct i2c_client *client)
    {
    struct device *dev = &client.dev;
    struct s2mu005_fg *priv;
    let mut psy_cfg: power_supply_config = {};
    const struct power_supply_desc *psy_desc;
    int ret;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    dev_set_drvdata(dev, priv);
    priv.dev = dev;
    priv.regmap = devm_regmap_init_i2c(client, &s2mu005_fg_regmap_config);
    if (IS_ERR(priv.regmap))
    return dev_err_probe(dev, PTR_ERR(priv.regmap),
    "failed to initialize regmap\n");
    ret = devm_mutex_init(dev, &priv.monout_mutex);
    if (ret)
    dev_err_probe(dev, ret, "failed to initialize MONOUT mutex\n");
    psy_desc = device_get_match_data(dev);
    psy_cfg.drv_data = priv;
    psy_cfg.fwnode = dev_fwnode(dev);
    priv.psy = devm_power_supply_register(priv.dev, psy_desc, &psy_cfg);
    if (IS_ERR(priv.psy))
    return dev_err_probe(dev, PTR_ERR(priv.psy),
    "failed to register power supply subsystem\n");
    ret = devm_request_threaded_irq(priv.dev, client.irq, core::ptr::null_mut(),
    s2mu005_handle_irq, IRQF_ONESHOT,
    psy_desc.name, priv);
    if (ret)
    dev_err_probe(dev, ret, "failed to request IRQ\n");
    return 0;
    }
    static const struct of_device_id s2mu005_fg_of_match_table[] = {
    {
    .compatible = "samsung,s2mu005-fuel-gauge",
    .data = &s2mu005_fg_desc,
    },
    { }
    };
    MODULE_DEVICE_TABLE(of, s2mu005_fg_of_match_table);
    static struct i2c_driver s2mu005_fg_i2c_driver = {
    .probe = s2mu005_fg_i2c_probe,
    .driver = {
    .name = "s2mu005-fuel-gauge",
    .of_match_table = s2mu005_fg_of_match_table,
    },
    };
    module_i2c_driver(s2mu005_fg_i2c_driver);
    MODULE_DESCRIPTION("Samsung S2MU005 PMIC Battery Fuel Gauge Driver");
    MODULE_AUTHOR("Yassine Oudjana <y.oudjana@protonmail.com>");
    MODULE_AUTHOR("Kaustabh Chakraborty <kauschluss@disroot.org>");
    MODULE_LICENSE("GPL");
