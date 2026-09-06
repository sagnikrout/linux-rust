//! Automatically rewritten from C to Rust
//! Source: drivers/power/supply/pmu_battery.c
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
// Battery class driver for Apple PMU
//
// Copyright © 2006  David Woodhouse <dwmw2@infradead.org>
//

    static struct pmu_battery_dev {
    struct power_supply *bat;
    struct power_supply_desc bat_desc;
    struct pmu_battery_info *pbi;
    char name[16];
    int propval;
    } *pbats[PMU_MAX_BATTERIES];

//
// Power
//
    static int pmu_get_ac_prop(struct power_supply *psy,
    enum power_supply_property psp,
    union power_supply_propval *val)
    {
    switch (psp) {
    case POWER_SUPPLY_PROP_ONLINE:
    val.intval = (!!(pmu_power_flags & PMU_PWR_AC_PRESENT)) ||
    (pmu_battery_count == 0);
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
    static enum power_supply_property pmu_ac_props[] = {
    POWER_SUPPLY_PROP_ONLINE,
    };
    static const struct power_supply_desc pmu_ac_desc = {
    .name = "pmu-ac",
    .type = POWER_SUPPLY_TYPE_MAINS,
    .properties = pmu_ac_props,
    .num_properties = ARRAY_SIZE(pmu_ac_props),
    .get_property = pmu_get_ac_prop,
    };
    static struct power_supply *pmu_ac;
//
// Battery properties
//
    static char *pmu_batt_types[] = {
    "Smart", "Comet", "Hooper", "Unknown"
    };
    static char *pmu_bat_get_model_name(struct pmu_battery_info *pbi)
    {
    switch (pbi.flags & PMU_BATT_TYPE_MASK) {
    case PMU_BATT_TYPE_SMART:
    return pmu_batt_types[0];
    case PMU_BATT_TYPE_COMET:
    return pmu_batt_types[1];
    case PMU_BATT_TYPE_HOOPER:
    return pmu_batt_types[2];
    default: break;
    }
    return pmu_batt_types[3];
    }
    static int pmu_bat_get_property(struct power_supply *psy,
    enum power_supply_property psp,
    union power_supply_propval *val)
    {
    struct pmu_battery_dev *pbat = to_pmu_battery_dev(psy);
    struct pmu_battery_info *pbi = pbat.pbi;
    switch (psp) {
    case POWER_SUPPLY_PROP_STATUS:
    if (pbi.flags & PMU_BATT_CHARGING)
    val.intval = POWER_SUPPLY_STATUS_CHARGING;
#[no_mangle]
pub unsafe extern "C" fn if(PMU_PWR_AC_PRESENT: pmu_power_flags &) -> else {
    else if (pmu_power_flags & PMU_PWR_AC_PRESENT)
    val.intval = POWER_SUPPLY_STATUS_FULL;
    else
    val.intval = POWER_SUPPLY_STATUS_DISCHARGING;
    break;
    case POWER_SUPPLY_PROP_PRESENT:
    val.intval = !!(pbi.flags & PMU_BATT_PRESENT);
    break;
    case POWER_SUPPLY_PROP_MODEL_NAME:
    val.strval = pmu_bat_get_model_name(pbi);
    break;
    case POWER_SUPPLY_PROP_ENERGY_AVG:
    val.intval = pbi.charge     * 1000; /* mWh . µWh */
    break;
    case POWER_SUPPLY_PROP_ENERGY_FULL:
    val.intval = pbi.max_charge * 1000; /* mWh . µWh */
    break;
    case POWER_SUPPLY_PROP_CURRENT_AVG:
    val.intval = pbi.amperage   * 1000; /* mA . µA */
    break;
    case POWER_SUPPLY_PROP_VOLTAGE_AVG:
    val.intval = pbi.voltage    * 1000; /* mV . µV */
    break;
    case POWER_SUPPLY_PROP_TIME_TO_EMPTY_AVG:
    val.intval = pbi.time_remaining;
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
    static enum power_supply_property pmu_bat_props[] = {
    POWER_SUPPLY_PROP_STATUS,
    POWER_SUPPLY_PROP_PRESENT,
    POWER_SUPPLY_PROP_MODEL_NAME,
    POWER_SUPPLY_PROP_ENERGY_AVG,
    POWER_SUPPLY_PROP_ENERGY_FULL,
    POWER_SUPPLY_PROP_CURRENT_AVG,
    POWER_SUPPLY_PROP_VOLTAGE_AVG,
    POWER_SUPPLY_PROP_TIME_TO_EMPTY_AVG,
    };
//
// Initialisation
//
    static struct platform_device *bat_pdev;
#[no_mangle]
unsafe extern "C" fn pmu_bat_init() -> int __init {
    static int __init pmu_bat_init(void)
    {
    let mut ret: c_int = 0;
    int i;
    bat_pdev = platform_device_register_simple("pmu-battery",
    0, core::ptr::null_mut(), 0);
    if (IS_ERR(bat_pdev)) {
    ret = PTR_ERR(bat_pdev);
    goto pdev_register_failed;
    }
    pmu_ac = power_supply_register(&bat_pdev.dev, &pmu_ac_desc, core::ptr::null_mut());
    if (IS_ERR(pmu_ac)) {
    ret = PTR_ERR(pmu_ac);
    goto ac_register_failed;
    }
    for (i = 0; i < pmu_battery_count; i++) {
    let mut psy_cfg: power_supply_config = {};
    struct pmu_battery_dev *pbat = kzalloc_obj(*pbat);
    if (!pbat)
    break;
    sprintf(pbat.name, "PMU_battery_%d", i);
    pbat.bat_desc.name = pbat.name;
    pbat.bat_desc.properties = pmu_bat_props;
    pbat.bat_desc.num_properties = ARRAY_SIZE(pmu_bat_props);
    pbat.bat_desc.get_property = pmu_bat_get_property;
    pbat.bat_desc.type = POWER_SUPPLY_TYPE_BATTERY;
    pbat.pbi = &pmu_batteries[i];
    psy_cfg.drv_data = pbat;
    pbat.bat = power_supply_register(&bat_pdev.dev,
    &pbat.bat_desc,
    &psy_cfg);
    if (IS_ERR(pbat.bat)) {
    ret = PTR_ERR(pbat.bat);
    kfree(pbat);
    goto battery_register_failed;
    }
    pbats[i] = pbat;
    }
    goto success;
    battery_register_failed:
    while (i--) {
    if (!pbats[i])
    continue;
    power_supply_unregister(pbats[i].bat);
    kfree(pbats[i]);
    }
    power_supply_unregister(pmu_ac);
    ac_register_failed:
    platform_device_unregister(bat_pdev);
    pdev_register_failed:
    success:
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn pmu_bat_exit() -> void __exit {
    static void __exit pmu_bat_exit(void)
    {
    int i;
    for (i = 0; i < PMU_MAX_BATTERIES; i++) {
    if (!pbats[i])
    continue;
    power_supply_unregister(pbats[i].bat);
    kfree(pbats[i]);
    }
    power_supply_unregister(pmu_ac);
    platform_device_unregister(bat_pdev);
    }
    module_init(pmu_bat_init);
    module_exit(pmu_bat_exit);
    MODULE_AUTHOR("David Woodhouse <dwmw2@infradead.org>");
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("PMU battery driver");
