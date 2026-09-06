//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/hwmon.h
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

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hwmon_sensor_types {
    hwmon_chip,
    hwmon_temp,
    hwmon_in,
    hwmon_curr,
    hwmon_power,
    hwmon_energy,
    hwmon_energy64,
    hwmon_humidity,
    hwmon_fan,
    hwmon_pwm,
    hwmon_intrusion,
    hwmon_max,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hwmon_chip_attributes {
    hwmon_chip_temp_reset_history,
    hwmon_chip_in_reset_history,
    hwmon_chip_curr_reset_history,
    hwmon_chip_power_reset_history,
    hwmon_chip_register_tz,
    hwmon_chip_update_interval,
    hwmon_chip_update_interval_us,
    hwmon_chip_alarms,
    hwmon_chip_samples,
    hwmon_chip_curr_samples,
    hwmon_chip_in_samples,
    hwmon_chip_power_samples,
    hwmon_chip_temp_samples,
    hwmon_chip_beep_enable,
    hwmon_chip_pec,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hwmon_temp_attributes {
    hwmon_temp_enable,
    hwmon_temp_input,
    hwmon_temp_type,
    hwmon_temp_lcrit,
    hwmon_temp_lcrit_hyst,
    hwmon_temp_min,
    hwmon_temp_min_hyst,
    hwmon_temp_max,
    hwmon_temp_max_hyst,
    hwmon_temp_crit,
    hwmon_temp_crit_hyst,
    hwmon_temp_emergency,
    hwmon_temp_emergency_hyst,
    hwmon_temp_alarm,
    hwmon_temp_lcrit_alarm,
    hwmon_temp_min_alarm,
    hwmon_temp_max_alarm,
    hwmon_temp_crit_alarm,
    hwmon_temp_emergency_alarm,
    hwmon_temp_fault,
    hwmon_temp_offset,
    hwmon_temp_label,
    hwmon_temp_lowest,
    hwmon_temp_highest,
    hwmon_temp_reset_history,
    hwmon_temp_rated_min,
    hwmon_temp_rated_max,
    hwmon_temp_beep,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hwmon_in_attributes {
    hwmon_in_enable,
    hwmon_in_input,
    hwmon_in_min,
    hwmon_in_max,
    hwmon_in_lcrit,
    hwmon_in_crit,
    hwmon_in_average,
    hwmon_in_lowest,
    hwmon_in_highest,
    hwmon_in_reset_history,
    hwmon_in_label,
    hwmon_in_alarm,
    hwmon_in_min_alarm,
    hwmon_in_max_alarm,
    hwmon_in_lcrit_alarm,
    hwmon_in_crit_alarm,
    hwmon_in_rated_min,
    hwmon_in_rated_max,
    hwmon_in_beep,
    hwmon_in_fault,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hwmon_curr_attributes {
    hwmon_curr_enable,
    hwmon_curr_input,
    hwmon_curr_min,
    hwmon_curr_max,
    hwmon_curr_lcrit,
    hwmon_curr_crit,
    hwmon_curr_average,
    hwmon_curr_lowest,
    hwmon_curr_highest,
    hwmon_curr_reset_history,
    hwmon_curr_label,
    hwmon_curr_alarm,
    hwmon_curr_min_alarm,
    hwmon_curr_max_alarm,
    hwmon_curr_lcrit_alarm,
    hwmon_curr_crit_alarm,
    hwmon_curr_rated_min,
    hwmon_curr_rated_max,
    hwmon_curr_beep,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hwmon_power_attributes {
    hwmon_power_enable,
    hwmon_power_average,
    hwmon_power_average_interval,
    hwmon_power_average_interval_max,
    hwmon_power_average_interval_min,
    hwmon_power_average_highest,
    hwmon_power_average_lowest,
    hwmon_power_average_max,
    hwmon_power_average_min,
    hwmon_power_input,
    hwmon_power_input_highest,
    hwmon_power_input_lowest,
    hwmon_power_reset_history,
    hwmon_power_accuracy,
    hwmon_power_cap,
    hwmon_power_cap_hyst,
    hwmon_power_cap_max,
    hwmon_power_cap_min,
    hwmon_power_min,
    hwmon_power_max,
    hwmon_power_crit,
    hwmon_power_lcrit,
    hwmon_power_label,
    hwmon_power_alarm,
    hwmon_power_cap_alarm,
    hwmon_power_min_alarm,
    hwmon_power_max_alarm,
    hwmon_power_lcrit_alarm,
    hwmon_power_crit_alarm,
    hwmon_power_rated_min,
    hwmon_power_rated_max,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hwmon_energy_attributes {
    hwmon_energy_enable,
    hwmon_energy_input,
    hwmon_energy_label,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hwmon_humidity_attributes {
    hwmon_humidity_enable,
    hwmon_humidity_input,
    hwmon_humidity_label,
    hwmon_humidity_min,
    hwmon_humidity_min_hyst,
    hwmon_humidity_max,
    hwmon_humidity_max_hyst,
    hwmon_humidity_alarm,
    hwmon_humidity_fault,
    hwmon_humidity_rated_min,
    hwmon_humidity_rated_max,
    hwmon_humidity_min_alarm,
    hwmon_humidity_max_alarm,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hwmon_fan_attributes {
    hwmon_fan_enable,
    hwmon_fan_input,
    hwmon_fan_label,
    hwmon_fan_min,
    hwmon_fan_max,
    hwmon_fan_div,
    hwmon_fan_pulses,
    hwmon_fan_target,
    hwmon_fan_alarm,
    hwmon_fan_min_alarm,
    hwmon_fan_max_alarm,
    hwmon_fan_fault,
    hwmon_fan_beep,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hwmon_pwm_attributes {
    hwmon_pwm_input,
    hwmon_pwm_enable,
    hwmon_pwm_mode,
    hwmon_pwm_freq,
    hwmon_pwm_auto_channels_temp,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hwmon_intrusion_attributes {
    hwmon_intrusion_alarm,
    hwmon_intrusion_beep,
}

//
// struct hwmon_ops - hwmon device operations
// @visible:	Static visibility. If non-zero, 'is_visible' is ignored.
// @is_visible: Callback to return attribute visibility. Mandatory unless
// 'visible' is non-zero.
// Parameters are:
// @const void *drvdata:
// Pointer to driver-private data structure passed
// as argument to hwmon_device_register_with_info().
// @type:	Sensor type
// @attr:	Sensor attribute
// @channel:
// Channel number
// The function returns the file permissions.
// If the return value is 0, no attribute will be created.
// @read:	Read callback for data attributes. Mandatory if readable
// data attributes are present.
// Parameters are:
// @dev:	Pointer to hardware monitoring device
// @type:	Sensor type
// @attr:	Sensor attribute
// @channel:
// Channel number
// @val:	Pointer to returned value
// The function returns 0 on success or a negative error number.
// @read_string:
// Read callback for string attributes. Mandatory if string
// attributes are present.
// Parameters are:
// @dev:	Pointer to hardware monitoring device
// @type:	Sensor type
// @attr:	Sensor attribute
// @channel:
// Channel number
// @str:	Pointer to returned string
// The function returns 0 on success or a negative error number.
// @write:	Write callback for data attributes. Mandatory if writeable
// data attributes are present.
// Parameters are:
// @dev:	Pointer to hardware monitoring device
// @type:	Sensor type
// @attr:	Sensor attribute
// @channel:
// Channel number
// @val:	Value to write
// The function returns 0 on success or a negative error number.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hwmon_ops {
    pub visible: umode_t,
    pub channel): u32 attr, int,
    pub val): *mut u32 attr, int channel, long,
    pub str): *const u32 attr, int channel, char,
    pub val): u32 attr, int channel, long,
}

//
// struct hwmon_channel_info - Channel information
// @type:	Channel type.
// @config:	Pointer to NULL-terminated list of channel parameters.
// Use for per-channel attributes.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hwmon_channel_info {
    pub type: hwmon_sensor_types,
    pub config: *const u32,
}

//
// struct hwmon_chip_info - Chip configuration
// @ops:	Pointer to hwmon operations.
// @info:	Null-terminated list of channel information.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hwmon_chip_info {
    pub ops: *const hwmon_ops,
    pub info: *const *const hwmon_channel_info,
}

// hwmon_device_register() is deprecated
//
// hwmon_device_register_with_groups() and
// devm_hwmon_device_register_with_groups() are deprecated.
//
extern "C" {
    pub fn hwmon_device_unregister(dev: *mut device);
}
extern "C" {
    pub fn hwmon_lock(dev: *mut device);
}
extern "C" {
    pub fn hwmon_unlock(dev: *mut device);
}
//
// hwmon_is_bad_char - Is the char invalid in a hwmon name
// @ch: the char to be considered
//
// hwmon_is_bad_char() can be used to determine if the given character
// may not be used in a hwmon name.
//
// Returns true if the char is invalid, false otherwise.
//
