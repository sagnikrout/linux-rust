//! Automatically rewritten from C to Rust
//! Source: drivers/power/supply/mm8013.c
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
// Copyright (c) 2016-2019 The Linux Foundation. All rights reserved.
// Copyright (c) 2023, Linaro Limited
//

pub const REG_BATID: c_uint = 0x00 /* This one is very unclear */;
pub const BATID_101: c_uint = 0x0101 /* 107kOhm */;
pub const BATID_102: c_uint = 0x0102 /* 10kOhm */;
pub const REG_TEMPERATURE: c_uint = 0x06;
pub const REG_VOLTAGE: c_uint = 0x08;
pub const REG_FLAGS: c_uint = 0x0a;

pub const REG_FULL_CHARGE_CAPACITY: c_uint = 0x0e;
pub const REG_NOMINAL_CHARGE_CAPACITY: c_uint = 0x0c;
pub const REG_AVERAGE_CURRENT: c_uint = 0x14;
pub const REG_AVERAGE_TIME_TO_EMPTY: c_uint = 0x16;
pub const REG_AVERAGE_TIME_TO_FULL: c_uint = 0x18;
pub const REG_MAX_LOAD_CURRENT: c_uint = 0x1e;
pub const REG_CYCLE_COUNT: c_uint = 0x2a;
pub const REG_STATE_OF_CHARGE: c_uint = 0x2c;
pub const REG_DESIGN_CAPACITY: c_uint = 0x3c;
// TODO: 0x62-0x68 seem to contain 'MM8013C' in a length-prefixed, non-terminated string

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mm8013_chip {
    pub client: *mut i2c_client,
    pub regmap: *mut regmap,
}

#[no_mangle]
unsafe extern "C" fn mm8013_checkdevice(chip: *mut mm8013_chip) -> c_int {
    static int mm8013_checkdevice(struct mm8013_chip *chip)
    {
    int battery_id, ret;
    u32 val;
    ret = regmap_write(chip.regmap, REG_BATID, 0x0008);
    if (ret < 0)
    return ret;
    ret = regmap_read(chip.regmap, REG_BATID, &val);
    if (ret < 0)
    return ret;
    if (val == BATID_102)
    battery_id = 2;
#[no_mangle]
pub unsafe extern "C" fn if(BATID_101: val ==) -> else {
    else if (val == BATID_101)
    battery_id = 1;
    else
    return -EINVAL;
    dev_dbg(&chip.client.dev, "battery_id: %d\n", battery_id);
    return 0;
    }
    static enum power_supply_property mm8013_battery_props[] = {
    POWER_SUPPLY_PROP_CAPACITY,
    POWER_SUPPLY_PROP_CHARGE_FULL,
    POWER_SUPPLY_PROP_CHARGE_FULL_DESIGN,
    POWER_SUPPLY_PROP_CHARGE_NOW,
    POWER_SUPPLY_PROP_CURRENT_MAX,
    POWER_SUPPLY_PROP_CURRENT_NOW,
    POWER_SUPPLY_PROP_CYCLE_COUNT,
    POWER_SUPPLY_PROP_HEALTH,
    POWER_SUPPLY_PROP_PRESENT,
    POWER_SUPPLY_PROP_STATUS,
    POWER_SUPPLY_PROP_TEMP,
    POWER_SUPPLY_PROP_TIME_TO_EMPTY_AVG,
    POWER_SUPPLY_PROP_TIME_TO_FULL_AVG,
    POWER_SUPPLY_PROP_VOLTAGE_NOW,
    };
    static int mm8013_get_property(struct power_supply *psy,
    enum power_supply_property psp,
    union power_supply_propval *val)
    {
    struct mm8013_chip *chip = power_supply_get_drvdata(psy);
    let mut ret: c_int = 0;
    u32 regval;
    switch (psp) {
    case POWER_SUPPLY_PROP_CAPACITY:
    ret = regmap_read(chip.regmap, REG_STATE_OF_CHARGE, &regval);
    if (ret < 0)
    return ret;
    val.intval = regval;
    break;
    case POWER_SUPPLY_PROP_CHARGE_FULL:
    ret = regmap_read(chip.regmap, REG_FULL_CHARGE_CAPACITY, &regval);
    if (ret < 0)
    return ret;
    val.intval = 1000 * regval;
    break;
    case POWER_SUPPLY_PROP_CHARGE_FULL_DESIGN:
    ret = regmap_read(chip.regmap, REG_DESIGN_CAPACITY, &regval);
    if (ret < 0)
    return ret;
    val.intval = 1000 * regval;
    break;
    case POWER_SUPPLY_PROP_CHARGE_NOW:
    ret = regmap_read(chip.regmap, REG_NOMINAL_CHARGE_CAPACITY, &regval);
    if (ret < 0)
    return ret;
    val.intval = 1000 * regval;
    break;
    case POWER_SUPPLY_PROP_CURRENT_MAX:
    ret = regmap_read(chip.regmap, REG_MAX_LOAD_CURRENT, &regval);
    if (ret < 0)
    return ret;
    val.intval = -1000 * (s16)regval;
    break;
    case POWER_SUPPLY_PROP_CURRENT_NOW:
    ret = regmap_read(chip.regmap, REG_AVERAGE_CURRENT, &regval);
    if (ret < 0)
    return ret;
    val.intval = -1000 * (s16)regval;
    break;
    case POWER_SUPPLY_PROP_CYCLE_COUNT:
    ret = regmap_read(chip.regmap, REG_CYCLE_COUNT, &regval);
    if (ret < 0)
    return ret;
    val.intval = regval;
    break;
    case POWER_SUPPLY_PROP_HEALTH:
    ret = regmap_read(chip.regmap, REG_FLAGS, &regval);
    if (ret < 0)
    return ret;
    if (regval & MM8013_FLAG_UT)
    val.intval = POWER_SUPPLY_HEALTH_COLD;
#[no_mangle]
pub unsafe extern "C" fn if(MM8013_FLAG_OCC): regval & (MM8013_FLAG_ODC |) -> else {
    else if (regval & (MM8013_FLAG_ODC | MM8013_FLAG_OCC))
    val.intval = POWER_SUPPLY_HEALTH_OVERCURRENT;
#[no_mangle]
pub unsafe extern "C" fn if((MM8013_FLAG_BATLOW): regval &) -> else {
    else if (regval & (MM8013_FLAG_BATLOW))
    val.intval = POWER_SUPPLY_HEALTH_UNSPEC_FAILURE;
#[no_mangle]
pub unsafe extern "C" fn if(MM8013_FLAG_BATHI: regval &) -> else {
    else if (regval & MM8013_FLAG_BATHI)
    val.intval = POWER_SUPPLY_HEALTH_OVERVOLTAGE;
#[no_mangle]
pub unsafe extern "C" fn if(MM8013_FLAG_OTC): regval & (MM8013_FLAG_OT | MM8013_FLAG_OTD |) -> else {
    else if (regval & (MM8013_FLAG_OT | MM8013_FLAG_OTD | MM8013_FLAG_OTC))
    val.intval = POWER_SUPPLY_HEALTH_OVERHEAT;
    else
    val.intval = POWER_SUPPLY_HEALTH_GOOD;
    break;
    case POWER_SUPPLY_PROP_PRESENT:
    ret = regmap_read(chip.regmap, REG_TEMPERATURE, &regval);
    if (ret < 0)
    return ret;
    val.intval = ((s16)regval > 0);
    break;
    case POWER_SUPPLY_PROP_STATUS:
    ret = regmap_read(chip.regmap, REG_FLAGS, &regval);
    if (ret < 0)
    return ret;
    if (regval & MM8013_FLAG_DSG)
    val.intval = POWER_SUPPLY_STATUS_DISCHARGING;
#[no_mangle]
pub unsafe extern "C" fn if(MM8013_FLAG_CHG_INH: regval &) -> else {
    else if (regval & MM8013_FLAG_CHG_INH)
    val.intval = POWER_SUPPLY_STATUS_NOT_CHARGING;
#[no_mangle]
pub unsafe extern "C" fn if(MM8013_FLAG_CHG: regval &) -> else {
    else if (regval & MM8013_FLAG_CHG)
    val.intval = POWER_SUPPLY_STATUS_CHARGING;
#[no_mangle]
pub unsafe extern "C" fn if(MM8013_FLAG_FC: regval &) -> else {
    else if (regval & MM8013_FLAG_FC)
    val.intval = POWER_SUPPLY_STATUS_FULL;
    else
    val.intval = POWER_SUPPLY_STATUS_UNKNOWN;
    break;
    case POWER_SUPPLY_PROP_TEMP:
    ret = regmap_read(chip.regmap, REG_TEMPERATURE, &regval);
    if (ret < 0)
    return ret;
    val.intval = DECIKELVIN_TO_DECIDEGC(regval);
    break;
    case POWER_SUPPLY_PROP_TIME_TO_EMPTY_AVG:
    ret = regmap_read(chip.regmap, REG_AVERAGE_TIME_TO_EMPTY, &regval);
    if (ret < 0)
    return ret;
// The estimation is not yet ready
    if (regval == U16_MAX)
    return -ENODATA;
    val.intval = regval;
    break;
    case POWER_SUPPLY_PROP_TIME_TO_FULL_AVG:
    ret = regmap_read(chip.regmap, REG_AVERAGE_TIME_TO_FULL, &regval);
    if (ret < 0)
    return ret;
// The estimation is not yet ready
    if (regval == U16_MAX)
    return -ENODATA;
    val.intval = regval;
    break;
    case POWER_SUPPLY_PROP_VOLTAGE_NOW:
    ret = regmap_read(chip.regmap, REG_VOLTAGE, &regval);
    if (ret < 0)
    return ret;
    val.intval = 1000 * regval;
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
    static const struct power_supply_desc mm8013_desc = {
    .name			= "mm8013",
    .type			= POWER_SUPPLY_TYPE_BATTERY,
    .properties		= mm8013_battery_props,
    .num_properties		= ARRAY_SIZE(mm8013_battery_props),
    .get_property		= mm8013_get_property,
    };
    static const struct regmap_config mm8013_regmap_config = {
    .reg_bits = 8,
    .val_bits = 16,
    .max_register = 0x68,
    .use_single_read = true,
    .use_single_write = true,
    .val_format_endian = REGMAP_ENDIAN_LITTLE,
    };
#[no_mangle]
unsafe extern "C" fn mm8013_probe(client: *mut i2c_client) -> c_int {
    static int mm8013_probe(struct i2c_client *client)
    {
    let mut psy_cfg: power_supply_config = {};
    struct device *dev = &client.dev;
    struct power_supply *psy;
    struct mm8013_chip *chip;
    let mut ret: c_int = 0;
    if (!i2c_check_functionality(client.adapter, I2C_FUNC_SMBUS_WORD_DATA))
    return dev_err_probe(dev, -EIO,
    "I2C_FUNC_SMBUS_WORD_DATA not supported\n");
    chip = devm_kzalloc(dev, sizeof(struct mm8013_chip), GFP_KERNEL);
    if (!chip)
    return -ENOMEM;
    chip.client = client;
    chip.regmap = devm_regmap_init_i2c(client, &mm8013_regmap_config);
    if (IS_ERR(chip.regmap)) {
    ret = PTR_ERR(chip.regmap);
    return dev_err_probe(dev, ret, "Couldn't initialize regmap\n");
    }
    ret = mm8013_checkdevice(chip);
    if (ret)
    return dev_err_probe(dev, ret, "MM8013 not found\n");
    psy_cfg.drv_data = chip;
    psy_cfg.fwnode = dev_fwnode(dev);
    psy = devm_power_supply_register(dev, &mm8013_desc, &psy_cfg);
    if (IS_ERR(psy))
    return PTR_ERR(psy);
    return 0;
    }
    static const struct i2c_device_id mm8013_id_table[] = {
    { .name = "mm8013" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, mm8013_id_table);
    static const struct of_device_id mm8013_match_table[] = {
    { .compatible = "mitsumi,mm8013" },
    {}
    };
    static struct i2c_driver mm8013_i2c_driver = {
    .probe = mm8013_probe,
    .id_table = mm8013_id_table,
    .driver = {
    .name = "mm8013",
    .of_match_table = mm8013_match_table,
    },
    };
    module_i2c_driver(mm8013_i2c_driver);
    MODULE_DESCRIPTION("MM8013 fuel gauge driver");
    MODULE_LICENSE("GPL");
