//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/pm_opp.h
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
// Generic OPP Interface
//
// Copyright (C) 2009-2010 Texas Instruments Incorporated.
// Nishanth Menon
// Romit Dasgupta
// Kevin Hilman
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dev_pm_opp_event {
    OPP_EVENT_ADD, OPP_EVENT_REMOVE, OPP_EVENT_ENABLE, OPP_EVENT_DISABLE,
    OPP_EVENT_ADJUST_VOLTAGE,
}

//
// struct dev_pm_opp_supply - Power supply voltage/current values
// @u_volt:	Target voltage in microvolts corresponding to this OPP
// @u_volt_min:	Minimum voltage in microvolts corresponding to this OPP
// @u_volt_max:	Maximum voltage in microvolts corresponding to this OPP
// @u_amp:	Maximum current drawn by the device in microamperes
// @u_watt:	Power used by the device in microwatts
//
// This structure stores the voltage/current/power values for a single power
// supply.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dev_pm_opp_supply {
    pub u_volt: c_ulong,
    pub u_volt_min: c_ulong,
    pub u_volt_max: c_ulong,
    pub u_amp: c_ulong,
    pub u_watt: c_ulong,
}

//
// struct dev_pm_opp_config - Device OPP configuration values
// @clk_names: Clk names, NULL terminated array.
// @config_clks: Custom set clk helper.
// @prop_name: Name to postfix to properties.
// @config_regulators: Custom set regulator helper.
// @supported_hw: Array of hierarchy of versions to match.
// @supported_hw_count: Number of elements in the array.
// @regulator_names: Array of pointers to the names of the regulator, NULL terminated.
// @required_dev: The required OPP device.
// @required_dev_index: The index of the required OPP for the @required_dev.
//
// This structure contains platform specific OPP configurations for the device.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dev_pm_opp_config {
// NULL terminated
    pub clk_names: *const *const c_char,
    pub config_clks: config_clks_t,
    pub prop_name: *const c_char,
    pub config_regulators: config_regulators_t,
    pub supported_hw: *const c_uint,
    pub supported_hw_count: c_uint,
    pub regulator_names: *const *const c_char,
    pub required_dev: *mut device,
    pub required_dev_index: c_uint,
}

//
// struct dev_pm_opp_data - The data to use to initialize an OPP.
// @turbo: Flag to indicate whether the OPP is to be marked turbo or not.
// @level: The performance level for the OPP. Set level to OPP_LEVEL_UNSET if
// level field isn't used.
// @freq: The clock rate in Hz for the OPP.
// @u_volt: The voltage in uV for the OPP.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dev_pm_opp_data {
    pub turbo: bool,
    pub level: c_uint,
    pub freq: c_ulong,
    pub u_volt: c_ulong,
}

//
// struct dev_pm_opp_key - Key used to identify OPP entries
// @freq:       Frequency in Hz. Use 0 if frequency is not to be matched.
// @level:      Performance level associated with the OPP entry.
// Use OPP_LEVEL_UNSET if level is not to be matched.
// @bw:         Bandwidth associated with the OPP entry.
// Use 0 if bandwidth is not to be matched.
//
// This structure is used to uniquely identify an OPP entry based on
// frequency, performance level, and bandwidth. Each field can be
// selectively ignored during matching by setting it to its respective
// NOP value.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dev_pm_opp_key {
    pub freq: c_ulong,
    pub level: c_uint,
    pub bw: u32,
}

extern "C" {
    pub fn dev_pm_opp_put_opp_table(opp_table: *mut opp_table);
}
extern "C" {
    pub fn dev_pm_opp_get_bw(opp: *mut dev_pm_opp, peak: bool, index: c_int) -> c_ulong;
}
extern "C" {
    pub fn dev_pm_opp_get_voltage(opp: *mut dev_pm_opp) -> c_ulong;
}
extern "C" {
    pub fn dev_pm_opp_get_supplies(opp: *mut dev_pm_opp, supplies: *mut dev_pm_opp_supply) -> c_int;
}
extern "C" {
    pub fn dev_pm_opp_get_power(opp: *mut dev_pm_opp) -> c_ulong;
}
extern "C" {
    pub fn dev_pm_opp_get_freq_indexed(opp: *mut dev_pm_opp, index: u32) -> c_ulong;
}
extern "C" {
    pub fn dev_pm_opp_get_level(opp: *mut dev_pm_opp) -> c_uint;
}
extern "C" {
    pub fn dev_pm_opp_is_turbo(opp: *mut dev_pm_opp) -> bool;
}
extern "C" {
    pub fn dev_pm_opp_get_opp_count(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn dev_pm_opp_get_max_clock_latency(dev: *mut device) -> c_ulong;
}
extern "C" {
    pub fn dev_pm_opp_get_max_volt_latency(dev: *mut device) -> c_ulong;
}
extern "C" {
    pub fn dev_pm_opp_get_max_transition_latency(dev: *mut device) -> c_ulong;
}
extern "C" {
    pub fn dev_pm_opp_get_suspend_opp_freq(dev: *mut device) -> c_ulong;
}
extern "C" {
    pub fn dev_pm_opp_put(opp: *mut dev_pm_opp);
}
extern "C" {
    pub fn dev_pm_opp_add_dynamic(dev: *mut device, opp: *mut dev_pm_opp_data) -> c_int;
}
extern "C" {
    pub fn dev_pm_opp_remove(dev: *mut device, freq: c_ulong);
}
extern "C" {
    pub fn dev_pm_opp_remove_all_dynamic(dev: *mut device);
}
extern "C" {
    pub fn dev_pm_opp_enable(dev: *mut device, freq: c_ulong) -> c_int;
}
extern "C" {
    pub fn dev_pm_opp_disable(dev: *mut device, freq: c_ulong) -> c_int;
}
extern "C" {
    pub fn dev_pm_opp_register_notifier(dev: *mut device, nb: *mut notifier_block) -> c_int;
}
extern "C" {
    pub fn dev_pm_opp_unregister_notifier(dev: *mut device, nb: *mut notifier_block) -> c_int;
}
extern "C" {
    pub fn dev_pm_opp_set_config(dev: *mut device, config: *mut dev_pm_opp_config) -> c_int;
}
extern "C" {
    pub fn devm_pm_opp_set_config(dev: *mut device, config: *mut dev_pm_opp_config) -> c_int;
}
extern "C" {
    pub fn dev_pm_opp_clear_config(token: c_int);
}
extern "C" {
    pub fn dev_pm_opp_xlate_performance_state(src_table: *mut opp_table, dst_table: *mut opp_table, pstate: c_uint) -> c_int;
}
extern "C" {
    pub fn dev_pm_opp_set_rate(dev: *mut device, target_freq: c_ulong) -> c_int;
}
extern "C" {
    pub fn dev_pm_opp_set_opp(dev: *mut device, opp: *mut dev_pm_opp) -> c_int;
}
extern "C" {
    pub fn dev_pm_opp_set_sharing_cpus(cpu_dev: *mut device, cpumask: *const cpumask) -> c_int;
}
extern "C" {
    pub fn dev_pm_opp_get_sharing_cpus(cpu_dev: *mut device, cpumask: *mut cpumask) -> c_int;
}
extern "C" {
    pub fn dev_pm_opp_remove_table(dev: *mut device);
}
extern "C" {
    pub fn dev_pm_opp_cpumask_remove_table(cpumask: *const cpumask);
}
extern "C" {
    pub fn dev_pm_opp_sync_regulators(dev: *mut device) -> c_int;
}

extern "C" {
    pub fn ERR_PTR(_arg: -EOPNOTSUPP) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -EOPNOTSUPP) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -EOPNOTSUPP) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -EOPNOTSUPP) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -EOPNOTSUPP) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -EOPNOTSUPP) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -EOPNOTSUPP) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -EOPNOTSUPP) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -EOPNOTSUPP) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -EOPNOTSUPP) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -EOPNOTSUPP) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -EOPNOTSUPP) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -EOPNOTSUPP) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -EOPNOTSUPP) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -EOPNOTSUPP) -> return;
}

extern "C" {
    pub fn dev_pm_opp_init_cpufreq_table(dev: *mut device, table: *mut cpufreq_frequency_table) -> c_int;
}
extern "C" {
    pub fn dev_pm_opp_free_cpufreq_table(dev: *mut device, table: *mut cpufreq_frequency_table);
}

extern "C" {
    pub fn dev_pm_opp_of_add_table(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn dev_pm_opp_of_add_table_indexed(dev: *mut device, index: c_int) -> c_int;
}
extern "C" {
    pub fn devm_pm_opp_of_add_table_indexed(dev: *mut device, index: c_int) -> c_int;
}
extern "C" {
    pub fn dev_pm_opp_of_remove_table(dev: *mut device);
}
extern "C" {
    pub fn devm_pm_opp_of_add_table(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn dev_pm_opp_of_cpumask_add_table(cpumask: *const cpumask) -> c_int;
}
extern "C" {
    pub fn dev_pm_opp_of_cpumask_remove_table(cpumask: *const cpumask);
}
extern "C" {
    pub fn dev_pm_opp_of_get_sharing_cpus(cpu_dev: *mut device, cpumask: *mut cpumask) -> c_int;
}
extern "C" {
    pub fn of_get_required_opp_performance_state(np: *mut device_node, index: c_int) -> c_int;
}
extern "C" {
    pub fn dev_pm_opp_of_has_required_opp(dev: *mut device) -> bool;
}
extern "C" {
    pub fn dev_pm_opp_of_find_icc_paths(dev: *mut device, opp_table: *mut opp_table) -> c_int;
}
extern "C" {
    pub fn dev_pm_opp_of_register_em(dev: *mut device, cpus: *mut cpumask) -> c_int;
}

// Scope based cleanup macro for OPP reference counting
// Scope based cleanup macro for OPP table reference counting
// OPP Configuration helpers
extern "C" {
    pub fn dev_pm_opp_add_dynamic(_arg: dev, _arg: &data) -> return;
}
// Regulators helpers
extern "C" {
    pub fn dev_pm_opp_set_config(_arg: dev, _arg: &config) -> return;
}
extern "C" {
    pub fn devm_pm_opp_set_config(_arg: dev, _arg: &config) -> return;
}
// Supported-hw helpers
extern "C" {
    pub fn dev_pm_opp_set_config(_arg: dev, _arg: &config) -> return;
}
extern "C" {
    pub fn devm_pm_opp_set_config(_arg: dev, _arg: &config) -> return;
}
// clkname helpers
extern "C" {
    pub fn dev_pm_opp_set_config(_arg: dev, _arg: &config) -> return;
}
extern "C" {
    pub fn devm_pm_opp_set_config(_arg: dev, _arg: &config) -> return;
}
// config-regulators helpers
extern "C" {
    pub fn dev_pm_opp_set_config(_arg: dev, _arg: &config) -> return;
}
// prop-name helpers
extern "C" {
    pub fn dev_pm_opp_set_config(_arg: dev, _arg: &config) -> return;
}
extern "C" {
    pub fn dev_pm_opp_get_freq_indexed(_arg: opp, _arg: 0) -> return;
}
extern "C" {
    pub fn __free(dev_pm_opp_find_level_exact(dev: put_opp) =, _arg: level) -> *mut dev_pm_opp opp;
}
extern "C" {
    pub fn PTR_ERR(_arg: opp) -> return;
}
extern "C" {
    pub fn dev_pm_opp_set_opp(_arg: dev, _arg: opp) -> return;
}
