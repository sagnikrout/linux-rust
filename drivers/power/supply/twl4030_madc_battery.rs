//! Automatically rewritten from C to Rust
//! Source: drivers/power/supply/twl4030_madc_battery.c
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
// Dumb driver for LiIon batteries using TWL4030 madc.
//
// Copyright 2013 Golden Delicious Computers
// Lukas Märdian <lukas@goldelico.com>
//
// Based on dumb driver for gta01 battery
// Copyright 2009 Openmoko, Inc
// Balaji Rao <balajirrao@openmoko.org>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct twl4030_madc_battery {
    pub psy: *mut power_supply,
    pub pdata: *mut twl4030_madc_bat_platform_data,
    pub channel_temp: *mut iio_channel,
    pub channel_ichg: *mut iio_channel,
    pub channel_vbat: *mut iio_channel,
}

    static enum power_supply_property twl4030_madc_bat_props[] = {
    POWER_SUPPLY_PROP_PRESENT,
    POWER_SUPPLY_PROP_STATUS,
    POWER_SUPPLY_PROP_TECHNOLOGY,
    POWER_SUPPLY_PROP_VOLTAGE_NOW,
    POWER_SUPPLY_PROP_CURRENT_NOW,
    POWER_SUPPLY_PROP_CAPACITY,
    POWER_SUPPLY_PROP_CHARGE_FULL,
    POWER_SUPPLY_PROP_CHARGE_NOW,
    POWER_SUPPLY_PROP_TEMP,
    POWER_SUPPLY_PROP_TIME_TO_EMPTY_NOW,
    };
#[no_mangle]
unsafe extern "C" fn madc_read(channel: *mut iio_channel) -> c_int {
    static int madc_read(struct iio_channel *channel)
    {
    int val, err;
    err = iio_read_channel_processed(channel, &val);
    if (err < 0)
    return err;
    return val;
    }
#[no_mangle]
unsafe extern "C" fn twl4030_madc_bat_get_charging_status(bt: *mut twl4030_madc_battery) -> c_int {
    static int twl4030_madc_bat_get_charging_status(struct twl4030_madc_battery *bt)
    {
    return (madc_read(bt.channel_ichg) > 0) ? 1 : 0;
    }
#[no_mangle]
unsafe extern "C" fn twl4030_madc_bat_get_voltage(bt: *mut twl4030_madc_battery) -> c_int {
    static int twl4030_madc_bat_get_voltage(struct twl4030_madc_battery *bt)
    {
    return madc_read(bt.channel_vbat);
    }
#[no_mangle]
unsafe extern "C" fn twl4030_madc_bat_get_current(bt: *mut twl4030_madc_battery) -> c_int {
    static int twl4030_madc_bat_get_current(struct twl4030_madc_battery *bt)
    {
    return madc_read(bt.channel_ichg) * 1000;
    }
#[no_mangle]
unsafe extern "C" fn twl4030_madc_bat_get_temp(bt: *mut twl4030_madc_battery) -> c_int {
    static int twl4030_madc_bat_get_temp(struct twl4030_madc_battery *bt)
    {
    return madc_read(bt.channel_temp) * 10;
    }
    static int twl4030_madc_bat_voltscale(struct twl4030_madc_battery *bat,
    int volt)
    {
    struct twl4030_madc_bat_calibration *calibration;
    int i, res = 0;
// choose charging curve
    if (twl4030_madc_bat_get_charging_status(bat))
    calibration = bat.pdata.charging;
    else
    calibration = bat.pdata.discharging;
    if (volt > calibration[0].voltage) {
    res = calibration[0].level;
    } else {
    for (i = 0; calibration[i+1].voltage >= 0; i++) {
    if (volt <= calibration[i].voltage &&
    volt >= calibration[i+1].voltage) {
// interval found - interpolate within range
    res = calibration[i].level -
    ((calibration[i].voltage - volt) *
    (calibration[i].level -
    calibration[i+1].level)) /
    (calibration[i].voltage -
    calibration[i+1].voltage);
    break;
    }
    }
    }
    return res;
    }
    static int twl4030_madc_bat_get_property(struct power_supply *psy,
    enum power_supply_property psp,
    union power_supply_propval *val)
    {
    struct twl4030_madc_battery *bat = power_supply_get_drvdata(psy);
    switch (psp) {
    case POWER_SUPPLY_PROP_STATUS:
    if (twl4030_madc_bat_voltscale(bat,
    twl4030_madc_bat_get_voltage(bat)) > 95)
    val.intval = POWER_SUPPLY_STATUS_FULL;
    else {
    if (twl4030_madc_bat_get_charging_status(bat))
    val.intval = POWER_SUPPLY_STATUS_CHARGING;
    else
    val.intval = POWER_SUPPLY_STATUS_DISCHARGING;
    }
    break;
    case POWER_SUPPLY_PROP_VOLTAGE_NOW:
    val.intval = twl4030_madc_bat_get_voltage(bat) * 1000;
    break;
    case POWER_SUPPLY_PROP_TECHNOLOGY:
    val.intval = POWER_SUPPLY_TECHNOLOGY_LION;
    break;
    case POWER_SUPPLY_PROP_CURRENT_NOW:
    val.intval = twl4030_madc_bat_get_current(bat);
    break;
    case POWER_SUPPLY_PROP_PRESENT:
// assume battery is always present
    val.intval = 1;
    break;
    case POWER_SUPPLY_PROP_CHARGE_NOW: {
    int percent = twl4030_madc_bat_voltscale(bat,
    twl4030_madc_bat_get_voltage(bat));
    val.intval = (percent * bat.pdata.capacity) / 100;
    break;
    }
    case POWER_SUPPLY_PROP_CAPACITY:
    val.intval = twl4030_madc_bat_voltscale(bat,
    twl4030_madc_bat_get_voltage(bat));
    break;
    case POWER_SUPPLY_PROP_CHARGE_FULL:
    val.intval = bat.pdata.capacity;
    break;
    case POWER_SUPPLY_PROP_TEMP:
    val.intval = twl4030_madc_bat_get_temp(bat);
    break;
    case POWER_SUPPLY_PROP_TIME_TO_EMPTY_NOW: {
    int percent = twl4030_madc_bat_voltscale(bat,
    twl4030_madc_bat_get_voltage(bat));
// in mAh
    let mut chg: c_int = (percent * (bat.pdata.capacity/1000))/100;
// assume discharge with 400 mA (ca. 1.5W)
    val.intval = (3600l * chg) / 400;
    break;
    }
    default:
    return -EINVAL;
    }
    return 0;
    }
    static const struct power_supply_desc twl4030_madc_bat_desc = {
    .name			= "twl4030_battery",
    .type			= POWER_SUPPLY_TYPE_BATTERY,
    .properties		= twl4030_madc_bat_props,
    .num_properties		= ARRAY_SIZE(twl4030_madc_bat_props),
    .get_property		= twl4030_madc_bat_get_property,
    .external_power_changed	= power_supply_changed,
    };
#[no_mangle]
unsafe extern "C" fn twl4030_cmp(a: *const c_void, b: *const c_void) -> c_int {
    static int twl4030_cmp(const void *a, const void *b)
    {
    return ((struct twl4030_madc_bat_calibration *)b).voltage -
    ((struct twl4030_madc_bat_calibration *)a).voltage;
    }
#[no_mangle]
unsafe extern "C" fn twl4030_madc_battery_probe(pdev: *mut platform_device) -> c_int {
    static int twl4030_madc_battery_probe(struct platform_device *pdev)
    {
    struct twl4030_madc_battery *twl4030_madc_bat;
    struct twl4030_madc_bat_platform_data *pdata = pdev.dev.platform_data;
    let mut psy_cfg: power_supply_config = {};
    twl4030_madc_bat = devm_kzalloc(&pdev.dev, sizeof(*twl4030_madc_bat),
    GFP_KERNEL);
    if (!twl4030_madc_bat)
    return -ENOMEM;
    twl4030_madc_bat.channel_temp = devm_iio_channel_get(&pdev.dev, "temp");
    if (IS_ERR(twl4030_madc_bat.channel_temp))
    return PTR_ERR(twl4030_madc_bat.channel_temp);
    twl4030_madc_bat.channel_ichg = devm_iio_channel_get(&pdev.dev, "ichg");
    if (IS_ERR(twl4030_madc_bat.channel_ichg))
    return PTR_ERR(twl4030_madc_bat.channel_ichg);
    twl4030_madc_bat.channel_vbat = devm_iio_channel_get(&pdev.dev, "vbat");
    if (IS_ERR(twl4030_madc_bat.channel_vbat))
    return PTR_ERR(twl4030_madc_bat.channel_vbat);
// sort charging and discharging calibration data
    sort(pdata.charging, pdata.charging_size,
    sizeof(struct twl4030_madc_bat_calibration),
    twl4030_cmp, core::ptr::null_mut());
    sort(pdata.discharging, pdata.discharging_size,
    sizeof(struct twl4030_madc_bat_calibration),
    twl4030_cmp, core::ptr::null_mut());
    twl4030_madc_bat.pdata = pdata;
    psy_cfg.drv_data = twl4030_madc_bat;
    twl4030_madc_bat.psy = devm_power_supply_register(&pdev.dev,
    &twl4030_madc_bat_desc,
    &psy_cfg);
    if (IS_ERR(twl4030_madc_bat.psy))
    return PTR_ERR(twl4030_madc_bat.psy);
    return 0;
    }
    static struct platform_driver twl4030_madc_battery_driver = {
    .driver = {
    .name = "twl4030_madc_battery",
    },
    .probe  = twl4030_madc_battery_probe,
    };
    module_platform_driver(twl4030_madc_battery_driver);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Lukas Märdian <lukas@goldelico.com>");
    MODULE_DESCRIPTION("twl4030_madc battery driver");
    MODULE_ALIAS("platform:twl4030_madc_battery");
    MODULE_IMPORT_NS("IIO_CONSUMER");
