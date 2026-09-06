//! Automatically rewritten from C to Rust
//! Source: drivers/power/supply/acer_a500_battery.c
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
// Battery driver for Acer Iconia Tab A500.
//
// Copyright 2020 GRATE-driver project.
//
// Based on downstream driver from Acer Inc.
// Based on NVIDIA Gas Gauge driver for SBS Compliant Batteries.
//
// Copyright (c) 2010, NVIDIA Corporation.
//

    enum {
    REG_CAPACITY,
    REG_VOLTAGE,
    REG_CURRENT,
    REG_DESIGN_CAPACITY,
    REG_TEMPERATURE,
    };

    .psp = POWER_SUPPLY_PROP_ ## _psp,	\
    .reg = _reg,				\
    }
    static const struct battery_register {
    enum power_supply_property psp;
    unsigned int reg;
    } ec_data[] = {
    [REG_CAPACITY]		= EC_DATA(0x00, CAPACITY),
    [REG_VOLTAGE]		= EC_DATA(0x01, VOLTAGE_NOW),
    [REG_CURRENT]		= EC_DATA(0x03, CURRENT_NOW),
    [REG_DESIGN_CAPACITY]	= EC_DATA(0x08, CHARGE_FULL_DESIGN),
    [REG_TEMPERATURE]	= EC_DATA(0x0a, TEMP),
    };
    static const enum power_supply_property a500_battery_properties[] = {
    POWER_SUPPLY_PROP_CAPACITY,
    POWER_SUPPLY_PROP_CHARGE_FULL_DESIGN,
    POWER_SUPPLY_PROP_CURRENT_NOW,
    POWER_SUPPLY_PROP_PRESENT,
    POWER_SUPPLY_PROP_STATUS,
    POWER_SUPPLY_PROP_TECHNOLOGY,
    POWER_SUPPLY_PROP_TEMP,
    POWER_SUPPLY_PROP_VOLTAGE_NOW,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct a500_battery {
    pub poll_work: delayed_work,
    pub psy: *mut power_supply,
    pub regmap: *mut regmap,
    pub capacity: c_uint,
}

#[no_mangle]
unsafe extern "C" fn a500_battery_update_capacity(bat: *mut a500_battery) -> bool {
    static bool a500_battery_update_capacity(struct a500_battery *bat)
    {
    unsigned int capacity;
    int err;
    err = regmap_read(bat.regmap, ec_data[REG_CAPACITY].reg, &capacity);
    if (err)
    return false;
// capacity can be >100% even if max value is 100%
    capacity = min(capacity, 100u);
    if (bat.capacity != capacity) {
    bat.capacity = capacity;
    return true;
    }
    return false;
    }
#[no_mangle]
unsafe extern "C" fn a500_battery_get_status(bat: *mut a500_battery) -> c_int {
    static int a500_battery_get_status(struct a500_battery *bat)
    {
    if (bat.capacity < 100) {
    if (power_supply_am_i_supplied(bat.psy))
    return POWER_SUPPLY_STATUS_CHARGING;
    else
    return POWER_SUPPLY_STATUS_DISCHARGING;
    }
    return POWER_SUPPLY_STATUS_FULL;
    }
    static void a500_battery_unit_adjustment(struct device *dev,
    enum power_supply_property psp,
    union power_supply_propval *val)
    {
    let mut base_unit_conversion: c_uint = 1000;
    let mut temp_kelvin_to_celsius: c_uint = 2731;
    switch (psp) {
    case POWER_SUPPLY_PROP_CHARGE_FULL_DESIGN:
    case POWER_SUPPLY_PROP_CURRENT_NOW:
    case POWER_SUPPLY_PROP_VOLTAGE_NOW:
    val.intval *= base_unit_conversion;
    break;
    case POWER_SUPPLY_PROP_TEMP:
    val.intval -= temp_kelvin_to_celsius;
    break;
    case POWER_SUPPLY_PROP_PRESENT:
    val.intval = !!val.intval;
    break;
    default:
    dev_dbg(dev,
    "%s: no need for unit conversion %d\n", __func__, psp);
    }
    }
    static int a500_battery_get_ec_data_index(struct device *dev,
    enum power_supply_property psp)
    {
    unsigned int i;
//
// DESIGN_CAPACITY register always returns a non-zero value if
// battery is connected and zero if disconnected, hence we'll use
// it for judging the battery presence.
//
    if (psp == POWER_SUPPLY_PROP_PRESENT)
    psp = POWER_SUPPLY_PROP_CHARGE_FULL_DESIGN;
    for (i = 0; i < ARRAY_SIZE(ec_data); i++)
    if (psp == ec_data[i].psp)
    return i;
    dev_dbg(dev, "%s: invalid property %u\n", __func__, psp);
    return -EINVAL;
    }
    static int a500_battery_get_property(struct power_supply *psy,
    enum power_supply_property psp,
    union power_supply_propval *val)
    {
    struct a500_battery *bat = power_supply_get_drvdata(psy);
    struct device *dev = psy.dev.parent;
    let mut ret: c_int = 0;
    switch (psp) {
    case POWER_SUPPLY_PROP_STATUS:
    val.intval = a500_battery_get_status(bat);
    break;
    case POWER_SUPPLY_PROP_TECHNOLOGY:
    val.intval = POWER_SUPPLY_TECHNOLOGY_LION;
    break;
    case POWER_SUPPLY_PROP_CAPACITY:
    a500_battery_update_capacity(bat);
    val.intval = bat.capacity;
    break;
    case POWER_SUPPLY_PROP_CHARGE_FULL_DESIGN:
    case POWER_SUPPLY_PROP_CURRENT_NOW:
    case POWER_SUPPLY_PROP_VOLTAGE_NOW:
    case POWER_SUPPLY_PROP_PRESENT:
    case POWER_SUPPLY_PROP_TEMP:
    ret = a500_battery_get_ec_data_index(dev, psp);
    if (ret < 0)
    break;
    ret = regmap_read(bat.regmap, ec_data[ret].reg, &val.intval);
    break;
    default:
    dev_err(dev, "%s: invalid property %u\n", __func__, psp);
    return -EINVAL;
    }
    if (!ret) {
// convert units to match requirements of power supply class
    a500_battery_unit_adjustment(dev, psp, val);
    }
    dev_dbg(dev, "%s: property = %d, value = %x\n",
    __func__, psp, val.intval);
// return NODATA for properties if battery not presents
    if (ret)
    return -ENODATA;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn a500_battery_poll_work(work: *mut work_struct) {
    static void a500_battery_poll_work(struct work_struct *work)
    {
    struct a500_battery *bat;
    bool capacity_changed;
    bat = container_of(work, struct a500_battery, poll_work.work);
    capacity_changed = a500_battery_update_capacity(bat);
    if (capacity_changed)
    power_supply_changed(bat.psy);
// continuously send uevent notification
    schedule_delayed_work(&bat.poll_work, 30 * HZ);
    }
    static const struct power_supply_desc a500_battery_desc = {
    .name = "ec-battery",
    .type = POWER_SUPPLY_TYPE_BATTERY,
    .properties = a500_battery_properties,
    .get_property = a500_battery_get_property,
    .num_properties = ARRAY_SIZE(a500_battery_properties),
    .external_power_changed = power_supply_changed,
    };
#[no_mangle]
unsafe extern "C" fn a500_battery_probe(pdev: *mut platform_device) -> c_int {
    static int a500_battery_probe(struct platform_device *pdev)
    {
    let mut psy_cfg: power_supply_config = {};
    struct a500_battery *bat;
    bat = devm_kzalloc(&pdev.dev, sizeof(*bat), GFP_KERNEL);
    if (!bat)
    return -ENOMEM;
    platform_set_drvdata(pdev, bat);
    psy_cfg.fwnode = dev_fwnode(pdev.dev.parent);
    psy_cfg.drv_data = bat;
    psy_cfg.no_wakeup_source = true;
    bat.regmap = dev_get_regmap(pdev.dev.parent, "KB930");
    if (!bat.regmap)
    return -EINVAL;
    bat.psy = devm_power_supply_register(&pdev.dev,
    &a500_battery_desc,
    &psy_cfg);
    if (IS_ERR(bat.psy))
    return dev_err_probe(&pdev.dev, PTR_ERR(bat.psy),
    "failed to register battery\n");
    INIT_DELAYED_WORK(&bat.poll_work, a500_battery_poll_work);
    schedule_delayed_work(&bat.poll_work, HZ);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn a500_battery_remove(pdev: *mut platform_device) {
    static void a500_battery_remove(struct platform_device *pdev)
    {
    struct a500_battery *bat = dev_get_drvdata(&pdev.dev);
    cancel_delayed_work_sync(&bat.poll_work);
    }
#[no_mangle]
unsafe extern "C" fn a500_battery_suspend(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused a500_battery_suspend(struct device *dev)
    {
    struct a500_battery *bat = dev_get_drvdata(dev);
    cancel_delayed_work_sync(&bat.poll_work);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn a500_battery_resume(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused a500_battery_resume(struct device *dev)
    {
    struct a500_battery *bat = dev_get_drvdata(dev);
    schedule_delayed_work(&bat.poll_work, HZ);
    return 0;
    }
    static SIMPLE_DEV_PM_OPS(a500_battery_pm_ops,
    a500_battery_suspend, a500_battery_resume);
    static struct platform_driver a500_battery_driver = {
    .driver = {
    .name = "acer-a500-iconia-battery",
    .pm = &a500_battery_pm_ops,
    },
    .probe = a500_battery_probe,
    .remove = a500_battery_remove,
    };
    module_platform_driver(a500_battery_driver);
    MODULE_DESCRIPTION("Battery gauge driver for Acer Iconia Tab A500");
    MODULE_AUTHOR("Dmitry Osipenko <digetx@gmail.com>");
    MODULE_ALIAS("platform:acer-a500-iconia-battery");
    MODULE_LICENSE("GPL");
