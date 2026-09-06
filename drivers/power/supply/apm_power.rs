//! Automatically rewritten from C to Rust
//! Source: drivers/power/supply/apm_power.c
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


//
// Copyright © 2007 Anton Vorontsov <cbou@mail.ru>
// Copyright © 2007 Eugeny Boger <eugenyboger@dgap.mipt.ru>
//
// Author: Eugeny Boger <eugenyboger@dgap.mipt.ru>
//
// Use consistent with the GNU GPL is permitted,
// provided that this copyright notice is
// preserved in its entirety in all copies and derived works.
//

    POWER_SUPPLY_PROP_##prop, val))

    prop, val))

    static DEFINE_MUTEX(apm_mutex);
    static struct power_supply *main_battery;
    enum apm_source {
    SOURCE_ENERGY,
    SOURCE_CHARGE,
    SOURCE_VOLTAGE,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct find_bat_param {
    pub main: *mut power_supply,
    pub bat: *mut power_supply,
    pub max_charge_bat: *mut power_supply,
    pub max_energy_bat: *mut power_supply,
    pub full: union power_supply_propval,
    pub max_charge: c_int,
    pub max_energy: c_int,
}

#[no_mangle]
unsafe extern "C" fn __find_main_battery(psy: *mut power_supply, data: *mut c_void) -> c_int {
    static int __find_main_battery(struct power_supply *psy, void *data)
    {
    struct find_bat_param *bp = (struct find_bat_param *)data;
    bp.bat = psy;
    if (bp.bat.desc.use_for_apm) {
// nice, we explicitly asked to report this battery.
    bp.main = bp.bat;
    return 1;
    }
    if (!PSY_PROP(bp.bat, CHARGE_FULL_DESIGN, &bp.full) ||
    !PSY_PROP(bp.bat, CHARGE_FULL, &bp.full)) {
    if (bp.full.intval > bp.max_charge) {
    bp.max_charge_bat = bp.bat;
    bp.max_charge = bp.full.intval;
    }
    } else if (!PSY_PROP(bp.bat, ENERGY_FULL_DESIGN, &bp.full) ||
    !PSY_PROP(bp.bat, ENERGY_FULL, &bp.full)) {
    if (bp.full.intval > bp.max_energy) {
    bp.max_energy_bat = bp.bat;
    bp.max_energy = bp.full.intval;
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn find_main_battery() {
    static void find_main_battery(void)
    {
    struct find_bat_param bp;
    int error;
    memset(&bp, 0, sizeof(struct find_bat_param));
    main_battery = core::ptr::null_mut();
    bp.main = main_battery;
    error = power_supply_for_each_psy(&bp, __find_main_battery);
    if (error) {
    main_battery = bp.main;
    return;
    }
    if ((bp.max_energy_bat && bp.max_charge_bat) &&
    (bp.max_energy_bat != bp.max_charge_bat)) {
// try guess battery with more capacity
    if (!PSY_PROP(bp.max_charge_bat, VOLTAGE_MAX_DESIGN,
    &bp.full)) {
    if (bp.max_energy > bp.max_charge * bp.full.intval)
    main_battery = bp.max_energy_bat;
    else
    main_battery = bp.max_charge_bat;
    } else if (!PSY_PROP(bp.max_energy_bat, VOLTAGE_MAX_DESIGN,
    &bp.full)) {
    if (bp.max_charge > bp.max_energy / bp.full.intval)
    main_battery = bp.max_charge_bat;
    else
    main_battery = bp.max_energy_bat;
    } else {
// give up, choice any
    main_battery = bp.max_energy_bat;
    }
    } else if (bp.max_charge_bat) {
    main_battery = bp.max_charge_bat;
    } else if (bp.max_energy_bat) {
    main_battery = bp.max_energy_bat;
    } else {
// give up, try the last if any
    main_battery = bp.bat;
    }
    }
#[no_mangle]
unsafe extern "C" fn do_calculate_time(status: c_int, source: enum apm_source) -> c_int {
    static int do_calculate_time(int status, enum apm_source source)
    {
    union power_supply_propval full;
    union power_supply_propval empty;
    union power_supply_propval cur;
    union power_supply_propval I;
    enum power_supply_property full_prop;
    enum power_supply_property full_design_prop;
    enum power_supply_property empty_prop;
    enum power_supply_property empty_design_prop;
    enum power_supply_property cur_avg_prop;
    enum power_supply_property cur_now_prop;
    if (MPSY_PROP(CURRENT_AVG, &I)) {
// if battery can't report average value, use momentary
    if (MPSY_PROP(CURRENT_NOW, &I))
    return -1;
    }
    if (!I.intval)
    return 0;
    switch (source) {
    case SOURCE_CHARGE:
    full_prop = POWER_SUPPLY_PROP_CHARGE_FULL;
    full_design_prop = POWER_SUPPLY_PROP_CHARGE_FULL_DESIGN;
    empty_prop = POWER_SUPPLY_PROP_CHARGE_EMPTY;
    empty_design_prop = POWER_SUPPLY_PROP_CHARGE_EMPTY;
    cur_avg_prop = POWER_SUPPLY_PROP_CHARGE_AVG;
    cur_now_prop = POWER_SUPPLY_PROP_CHARGE_NOW;
    break;
    case SOURCE_ENERGY:
    full_prop = POWER_SUPPLY_PROP_ENERGY_FULL;
    full_design_prop = POWER_SUPPLY_PROP_ENERGY_FULL_DESIGN;
    empty_prop = POWER_SUPPLY_PROP_ENERGY_EMPTY;
    empty_design_prop = POWER_SUPPLY_PROP_CHARGE_EMPTY;
    cur_avg_prop = POWER_SUPPLY_PROP_ENERGY_AVG;
    cur_now_prop = POWER_SUPPLY_PROP_ENERGY_NOW;
    break;
    case SOURCE_VOLTAGE:
    full_prop = POWER_SUPPLY_PROP_VOLTAGE_MAX;
    full_design_prop = POWER_SUPPLY_PROP_VOLTAGE_MAX_DESIGN;
    empty_prop = POWER_SUPPLY_PROP_VOLTAGE_MIN;
    empty_design_prop = POWER_SUPPLY_PROP_VOLTAGE_MIN_DESIGN;
    cur_avg_prop = POWER_SUPPLY_PROP_VOLTAGE_AVG;
    cur_now_prop = POWER_SUPPLY_PROP_VOLTAGE_NOW;
    break;
    default:
    printk(KERN_ERR "Unsupported source: %d\n", source);
    return -1;
    }
    if (_MPSY_PROP(full_prop, &full)) {
// if battery can't report this property, use design value
    if (_MPSY_PROP(full_design_prop, &full))
    return -1;
    }
    if (_MPSY_PROP(empty_prop, &empty)) {
// if battery can't report this property, use design value
    if (_MPSY_PROP(empty_design_prop, &empty))
    empty.intval = 0;
    }
    if (_MPSY_PROP(cur_avg_prop, &cur)) {
// if battery can't report average value, use momentary
    if (_MPSY_PROP(cur_now_prop, &cur))
    return -1;
    }
    if (status == POWER_SUPPLY_STATUS_CHARGING)
    return ((cur.intval - full.intval) * 60L) / I.intval;
    else
    return -((cur.intval - empty.intval) * 60L) / I.intval;
    }
#[no_mangle]
unsafe extern "C" fn calculate_time(status: c_int) -> c_int {
    static int calculate_time(int status)
    {
    int time;
    time = do_calculate_time(status, SOURCE_ENERGY);
    if (time != -1)
    return time;
    time = do_calculate_time(status, SOURCE_CHARGE);
    if (time != -1)
    return time;
    time = do_calculate_time(status, SOURCE_VOLTAGE);
    if (time != -1)
    return time;
    return -1;
    }
#[no_mangle]
unsafe extern "C" fn calculate_capacity(source: enum apm_source) -> c_int {
    static int calculate_capacity(enum apm_source source)
    {
    enum power_supply_property full_prop, empty_prop;
    enum power_supply_property full_design_prop, empty_design_prop;
    enum power_supply_property now_prop, avg_prop;
    union power_supply_propval empty, full, cur;
    int ret;
    switch (source) {
    case SOURCE_CHARGE:
    full_prop = POWER_SUPPLY_PROP_CHARGE_FULL;
    empty_prop = POWER_SUPPLY_PROP_CHARGE_EMPTY;
    full_design_prop = POWER_SUPPLY_PROP_CHARGE_FULL_DESIGN;
    empty_design_prop = POWER_SUPPLY_PROP_CHARGE_EMPTY_DESIGN;
    now_prop = POWER_SUPPLY_PROP_CHARGE_NOW;
    avg_prop = POWER_SUPPLY_PROP_CHARGE_AVG;
    break;
    case SOURCE_ENERGY:
    full_prop = POWER_SUPPLY_PROP_ENERGY_FULL;
    empty_prop = POWER_SUPPLY_PROP_ENERGY_EMPTY;
    full_design_prop = POWER_SUPPLY_PROP_ENERGY_FULL_DESIGN;
    empty_design_prop = POWER_SUPPLY_PROP_ENERGY_EMPTY_DESIGN;
    now_prop = POWER_SUPPLY_PROP_ENERGY_NOW;
    avg_prop = POWER_SUPPLY_PROP_ENERGY_AVG;
    break;
    case SOURCE_VOLTAGE:
    full_prop = POWER_SUPPLY_PROP_VOLTAGE_MAX;
    empty_prop = POWER_SUPPLY_PROP_VOLTAGE_MIN;
    full_design_prop = POWER_SUPPLY_PROP_VOLTAGE_MAX_DESIGN;
    empty_design_prop = POWER_SUPPLY_PROP_VOLTAGE_MIN_DESIGN;
    now_prop = POWER_SUPPLY_PROP_VOLTAGE_NOW;
    avg_prop = POWER_SUPPLY_PROP_VOLTAGE_AVG;
    break;
    default:
    printk(KERN_ERR "Unsupported source: %d\n", source);
    return -1;
    }
    if (_MPSY_PROP(full_prop, &full)) {
// if battery can't report this property, use design value
    if (_MPSY_PROP(full_design_prop, &full))
    return -1;
    }
    if (_MPSY_PROP(avg_prop, &cur)) {
// if battery can't report average value, use momentary
    if (_MPSY_PROP(now_prop, &cur))
    return -1;
    }
    if (_MPSY_PROP(empty_prop, &empty)) {
// if battery can't report this property, use design value
    if (_MPSY_PROP(empty_design_prop, &empty))
    empty.intval = 0;
    }
    if (full.intval - empty.intval)
    ret =  ((cur.intval - empty.intval) * 100L) /
    (full.intval - empty.intval);
    else
    return -1;
    if (ret > 100)
    return 100;
#[no_mangle]
pub unsafe extern "C" fn if(0: ret <) -> else {
    else if (ret < 0)
    return 0;
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn apm_battery_apm_get_power_status(info: *mut apm_power_info) {
    static void apm_battery_apm_get_power_status(struct apm_power_info *info)
    {
    union power_supply_propval status;
    union power_supply_propval capacity, time_to_full, time_to_empty;
    mutex_lock(&apm_mutex);
    find_main_battery();
    if (!main_battery) {
    mutex_unlock(&apm_mutex);
    return;
    }
// status
    if (MPSY_PROP(STATUS, &status))
    status.intval = POWER_SUPPLY_STATUS_UNKNOWN;
// ac line status
    if ((status.intval == POWER_SUPPLY_STATUS_CHARGING) ||
    (status.intval == POWER_SUPPLY_STATUS_NOT_CHARGING) ||
    (status.intval == POWER_SUPPLY_STATUS_FULL))
    info.ac_line_status = APM_AC_ONLINE;
    else
    info.ac_line_status = APM_AC_OFFLINE;
// battery life (i.e. capacity, in percents)
    if (MPSY_PROP(CAPACITY, &capacity) == 0) {
    info.battery_life = capacity.intval;
    } else {
// try calculate using energy
    info.battery_life = calculate_capacity(SOURCE_ENERGY);
// if failed try calculate using charge instead
    if (info.battery_life == -1)
    info.battery_life = calculate_capacity(SOURCE_CHARGE);
    if (info.battery_life == -1)
    info.battery_life = calculate_capacity(SOURCE_VOLTAGE);
    }
// charging status
    if (status.intval == POWER_SUPPLY_STATUS_CHARGING) {
    info.battery_status = APM_BATTERY_STATUS_CHARGING;
    } else {
    if (info.battery_life > 50)
    info.battery_status = APM_BATTERY_STATUS_HIGH;
#[no_mangle]
pub unsafe extern "C" fn if(5: info->battery_life >) -> else {
    else if (info.battery_life > 5)
    info.battery_status = APM_BATTERY_STATUS_LOW;
    else
    info.battery_status = APM_BATTERY_STATUS_CRITICAL;
    }
    info.battery_flag = info.battery_status;
// time
    info.units = APM_UNITS_MINS;
    if (status.intval == POWER_SUPPLY_STATUS_CHARGING) {
    if (!MPSY_PROP(TIME_TO_FULL_AVG, &time_to_full) ||
    !MPSY_PROP(TIME_TO_FULL_NOW, &time_to_full))
    info.time = time_to_full.intval / 60;
    else
    info.time = calculate_time(status.intval);
    } else {
    if (!MPSY_PROP(TIME_TO_EMPTY_AVG, &time_to_empty) ||
    !MPSY_PROP(TIME_TO_EMPTY_NOW, &time_to_empty))
    info.time = time_to_empty.intval / 60;
    else
    info.time = calculate_time(status.intval);
    }
    mutex_unlock(&apm_mutex);
    }
#[no_mangle]
unsafe extern "C" fn apm_battery_init() -> int __init {
    static int __init apm_battery_init(void)
    {
    printk(KERN_INFO "APM Battery Driver\n");
    apm_get_power_status = apm_battery_apm_get_power_status;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn apm_battery_exit() -> void __exit {
    static void __exit apm_battery_exit(void)
    {
    if (apm_get_power_status == apm_battery_apm_get_power_status)
    apm_get_power_status = core::ptr::null_mut();
    }
    module_init(apm_battery_init);
    module_exit(apm_battery_exit);
    MODULE_AUTHOR("Eugeny Boger <eugenyboger@dgap.mipt.ru>");
    MODULE_DESCRIPTION("APM emulation driver for battery monitoring class");
    MODULE_LICENSE("GPL");
