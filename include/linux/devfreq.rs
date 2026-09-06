//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/devfreq.h
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
// devfreq: Generic Dynamic Voltage and Frequency Scaling (DVFS) Framework
// for Non-CPU Devices.
//
// Copyright (C) 2011 Samsung Electronics
// MyungJoo Ham <myungjoo.ham@samsung.com>
//

// DEVFREQ governor name

// DEVFREQ notifier interface

// Transition notifiers of DEVFREQ_TRANSITION_NOTIFIER

// DEVFREQ work timers
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum devfreq_timer {
    DEVFREQ_TIMER_DEFERRABLE = 0,
    DEVFREQ_TIMER_DELAYED,
    DEVFREQ_TIMER_NUM,
}

//
// struct devfreq_dev_status - Data given from devfreq user device to
// governors. Represents the performance
// statistics.
// @total_time:		The total time represented by this instance of
// devfreq_dev_status
// @busy_time:		The time that the device was working among the
// total_time.
// @current_frequency:	The operating frequency.
// @private_data:	An entry not specified by the devfreq framework.
// A device and a specific governor may have their
// own protocol with private_data. However, because
// this is governor-specific, a governor using this
// will be only compatible with devices aware of it.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct devfreq_dev_status {
// both since the last measure
    pub total_time: c_ulong,
    pub busy_time: c_ulong,
    pub current_frequency: c_ulong,
    pub private_data: *mut c_void,
}

//
// The resulting frequency should be at most this. (this bound is the
// least upper bound; thus, the resulting freq should be lower or same)
// If the flag is not set, the resulting frequency should be at most the
// bound (greatest lower bound)
//
pub const DEVFREQ_FLAG_LEAST_UPPER_BOUND: c_uint = 0x1;
//
// struct devfreq_dev_profile - Devfreq's user device profile
// @initial_freq:	The operating frequency when devfreq_add_device() is
// called.
// @polling_ms:		The polling interval in ms. 0 disables polling.
// @timer:		Timer type is either deferrable or delayed timer.
// @target:		The device should set its operating frequency at
// freq or lowest-upper-than-freq value. If freq is
// higher than any operable frequency, set maximum.
// Before returning, target function should set
// freq at the current frequency.
// The "flags" parameter's possible values are
// explained above with "DEVFREQ_FLAG_*" macros.
// @get_dev_status:	The device should provide the current performance
// status to devfreq. Governors are recommended not to
// use this directly. Instead, governors are recommended
// to use devfreq_update_stats() along with
// devfreq.last_status.
// @get_cur_freq:	The device should provide the current frequency
// at which it is operating.
// @exit:		An optional callback that is called when devfreq
// is removing the devfreq object due to error or
// from devfreq_remove_device() call. If the user
// has registered devfreq->nb at a notifier-head,
// this is the time to unregister it.
// @freq_table:		Optional list of frequencies to support statistics
// and freq_table must be generated in ascending order.
// @max_state:		The size of freq_table.
//
// @is_cooling_device: A self-explanatory boolean giving the device a
// cooling effect property.
// @dev_groups:		Optional device-specific sysfs attribute groups that to
// be attached to the devfreq device.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct devfreq_dev_profile {
    pub initial_freq: c_ulong,
    pub polling_ms: c_uint,
    pub timer: devfreq_timer,
    pub flags): *mut *mut *mut *mut int (target)(struct device dev, unsigned long freq, u32,
    pub stat): *mut devfreq_dev_status,
    pub freq): *mut *mut *mut int (get_cur_freq)(struct device dev, unsigned long,
    pub dev): *mut *mut void (exit)(struct device,
    pub freq_table: *mut c_ulong,
    pub max_state: c_uint,
    pub is_cooling_device: bool,
    pub dev_groups: *const attribute_group,
}

//
// struct devfreq_stats - Statistics of devfreq device behavior
// @total_trans:	Number of devfreq transitions.
// @trans_table:	Statistics of devfreq transitions.
// @time_in_state:	Statistics of devfreq states.
// @last_update:	The last time stats were updated.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct devfreq_stats {
    pub total_trans: c_uint,
    pub trans_table: *mut c_uint,
    pub time_in_state: *mut u64,
    pub last_update: u64,
}

//
// struct devfreq - Device devfreq structure
// @node:	list node - contains the devices with devfreq that have been
// registered.
// @lock:	a mutex to protect accessing devfreq.
// @dev:	device registered by devfreq class. dev.parent is the device
// using devfreq.
// @profile:	device-specific devfreq profile
// @governor:	method how to choose frequency based on the usage.
// @opp_table:	Reference to OPP table of dev.parent, if one exists.
// @nb:		notifier block used to notify devfreq object that it should
// reevaluate operable frequencies. Devfreq users may use
// devfreq.nb to the corresponding register notifier call chain.
// @work:	delayed work for load monitoring.
// @freq_table:		current frequency table used by the devfreq driver.
// @max_state:		count of entry present in the frequency table.
// @previous_freq:	previously configured frequency value.
// @last_status:	devfreq user device info, performance statistics
// @data:	devfreq driver pass to governors, governor should not change it.
// @governor_data:	private data for governors, devfreq core doesn't touch it.
// @user_min_freq_req:	PM QoS minimum frequency request from user (via sysfs)
// @user_max_freq_req:	PM QoS maximum frequency request from user (via sysfs)
// @scaling_min_freq:	Limit minimum frequency requested by OPP interface
// @scaling_max_freq:	Limit maximum frequency requested by OPP interface
// @stop_polling:	 devfreq polling status of a device.
// @suspend_freq:	 frequency of a device set during suspend phase.
// @resume_freq:	 frequency of a device set in resume phase.
// @suspend_count:	 suspend requests counter for a device.
// @stats:	Statistics of devfreq device behavior
// @transition_notifier_list: list head of DEVFREQ_TRANSITION_NOTIFIER notifier
// @cdev:	Cooling device pointer if the devfreq has cooling property
// @nb_min:		Notifier block for DEV_PM_QOS_MIN_FREQUENCY
// @nb_max:		Notifier block for DEV_PM_QOS_MAX_FREQUENCY
//
// This structure stores the devfreq information for a given device.
//
// Note that when a governor accesses entries in struct devfreq in its
// functions except for the context of callbacks defined in struct
// devfreq_governor, the governor should protect its access with the
// struct mutex lock in struct devfreq. A governor may use this mutex
// to protect its own private data in ``void *data`` as well.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct devfreq {
    pub node: list_head,
    pub lock: mutex,
    pub dev: device,
    pub profile: *mut devfreq_dev_profile,
    pub governor: *const devfreq_governor,
    pub opp_table: *mut opp_table,
    pub nb: notifier_block,
    pub work: delayed_work,
    pub freq_table: *mut c_ulong,
    pub max_state: c_uint,
    pub previous_freq: c_ulong,
    pub last_status: devfreq_dev_status,
    pub data: *mut c_void,
    pub governor_data: *mut c_void,
    pub user_min_freq_req: dev_pm_qos_request,
    pub user_max_freq_req: dev_pm_qos_request,
    pub scaling_min_freq: c_ulong,
    pub scaling_max_freq: c_ulong,
    pub stop_polling: bool,
    pub suspend_freq: c_ulong,
    pub resume_freq: c_ulong,
    pub suspend_count: core::sync::atomic::AtomicI32,
// information for device frequency transitions
    pub stats: devfreq_stats,
    pub transition_notifier_list: srcu_notifier_head,
// Pointer to the cooling device if used for thermal mitigation
    pub cdev: *mut thermal_cooling_device,
    pub nb_min: notifier_block,
    pub nb_max: notifier_block,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct devfreq_freqs {
    pub old: c_ulong,
    pub new: c_ulong,
}

extern "C" {
    pub fn devfreq_remove_device(devfreq: *mut devfreq) -> c_int;
}
extern "C" {
    pub fn devm_devfreq_remove_device(dev: *mut device, devfreq: *mut devfreq);
}
// Supposed to be called by PM callbacks
extern "C" {
    pub fn devfreq_suspend_device(devfreq: *mut devfreq) -> c_int;
}
extern "C" {
    pub fn devfreq_resume_device(devfreq: *mut devfreq) -> c_int;
}
extern "C" {
    pub fn devfreq_suspend();
}
extern "C" {
    pub fn devfreq_resume();
}
// update_devfreq() - Reevaluate the device and configure frequency
extern "C" {
    pub fn update_devfreq(devfreq: *mut devfreq) -> c_int;
}
// Helper functions for devfreq user device driver with OPP.

//
// struct devfreq_simple_ondemand_data - ``void *data`` fed to struct devfreq
// and devfreq_add_device
// @upthreshold:	If the load is over this value, the frequency jumps.
// Specify 0 to use the default. Valid value = 0 to 100.
// @downdifferential:	If the load is under upthreshold - downdifferential,
// the governor may consider slowing the frequency down.
// Specify 0 to use the default. Valid value = 0 to 100.
// downdifferential < upthreshold must hold.
//
// If the fed devfreq_simple_ondemand_data pointer is NULL to the governor,
// the governor uses the default values.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct devfreq_simple_ondemand_data {
    pub upthreshold: c_uint,
    pub downdifferential: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum devfreq_parent_dev_type {
    DEVFREQ_PARENT_DEV,
    CPUFREQ_PARENT_DEV,
}

//
// struct devfreq_passive_data - ``void *data`` fed to struct devfreq
// and devfreq_add_device
// @parent:	the devfreq instance of parent device.
// @get_target_freq:	Optional callback, Returns desired operating frequency
// for the device using passive governor. That is called
// when passive governor should decide the next frequency
// by using the new frequency of parent devfreq device
// using governors except for passive governor.
// If the devfreq device has the specific method to decide
// the next frequency, should use this callback.
// @parent_type:	the parent type of the device.
// @this:		the devfreq instance of own device.
// @nb:			the notifier block for DEVFREQ_TRANSITION_NOTIFIER or
// CPUFREQ_TRANSITION_NOTIFIER list.
// @cpu_data_list:	the list of cpu frequency data for all cpufreq_policy.
//
// The devfreq_passive_data have to set the devfreq instance of parent
// device with governors except for the passive governor. But, don't need to
// initialize the 'this' and 'nb' field because the devfreq core will handle
// them.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct devfreq_passive_data {
// Should set the devfreq instance of parent device
    pub parent: *mut devfreq,
// Optional callback to decide the next frequency of passvice device
    pub freq): *mut *mut *mut int (get_target_freq)(struct devfreq this, unsigned long,
// Should set the type of parent device
    pub parent_type: devfreq_parent_dev_type,
// For passive governor's internal use. Don't need to set them
    pub this: *mut devfreq,
    pub nb: notifier_block,
    pub cpu_data_list: list_head,
}

extern "C" {
    pub fn ERR_PTR(_arg: -ENOSYS) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -ENOSYS) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -EINVAL) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -ENODEV) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -ENODEV) -> return;
}

