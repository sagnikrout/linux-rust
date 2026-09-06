//! Automatically rewritten from C to Rust
//! Source: drivers/acpi/sbs.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// sbs.c - ACPI Smart Battery System Driver ($Revision: 2.0 $)
//
// Copyright (c) 2007 Alexey Starikovskiy <astarikovskiy@suse.de>
// Copyright (c) 2005-2007 Vladimir Lebedev <vladimir.p.lebedev@intel.com>
// Copyright (c) 2005 Rich Townsend <rhdt@bartol.udel.edu>
//

pub const ACPI_SBS_NOTIFY_STATUS: c_uint = 0x80;
pub const ACPI_SBS_NOTIFY_INFO: c_uint = 0x81;
    MODULE_AUTHOR("Alexey Starikovskiy <astarikovskiy@suse.de>");
    MODULE_DESCRIPTION("Smart Battery System ACPI interface driver");
    MODULE_LICENSE("GPL");
    let mut cache_time: static unsigned int = 1000;
    module_param(cache_time, uint, 0644);
    MODULE_PARM_DESC(cache_time, "cache time in milliseconds");
pub const MAX_SBS_BAT: c_int = 4;
pub const ACPI_SBS_BLOCK_MAX: c_int = 32;
    static const struct acpi_device_id sbs_device_ids[] = {
    {"ACPI0002", 0},
    {"", 0},
    };
    MODULE_DEVICE_TABLE(acpi, sbs_device_ids);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_battery {
    pub bat: *mut power_supply,
    pub bat_desc: power_supply_desc,
    pub sbs: *mut acpi_sbs,
    pub update_time: c_ulong,
    pub name: [c_char; 8],
    pub manufacturer_name: [c_char; ACPI_SBS_BLOCK_MAX],
    pub device_name: [c_char; ACPI_SBS_BLOCK_MAX],
    pub device_chemistry: [c_char; ACPI_SBS_BLOCK_MAX],
    pub alarm_capacity: u16,
    pub full_charge_capacity: u16,
    pub design_capacity: u16,
    pub design_voltage: u16,
    pub serial_number: u16,
    pub cycle_count: u16,
    pub temp_now: u16,
    pub voltage_now: u16,
    pub rate_now: i16,
    pub rate_avg: i16,
    pub capacity_now: u16,
    pub state_of_charge: u16,
    pub state: u16,
    pub mode: u16,
    pub spec: u16,
    pub id: u8,
    pub present:1: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_sbs {
    pub charger: *mut power_supply,
    pub device: *mut acpi_device,
    pub hc: *mut acpi_smb_hc,
    pub lock: mutex,
    pub battery: [acpi_battery; MAX_SBS_BAT],
    pub batteries_supported:4: u8,
    pub manager_present:1: u8,
    pub charger_present:1: u8,
    pub charger_exists:1: u8,
}

    static void acpi_sbs_remove(struct platform_device *pdev);
    static int acpi_battery_get_state(struct acpi_battery *battery);
#[no_mangle]
pub unsafe extern "C" fn battery_scale(log: c_int) -> c_int {
    static inline int battery_scale(int log)
    {
    let mut scale: c_int = 1;
    while (log--)
    scale *= 10;
    return scale;
    }
#[no_mangle]
pub unsafe extern "C" fn acpi_battery_vscale(battery: *mut acpi_battery) -> c_int {
    static inline int acpi_battery_vscale(struct acpi_battery *battery)
    {
    return battery_scale((battery.spec & 0x0f00) >> 8);
    }
#[no_mangle]
pub unsafe extern "C" fn acpi_battery_ipscale(battery: *mut acpi_battery) -> c_int {
    static inline int acpi_battery_ipscale(struct acpi_battery *battery)
    {
    return battery_scale((battery.spec & 0xf000) >> 12);
    }
#[no_mangle]
pub unsafe extern "C" fn acpi_battery_mode(battery: *mut acpi_battery) -> c_int {
    static inline int acpi_battery_mode(struct acpi_battery *battery)
    {
    return (battery.mode & 0x8000);
    }
#[no_mangle]
pub unsafe extern "C" fn acpi_battery_scale(battery: *mut acpi_battery) -> c_int {
    static inline int acpi_battery_scale(struct acpi_battery *battery)
    {
    return (acpi_battery_mode(battery) ? 10 : 1) *
    acpi_battery_ipscale(battery);
    }
    static int sbs_get_ac_property(struct power_supply *psy,
    enum power_supply_property psp,
    union power_supply_propval *val)
    {
    struct acpi_sbs *sbs = to_acpi_sbs(psy);
    switch (psp) {
    case POWER_SUPPLY_PROP_ONLINE:
    val.intval = sbs.charger_present;
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn acpi_battery_technology(battery: *mut acpi_battery) -> c_int {
    static int acpi_battery_technology(struct acpi_battery *battery)
    {
    if (!strcasecmp("NiCd", battery.device_chemistry))
    return POWER_SUPPLY_TECHNOLOGY_NiCd;
    if (!strcasecmp("NiMH", battery.device_chemistry))
    return POWER_SUPPLY_TECHNOLOGY_NiMH;
    if (!strcasecmp("LION", battery.device_chemistry))
    return POWER_SUPPLY_TECHNOLOGY_LION;
    if (!strcasecmp("LiP", battery.device_chemistry))
    return POWER_SUPPLY_TECHNOLOGY_LIPO;
    return POWER_SUPPLY_TECHNOLOGY_UNKNOWN;
    }
    static int acpi_sbs_battery_get_property(struct power_supply *psy,
    enum power_supply_property psp,
    union power_supply_propval *val)
    {
    struct acpi_battery *battery = to_acpi_battery(psy);
    if ((!battery.present) && psp != POWER_SUPPLY_PROP_PRESENT)
    return -ENODEV;
    acpi_battery_get_state(battery);
    switch (psp) {
    case POWER_SUPPLY_PROP_STATUS:
    if (battery.rate_now < 0)
    val.intval = POWER_SUPPLY_STATUS_DISCHARGING;
#[no_mangle]
pub unsafe extern "C" fn if(0: battery->rate_now >) -> else {
    else if (battery.rate_now > 0)
    val.intval = POWER_SUPPLY_STATUS_CHARGING;
    else
    val.intval = POWER_SUPPLY_STATUS_FULL;
    break;
    case POWER_SUPPLY_PROP_PRESENT:
    val.intval = battery.present;
    break;
    case POWER_SUPPLY_PROP_TECHNOLOGY:
    val.intval = acpi_battery_technology(battery);
    break;
    case POWER_SUPPLY_PROP_CYCLE_COUNT:
    val.intval = battery.cycle_count;
    break;
    case POWER_SUPPLY_PROP_VOLTAGE_MIN_DESIGN:
    val.intval = battery.design_voltage *
    acpi_battery_vscale(battery) * 1000;
    break;
    case POWER_SUPPLY_PROP_VOLTAGE_NOW:
    val.intval = battery.voltage_now *
    acpi_battery_vscale(battery) * 1000;
    break;
    case POWER_SUPPLY_PROP_CURRENT_NOW:
    case POWER_SUPPLY_PROP_POWER_NOW:
    val.intval = abs(battery.rate_now) *
    acpi_battery_ipscale(battery) * 1000;
    val.intval *= (acpi_battery_mode(battery)) ?
    (battery.voltage_now *
    acpi_battery_vscale(battery) / 1000) : 1;
    break;
    case POWER_SUPPLY_PROP_CURRENT_AVG:
    case POWER_SUPPLY_PROP_POWER_AVG:
    val.intval = abs(battery.rate_avg) *
    acpi_battery_ipscale(battery) * 1000;
    val.intval *= (acpi_battery_mode(battery)) ?
    (battery.voltage_now *
    acpi_battery_vscale(battery) / 1000) : 1;
    break;
    case POWER_SUPPLY_PROP_CAPACITY:
    val.intval = battery.state_of_charge;
    break;
    case POWER_SUPPLY_PROP_CHARGE_FULL_DESIGN:
    case POWER_SUPPLY_PROP_ENERGY_FULL_DESIGN:
    val.intval = battery.design_capacity *
    acpi_battery_scale(battery) * 1000;
    break;
    case POWER_SUPPLY_PROP_CHARGE_FULL:
    case POWER_SUPPLY_PROP_ENERGY_FULL:
    val.intval = battery.full_charge_capacity *
    acpi_battery_scale(battery) * 1000;
    break;
    case POWER_SUPPLY_PROP_CHARGE_NOW:
    case POWER_SUPPLY_PROP_ENERGY_NOW:
    val.intval = battery.capacity_now *
    acpi_battery_scale(battery) * 1000;
    break;
    case POWER_SUPPLY_PROP_TEMP:
    val.intval = battery.temp_now - 2730;	// dK . dC
    break;
    case POWER_SUPPLY_PROP_MODEL_NAME:
    val.strval = battery.device_name;
    break;
    case POWER_SUPPLY_PROP_MANUFACTURER:
    val.strval = battery.manufacturer_name;
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
    static const enum power_supply_property sbs_ac_props[] = {
    POWER_SUPPLY_PROP_ONLINE,
    };
    static const enum power_supply_property sbs_charge_battery_props[] = {
    POWER_SUPPLY_PROP_STATUS,
    POWER_SUPPLY_PROP_PRESENT,
    POWER_SUPPLY_PROP_TECHNOLOGY,
    POWER_SUPPLY_PROP_CYCLE_COUNT,
    POWER_SUPPLY_PROP_VOLTAGE_MIN_DESIGN,
    POWER_SUPPLY_PROP_VOLTAGE_NOW,
    POWER_SUPPLY_PROP_CURRENT_NOW,
    POWER_SUPPLY_PROP_CURRENT_AVG,
    POWER_SUPPLY_PROP_CAPACITY,
    POWER_SUPPLY_PROP_CHARGE_FULL_DESIGN,
    POWER_SUPPLY_PROP_CHARGE_FULL,
    POWER_SUPPLY_PROP_CHARGE_NOW,
    POWER_SUPPLY_PROP_TEMP,
    POWER_SUPPLY_PROP_MODEL_NAME,
    POWER_SUPPLY_PROP_MANUFACTURER,
    };
    static const enum power_supply_property sbs_energy_battery_props[] = {
    POWER_SUPPLY_PROP_STATUS,
    POWER_SUPPLY_PROP_PRESENT,
    POWER_SUPPLY_PROP_TECHNOLOGY,
    POWER_SUPPLY_PROP_VOLTAGE_MIN_DESIGN,
    POWER_SUPPLY_PROP_VOLTAGE_NOW,
    POWER_SUPPLY_PROP_CURRENT_NOW,
    POWER_SUPPLY_PROP_CURRENT_AVG,
    POWER_SUPPLY_PROP_POWER_NOW,
    POWER_SUPPLY_PROP_POWER_AVG,
    POWER_SUPPLY_PROP_CAPACITY,
    POWER_SUPPLY_PROP_ENERGY_FULL_DESIGN,
    POWER_SUPPLY_PROP_ENERGY_FULL,
    POWER_SUPPLY_PROP_ENERGY_NOW,
    POWER_SUPPLY_PROP_TEMP,
    POWER_SUPPLY_PROP_MODEL_NAME,
    POWER_SUPPLY_PROP_MANUFACTURER,
    };
    static const struct power_supply_desc acpi_sbs_charger_desc = {
    .name		= "sbs-charger",
    .type		= POWER_SUPPLY_TYPE_MAINS,
    .properties	= sbs_ac_props,
    .num_properties	= ARRAY_SIZE(sbs_ac_props),
    .get_property	= sbs_get_ac_property,
    };
// --------------------------------------------------------------------------
    Smart Battery System Management
    -------------------------------------------------------------------------- */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_battery_reader {
    pub /: *mut *mut u8 command; / command for battery,
    pub /: *mut *mut u8 mode; / word or block?,
    pub /: *mut *mut size_t offset; / offset inside struct acpi_sbs_battery,
}

    static struct acpi_battery_reader info_readers[] = {
    {0x01, SMBUS_READ_WORD, offsetof(struct acpi_battery, alarm_capacity)},
    {0x03, SMBUS_READ_WORD, offsetof(struct acpi_battery, mode)},
    {0x10, SMBUS_READ_WORD, offsetof(struct acpi_battery, full_charge_capacity)},
    {0x17, SMBUS_READ_WORD, offsetof(struct acpi_battery, cycle_count)},
    {0x18, SMBUS_READ_WORD, offsetof(struct acpi_battery, design_capacity)},
    {0x19, SMBUS_READ_WORD, offsetof(struct acpi_battery, design_voltage)},
    {0x1a, SMBUS_READ_WORD, offsetof(struct acpi_battery, spec)},
    {0x1c, SMBUS_READ_WORD, offsetof(struct acpi_battery, serial_number)},
    {0x20, SMBUS_READ_BLOCK, offsetof(struct acpi_battery, manufacturer_name)},
    {0x21, SMBUS_READ_BLOCK, offsetof(struct acpi_battery, device_name)},
    {0x22, SMBUS_READ_BLOCK, offsetof(struct acpi_battery, device_chemistry)},
    };
    static struct acpi_battery_reader state_readers[] = {
    {0x08, SMBUS_READ_WORD, offsetof(struct acpi_battery, temp_now)},
    {0x09, SMBUS_READ_WORD, offsetof(struct acpi_battery, voltage_now)},
    {0x0a, SMBUS_READ_WORD, offsetof(struct acpi_battery, rate_now)},
    {0x0b, SMBUS_READ_WORD, offsetof(struct acpi_battery, rate_avg)},
    {0x0f, SMBUS_READ_WORD, offsetof(struct acpi_battery, capacity_now)},
    {0x0e, SMBUS_READ_WORD, offsetof(struct acpi_battery, state_of_charge)},
    {0x16, SMBUS_READ_WORD, offsetof(struct acpi_battery, state)},
    };
#[no_mangle]
unsafe extern "C" fn acpi_manager_get_info(sbs: *mut acpi_sbs) -> c_int {
    static int acpi_manager_get_info(struct acpi_sbs *sbs)
    {
    let mut result: c_int = 0;
    u16 battery_system_info;
    result = acpi_smbus_read(sbs.hc, SMBUS_READ_WORD, ACPI_SBS_MANAGER,
    0x04, (u8 *)&battery_system_info);
    if (!result)
    sbs.batteries_supported = battery_system_info & 0x000f;
    return result;
    }
#[no_mangle]
unsafe extern "C" fn acpi_battery_get_info(battery: *mut acpi_battery) -> c_int {
    static int acpi_battery_get_info(struct acpi_battery *battery)
    {
    int i, result = 0;
    for (i = 0; i < ARRAY_SIZE(info_readers); ++i) {
    result = acpi_smbus_read(battery.sbs.hc,
    info_readers[i].mode,
    ACPI_SBS_BATTERY,
    info_readers[i].command,
    (u8 *) battery +
    info_readers[i].offset);
    if (result)
    break;
    }
    return result;
    }
#[no_mangle]
unsafe extern "C" fn acpi_battery_get_state(battery: *mut acpi_battery) -> c_int {
    static int acpi_battery_get_state(struct acpi_battery *battery)
    {
    int i, result = 0;
    if (battery.update_time &&
    time_before(jiffies, battery.update_time +
    msecs_to_jiffies(cache_time)))
    return 0;
    for (i = 0; i < ARRAY_SIZE(state_readers); ++i) {
    result = acpi_smbus_read(battery.sbs.hc,
    state_readers[i].mode,
    ACPI_SBS_BATTERY,
    state_readers[i].command,
    (u8 *)battery +
    state_readers[i].offset);
    if (result)
    goto end;
    }
    end:
    battery.update_time = jiffies;
    return result;
    }
#[no_mangle]
unsafe extern "C" fn acpi_battery_get_alarm(battery: *mut acpi_battery) -> c_int {
    static int acpi_battery_get_alarm(struct acpi_battery *battery)
    {
    return acpi_smbus_read(battery.sbs.hc, SMBUS_READ_WORD,
    ACPI_SBS_BATTERY, 0x01,
    (u8 *)&battery.alarm_capacity);
    }
#[no_mangle]
unsafe extern "C" fn acpi_battery_set_alarm(battery: *mut acpi_battery) -> c_int {
    static int acpi_battery_set_alarm(struct acpi_battery *battery)
    {
    struct acpi_sbs *sbs = battery.sbs;
    u16 value, sel = 1 << (battery.id + 12);
    int ret;
    if (sbs.manager_present) {
    ret = acpi_smbus_read(sbs.hc, SMBUS_READ_WORD, ACPI_SBS_MANAGER,
    0x01, (u8 *)&value);
    if (ret)
    goto end;
    if ((value & 0xf000) != sel) {
    value &= 0x0fff;
    value |= sel;
    ret = acpi_smbus_write(sbs.hc, SMBUS_WRITE_WORD,
    ACPI_SBS_MANAGER,
    0x01, (u8 *)&value, 2);
    if (ret)
    goto end;
    }
    }
    ret = acpi_smbus_write(sbs.hc, SMBUS_WRITE_WORD, ACPI_SBS_BATTERY,
    0x01, (u8 *)&battery.alarm_capacity, 2);
    end:
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn acpi_ac_get_present(sbs: *mut acpi_sbs) -> c_int {
    static int acpi_ac_get_present(struct acpi_sbs *sbs)
    {
    int result;
    u16 status;
    result = acpi_smbus_read(sbs.hc, SMBUS_READ_WORD, ACPI_SBS_CHARGER,
    0x13, (u8 *) & status);
    if (result)
    return result;
//
// The spec requires that bit 4 always be 1. If it's not set, assume
// that the implementation doesn't support an SBS charger.
//
// And on some MacBooks a status of 0xffff is always returned, no
// matter whether the charger is plugged in or not, which is also
// wrong, so ignore the SBS charger for those too.
//
    if (!((status >> 4) & 0x1) || status == 0xffff)
    return -ENODEV;
    sbs.charger_present = (status >> 15) & 0x1;
    return 0;
    }
    static ssize_t acpi_battery_alarm_show(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    struct acpi_battery *battery = to_acpi_battery(dev_get_drvdata(dev));
    acpi_battery_get_alarm(battery);
    return sprintf(buf, "%d\n", battery.alarm_capacity *
    acpi_battery_scale(battery) * 1000);
    }
    static ssize_t acpi_battery_alarm_store(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t count)
    {
    unsigned long x;
    struct acpi_battery *battery = to_acpi_battery(dev_get_drvdata(dev));
    if (sscanf(buf, "%lu\n", &x) == 1)
    battery.alarm_capacity = x /
    (1000 * acpi_battery_scale(battery));
    if (battery.present)
    acpi_battery_set_alarm(battery);
    return count;
    }
    static struct device_attribute alarm_attr = {
    .attr = {.name = "alarm", .mode = 0644},
    .show = acpi_battery_alarm_show,
    .store = acpi_battery_alarm_store,
    };
    static struct attribute *acpi_battery_attrs[] = {
    &alarm_attr.attr,
    core::ptr::null_mut()
    };
    ATTRIBUTE_GROUPS(acpi_battery);
// --------------------------------------------------------------------------
    Driver Interface
    -------------------------------------------------------------------------- */
#[no_mangle]
unsafe extern "C" fn acpi_battery_read(battery: *mut acpi_battery) -> c_int {
    static int acpi_battery_read(struct acpi_battery *battery)
    {
    int result, saved_present = battery.present;
    u16 state;
    if (battery.sbs.manager_present) {
    result = acpi_smbus_read(battery.sbs.hc, SMBUS_READ_WORD,
    ACPI_SBS_MANAGER, 0x01, (u8 *)&state);
    if (result)
    return result;
    battery.present = !!(state & (1 << battery.id));
    if (!battery.present)
    return 0;
// Masking necessary for Smart Battery Selectors
    state = 0x0fff;
    state |= 1 << (battery.id + 12);
    acpi_smbus_write(battery.sbs.hc, SMBUS_WRITE_WORD,
    ACPI_SBS_MANAGER, 0x01, (u8 *)&state, 2);
    } else {
    if (battery.id == 0) {
    battery.present = 1;
    } else {
    if (!battery.present)
    return 0;
    }
    }
    if (saved_present != battery.present) {
    battery.update_time = 0;
    result = acpi_battery_get_info(battery);
    if (result) {
    battery.present = 0;
    return result;
    }
    }
    result = acpi_battery_get_state(battery);
    if (result)
    battery.present = 0;
    return result;
    }
// Smart Battery
#[no_mangle]
unsafe extern "C" fn acpi_battery_add(sbs: *mut acpi_sbs, id: c_int) -> c_int {
    static int acpi_battery_add(struct acpi_sbs *sbs, int id)
    {
    struct acpi_battery *battery = &sbs.battery[id];
    struct power_supply_config psy_cfg = {
    .drv_data = battery,
    .attr_grp = acpi_battery_groups,
    };
    int result;
    battery.id = id;
    battery.sbs = sbs;
    result = acpi_battery_read(battery);
    if (result)
    return result;
    sprintf(battery.name, ACPI_BATTERY_DIR_NAME, id);
    battery.bat_desc.name = battery.name;
    battery.bat_desc.type = POWER_SUPPLY_TYPE_BATTERY;
    if (!acpi_battery_mode(battery)) {
    battery.bat_desc.properties = sbs_charge_battery_props;
    battery.bat_desc.num_properties =
    ARRAY_SIZE(sbs_charge_battery_props);
    } else {
    battery.bat_desc.properties = sbs_energy_battery_props;
    battery.bat_desc.num_properties =
    ARRAY_SIZE(sbs_energy_battery_props);
    }
    battery.bat_desc.get_property = acpi_sbs_battery_get_property;
    battery.bat = power_supply_register(&sbs.device.dev,
    &battery.bat_desc, &psy_cfg);
    if (IS_ERR(battery.bat)) {
    result = PTR_ERR(battery.bat);
    battery.bat = core::ptr::null_mut();
    goto end;
    }
    end:
    pr_info("%s [%s]: Battery Slot [%s] (battery %s)\n",
    ACPI_SBS_DEVICE_NAME, acpi_device_bid(sbs.device),
    battery.name, battery.present ? "present" : "absent");
    return result;
    }
#[no_mangle]
unsafe extern "C" fn acpi_battery_remove(sbs: *mut acpi_sbs, id: c_int) {
    static void acpi_battery_remove(struct acpi_sbs *sbs, int id)
    {
    struct acpi_battery *battery = &sbs.battery[id];
    if (battery.bat)
    power_supply_unregister(battery.bat);
    }
#[no_mangle]
unsafe extern "C" fn acpi_charger_add(sbs: *mut acpi_sbs) -> c_int {
    static int acpi_charger_add(struct acpi_sbs *sbs)
    {
    int result;
    let mut psy_cfg: power_supply_config = { .drv_data = sbs, };
    result = acpi_ac_get_present(sbs);
    if (result)
    goto end;
    sbs.charger_exists = 1;
    sbs.charger = power_supply_register(&sbs.device.dev,
    &acpi_sbs_charger_desc, &psy_cfg);
    if (IS_ERR(sbs.charger)) {
    result = PTR_ERR(sbs.charger);
    sbs.charger = core::ptr::null_mut();
    }
    pr_info("%s [%s]: AC Adapter [%s] (%s)\n",
    ACPI_SBS_DEVICE_NAME, acpi_device_bid(sbs.device),
    ACPI_AC_DIR_NAME, sbs.charger_present ? "on-line" : "off-line");
    end:
    return result;
    }
#[no_mangle]
unsafe extern "C" fn acpi_charger_remove(sbs: *mut acpi_sbs) {
    static void acpi_charger_remove(struct acpi_sbs *sbs)
    {
    if (sbs.charger)
    power_supply_unregister(sbs.charger);
    }
#[no_mangle]
unsafe extern "C" fn acpi_sbs_callback(context: *mut c_void) {
    static void acpi_sbs_callback(void *context)
    {
    int id;
    struct acpi_sbs *sbs = context;
    struct acpi_battery *bat;
    let mut saved_charger_state: u8 = sbs.charger_present;
    u8 saved_battery_state;
    if (sbs.charger_exists) {
    acpi_ac_get_present(sbs);
    if (sbs.charger_present != saved_charger_state)
    power_supply_changed(sbs.charger);
    }
    if (sbs.manager_present) {
    for (id = 0; id < MAX_SBS_BAT; ++id) {
    if (!(sbs.batteries_supported & (1 << id)))
    continue;
    bat = &sbs.battery[id];
    saved_battery_state = bat.present;
    acpi_battery_read(bat);
    if (saved_battery_state == bat.present)
    continue;
    power_supply_changed(bat.bat);
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn acpi_sbs_probe(pdev: *mut platform_device) -> c_int {
    static int acpi_sbs_probe(struct platform_device *pdev)
    {
    struct acpi_device *device;
    struct acpi_sbs *sbs;
    let mut result: c_int = 0;
    int id;
    device = ACPI_COMPANION(&pdev.dev);
    if (!device)
    return -ENODEV;
    sbs = kzalloc_obj(struct acpi_sbs);
    if (!sbs) {
    result = -ENOMEM;
    goto end;
    }
    mutex_init(&sbs.lock);
    platform_set_drvdata(pdev, sbs);
    sbs.hc = dev_get_drvdata(pdev.dev.parent);
    sbs.device = device;
    result = acpi_charger_add(sbs);
    if (result && result != -ENODEV)
    goto end;
    result = 0;
    if (!x86_apple_machine) {
    result = acpi_manager_get_info(sbs);
    if (!result) {
    sbs.manager_present = 1;
    for (id = 0; id < MAX_SBS_BAT; ++id)
    if ((sbs.batteries_supported & (1 << id)))
    acpi_battery_add(sbs, id);
    }
    }
    if (!sbs.manager_present)
    acpi_battery_add(sbs, 0);
    acpi_smbus_register_callback(sbs.hc, acpi_sbs_callback, sbs);
    end:
    if (result)
    acpi_sbs_remove(pdev);
    return result;
    }
#[no_mangle]
unsafe extern "C" fn acpi_sbs_remove(pdev: *mut platform_device) {
    static void acpi_sbs_remove(struct platform_device *pdev)
    {
    struct acpi_sbs *sbs = platform_get_drvdata(pdev);
    int id;
    mutex_lock(&sbs.lock);
    acpi_smbus_unregister_callback(sbs.hc);
    for (id = 0; id < MAX_SBS_BAT; ++id)
    acpi_battery_remove(sbs, id);
    acpi_charger_remove(sbs);
    mutex_unlock(&sbs.lock);
    mutex_destroy(&sbs.lock);
    kfree(sbs);
    }

#[no_mangle]
unsafe extern "C" fn acpi_sbs_resume(dev: *mut device) -> c_int {
    static int acpi_sbs_resume(struct device *dev)
    {
    acpi_sbs_callback(dev_get_drvdata(dev));
    return 0;
    }

    static SIMPLE_DEV_PM_OPS(acpi_sbs_pm, core::ptr::null_mut(), acpi_sbs_resume);
    static struct platform_driver acpi_sbs_driver = {
    .probe = acpi_sbs_probe,
    .remove = acpi_sbs_remove,
    .driver = {
    .name = "acpi-sbs",
    .acpi_match_table = sbs_device_ids,
    .pm = &acpi_sbs_pm,
    },
    };
    module_platform_driver(acpi_sbs_driver);
