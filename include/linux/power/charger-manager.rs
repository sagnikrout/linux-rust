//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/power/charger-manager.h
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
// Copyright (C) 2011 Samsung Electronics Co., Ltd.
// MyungJoo.Ham <myungjoo.ham@samsung.com>
//
// Charger Manager.
// This framework enables to control and multiple chargers and to
// monitor charging even in the context of suspend-to-RAM with
// an interface combining the chargers.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum data_source {
    CM_BATTERY_PRESENT,
    CM_NO_BATTERY,
    CM_FUEL_GAUGE,
    CM_CHARGER_STAT,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum polling_modes {
    CM_POLL_DISABLE = 0,
    CM_POLL_ALWAYS,
    CM_POLL_EXTERNAL_POWER_ONLY,
    CM_POLL_CHARGING_ONLY,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cm_batt_temp {
    CM_BATT_OK = 0,
    CM_BATT_OVERHEAT,
    CM_BATT_COLD,
}

//
// struct charger_cable
// @extcon_name: the name of extcon device.
// @name: the name of the cable connector
// @extcon_dev: the extcon device.
// @wq: the workqueue to control charger according to the state of
// charger cable. If charger cable is attached, enable charger.
// But if charger cable is detached, disable charger.
// @nb: the notifier block to receive changed state from EXTCON
// (External Connector) when charger cable is attached/detached.
// @attached: the state of charger cable.
// true: the charger cable is attached
// false: the charger cable is detached
// @charger: the instance of struct charger_regulator.
// @cm: the Charger Manager representing the battery.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct charger_cable {
    pub extcon_name: *const c_char,
    pub name: *const c_char,
    pub extcon_dev: *mut extcon_dev,
    pub extcon_type: u64,
// The charger-manager use Extcon framework
    pub wq: work_struct,
    pub nb: notifier_block,
// The state of charger cable
    pub attached: bool,
    pub charger: *mut charger_regulator,
//
// Set min/max current of regulator to protect over-current issue
// according to a kind of charger cable when cable is attached.
//
    pub min_uA: c_int,
    pub max_uA: c_int,
    pub cm: *mut charger_manager,
}

//
// struct charger_regulator
// @regulator_name: the name of regulator for using charger.
// @consumer: the regulator consumer for the charger.
// @externally_control:
// Set if the charger-manager cannot control charger,
// the charger will be maintained with disabled state.
// @cables:
// the array of charger cables to enable/disable charger
// and set current limit according to constraint data of
// struct charger_cable if only charger cable included
// in the array of charger cables is attached/detached.
// @num_cables: the number of charger cables.
// @attr_g: Attribute group for the charger(regulator)
// @attr_name: "name" sysfs entry
// @attr_state: "state" sysfs entry
// @attr_externally_control: "externally_control" sysfs entry
// @attrs: Arrays pointing to attr_name/state/externally_control for attr_g
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct charger_regulator {
// The name of regulator for charging
    pub regulator_name: *const c_char,
    pub consumer: *mut regulator,
// charger never on when system is on
    pub externally_control: c_int,
//
// Store constraint information related to current limit,
// each cable have different condition for charging.
//
    pub cables: *mut charger_cable,
    pub num_cables: c_int,
    pub attr_grp: attribute_group,
    pub attr_name: device_attribute,
    pub attr_state: device_attribute,
    pub attr_externally_control: device_attribute,
    pub attrs: [*mut attribute; 4],
    pub cm: *mut charger_manager,
}

//
// struct charger_desc
// @psy_name: the name of power-supply-class for charger manager
// @polling_mode:
// Determine which polling mode will be used
// @fullbatt_vchkdrop_uV:
// Check voltage drop after the battery is fully charged.
// If it has dropped more than fullbatt_vchkdrop_uV
// CM will restart charging.
// @fullbatt_uV: voltage in microvolt
// If VBATT >= fullbatt_uV, it is assumed to be full.
// @fullbatt_soc: state of Charge in %
// If state of Charge >= fullbatt_soc, it is assumed to be full.
// @fullbatt_full_capacity: full capacity measure
// If full capacity of battery >= fullbatt_full_capacity,
// it is assumed to be full.
// @polling_interval_ms: interval in millisecond at which
// charger manager will monitor battery health
// @battery_present:
// Specify where information for existence of battery can be obtained
// @psy_charger_stat: the names of power-supply for chargers
// @num_charger_regulator: the number of entries in charger_regulators
// @charger_regulators: array of charger regulators
// @psy_fuel_gauge: the name of power-supply for fuel gauge
// @thermal_zone : the name of thermal zone for battery
// @temp_min : Minimum battery temperature for charging.
// @temp_max : Maximum battery temperature for charging.
// @temp_diff : Temperature difference to restart charging.
// @measure_battery_temp:
// true: measure battery temperature
// false: measure ambient temperature
// @charging_max_duration_ms: Maximum possible duration for charging
// If whole charging duration exceed 'charging_max_duration_ms',
// cm stop charging.
// @discharging_max_duration_ms:
// Maximum possible duration for discharging with charger cable
// after full-batt. If discharging duration exceed 'discharging
// max_duration_ms', cm start charging.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct charger_desc {
    pub psy_name: *const c_char,
    pub polling_mode: polling_modes,
    pub polling_interval_ms: c_uint,
    pub fullbatt_vchkdrop_uV: c_uint,
    pub fullbatt_uV: c_uint,
    pub fullbatt_soc: c_uint,
    pub fullbatt_full_capacity: c_uint,
    pub battery_present: data_source,
    pub psy_charger_stat: *const c_char,
    pub num_charger_regulators: c_int,
    pub charger_regulators: *mut charger_regulator,
    pub sysfs_groups: *const attribute_group,
    pub psy_fuel_gauge: *const c_char,
    pub thermal_zone: *const c_char,
    pub temp_min: c_int,
    pub temp_max: c_int,
    pub temp_diff: c_int,
    pub measure_battery_temp: bool,
    pub charging_max_duration_ms: u32,
    pub discharging_max_duration_ms: u32,
}

pub const PSY_NAME_MAX: c_int = 30;
//
// struct charger_manager
// @entry: entry for list
// @dev: device pointer
// @desc: instance of charger_desc
// @fuel_gauge: power_supply for fuel gauge
// @charger_stat: array of power_supply for chargers
// @tzd_batt : thermal zone device for battery
// @charger_enabled: the state of charger
// @emergency_stop:
// When setting true, stop charging
// @psy_name_buf: the name of power-supply-class for charger manager
// @charger_psy: power_supply for charger manager
// @status_save_ext_pwr_inserted:
// saved status of external power before entering suspend-to-RAM
// @status_save_batt:
// saved status of battery before entering suspend-to-RAM
// @charging_start_time: saved start time of enabling charging
// @charging_end_time: saved end time of disabling charging
// @battery_status: Current battery status
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct charger_manager {
    pub entry: list_head,
    pub dev: *mut device,
    pub desc: *mut charger_desc,

    pub tzd_batt: *mut thermal_zone_device,

    pub charger_enabled: bool,
    pub emergency_stop: c_int,
    pub 1]: char psy_name_buf[PSY_NAME_MAX +,
    pub charger_psy_desc: power_supply_desc,
    pub charger_psy: *mut power_supply,
    pub charging_start_time: u64,
    pub charging_end_time: u64,
    pub battery_status: c_int,
}
