//! Automatically rewritten from C to Rust
//! Source: drivers/power/supply/ip5xxx_power.c
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
// Copyright (C) 2021 Samuel Holland <samuel@sholland.org>

pub const IP5XXX_BAT_TYPE_4_2V: c_uint = 0x0;
pub const IP5XXX_BAT_TYPE_4_3V: c_uint = 0x1;
pub const IP5XXX_BAT_TYPE_4_35V: c_uint = 0x2;
pub const IP5XXX_BAT_TYPE_4_4V: c_uint = 0x3;
pub const IP5XXX_CHG_STAT_IDLE: c_uint = 0x0;
pub const IP5XXX_CHG_STAT_TRICKLE: c_uint = 0x1;
pub const IP5XXX_CHG_STAT_CONST_VOLT: c_uint = 0x2;
pub const IP5XXX_CHG_STAT_CONST_CUR: c_uint = 0x3;
pub const IP5XXX_CHG_STAT_CONST_VOLT_STOP: c_uint = 0x4;
pub const IP5XXX_CHG_STAT_FULL: c_uint = 0x5;
pub const IP5XXX_CHG_STAT_TIMEOUT: c_uint = 0x6;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip5xxx {
    pub regmap: *mut regmap,
    pub initialized: bool,
    struct {
    struct {
// Charger enable
    pub enable: *mut regmap_field,
// Constant voltage value
    pub const_volt_sel: *mut regmap_field,
// Constant current value
    pub const_curr_sel: *mut regmap_field,
// Charger status
    pub status: *mut regmap_field,
// Charging ended flag
    pub chg_end: *mut regmap_field,
// Timeout flags (CV, charge, trickle)
    pub timeout: *mut regmap_field,
// Overvoltage limit
    pub vin_overvolt: *mut regmap_field,
    pub charger: },
    struct {
// Boost converter enable
    pub enable: *mut regmap_field,
    struct {
// Light load shutdown enable
    pub enable: *mut regmap_field,
// Light load shutdown current limit
    pub i_limit: *mut regmap_field,
    pub light_load_shutdown: },
// Automatic powerup on increased load
    pub load_powerup_en: *mut regmap_field,
// Automatic powerup on VIN pull-out
    pub vin_pullout_en: *mut regmap_field,
// Undervoltage limit
    pub undervolt_limit: *mut regmap_field,
// Light load status flag
    pub light_load_status: *mut regmap_field,
    pub boost: },
    struct {
// NTC disable
    pub ntc_dis: *mut regmap_field,
// Battery voltage type
    pub type: *mut regmap_field,
// Battery voltage autoset from Vset pin
    pub vset_en: *mut regmap_field,
    struct {
// Battery measurement registers
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip5xxx_battery_adc_regs {
    pub low: *mut regmap_field,
    pub high: *mut regmap_field,
    pub open_volt: } volt, curr,,
    pub adc: },
    pub battery: },
    struct {
// Double/long press shutdown enable
    pub shdn_enable: *mut regmap_field,
// WLED activation: double press or long press
    pub wled_mode: *mut regmap_field,
// Shutdown activation: double press or long press
    pub shdn_mode: *mut regmap_field,
// Long press time
    pub long_press_time: *mut regmap_field,
// Button pressed
    pub pressed: *mut regmap_field,
// Button long-pressed
    pub long_pressed: *mut regmap_field,
// Button short-pressed
    pub short_pressed: *mut regmap_field,
    pub btn: },
    struct {
// WLED enable
    pub enable: *mut regmap_field,
// WLED detect
    pub detect_en: *mut regmap_field,
// WLED present
    pub present: *mut regmap_field,
    pub wled: },
    pub regs: },
// Maximum supported battery voltage (via regs.battery.type)
    pub vbat_max: c_int,
// Scaling constants for regs.boost.undervolt_limit
    struct {
    pub setpoint: c_int,
    pub microvolts_per_bit: c_int,
    pub boost_undervolt: },
// Scaling constants for regs.charger.const_curr_sel
    struct {
    pub setpoint: c_int,
    pub const_curr: },
// Whether regs.charger.chg_end is inverted
    pub chg_end_inverted: u8,
}

// Register fields layout. Unsupported registers marked as { .lsb = 1 }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip5xxx_regfield_config {
    pub charger_enable: reg_field,
    pub charger_const_volt_sel: reg_field,
    pub charger_const_curr_sel: reg_field,
    pub charger_status: reg_field,
    pub charger_chg_end: reg_field,
    pub charger_timeout: reg_field,
    pub charger_vin_overvolt: reg_field,
    pub boost_enable: reg_field,
    pub boost_llshdn_enable: reg_field,
    pub boost_llshdn_i_limit: reg_field,
    pub boost_load_powerup_en: reg_field,
    pub boost_vin_pullout_en: reg_field,
    pub boost_undervolt_limit: reg_field,
    pub boost_light_load_status: reg_field,
    pub battery_ntc_dis: reg_field,
    pub battery_type: reg_field,
    pub battery_vset_en: reg_field,
    pub battery_adc_volt_low: reg_field,
    pub battery_adc_volt_high: reg_field,
    pub battery_adc_curr_low: reg_field,
    pub battery_adc_curr_high: reg_field,
    pub battery_adc_ovolt_low: reg_field,
    pub battery_adc_ovolt_high: reg_field,
    pub btn_shdn_enable: reg_field,
    pub btn_wled_mode: reg_field,
    pub btn_shdn_mode: reg_field,
    pub btn_long_press_time: reg_field,
    pub btn_pressed: reg_field,
    pub btn_long_pressed: reg_field,
    pub btn_short_pressed: reg_field,
    pub wled_enable: reg_field,
    pub wled_detect_en: reg_field,
    pub wled_present: reg_field,
    pub vbat_max: c_int,
    pub boost_undervolt_setpoint: c_int,
    pub boost_undervolt_uv_per_bit: c_int,
    pub const_curr_setpoint: c_int,
    pub chg_end_inverted: u8,
}

//
// The IP5xxx charger only responds on I2C when it is "awake". The charger is
// generally only awake when VIN is powered or when its boost converter is
// enabled. Going into shutdown resets all register values. To handle this:
// 1) When any bus error occurs, assume the charger has gone into shutdown.
// 2) Attempt the initialization sequence on each subsequent register access
// until it succeeds.
//
    static int ip5xxx_read(struct ip5xxx *ip5xxx, struct regmap_field *field,
    unsigned int *val)
    {
    int ret;
    if (!field)
    return -EOPNOTSUPP;
    ret = regmap_field_read(field, val);
    if (ret)
    ip5xxx.initialized = false;
    return ret;
    }
    static int ip5xxx_write(struct ip5xxx *ip5xxx, struct regmap_field *field,
    unsigned int val)
    {
    int ret;
    if (!field)
    return -EOPNOTSUPP;
    ret = regmap_field_write(field, val);
    if (ret)
    ip5xxx.initialized = false;
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ip5xxx_initialize(psy: *mut power_supply) -> c_int {
    static int ip5xxx_initialize(struct power_supply *psy)
    {
    struct ip5xxx *ip5xxx = power_supply_get_drvdata(psy);
    int ret;
    if (ip5xxx.initialized)
    return 0;
//
// Disable shutdown under light load.
// Enable power on when under load.
//
    if (ip5xxx.regs.boost.light_load_shutdown.enable) {
    ret = ip5xxx_write(ip5xxx, ip5xxx.regs.boost.light_load_shutdown.enable, 0);
    if (ret)
    return ret;
    }
    ret = ip5xxx_write(ip5xxx, ip5xxx.regs.boost.load_powerup_en, 1);
    if (ret)
    return ret;
//
// Enable shutdown after a long button press (as configured below).
//
    ret = ip5xxx_write(ip5xxx, ip5xxx.regs.btn.shdn_enable, 1);
    if (ret)
    return ret;
//
// Power on automatically when VIN is removed.
//
    ret = ip5xxx_write(ip5xxx, ip5xxx.regs.boost.vin_pullout_en, 1);
    if (ret)
    return ret;
//
// Enable the NTC.
// Configure the button for two presses => LED, long press => shutdown.
//
    if (ip5xxx.regs.battery.ntc_dis) {
    ret = ip5xxx_write(ip5xxx, ip5xxx.regs.battery.ntc_dis, 0);
    if (ret)
    return ret;
    }
    ret = ip5xxx_write(ip5xxx, ip5xxx.regs.btn.wled_mode, 1);
    if (ret)
    return ret;
    ret = ip5xxx_write(ip5xxx, ip5xxx.regs.btn.shdn_mode, 1);
    if (ret)
    return ret;
    ip5xxx.initialized = true;
    dev_dbg(psy.dev.parent, "Initialized after power on\n");
    return 0;
    }
    static const enum power_supply_property ip5xxx_battery_properties[] = {
    POWER_SUPPLY_PROP_STATUS,
    POWER_SUPPLY_PROP_CHARGE_TYPE,
    POWER_SUPPLY_PROP_HEALTH,
    POWER_SUPPLY_PROP_VOLTAGE_MAX_DESIGN,
    POWER_SUPPLY_PROP_VOLTAGE_NOW,
    POWER_SUPPLY_PROP_VOLTAGE_OCV,
    POWER_SUPPLY_PROP_CURRENT_NOW,
    POWER_SUPPLY_PROP_CONSTANT_CHARGE_CURRENT,
    POWER_SUPPLY_PROP_CONSTANT_CHARGE_CURRENT_MAX,
    POWER_SUPPLY_PROP_CONSTANT_CHARGE_VOLTAGE,
    POWER_SUPPLY_PROP_CONSTANT_CHARGE_VOLTAGE_MAX,
    };
#[no_mangle]
unsafe extern "C" fn ip5xxx_battery_get_status(ip5xxx: *mut ip5xxx, val: *mut c_int) -> c_int {
    static int ip5xxx_battery_get_status(struct ip5xxx *ip5xxx, int *val)
    {
    unsigned int rval;
    int ret;
    if (!ip5xxx.regs.charger.status) {
// Fall-back to Charging Ended bit
    ret = ip5xxx_read(ip5xxx, ip5xxx.regs.charger.chg_end, &rval);
    if (ret)
    return ret;
    if (rval == ip5xxx.chg_end_inverted)
// val = POWER_SUPPLY_STATUS_CHARGING;
    else
// val = POWER_SUPPLY_STATUS_NOT_CHARGING;
    return 0;
    }
    ret = ip5xxx_read(ip5xxx, ip5xxx.regs.charger.status, &rval);
    if (ret)
    return ret;
    switch (rval) {
    case IP5XXX_CHG_STAT_IDLE:
// val = POWER_SUPPLY_STATUS_DISCHARGING;
    break;
    case IP5XXX_CHG_STAT_TRICKLE:
    case IP5XXX_CHG_STAT_CONST_CUR:
    case IP5XXX_CHG_STAT_CONST_VOLT:
// val = POWER_SUPPLY_STATUS_CHARGING;
    break;
    case IP5XXX_CHG_STAT_CONST_VOLT_STOP:
    case IP5XXX_CHG_STAT_FULL:
// val = POWER_SUPPLY_STATUS_FULL;
    break;
    case IP5XXX_CHG_STAT_TIMEOUT:
// val = POWER_SUPPLY_STATUS_NOT_CHARGING;
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ip5xxx_battery_get_charge_type(ip5xxx: *mut ip5xxx, val: *mut c_int) -> c_int {
    static int ip5xxx_battery_get_charge_type(struct ip5xxx *ip5xxx, int *val)
    {
    unsigned int rval;
    int ret;
    ret = ip5xxx_read(ip5xxx, ip5xxx.regs.charger.status, &rval);
    if (ret)
    return ret;
    switch (rval) {
    case IP5XXX_CHG_STAT_IDLE:
    case IP5XXX_CHG_STAT_CONST_VOLT_STOP:
    case IP5XXX_CHG_STAT_FULL:
    case IP5XXX_CHG_STAT_TIMEOUT:
// val = POWER_SUPPLY_CHARGE_TYPE_NONE;
    break;
    case IP5XXX_CHG_STAT_TRICKLE:
// val = POWER_SUPPLY_CHARGE_TYPE_TRICKLE;
    break;
    case IP5XXX_CHG_STAT_CONST_CUR:
    case IP5XXX_CHG_STAT_CONST_VOLT:
// val = POWER_SUPPLY_CHARGE_TYPE_STANDARD;
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ip5xxx_battery_get_health(ip5xxx: *mut ip5xxx, val: *mut c_int) -> c_int {
    static int ip5xxx_battery_get_health(struct ip5xxx *ip5xxx, int *val)
    {
    unsigned int rval;
    int ret;
    ret = ip5xxx_read(ip5xxx, ip5xxx.regs.charger.timeout, &rval);
    if (ret)
    return ret;
    if (rval)
// val = POWER_SUPPLY_HEALTH_SAFETY_TIMER_EXPIRE;
    else
// val = POWER_SUPPLY_HEALTH_GOOD;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ip5xxx_battery_get_voltage_max(ip5xxx: *mut ip5xxx, val: *mut c_int) -> c_int {
    static int ip5xxx_battery_get_voltage_max(struct ip5xxx *ip5xxx, int *val)
    {
    unsigned int rval;
    int ret;
    ret = ip5xxx_read(ip5xxx, ip5xxx.regs.battery.type, &rval);
    if (ret)
    return ret;
//
// It is not clear what this will return if
// IP5XXX_CHG_CTL4_BAT_TYPE_SEL_EN is not set...
//
    switch (rval) {
    case IP5XXX_BAT_TYPE_4_2V:
// val = 4200000;
    break;
    case IP5XXX_BAT_TYPE_4_3V:
// val = 4300000;
    break;
    case IP5XXX_BAT_TYPE_4_35V:
// val = 4350000;
    break;
    case IP5XXX_BAT_TYPE_4_4V:
// val = 4400000;
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
    static int ip5xxx_battery_read_adc(struct ip5xxx *ip5xxx,
    struct ip5xxx_battery_adc_regs *regs, int *val)
    {
    unsigned int hi, lo;
    int ret;
    ret = ip5xxx_read(ip5xxx, regs.low, &lo);
    if (ret)
    return ret;
    ret = ip5xxx_read(ip5xxx, regs.high, &hi);
    if (ret)
    return ret;
// val = sign_extend32(hi << 8 | lo, 13);
    return 0;
    }
    static int ip5xxx_battery_get_property(struct power_supply *psy,
    enum power_supply_property psp,
    union power_supply_propval *val)
    {
    struct ip5xxx *ip5xxx = power_supply_get_drvdata(psy);
    int raw, ret, vmax;
    unsigned int rval;
    ret = ip5xxx_initialize(psy);
    if (ret)
    return ret;
    switch (psp) {
    case POWER_SUPPLY_PROP_STATUS:
    return ip5xxx_battery_get_status(ip5xxx, &val.intval);
    case POWER_SUPPLY_PROP_CHARGE_TYPE:
    return ip5xxx_battery_get_charge_type(ip5xxx, &val.intval);
    case POWER_SUPPLY_PROP_HEALTH:
    return ip5xxx_battery_get_health(ip5xxx, &val.intval);
    case POWER_SUPPLY_PROP_VOLTAGE_MAX_DESIGN:
    return ip5xxx_battery_get_voltage_max(ip5xxx, &val.intval);
    case POWER_SUPPLY_PROP_VOLTAGE_NOW:
    ret = ip5xxx_battery_read_adc(ip5xxx, &ip5xxx.regs.battery.adc.volt, &raw);
    if (ret)
    return ret;
    val.intval = 2600000 + DIV_ROUND_CLOSEST(raw * 26855, 100);
    return 0;
    case POWER_SUPPLY_PROP_VOLTAGE_OCV:
    ret = ip5xxx_battery_read_adc(ip5xxx, &ip5xxx.regs.battery.adc.open_volt, &raw);
    if (ret)
    return ret;
    val.intval = 2600000 + DIV_ROUND_CLOSEST(raw * 26855, 100);
    return 0;
    case POWER_SUPPLY_PROP_CURRENT_NOW:
    ret = ip5xxx_battery_read_adc(ip5xxx, &ip5xxx.regs.battery.adc.curr, &raw);
    if (ret)
    return ret;
    val.intval = DIV_ROUND_CLOSEST(raw * 149197, 200);
    return 0;
    case POWER_SUPPLY_PROP_CONSTANT_CHARGE_CURRENT:
    ret = ip5xxx_read(ip5xxx, ip5xxx.regs.charger.const_curr_sel, &rval);
    if (ret)
    return ret;
    val.intval = ip5xxx.const_curr.setpoint + 100000 * rval;
    return 0;
    case POWER_SUPPLY_PROP_CONSTANT_CHARGE_CURRENT_MAX:
    val.intval = 100000 * 0x1f;
    return 0;
    case POWER_SUPPLY_PROP_CONSTANT_CHARGE_VOLTAGE:
    ret = ip5xxx_battery_get_voltage_max(ip5xxx, &vmax);
    if (ret)
    return ret;
    ret = ip5xxx_read(ip5xxx, ip5xxx.regs.charger.const_volt_sel, &rval);
    if (ret)
    return ret;
    val.intval = vmax + 14000 * rval;
    return 0;
    case POWER_SUPPLY_PROP_CONSTANT_CHARGE_VOLTAGE_MAX:
    ret = ip5xxx_battery_get_voltage_max(ip5xxx, &vmax);
    if (ret)
    return ret;
    val.intval = vmax + 14000 * 3;
    return 0;
    default:
    return -EINVAL;
    }
    }
#[no_mangle]
unsafe extern "C" fn ip5xxx_battery_set_voltage_max(ip5xxx: *mut ip5xxx, val: c_int) -> c_int {
    static int ip5xxx_battery_set_voltage_max(struct ip5xxx *ip5xxx, int val)
    {
    unsigned int rval;
    int ret;
    if (val > ip5xxx.vbat_max)
    return -EINVAL;
    switch (val) {
    case 4200000:
    rval = IP5XXX_BAT_TYPE_4_2V;
    break;
    case 4300000:
    rval = IP5XXX_BAT_TYPE_4_3V;
    break;
    case 4350000:
    rval = IP5XXX_BAT_TYPE_4_35V;
    break;
    case 4400000:
    rval = IP5XXX_BAT_TYPE_4_4V;
    break;
    default:
    return -EINVAL;
    }
    ret = ip5xxx_write(ip5xxx, ip5xxx.regs.battery.type, rval);
    if (ret)
    return ret;
// Don't try to auto-detect battery type, even if the IC could
    if (ip5xxx.regs.battery.vset_en) {
    ret = ip5xxx_write(ip5xxx, ip5xxx.regs.battery.vset_en, 1);
    if (ret)
    return ret;
    }
    return 0;
    }
    static int ip5xxx_battery_set_property(struct power_supply *psy,
    enum power_supply_property psp,
    const union power_supply_propval *val)
    {
    struct ip5xxx *ip5xxx = power_supply_get_drvdata(psy);
    unsigned int rval;
    int ret, vmax;
    ret = ip5xxx_initialize(psy);
    if (ret)
    return ret;
    switch (psp) {
    case POWER_SUPPLY_PROP_STATUS:
    switch (val.intval) {
    case POWER_SUPPLY_STATUS_CHARGING:
    rval = 1;
    break;
    case POWER_SUPPLY_STATUS_DISCHARGING:
    case POWER_SUPPLY_STATUS_NOT_CHARGING:
    rval = 0;
    break;
    default:
    return -EINVAL;
    }
    return ip5xxx_write(ip5xxx, ip5xxx.regs.charger.enable, rval);
    case POWER_SUPPLY_PROP_VOLTAGE_MAX_DESIGN:
    return ip5xxx_battery_set_voltage_max(ip5xxx, val.intval);
    case POWER_SUPPLY_PROP_CONSTANT_CHARGE_CURRENT:
    rval = (val.intval - ip5xxx.const_curr.setpoint) / 100000;
    return ip5xxx_write(ip5xxx, ip5xxx.regs.charger.const_curr_sel, rval);
    case POWER_SUPPLY_PROP_CONSTANT_CHARGE_VOLTAGE:
    ret = ip5xxx_battery_get_voltage_max(ip5xxx, &vmax);
    if (ret)
    return ret;
    rval = (val.intval - vmax) / 14000;
    return ip5xxx_write(ip5xxx, ip5xxx.regs.charger.const_volt_sel, rval);
    default:
    return -EINVAL;
    }
    }
    static int ip5xxx_battery_property_is_writeable(struct power_supply *psy,
    enum power_supply_property psp)
    {
    return psp == POWER_SUPPLY_PROP_STATUS ||
    psp == POWER_SUPPLY_PROP_VOLTAGE_MAX_DESIGN ||
    psp == POWER_SUPPLY_PROP_CONSTANT_CHARGE_CURRENT ||
    psp == POWER_SUPPLY_PROP_CONSTANT_CHARGE_VOLTAGE;
    }
    static const struct power_supply_desc ip5xxx_battery_desc = {
    .name			= "ip5xxx-battery",
    .type			= POWER_SUPPLY_TYPE_BATTERY,
    .properties		= ip5xxx_battery_properties,
    .num_properties		= ARRAY_SIZE(ip5xxx_battery_properties),
    .get_property		= ip5xxx_battery_get_property,
    .set_property		= ip5xxx_battery_set_property,
    .property_is_writeable	= ip5xxx_battery_property_is_writeable,
    };
    static const enum power_supply_property ip5xxx_boost_properties[] = {
    POWER_SUPPLY_PROP_ONLINE,
    POWER_SUPPLY_PROP_VOLTAGE_MIN_DESIGN,
    };
    static int ip5xxx_boost_get_property(struct power_supply *psy,
    enum power_supply_property psp,
    union power_supply_propval *val)
    {
    struct ip5xxx *ip5xxx = power_supply_get_drvdata(psy);
    unsigned int rval;
    int ret;
    ret = ip5xxx_initialize(psy);
    if (ret)
    return ret;
    switch (psp) {
    case POWER_SUPPLY_PROP_ONLINE:
    ret = ip5xxx_read(ip5xxx, ip5xxx.regs.boost.enable, &rval);
    if (ret)
    return ret;
    val.intval = !!rval;
    return 0;
    case POWER_SUPPLY_PROP_VOLTAGE_MIN_DESIGN:
    ret = ip5xxx_read(ip5xxx, ip5xxx.regs.boost.undervolt_limit, &rval);
    if (ret)
    return ret;
    val.intval = ip5xxx.boost_undervolt.setpoint +
    ip5xxx.boost_undervolt.microvolts_per_bit * rval;
    return 0;
    default:
    return -EINVAL;
    }
    }
    static int ip5xxx_boost_set_property(struct power_supply *psy,
    enum power_supply_property psp,
    const union power_supply_propval *val)
    {
    struct ip5xxx *ip5xxx = power_supply_get_drvdata(psy);
    unsigned int rval;
    int ret;
    ret = ip5xxx_initialize(psy);
    if (ret)
    return ret;
    switch (psp) {
    case POWER_SUPPLY_PROP_ONLINE:
    return ip5xxx_write(ip5xxx, ip5xxx.regs.boost.enable, !!val.intval);
    case POWER_SUPPLY_PROP_VOLTAGE_MIN_DESIGN:
    rval = (val.intval - ip5xxx.boost_undervolt.setpoint) /
    ip5xxx.boost_undervolt.microvolts_per_bit;
    return ip5xxx_write(ip5xxx, ip5xxx.regs.boost.undervolt_limit, rval);
    default:
    return -EINVAL;
    }
    }
    static int ip5xxx_boost_property_is_writeable(struct power_supply *psy,
    enum power_supply_property psp)
    {
    return true;
    }
    static const struct power_supply_desc ip5xxx_boost_desc = {
    .name			= "ip5xxx-boost",
    .type			= POWER_SUPPLY_TYPE_USB,
    .properties		= ip5xxx_boost_properties,
    .num_properties		= ARRAY_SIZE(ip5xxx_boost_properties),
    .get_property		= ip5xxx_boost_get_property,
    .set_property		= ip5xxx_boost_set_property,
    .property_is_writeable	= ip5xxx_boost_property_is_writeable,
    };
    static const struct regmap_config ip5xxx_regmap_config = {
    .reg_bits		= 8,
    .val_bits		= 8,
    .max_register		= 0xa9,
    };
    static struct ip5xxx_regfield_config ip51xx_fields = {
    .charger_enable = REG_FIELD(0x01, 1, 1),
    .charger_const_volt_sel = REG_FIELD(0x24, 1, 2),
    .charger_const_curr_sel = REG_FIELD(0x25, 0, 4),
    .charger_status = REG_FIELD(0x71, 5, 7),
    .charger_chg_end = REG_FIELD(0x71, 3, 3),
    .charger_timeout = REG_FIELD(0x71, 0, 2),
    .charger_vin_overvolt = REG_FIELD(0x72, 5, 5),
    .boost_enable = REG_FIELD(0x01, 2, 2),
    .boost_llshdn_enable = REG_FIELD(0x02, 1, 1),
    .boost_llshdn_i_limit = REG_FIELD(0x0c, 3, 7),
    .boost_load_powerup_en = REG_FIELD(0x02, 0, 0),
    .boost_vin_pullout_en = REG_FIELD(0x04, 5, 5),
    .boost_undervolt_limit = REG_FIELD(0x22, 2, 3),
    .boost_light_load_status = REG_FIELD(0x72, 6, 6),
    .battery_ntc_dis = REG_FIELD(0x07, 6, 6),
    .battery_type = REG_FIELD(0x24, 5, 6),
    .battery_vset_en = REG_FIELD(0x26, 6, 6),
    .battery_adc_volt_low = REG_FIELD(0xa2, 0, 7),
    .battery_adc_volt_high = REG_FIELD(0xa3, 0, 5),
    .battery_adc_curr_low = REG_FIELD(0xa4, 0, 7),
    .battery_adc_curr_high = REG_FIELD(0xa5, 0, 5),
    .battery_adc_ovolt_low = REG_FIELD(0xa8, 0, 7),
    .battery_adc_ovolt_high = REG_FIELD(0xa9, 0, 5),
    .btn_shdn_enable = REG_FIELD(0x03, 5, 5),
    .btn_wled_mode = REG_FIELD(0x07, 1, 1),
    .btn_shdn_mode = REG_FIELD(0x07, 0, 0),
    .btn_long_press_time = REG_FIELD(0x03, 6, 7),
    .btn_pressed = REG_FIELD(0x77, 3, 3),
    .btn_long_pressed = REG_FIELD(0x77, 1, 1),
    .btn_short_pressed = REG_FIELD(0x77, 0, 0),
    .wled_enable = REG_FIELD(0x01, 3, 3),
    .wled_detect_en = REG_FIELD(0x01, 4, 4),
    .wled_present = REG_FIELD(0x72, 7, 7),
    .vbat_max = 4350000,
    .boost_undervolt_setpoint = 4530000,
    .boost_undervolt_uv_per_bit = 100000,
    };
    static struct ip5xxx_regfield_config ip5306_fields = {
    .charger_enable = REG_FIELD(0x00, 4, 4),
    .charger_const_volt_sel = REG_FIELD(0x22, 0, 1),
    .charger_const_curr_sel = REG_FIELD(0x24, 0, 4),
    .charger_status = REG_FIELD_UNSUPPORTED, // other bits...
    .charger_chg_end = REG_FIELD(0x71, 3, 3),
    .charger_timeout = REG_FIELD_UNSUPPORTED,
    .charger_vin_overvolt = REG_FIELD_UNSUPPORTED,
    .boost_enable = REG_FIELD(0x00, 5, 5),
    .boost_llshdn_enable = REG_FIELD_UNSUPPORTED,
    .boost_llshdn_i_limit = REG_FIELD_UNSUPPORTED,
    .boost_load_powerup_en = REG_FIELD(0x00, 2, 2),
    .boost_vin_pullout_en = REG_FIELD(0x01, 2, 2),
    .boost_undervolt_limit = REG_FIELD(0x21, 2, 4),
    .boost_light_load_status = REG_FIELD(0x72, 2, 2),
    .battery_ntc_dis = REG_FIELD_UNSUPPORTED,
    .battery_type = REG_FIELD(0x22, 2, 3),
    .battery_vset_en = REG_FIELD_UNSUPPORTED,
    .battery_adc_volt_low = REG_FIELD_UNSUPPORTED,
    .battery_adc_volt_high = REG_FIELD_UNSUPPORTED,
    .battery_adc_curr_low = REG_FIELD_UNSUPPORTED,
    .battery_adc_curr_high = REG_FIELD_UNSUPPORTED,
    .battery_adc_ovolt_low = REG_FIELD_UNSUPPORTED,
    .battery_adc_ovolt_high = REG_FIELD_UNSUPPORTED,
    .btn_shdn_enable = REG_FIELD(0x00, 0, 0),
    .btn_wled_mode = REG_FIELD(0x01, 6, 6),
    .btn_shdn_mode = REG_FIELD(0x01, 7, 7),
    .btn_long_press_time = REG_FIELD(0x02, 4, 4), // +1s
    .btn_pressed = REG_FIELD_UNSUPPORTED,
// TODO: double press
    .btn_long_pressed = REG_FIELD(0x77, 1, 1),
    .btn_short_pressed = REG_FIELD(0x77, 0, 0),
    .wled_enable = REG_FIELD_UNSUPPORTED,
    .wled_detect_en = REG_FIELD_UNSUPPORTED,
    .wled_present = REG_FIELD_UNSUPPORTED,
    .vbat_max = 4400000,
    .boost_undervolt_setpoint = 4450000,
    .boost_undervolt_uv_per_bit = 50000,
    .const_curr_setpoint = 50000,
    .chg_end_inverted = 1,
    };

    do { \
    if (likely(cfg._field.lsb <= cfg._field.msb)) { \
    struct regmap_field *_tmp = devm_regmap_field_alloc(dev, \
    ip5xxx.regmap, cfg._field); \
    if (!IS_ERR(_tmp)) \
    ip5xxx.regs._reg = _tmp; \
    } \
    } while (0)
    static void ip5xxx_setup_regs(struct device *dev, struct ip5xxx *ip5xxx,
    const struct ip5xxx_regfield_config *cfg)
    {
    ip5xxx_setup_reg(charger_enable, charger.enable);
    ip5xxx_setup_reg(charger_const_volt_sel, charger.const_volt_sel);
    ip5xxx_setup_reg(charger_const_curr_sel, charger.const_curr_sel);
    ip5xxx_setup_reg(charger_status, charger.status);
    ip5xxx_setup_reg(charger_chg_end, charger.chg_end);
    ip5xxx_setup_reg(charger_timeout, charger.timeout);
    ip5xxx_setup_reg(charger_vin_overvolt, charger.vin_overvolt);
    ip5xxx_setup_reg(boost_enable, boost.enable);
    ip5xxx_setup_reg(boost_llshdn_enable, boost.light_load_shutdown.enable);
    ip5xxx_setup_reg(boost_llshdn_i_limit, boost.light_load_shutdown.i_limit);
    ip5xxx_setup_reg(boost_load_powerup_en, boost.load_powerup_en);
    ip5xxx_setup_reg(boost_vin_pullout_en, boost.vin_pullout_en);
    ip5xxx_setup_reg(boost_undervolt_limit, boost.undervolt_limit);
    ip5xxx_setup_reg(boost_light_load_status, boost.light_load_status);
    ip5xxx_setup_reg(battery_ntc_dis, battery.ntc_dis);
    ip5xxx_setup_reg(battery_type, battery.type);
    ip5xxx_setup_reg(battery_vset_en, battery.vset_en);
    ip5xxx_setup_reg(battery_adc_volt_low, battery.adc.volt.low);
    ip5xxx_setup_reg(battery_adc_volt_high, battery.adc.volt.high);
    ip5xxx_setup_reg(battery_adc_curr_low, battery.adc.curr.low);
    ip5xxx_setup_reg(battery_adc_curr_high, battery.adc.curr.high);
    ip5xxx_setup_reg(battery_adc_ovolt_low, battery.adc.open_volt.low);
    ip5xxx_setup_reg(battery_adc_ovolt_high, battery.adc.open_volt.high);
    ip5xxx_setup_reg(btn_shdn_enable, btn.shdn_enable);
    ip5xxx_setup_reg(btn_wled_mode, btn.wled_mode);
    ip5xxx_setup_reg(btn_shdn_mode, btn.shdn_mode);
    ip5xxx_setup_reg(btn_long_press_time, btn.long_press_time);
    ip5xxx_setup_reg(btn_pressed, btn.pressed);
    ip5xxx_setup_reg(btn_long_pressed, btn.long_pressed);
    ip5xxx_setup_reg(btn_short_pressed, btn.short_pressed);
    ip5xxx_setup_reg(wled_enable, wled.enable);
    ip5xxx_setup_reg(wled_detect_en, wled.detect_en);
    ip5xxx_setup_reg(wled_present, wled.present);
    ip5xxx.vbat_max = cfg.vbat_max;
    ip5xxx.boost_undervolt.setpoint = cfg.boost_undervolt_setpoint;
    ip5xxx.boost_undervolt.microvolts_per_bit = cfg.boost_undervolt_uv_per_bit;
    ip5xxx.const_curr.setpoint = cfg.const_curr_setpoint;
    ip5xxx.chg_end_inverted = cfg.chg_end_inverted;
    }
#[no_mangle]
unsafe extern "C" fn ip5xxx_power_probe(client: *mut i2c_client) -> c_int {
    static int ip5xxx_power_probe(struct i2c_client *client)
    {
    const struct ip5xxx_regfield_config *fields;
    let mut psy_cfg: power_supply_config = {};
    struct device *dev = &client.dev;
    struct power_supply *psy;
    struct ip5xxx *ip5xxx;
    ip5xxx = devm_kzalloc(dev, sizeof(*ip5xxx), GFP_KERNEL);
    if (!ip5xxx)
    return -ENOMEM;
    ip5xxx.regmap = devm_regmap_init_i2c(client, &ip5xxx_regmap_config);
    if (IS_ERR(ip5xxx.regmap))
    return PTR_ERR(ip5xxx.regmap);
    fields = i2c_get_match_data(client) ?: &ip51xx_fields;
    ip5xxx_setup_regs(dev, ip5xxx, fields);
    psy_cfg.fwnode = dev_fwnode(dev);
    psy_cfg.drv_data = ip5xxx;
    psy = devm_power_supply_register(dev, &ip5xxx_battery_desc, &psy_cfg);
    if (IS_ERR(psy))
    return PTR_ERR(psy);
    psy = devm_power_supply_register(dev, &ip5xxx_boost_desc, &psy_cfg);
    if (IS_ERR(psy))
    return PTR_ERR(psy);
    return 0;
    }
    static const struct of_device_id ip5xxx_power_of_match[] = {
    { .compatible = "injoinic,ip5108", .data = &ip51xx_fields },
    { .compatible = "injoinic,ip5109", .data = &ip51xx_fields },
    { .compatible = "injoinic,ip5207", .data = &ip51xx_fields },
    { .compatible = "injoinic,ip5209", .data = &ip51xx_fields },
    { .compatible = "injoinic,ip5306", .data = &ip5306_fields },
    { }
    };
    MODULE_DEVICE_TABLE(of, ip5xxx_power_of_match);
    static struct i2c_driver ip5xxx_power_driver = {
    .probe		= ip5xxx_power_probe,
    .driver		= {
    .name		= "ip5xxx-power",
    .of_match_table	= ip5xxx_power_of_match,
    }
    };
    module_i2c_driver(ip5xxx_power_driver);
    MODULE_AUTHOR("Samuel Holland <samuel@sholland.org>");
    MODULE_DESCRIPTION("Injoinic IP5xxx power bank IC driver");
    MODULE_LICENSE("GPL");
