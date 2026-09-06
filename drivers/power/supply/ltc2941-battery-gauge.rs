//! Automatically rewritten from C to Rust
//! Source: drivers/power/supply/ltc2941-battery-gauge.c
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
// I2C client/driver for the Linear Technology LTC2941, LTC2942, LTC2943
// and LTC2944 Battery Gas Gauge IC
//
// Copyright (C) 2014 Topic Embedded Systems
//
// Author: Auryn Verwegen
// Author: Mike Looijmans
//

pub const LTC294X_MAX_VALUE: c_uint = 0xFFFF;
pub const LTC294X_MID_SUPPLY: c_uint = 0x7FFF;
pub const LTC2941_MAX_PRESCALER_EXP: c_int = 7;
pub const LTC2943_MAX_PRESCALER_EXP: c_int = 6;
    enum ltc294x_reg {
    LTC294X_REG_STATUS		= 0x00,
    LTC294X_REG_CONTROL		= 0x01,
    LTC294X_REG_ACC_CHARGE_MSB	= 0x02,
    LTC294X_REG_ACC_CHARGE_LSB	= 0x03,
    LTC294X_REG_CHARGE_THR_HIGH_MSB	= 0x04,
    LTC294X_REG_CHARGE_THR_HIGH_LSB	= 0x05,
    LTC294X_REG_CHARGE_THR_LOW_MSB	= 0x06,
    LTC294X_REG_CHARGE_THR_LOW_LSB	= 0x07,
    LTC294X_REG_VOLTAGE_MSB		= 0x08,
    LTC294X_REG_VOLTAGE_LSB		= 0x09,
    LTC2942_REG_TEMPERATURE_MSB	= 0x0C,
    LTC2942_REG_TEMPERATURE_LSB	= 0x0D,
    LTC2943_REG_CURRENT_MSB		= 0x0E,
    LTC2943_REG_CURRENT_LSB		= 0x0F,
    LTC2943_REG_TEMPERATURE_MSB	= 0x14,
    LTC2943_REG_TEMPERATURE_LSB	= 0x15,
    };
    enum ltc294x_id {
    LTC2941_ID,
    LTC2942_ID,
    LTC2943_ID,
    LTC2944_ID,
    };

    ((x << 3) & LTC294X_REG_CONTROL_PRESCALER_MASK)
pub const LTC294X_REG_CONTROL_ALCC_CONFIG_DISABLED: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ltc294x_info {
    pub /: *mut *mut *mut i2c_client client; / I2C Client pointer,
    pub /: *mut *mut *mut power_supply supply; / Supply pointer,
    pub /: *mut *mut power_supply_desc supply_desc; / Supply description,
    pub /: *mut *mut delayed_work work; / Work scheduler,
    pub /: *mut *mut enum ltc294x_id id; / Chip type,
    pub /: *mut *mut int charge; / Last charge register content,
    pub /: *mut *mut int r_sense; / mOhm,
    pub /: *mut *mut int Qlsb; / nAh,
}

    static inline int convert_bin_to_uAh(
    const struct ltc294x_info *info, int Q)
    {
    return ((Q * (info.Qlsb / 10))) / 100;
    }
    static inline int convert_uAh_to_bin(
    const struct ltc294x_info *info, int uAh)
    {
    int Q;
    Q = (uAh * 100) / (info.Qlsb/10);
    return (Q < LTC294X_MAX_VALUE) ? Q : LTC294X_MAX_VALUE;
    }
    static int ltc294x_read_regs(struct i2c_client *client,
    enum ltc294x_reg reg, u8 *buf, int num_regs)
    {
    int ret;
    struct i2c_msg msgs[2] = { };
    let mut reg_start: u8 = reg;
    msgs[0].addr	= client.addr;
    msgs[0].len	= 1;
    msgs[0].buf	= &reg_start;
    msgs[1].addr	= client.addr;
    msgs[1].len	= num_regs;
    msgs[1].buf	= buf;
    msgs[1].flags	= I2C_M_RD;
    ret = i2c_transfer(client.adapter, &msgs[0], 2);
    if (ret < 0) {
    dev_err(&client.dev, "ltc2941 read_reg(0x%x[%d]) failed: %pe\n",
    reg, num_regs, ERR_PTR(ret));
    return ret;
    }
    dev_dbg(&client.dev, "%s (%#x, %d) . %#x\n",
    __func__, reg, num_regs, *buf);
    return 0;
    }
    static int ltc294x_write_regs(struct i2c_client *client,
    enum ltc294x_reg reg, const u8 *buf, int num_regs)
    {
    int ret;
    let mut reg_start: u8 = reg;
    ret = i2c_smbus_write_i2c_block_data(client, reg_start, num_regs, buf);
    if (ret < 0) {
    dev_err(&client.dev, "ltc2941 write_reg(0x%x[%d]) failed: %pe\n",
    reg, num_regs, ERR_PTR(ret));
    return ret;
    }
    dev_dbg(&client.dev, "%s (%#x, %d) . %#x\n",
    __func__, reg, num_regs, *buf);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ltc294x_reset(info: *const ltc294x_info, prescaler_exp: c_int) -> c_int {
    static int ltc294x_reset(const struct ltc294x_info *info, int prescaler_exp)
    {
    int ret;
    u8 value;
    u8 control;
// Read status and control registers
    ret = ltc294x_read_regs(info.client, LTC294X_REG_CONTROL, &value, 1);
    if (ret < 0)
    return ret;
    control = LTC294X_REG_CONTROL_PRESCALER_SET(prescaler_exp) |
    LTC294X_REG_CONTROL_ALCC_CONFIG_DISABLED;
// Put device into "monitor" mode
    switch (info.id) {
    case LTC2942_ID:	/* 2942 measures every 2 sec */
    control |= LTC2942_REG_CONTROL_MODE_SCAN;
    break;
    case LTC2943_ID:
    case LTC2944_ID:	/* 2943 and 2944 measure every 10 sec */
    control |= LTC2943_REG_CONTROL_MODE_SCAN;
    break;
    default:
    break;
    }
    if (value != control) {
    ret = ltc294x_write_regs(info.client,
    LTC294X_REG_CONTROL, &control, 1);
    if (ret < 0)
    return ret;
    }
    return 0;
    }
    static int ltc294x_read_charge_register(const struct ltc294x_info *info,
    enum ltc294x_reg reg)
    {
    int ret;
    u8 datar[2];
    ret = ltc294x_read_regs(info.client, reg, &datar[0], 2);
    if (ret < 0)
    return ret;
    return (datar[0] << 8) + datar[1];
    }
    static int ltc294x_get_charge(const struct ltc294x_info *info,
    enum ltc294x_reg reg, int *val)
    {
    let mut value: c_int = ltc294x_read_charge_register(info, reg);
    if (value < 0)
    return value;
// When r_sense < 0, this counts up when the battery discharges
    if (info.Qlsb < 0)
    value -= 0xFFFF;
// val = convert_bin_to_uAh(info, value);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ltc294x_set_charge_now(info: *const ltc294x_info, val: c_int) -> c_int {
    static int ltc294x_set_charge_now(const struct ltc294x_info *info, int val)
    {
    int ret;
    u8 dataw[2];
    u8 ctrl_reg;
    s32 value;
    value = convert_uAh_to_bin(info, val);
// Direction depends on how sense+/- were connected
    if (info.Qlsb < 0)
    value += 0xFFFF;
    if ((value < 0) || (value > 0xFFFF)) /* input validation */
    return -EINVAL;
// Read control register
    ret = ltc294x_read_regs(info.client,
    LTC294X_REG_CONTROL, &ctrl_reg, 1);
    if (ret < 0)
    return ret;
// Disable analog section
    ctrl_reg |= LTC294X_REG_CONTROL_SHUTDOWN_MASK;
    ret = ltc294x_write_regs(info.client,
    LTC294X_REG_CONTROL, &ctrl_reg, 1);
    if (ret < 0)
    return ret;
// Set new charge value
    dataw[0] = I16_MSB(value);
    dataw[1] = I16_LSB(value);
    ret = ltc294x_write_regs(info.client,
    LTC294X_REG_ACC_CHARGE_MSB, &dataw[0], 2);
    if (ret < 0)
    goto error_exit;
// Enable analog section
    error_exit:
    ctrl_reg &= ~LTC294X_REG_CONTROL_SHUTDOWN_MASK;
    ret = ltc294x_write_regs(info.client,
    LTC294X_REG_CONTROL, &ctrl_reg, 1);
    return ret < 0 ? ret : 0;
    }
    static int ltc294x_set_charge_thr(const struct ltc294x_info *info,
    enum ltc294x_reg reg, int val)
    {
    u8 dataw[2];
    s32 value;
    value = convert_uAh_to_bin(info, val);
// Direction depends on how sense+/- were connected
    if (info.Qlsb < 0)
    value += 0xFFFF;
    if ((value < 0) || (value > 0xFFFF)) /* input validation */
    return -EINVAL;
// Set new charge value
    dataw[0] = I16_MSB(value);
    dataw[1] = I16_LSB(value);
    return ltc294x_write_regs(info.client, reg, &dataw[0], 2);
    }
    static int ltc294x_get_charge_counter(
    const struct ltc294x_info *info, int *val)
    {
    let mut value: c_int = ltc294x_read_charge_register(info, LTC294X_REG_ACC_CHARGE_MSB);
    if (value < 0)
    return value;
    value -= LTC294X_MID_SUPPLY;
// val = convert_bin_to_uAh(info, value);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ltc294x_get_voltage(info: *const ltc294x_info, val: *mut c_int) -> c_int {
    static int ltc294x_get_voltage(const struct ltc294x_info *info, int *val)
    {
    int ret;
    u8 datar[2];
    u32 value;
    ret = ltc294x_read_regs(info.client,
    LTC294X_REG_VOLTAGE_MSB, &datar[0], 2);
    value = (datar[0] << 8) | datar[1];
    switch (info.id) {
    case LTC2943_ID:
    value *= 23600 * 2;
    value /= 0xFFFF;
    value *= 1000 / 2;
    break;
    case LTC2944_ID:
    value *= 70800 / 5*4;
    value /= 0xFFFF;
    value *= 1000 * 5/4;
    break;
    default:
    value *= 6000 * 10;
    value /= 0xFFFF;
    value *= 1000 / 10;
    break;
    }
// val = value;
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ltc294x_get_current(info: *const ltc294x_info, val: *mut c_int) -> c_int {
    static int ltc294x_get_current(const struct ltc294x_info *info, int *val)
    {
    int ret;
    u8 datar[2];
    s32 value;
    ret = ltc294x_read_regs(info.client,
    LTC2943_REG_CURRENT_MSB, &datar[0], 2);
    value = (datar[0] << 8) | datar[1];
    value -= 0x7FFF;
    if (info.id == LTC2944_ID)
    value *= 64000;
    else
    value *= 60000;
// Value is in range -32k..+32k, r_sense is usually 10..50 mOhm,
// the formula below keeps everything in s32 range while preserving
// enough digits
// val = 1000 * (value / (info->r_sense * 0x7FFF)); /* in uA
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ltc294x_get_temperature(info: *const ltc294x_info, val: *mut c_int) -> c_int {
    static int ltc294x_get_temperature(const struct ltc294x_info *info, int *val)
    {
    enum ltc294x_reg reg;
    int ret;
    u8 datar[2];
    u32 value;
    if (info.id == LTC2942_ID) {
    reg = LTC2942_REG_TEMPERATURE_MSB;
    value = 6000;	/* Full-scale is 600 Kelvin */
    } else {
    reg = LTC2943_REG_TEMPERATURE_MSB;
    value = 5100;	/* Full-scale is 510 Kelvin */
    }
    ret = ltc294x_read_regs(info.client, reg, &datar[0], 2);
    value *= (datar[0] << 8) | datar[1];
// Convert to tenths of degree Celsius
// val = value / 0xFFFF - 2722;
    return ret;
    }
    static int ltc294x_get_property(struct power_supply *psy,
    enum power_supply_property prop,
    union power_supply_propval *val)
    {
    struct ltc294x_info *info = power_supply_get_drvdata(psy);
    switch (prop) {
    case POWER_SUPPLY_PROP_CHARGE_FULL:
    return ltc294x_get_charge(info, LTC294X_REG_CHARGE_THR_HIGH_MSB,
    &val.intval);
    case POWER_SUPPLY_PROP_CHARGE_EMPTY:
    return ltc294x_get_charge(info, LTC294X_REG_CHARGE_THR_LOW_MSB,
    &val.intval);
    case POWER_SUPPLY_PROP_CHARGE_NOW:
    return ltc294x_get_charge(info, LTC294X_REG_ACC_CHARGE_MSB,
    &val.intval);
    case POWER_SUPPLY_PROP_CHARGE_COUNTER:
    return ltc294x_get_charge_counter(info, &val.intval);
    case POWER_SUPPLY_PROP_VOLTAGE_NOW:
    return ltc294x_get_voltage(info, &val.intval);
    case POWER_SUPPLY_PROP_CURRENT_NOW:
    return ltc294x_get_current(info, &val.intval);
    case POWER_SUPPLY_PROP_TEMP:
    return ltc294x_get_temperature(info, &val.intval);
    default:
    return -EINVAL;
    }
    }
    static int ltc294x_set_property(struct power_supply *psy,
    enum power_supply_property psp,
    const union power_supply_propval *val)
    {
    struct ltc294x_info *info = power_supply_get_drvdata(psy);
    switch (psp) {
    case POWER_SUPPLY_PROP_CHARGE_FULL:
    return ltc294x_set_charge_thr(info,
    LTC294X_REG_CHARGE_THR_HIGH_MSB, val.intval);
    case POWER_SUPPLY_PROP_CHARGE_EMPTY:
    return ltc294x_set_charge_thr(info,
    LTC294X_REG_CHARGE_THR_LOW_MSB, val.intval);
    case POWER_SUPPLY_PROP_CHARGE_NOW:
    return ltc294x_set_charge_now(info, val.intval);
    default:
    return -EPERM;
    }
    }
    static int ltc294x_property_is_writeable(
    struct power_supply *psy, enum power_supply_property psp)
    {
    switch (psp) {
    case POWER_SUPPLY_PROP_CHARGE_FULL:
    case POWER_SUPPLY_PROP_CHARGE_EMPTY:
    case POWER_SUPPLY_PROP_CHARGE_NOW:
    return 1;
    default:
    return 0;
    }
    }
#[no_mangle]
unsafe extern "C" fn ltc294x_update(info: *mut ltc294x_info) {
    static void ltc294x_update(struct ltc294x_info *info)
    {
    let mut charge: c_int = ltc294x_read_charge_register(info, LTC294X_REG_ACC_CHARGE_MSB);
    if (charge != info.charge) {
    info.charge = charge;
    power_supply_changed(info.supply);
    }
    }
#[no_mangle]
unsafe extern "C" fn ltc294x_work(work: *mut work_struct) {
    static void ltc294x_work(struct work_struct *work)
    {
    struct ltc294x_info *info;
    info = container_of(work, struct ltc294x_info, work.work);
    ltc294x_update(info);
    schedule_delayed_work(&info.work, LTC294X_WORK_DELAY * HZ);
    }
    static enum power_supply_property ltc294x_properties[] = {
    POWER_SUPPLY_PROP_CHARGE_COUNTER,
    POWER_SUPPLY_PROP_CHARGE_FULL,
    POWER_SUPPLY_PROP_CHARGE_EMPTY,
    POWER_SUPPLY_PROP_CHARGE_NOW,
    POWER_SUPPLY_PROP_VOLTAGE_NOW,
    POWER_SUPPLY_PROP_TEMP,
    POWER_SUPPLY_PROP_CURRENT_NOW,
    };
#[no_mangle]
unsafe extern "C" fn ltc294x_i2c_probe(client: *mut i2c_client) -> c_int {
    static int ltc294x_i2c_probe(struct i2c_client *client)
    {
    let mut psy_cfg: power_supply_config = {};
    struct ltc294x_info *info;
    struct device_node *np;
    int ret;
    u32 prescaler_exp;
    s32 r_sense;
    u8 status;
    info = devm_kzalloc(&client.dev, sizeof(*info), GFP_KERNEL);
    if (info == core::ptr::null_mut())
    return -ENOMEM;
    i2c_set_clientdata(client, info);
    np = of_node_get(client.dev.of_node);
    info.id = (enum ltc294x_id) (uintptr_t) of_device_get_match_data(
    &client.dev);
    info.supply_desc.name = np.name;
// r_sense can be negative, when sense+ is connected to the battery
// instead of the sense-. This results in reversed measurements.
    ret = of_property_read_s32(np, "lltc,resistor-sense", &r_sense);
    if (ret < 0)
    return dev_err_probe(&client.dev, ret,
    "Could not find lltc,resistor-sense in devicetree\n");
    info.r_sense = r_sense;
    ret = of_property_read_u32(np, "lltc,prescaler-exponent",
    &prescaler_exp);
    if (ret < 0) {
    dev_warn(&client.dev,
    "lltc,prescaler-exponent not in devicetree\n");
    prescaler_exp = LTC2941_MAX_PRESCALER_EXP;
    }
    if (info.id == LTC2943_ID) {
    if (prescaler_exp > LTC2943_MAX_PRESCALER_EXP)
    prescaler_exp = LTC2943_MAX_PRESCALER_EXP;
    info.Qlsb = ((340 * 50000) / r_sense) >>
    (12 - 2*prescaler_exp);
    } else {
    if (prescaler_exp > LTC2941_MAX_PRESCALER_EXP)
    prescaler_exp = LTC2941_MAX_PRESCALER_EXP;
    info.Qlsb = ((85 * 50000) / r_sense) >>
    (7 - prescaler_exp);
    }
// Read status register to check for LTC2942
    if (info.id == LTC2941_ID || info.id == LTC2942_ID) {
    ret = ltc294x_read_regs(client, LTC294X_REG_STATUS, &status, 1);
    if (ret < 0)
    return dev_err_probe(&client.dev, ret,
    "Could not read status register\n");
    if (status & LTC2941_REG_STATUS_CHIP_ID)
    info.id = LTC2941_ID;
    else
    info.id = LTC2942_ID;
    }
    info.client = client;
    info.supply_desc.type = POWER_SUPPLY_TYPE_BATTERY;
    info.supply_desc.properties = ltc294x_properties;
    switch (info.id) {
    case LTC2944_ID:
    case LTC2943_ID:
    info.supply_desc.num_properties =
    ARRAY_SIZE(ltc294x_properties);
    break;
    case LTC2942_ID:
    info.supply_desc.num_properties =
    ARRAY_SIZE(ltc294x_properties) - 1;
    break;
    case LTC2941_ID:
    default:
    info.supply_desc.num_properties =
    ARRAY_SIZE(ltc294x_properties) - 3;
    break;
    }
    info.supply_desc.get_property = ltc294x_get_property;
    info.supply_desc.set_property = ltc294x_set_property;
    info.supply_desc.property_is_writeable = ltc294x_property_is_writeable;
    info.supply_desc.external_power_changed	= core::ptr::null_mut();
    psy_cfg.drv_data = info;
    ret = devm_delayed_work_autocancel(&client.dev, &info.work,
    ltc294x_work);
    if (ret)
    return ret;
    ret = ltc294x_reset(info, prescaler_exp);
    if (ret < 0)
    return dev_err_probe(&client.dev, ret,
    "Communication with chip failed\n");
    info.supply = devm_power_supply_register(&client.dev,
    &info.supply_desc, &psy_cfg);
    if (IS_ERR(info.supply))
    return dev_err_probe(&client.dev, PTR_ERR(info.supply),
    "failed to register ltc2941\n");
    schedule_delayed_work(&info.work, LTC294X_WORK_DELAY * HZ);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ltc294x_i2c_shutdown(client: *mut i2c_client) {
    static void ltc294x_i2c_shutdown(struct i2c_client *client)
    {
    struct ltc294x_info *info = i2c_get_clientdata(client);
    int ret;
    u8 value;
    u8 control;
// The LTC2941 does not need any special handling
    if (info.id == LTC2941_ID)
    return;
// Read control register
    ret = ltc294x_read_regs(info.client, LTC294X_REG_CONTROL, &value, 1);
    if (ret < 0)
    return;
// Disable continuous ADC conversion as this drains the battery
    control = LTC294X_REG_CONTROL_ADC_DISABLE(value);
    if (control != value)
    ltc294x_write_regs(info.client, LTC294X_REG_CONTROL,
    &control, 1);
    }

#[no_mangle]
unsafe extern "C" fn ltc294x_suspend(dev: *mut device) -> c_int {
    static int ltc294x_suspend(struct device *dev)
    {
    struct i2c_client *client = to_i2c_client(dev);
    struct ltc294x_info *info = i2c_get_clientdata(client);
    cancel_delayed_work(&info.work);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ltc294x_resume(dev: *mut device) -> c_int {
    static int ltc294x_resume(struct device *dev)
    {
    struct i2c_client *client = to_i2c_client(dev);
    struct ltc294x_info *info = i2c_get_clientdata(client);
    schedule_delayed_work(&info.work, LTC294X_WORK_DELAY * HZ);
    return 0;
    }
    static SIMPLE_DEV_PM_OPS(ltc294x_pm_ops, ltc294x_suspend, ltc294x_resume);

    static const struct i2c_device_id ltc294x_i2c_id[] = {
    { .name = "ltc2941", .driver_data = LTC2941_ID },
    { .name = "ltc2942", .driver_data = LTC2942_ID },
    { .name = "ltc2943", .driver_data = LTC2943_ID },
    { .name = "ltc2944", .driver_data = LTC2944_ID },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, ltc294x_i2c_id);
    static const struct of_device_id ltc294x_i2c_of_match[] = {
    {
    .compatible = "lltc,ltc2941",
    .data = (void *)LTC2941_ID,
    },
    {
    .compatible = "lltc,ltc2942",
    .data = (void *)LTC2942_ID,
    },
    {
    .compatible = "lltc,ltc2943",
    .data = (void *)LTC2943_ID,
    },
    {
    .compatible = "lltc,ltc2944",
    .data = (void *)LTC2944_ID,
    },
    { },
    };
    MODULE_DEVICE_TABLE(of, ltc294x_i2c_of_match);
    static struct i2c_driver ltc294x_driver = {
    .driver = {
    .name	= "LTC2941",
    .of_match_table = ltc294x_i2c_of_match,
    .pm	= LTC294X_PM_OPS,
    },
    .probe		= ltc294x_i2c_probe,
    .shutdown	= ltc294x_i2c_shutdown,
    .id_table	= ltc294x_i2c_id,
    };
    module_i2c_driver(ltc294x_driver);
    MODULE_AUTHOR("Auryn Verwegen, Topic Embedded Systems");
    MODULE_AUTHOR("Mike Looijmans, Topic Embedded Products");
    MODULE_DESCRIPTION("LTC2941/LTC2942/LTC2943/LTC2944 Battery Gas Gauge IC driver");
    MODULE_LICENSE("GPL");
