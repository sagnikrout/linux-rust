//! Automatically rewritten from C to Rust
//! Source: drivers/power/supply/cw2015_battery.c
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
// Fuel gauge driver for CellWise 2013 / 2015
//
// Copyright (C) 2012, RockChip
// Copyright (C) 2020, Tobias Schramm
//
// Authors: xuhuicong <xhc@rock-chips.com>
// Authors: Tobias Schramm <t.schramm@manjaro.org>
//

pub const CW2015_SIZE_BATINFO: c_int = 64;
pub const CW2015_RESET_TRIES: c_int = 5;
pub const CW2015_REG_VERSION: c_uint = 0x00;
pub const CW2015_REG_VCELL: c_uint = 0x02;
pub const CW2015_REG_SOC: c_uint = 0x04;
pub const CW2015_REG_RRT_ALERT: c_uint = 0x06;
pub const CW2015_REG_CONFIG: c_uint = 0x08;
pub const CW2015_REG_MODE: c_uint = 0x0A;
pub const CW2015_REG_BATINFO: c_uint = 0x10;

// reset gauge of no valid state of charge could be polled for 40s

// reset gauge if state of charge stuck for half an hour during charging

// poll interval from CellWise GPL Android driver example
pub const CW2015_DEFAULT_POLL_INTERVAL_MS: c_int = 8000;
pub const CW2015_AVERAGING_SAMPLES: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cw_battery {
    pub dev: *mut device,
    pub battery_workqueue: *mut workqueue_struct,
    pub battery_delay_work: delayed_work,
    pub regmap: *mut regmap,
    pub rk_bat: *mut power_supply,
    pub battery: *mut power_supply_battery_info,
    pub bat_profile: *mut u8,
    pub charger_attached: bool,
    pub battery_changed: bool,
    pub soc: c_int,
    pub voltage_mv: c_int,
    pub status: c_int,
    pub time_to_empty: c_int,
    pub charge_count: c_int,
    pub poll_interval_ms: u32,
    pub alert_level: u8,
    pub read_errors: c_uint,
    pub charge_stuck_cnt: c_uint,
}

#[no_mangle]
unsafe extern "C" fn cw_read_word(cw_bat: *mut cw_battery, reg: u8, val: *mut u16) -> c_int {
    static int cw_read_word(struct cw_battery *cw_bat, u8 reg, u16 *val)
    {
    __be16 value;
    int ret;
    ret = regmap_bulk_read(cw_bat.regmap, reg, &value, sizeof(value));
    if (ret)
    return ret;
// val = be16_to_cpu(value);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cw_update_profile(cw_bat: *mut cw_battery) -> c_int {
    static int cw_update_profile(struct cw_battery *cw_bat)
    {
    int ret;
    unsigned int reg_val;
    u8 reset_val;
// make sure gauge is not in sleep mode
    ret = regmap_read(cw_bat.regmap, CW2015_REG_MODE, &reg_val);
    if (ret)
    return ret;
    reset_val = reg_val;
    if ((reg_val & CW2015_MODE_SLEEP_MASK) == CW2015_MODE_SLEEP) {
    dev_err(cw_bat.dev,
    "Gauge is in sleep mode, can't update battery info\n");
    return -EINVAL;
    }
// write new battery info
    ret = regmap_raw_write(cw_bat.regmap, CW2015_REG_BATINFO,
    cw_bat.bat_profile,
    CW2015_SIZE_BATINFO);
    if (ret)
    return ret;
// set config update flag
    reg_val |= CW2015_CONFIG_UPDATE_FLG;
    reg_val &= ~CW2015_MASK_ATHD;
    reg_val |= CW2015_ATHD(cw_bat.alert_level);
    ret = regmap_write(cw_bat.regmap, CW2015_REG_CONFIG, reg_val);
    if (ret)
    return ret;
// reset gauge to apply new battery profile
    reset_val &= ~CW2015_MODE_RESTART;
    reg_val = reset_val | CW2015_MODE_RESTART;
    ret = regmap_write(cw_bat.regmap, CW2015_REG_MODE, reg_val);
    if (ret)
    return ret;
// wait for gauge to reset
    msleep(20);
// clear reset flag
    ret = regmap_write(cw_bat.regmap, CW2015_REG_MODE, reset_val);
    if (ret)
    return ret;
// wait for gauge to become ready
    ret = regmap_read_poll_timeout(cw_bat.regmap, CW2015_REG_SOC,
    reg_val, reg_val <= 100,
    10 * USEC_PER_MSEC, 10 * USEC_PER_SEC);
    if (ret)
    dev_err(cw_bat.dev,
    "Gauge did not become ready after profile upload\n");
    else
    dev_dbg(cw_bat.dev, "Battery profile updated\n");
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn cw_init(cw_bat: *mut cw_battery) -> c_int {
    static int cw_init(struct cw_battery *cw_bat)
    {
    int ret;
    let mut reg_val: c_uint = CW2015_MODE_SLEEP;
    if ((reg_val & CW2015_MODE_SLEEP_MASK) == CW2015_MODE_SLEEP) {
    reg_val = CW2015_MODE_NORMAL;
    ret = regmap_write(cw_bat.regmap, CW2015_REG_MODE, reg_val);
    if (ret)
    return ret;
    }
    ret = regmap_read(cw_bat.regmap, CW2015_REG_CONFIG, &reg_val);
    if (ret)
    return ret;
    if ((reg_val & CW2015_MASK_ATHD) != CW2015_ATHD(cw_bat.alert_level)) {
    dev_dbg(cw_bat.dev, "Setting new alert level\n");
    reg_val &= ~CW2015_MASK_ATHD;
    reg_val |= ~CW2015_ATHD(cw_bat.alert_level);
    ret = regmap_write(cw_bat.regmap, CW2015_REG_CONFIG, reg_val);
    if (ret)
    return ret;
    }
    ret = regmap_read(cw_bat.regmap, CW2015_REG_CONFIG, &reg_val);
    if (ret)
    return ret;
    if (!(reg_val & CW2015_CONFIG_UPDATE_FLG)) {
    dev_dbg(cw_bat.dev,
    "Battery profile not present, uploading battery profile\n");
    if (cw_bat.bat_profile) {
    ret = cw_update_profile(cw_bat);
    if (ret) {
    dev_err(cw_bat.dev,
    "Failed to upload battery profile\n");
    return ret;
    }
    } else {
    dev_warn(cw_bat.dev,
    "No profile specified, continuing without profile\n");
    }
    } else if (cw_bat.bat_profile) {
    u8 bat_info[CW2015_SIZE_BATINFO];
    ret = regmap_raw_read(cw_bat.regmap, CW2015_REG_BATINFO,
    bat_info, CW2015_SIZE_BATINFO);
    if (ret) {
    dev_err(cw_bat.dev,
    "Failed to read stored battery profile\n");
    return ret;
    }
    if (memcmp(bat_info, cw_bat.bat_profile, CW2015_SIZE_BATINFO)) {
    dev_warn(cw_bat.dev, "Replacing stored battery profile\n");
    ret = cw_update_profile(cw_bat);
    if (ret)
    return ret;
    }
    } else {
    dev_warn(cw_bat.dev,
    "Can't check current battery profile, no profile provided\n");
    }
    dev_dbg(cw_bat.dev, "Battery profile configured\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cw_power_on_reset(cw_bat: *mut cw_battery) -> c_int {
    static int cw_power_on_reset(struct cw_battery *cw_bat)
    {
    int ret;
    unsigned char reset_val;
    reset_val = CW2015_MODE_SLEEP;
    ret = regmap_write(cw_bat.regmap, CW2015_REG_MODE, reset_val);
    if (ret)
    return ret;
// wait for gauge to enter sleep
    msleep(20);
    reset_val = CW2015_MODE_NORMAL;
    ret = regmap_write(cw_bat.regmap, CW2015_REG_MODE, reset_val);
    if (ret)
    return ret;
    ret = cw_init(cw_bat);
    if (ret)
    return ret;
    return 0;
    }

    (((current) < (previous) + (up)) && ((current) > (previous) - (down)))
#[no_mangle]
unsafe extern "C" fn cw_get_soc(cw_bat: *mut cw_battery) -> c_int {
    static int cw_get_soc(struct cw_battery *cw_bat)
    {
    unsigned int soc;
    int ret;
    ret = regmap_read(cw_bat.regmap, CW2015_REG_SOC, &soc);
    if (ret)
    return ret;
    if (soc > 100) {
    int max_error_cycles =
    CW2015_BAT_SOC_ERROR_MS / cw_bat.poll_interval_ms;
    dev_err(cw_bat.dev, "Invalid SoC %d%%\n", soc);
    cw_bat.read_errors++;
    if (cw_bat.read_errors > max_error_cycles) {
    dev_warn(cw_bat.dev,
    "Too many invalid SoC reports, resetting gauge\n");
    cw_power_on_reset(cw_bat);
    cw_bat.read_errors = 0;
    }
    return cw_bat.soc;
    }
    cw_bat.read_errors = 0;
// Reset gauge if stuck while charging
    if (cw_bat.status == POWER_SUPPLY_STATUS_CHARGING && soc == cw_bat.soc) {
    int max_stuck_cycles =
    CW2015_BAT_CHARGING_STUCK_MS / cw_bat.poll_interval_ms;
    cw_bat.charge_stuck_cnt++;
    if (cw_bat.charge_stuck_cnt > max_stuck_cycles) {
    dev_warn(cw_bat.dev,
    "SoC stuck @%u%%, resetting gauge\n", soc);
    cw_power_on_reset(cw_bat);
    cw_bat.charge_stuck_cnt = 0;
    }
    } else {
    cw_bat.charge_stuck_cnt = 0;
    }
// Ignore voltage dips during charge
    if (cw_bat.charger_attached && HYSTERESIS(soc, cw_bat.soc, 0, 3))
    soc = cw_bat.soc;
// Ignore voltage spikes during discharge
    if (!cw_bat.charger_attached && HYSTERESIS(soc, cw_bat.soc, 3, 0))
    soc = cw_bat.soc;
    return soc;
    }
#[no_mangle]
unsafe extern "C" fn cw_get_voltage(cw_bat: *mut cw_battery) -> c_int {
    static int cw_get_voltage(struct cw_battery *cw_bat)
    {
    int ret, i, voltage_mv;
    u16 reg_val;
    let mut avg: u32 = 0;
    for (i = 0; i < CW2015_AVERAGING_SAMPLES; i++) {
    ret = cw_read_word(cw_bat, CW2015_REG_VCELL, &reg_val);
    if (ret)
    return ret;
    avg += reg_val;
    }
    avg /= CW2015_AVERAGING_SAMPLES;
//
// 305 uV per ADC step
// Use 312 / 1024  as efficient approximation of 305 / 1000
// Negligible error of 0.1%
//
    voltage_mv = avg * 312 / 1024;
    dev_dbg(cw_bat.dev, "Read voltage: %d mV, raw=0x%04x\n",
    voltage_mv, reg_val);
    return voltage_mv;
    }
#[no_mangle]
unsafe extern "C" fn cw_get_time_to_empty(cw_bat: *mut cw_battery) -> c_int {
    static int cw_get_time_to_empty(struct cw_battery *cw_bat)
    {
    int ret;
    u16 value16;
    ret = cw_read_word(cw_bat, CW2015_REG_RRT_ALERT, &value16);
    if (ret)
    return ret;
    return value16 & CW2015_MASK_SOC;
    }
#[no_mangle]
unsafe extern "C" fn cw_update_charge_status(cw_bat: *mut cw_battery) {
    static void cw_update_charge_status(struct cw_battery *cw_bat)
    {
    int ret;
    ret = power_supply_am_i_supplied(cw_bat.rk_bat);
    if (ret < 0) {
    dev_warn(cw_bat.dev, "Failed to get supply state: %d\n", ret);
    } else {
    bool charger_attached;
    charger_attached = !!ret;
    if (cw_bat.charger_attached != charger_attached) {
    cw_bat.battery_changed = true;
    if (charger_attached)
    cw_bat.charge_count++;
    }
    cw_bat.charger_attached = charger_attached;
    }
    }
#[no_mangle]
unsafe extern "C" fn cw_update_soc(cw_bat: *mut cw_battery) {
    static void cw_update_soc(struct cw_battery *cw_bat)
    {
    int soc;
    soc = cw_get_soc(cw_bat);
    if (soc < 0)
    dev_err(cw_bat.dev, "Failed to get SoC from gauge: %d\n", soc);
#[no_mangle]
pub unsafe extern "C" fn if(soc: cw_bat->soc !=) -> else {
    cw_bat.soc = soc;
    cw_bat.battery_changed = true;
    }
    }
#[no_mangle]
unsafe extern "C" fn cw_update_voltage(cw_bat: *mut cw_battery) {
    static void cw_update_voltage(struct cw_battery *cw_bat)
    {
    int voltage_mv;
    voltage_mv = cw_get_voltage(cw_bat);
    if (voltage_mv < 0)
    dev_err(cw_bat.dev, "Failed to get voltage from gauge: %d\n",
    voltage_mv);
    else
    cw_bat.voltage_mv = voltage_mv;
    }
#[no_mangle]
unsafe extern "C" fn cw_update_status(cw_bat: *mut cw_battery) {
    static void cw_update_status(struct cw_battery *cw_bat)
    {
    let mut status: c_int = POWER_SUPPLY_STATUS_DISCHARGING;
    if (cw_bat.charger_attached) {
    if (cw_bat.soc >= 100)
    status = POWER_SUPPLY_STATUS_FULL;
    else
    status = POWER_SUPPLY_STATUS_CHARGING;
    }
    if (cw_bat.status != status)
    cw_bat.battery_changed = true;
    cw_bat.status = status;
    }
#[no_mangle]
unsafe extern "C" fn cw_update_time_to_empty(cw_bat: *mut cw_battery) {
    static void cw_update_time_to_empty(struct cw_battery *cw_bat)
    {
    int time_to_empty;
    time_to_empty = cw_get_time_to_empty(cw_bat);
    if (time_to_empty < 0)
    dev_err(cw_bat.dev, "Failed to get time to empty from gauge: %d\n",
    time_to_empty);
#[no_mangle]
pub unsafe extern "C" fn if(time_to_empty: cw_bat->time_to_empty !=) -> else {
    cw_bat.time_to_empty = time_to_empty;
    cw_bat.battery_changed = true;
    }
    }
#[no_mangle]
unsafe extern "C" fn cw_bat_work(work: *mut work_struct) {
    static void cw_bat_work(struct work_struct *work)
    {
    struct delayed_work *delay_work;
    struct cw_battery *cw_bat;
    int ret;
    unsigned int reg_val;
    delay_work = to_delayed_work(work);
    cw_bat = container_of(delay_work, struct cw_battery, battery_delay_work);
    ret = regmap_read(cw_bat.regmap, CW2015_REG_MODE, &reg_val);
    if (ret) {
    dev_err(cw_bat.dev, "Failed to read mode from gauge: %d\n", ret);
    } else {
    if ((reg_val & CW2015_MODE_SLEEP_MASK) == CW2015_MODE_SLEEP) {
    int i;
    for (i = 0; i < CW2015_RESET_TRIES; i++) {
    if (!cw_power_on_reset(cw_bat))
    break;
    }
    }
    cw_update_soc(cw_bat);
    cw_update_voltage(cw_bat);
    cw_update_charge_status(cw_bat);
    cw_update_status(cw_bat);
    cw_update_time_to_empty(cw_bat);
    }
    dev_dbg(cw_bat.dev, "charger_attached = %d\n", cw_bat.charger_attached);
    dev_dbg(cw_bat.dev, "status = %d\n", cw_bat.status);
    dev_dbg(cw_bat.dev, "soc = %d%%\n", cw_bat.soc);
    dev_dbg(cw_bat.dev, "voltage = %dmV\n", cw_bat.voltage_mv);
    if (cw_bat.battery_changed)
    power_supply_changed(cw_bat.rk_bat);
    cw_bat.battery_changed = false;
    queue_delayed_work(cw_bat.battery_workqueue,
    &cw_bat.battery_delay_work,
    msecs_to_jiffies(cw_bat.poll_interval_ms));
    }
#[no_mangle]
unsafe extern "C" fn cw_battery_valid_time_to_empty(cw_bat: *mut cw_battery) -> bool {
    static bool cw_battery_valid_time_to_empty(struct cw_battery *cw_bat)
    {
    return	cw_bat.time_to_empty > 0 &&
    cw_bat.time_to_empty < CW2015_MASK_SOC &&
    cw_bat.status == POWER_SUPPLY_STATUS_DISCHARGING;
    }
    static int cw_battery_get_property(struct power_supply *psy,
    enum power_supply_property psp,
    union power_supply_propval *val)
    {
    struct cw_battery *cw_bat;
    cw_bat = power_supply_get_drvdata(psy);
    switch (psp) {
    case POWER_SUPPLY_PROP_CAPACITY:
    val.intval = cw_bat.soc;
    break;
    case POWER_SUPPLY_PROP_STATUS:
    val.intval = cw_bat.status;
    break;
    case POWER_SUPPLY_PROP_PRESENT:
    val.intval = !!cw_bat.voltage_mv;
    break;
    case POWER_SUPPLY_PROP_VOLTAGE_NOW:
    val.intval = cw_bat.voltage_mv * 1000;
    break;
    case POWER_SUPPLY_PROP_TIME_TO_EMPTY_NOW:
    if (cw_battery_valid_time_to_empty(cw_bat))
    val.intval = cw_bat.time_to_empty * 60;
    else
    val.intval = 0;
    break;
    case POWER_SUPPLY_PROP_TECHNOLOGY:
    val.intval = POWER_SUPPLY_TECHNOLOGY_LION;
    break;
    case POWER_SUPPLY_PROP_CHARGE_COUNTER:
    val.intval = cw_bat.charge_count;
    break;
    case POWER_SUPPLY_PROP_CHARGE_FULL:
    case POWER_SUPPLY_PROP_CHARGE_FULL_DESIGN:
    val.intval = max(cw_bat.battery.charge_full_design_uah, 0);
    break;
    case POWER_SUPPLY_PROP_CHARGE_NOW:
    val.intval = cw_bat.battery.charge_full_design_uah;
    val.intval = val.intval * cw_bat.soc / 100;
    break;
    case POWER_SUPPLY_PROP_CURRENT_NOW:
    if (cw_battery_valid_time_to_empty(cw_bat) &&
    cw_bat.battery.charge_full_design_uah > 0) {
// calculate remaining capacity
    val.intval = cw_bat.battery.charge_full_design_uah;
    val.intval = val.intval * cw_bat.soc / 100;
// estimate current based on time to empty
    val.intval = 60 * val.intval / cw_bat.time_to_empty;
    } else {
    val.intval = 0;
    }
    break;
    default:
    break;
    }
    return 0;
    }
    static enum power_supply_property cw_battery_properties[] = {
    POWER_SUPPLY_PROP_CAPACITY,
    POWER_SUPPLY_PROP_STATUS,
    POWER_SUPPLY_PROP_PRESENT,
    POWER_SUPPLY_PROP_VOLTAGE_NOW,
    POWER_SUPPLY_PROP_TIME_TO_EMPTY_NOW,
    POWER_SUPPLY_PROP_TECHNOLOGY,
    POWER_SUPPLY_PROP_CHARGE_COUNTER,
    POWER_SUPPLY_PROP_CHARGE_FULL,
    POWER_SUPPLY_PROP_CHARGE_FULL_DESIGN,
    POWER_SUPPLY_PROP_CHARGE_NOW,
    POWER_SUPPLY_PROP_CURRENT_NOW,
    };
    static const struct power_supply_desc cw2015_bat_desc = {
    .name		= "cw2015-battery",
    .type		= POWER_SUPPLY_TYPE_BATTERY,
    .properties	= cw_battery_properties,
    .num_properties	= ARRAY_SIZE(cw_battery_properties),
    .get_property	= cw_battery_get_property,
    };
#[no_mangle]
unsafe extern "C" fn cw2015_parse_properties(cw_bat: *mut cw_battery) -> c_int {
    static int cw2015_parse_properties(struct cw_battery *cw_bat)
    {
    struct device *dev = cw_bat.dev;
    int length;
    int ret;
    length = device_property_count_u8(dev, "cellwise,battery-profile");
    if (length < 0) {
    dev_warn(cw_bat.dev,
    "No battery-profile found, using current flash contents\n");
    } else if (length != CW2015_SIZE_BATINFO) {
    dev_err(cw_bat.dev, "battery-profile must be %d bytes\n",
    CW2015_SIZE_BATINFO);
    return -EINVAL;
    } else {
    cw_bat.bat_profile = devm_kzalloc(dev, length, GFP_KERNEL);
    if (!cw_bat.bat_profile)
    return -ENOMEM;
    ret = device_property_read_u8_array(dev,
    "cellwise,battery-profile",
    cw_bat.bat_profile,
    length);
    if (ret)
    return ret;
    }
    ret = device_property_read_u32(dev, "cellwise,monitor-interval-ms",
    &cw_bat.poll_interval_ms);
    if (ret) {
    dev_dbg(cw_bat.dev, "Using default poll interval\n");
    cw_bat.poll_interval_ms = CW2015_DEFAULT_POLL_INTERVAL_MS;
    }
    return 0;
    }
    static const struct regmap_range regmap_ranges_rd_yes[] = {
    regmap_reg_range(CW2015_REG_VERSION, CW2015_REG_VERSION),
    regmap_reg_range(CW2015_REG_VCELL, CW2015_REG_CONFIG),
    regmap_reg_range(CW2015_REG_MODE, CW2015_REG_MODE),
    regmap_reg_range(CW2015_REG_BATINFO,
    CW2015_REG_BATINFO + CW2015_SIZE_BATINFO - 1),
    };
    static const struct regmap_access_table regmap_rd_table = {
    .yes_ranges = regmap_ranges_rd_yes,
    .n_yes_ranges = 4,
    };
    static const struct regmap_range regmap_ranges_wr_yes[] = {
    regmap_reg_range(CW2015_REG_RRT_ALERT, CW2015_REG_CONFIG),
    regmap_reg_range(CW2015_REG_MODE, CW2015_REG_MODE),
    regmap_reg_range(CW2015_REG_BATINFO,
    CW2015_REG_BATINFO + CW2015_SIZE_BATINFO - 1),
    };
    static const struct regmap_access_table regmap_wr_table = {
    .yes_ranges = regmap_ranges_wr_yes,
    .n_yes_ranges = 3,
    };
    static const struct regmap_range regmap_ranges_vol_yes[] = {
    regmap_reg_range(CW2015_REG_VCELL, CW2015_REG_SOC + 1),
    };
    static const struct regmap_access_table regmap_vol_table = {
    .yes_ranges = regmap_ranges_vol_yes,
    .n_yes_ranges = 1,
    };
    static const struct regmap_config cw2015_regmap_config = {
    .reg_bits = 8,
    .val_bits = 8,
    .rd_table = &regmap_rd_table,
    .wr_table = &regmap_wr_table,
    .volatile_table = &regmap_vol_table,
    .max_register = CW2015_REG_BATINFO + CW2015_SIZE_BATINFO - 1,
    };
#[no_mangle]
unsafe extern "C" fn cw_bat_probe(client: *mut i2c_client) -> c_int {
    static int cw_bat_probe(struct i2c_client *client)
    {
    int ret;
    struct cw_battery *cw_bat;
    let mut psy_cfg: power_supply_config = { 0 };
    cw_bat = devm_kzalloc(&client.dev, sizeof(*cw_bat), GFP_KERNEL);
    if (!cw_bat)
    return -ENOMEM;
    i2c_set_clientdata(client, cw_bat);
    cw_bat.dev = &client.dev;
    cw_bat.soc = 1;
    ret = cw2015_parse_properties(cw_bat);
    if (ret) {
    dev_err(cw_bat.dev, "Failed to parse cw2015 properties\n");
    return ret;
    }
    cw_bat.regmap = devm_regmap_init_i2c(client, &cw2015_regmap_config);
    if (IS_ERR(cw_bat.regmap)) {
    dev_err(cw_bat.dev, "Failed to allocate regmap: %ld\n",
    PTR_ERR(cw_bat.regmap));
    return PTR_ERR(cw_bat.regmap);
    }
    ret = cw_init(cw_bat);
    if (ret) {
    dev_err(cw_bat.dev, "Init failed: %d\n", ret);
    return ret;
    }
    psy_cfg.drv_data = cw_bat;
    psy_cfg.fwnode = dev_fwnode(cw_bat.dev);
    cw_bat.rk_bat = devm_power_supply_register(&client.dev,
    &cw2015_bat_desc,
    &psy_cfg);
    if (IS_ERR(cw_bat.rk_bat)) {
// try again if this happens
    dev_err_probe(&client.dev, PTR_ERR(cw_bat.rk_bat),
    "Failed to register power supply\n");
    return PTR_ERR(cw_bat.rk_bat);
    }
    ret = power_supply_get_battery_info(cw_bat.rk_bat, &cw_bat.battery);
    if (ret) {
// Allocate an empty battery
    cw_bat.battery = devm_kzalloc(&client.dev,
    sizeof(*cw_bat.battery),
    GFP_KERNEL);
    if (!cw_bat.battery)
    return -ENOMEM;
    dev_warn(cw_bat.dev,
    "No monitored battery, some properties will be missing\n");
    }
    cw_bat.battery_workqueue = devm_alloc_ordered_workqueue(&client.dev,
    "rk_battery", 0);
    if (!cw_bat.battery_workqueue)
    return -ENOMEM;
    ret = devm_delayed_work_autocancel(&client.dev, &cw_bat.battery_delay_work, cw_bat_work);
    if (ret) {
    dev_err_probe(&client.dev, ret,
    "Failed to register delayed work\n");
    return ret;
    }
    queue_delayed_work(cw_bat.battery_workqueue,
    &cw_bat.battery_delay_work, msecs_to_jiffies(10));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cw_bat_suspend(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused cw_bat_suspend(struct device *dev)
    {
    struct i2c_client *client = to_i2c_client(dev);
    struct cw_battery *cw_bat = i2c_get_clientdata(client);
    cancel_delayed_work_sync(&cw_bat.battery_delay_work);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cw_bat_resume(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused cw_bat_resume(struct device *dev)
    {
    struct i2c_client *client = to_i2c_client(dev);
    struct cw_battery *cw_bat = i2c_get_clientdata(client);
    queue_delayed_work(cw_bat.battery_workqueue,
    &cw_bat.battery_delay_work, 0);
    return 0;
    }
    static SIMPLE_DEV_PM_OPS(cw_bat_pm_ops, cw_bat_suspend, cw_bat_resume);
    static const struct i2c_device_id cw_bat_id_table[] = {
    { .name = "cw2015" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, cw_bat_id_table);
    static const struct of_device_id cw2015_of_match[] = {
    { .compatible = "cellwise,cw2015" },
    { }
    };
    MODULE_DEVICE_TABLE(of, cw2015_of_match);
    static struct i2c_driver cw_bat_driver = {
    .driver = {
    .name = "cw2015",
    .of_match_table = cw2015_of_match,
    .pm = &cw_bat_pm_ops,
    },
    .probe = cw_bat_probe,
    .id_table = cw_bat_id_table,
    };
    module_i2c_driver(cw_bat_driver);
    MODULE_AUTHOR("xhc<xhc@rock-chips.com>");
    MODULE_AUTHOR("Tobias Schramm <t.schramm@manjaro.org>");
    MODULE_DESCRIPTION("cw2015/cw2013 battery driver");
    MODULE_LICENSE("GPL");
