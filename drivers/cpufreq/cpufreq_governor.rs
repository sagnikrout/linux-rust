//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/cpufreq/cpufreq_governor.h
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
// drivers/cpufreq/cpufreq_governor.h
//
// Header file for CPUFreq governors common code
//
// Copyright	(C) 2001 Russell King
// (C) 2003 Venkatesh Pallipadi <venkatesh.pallipadi@intel.com>.
// (C) 2003 Jun Nakajima <jun.nakajima@intel.com>
// (C) 2009 Alexander Clouter <alex@digriz.org.uk>
// (c) 2012 Viresh Kumar <viresh.kumar@linaro.org>
//

// Ondemand Sampling types
//
// Abbreviations:
// dbs: used as a shortform for demand based switching It helps to keep variable
// names smaller, simpler
// cdbs: common dbs
// od_*: On-demand governor
// cs_*: Conservative governor
//
// Governor demand based switching data (per-policy or global).
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dbs_data {
    pub attr_set: gov_attr_set,
    pub gov: *mut dbs_governor,
    pub tuners: *mut c_void,
    pub ignore_nice_load: c_uint,
    pub sampling_rate: c_uint,
    pub sampling_down_factor: c_uint,
    pub up_threshold: c_uint,
    pub io_is_busy: c_uint,
}

extern "C" {
    pub fn container_of(_arg: attr_set, dbs_data: struct, _arg: attr_set) -> return;
}

// Common to all CPUs of a policy
#[repr(C)]
#[derive(Copy, Clone)]
pub struct policy_dbs_info {
    pub policy: *mut cpufreq_policy,
//
// Per policy mutex that serializes load evaluation from limit-change
// and work-handler.
//
    pub update_mutex: mutex,
    pub last_sample_time: u64,
    pub sample_delay_ns: i64,
    pub work_count: core::sync::atomic::AtomicI32,
    pub irq_work: irq_work,
    pub work: work_struct,
// dbs_data may be shared between multiple policy objects
    pub dbs_data: *mut dbs_data,
    pub list: list_head,
// Multiplier for increasing sample delay temporarily.
    pub rate_mult: c_uint,
    pub /: *mut *mut unsigned int idle_periods; / For conservative,
// Status indicators
    pub /: *mut *mut bool is_shared; / This object is used by multiple CPUs,
    pub /: *mut *mut bool work_in_progress; / Work is being queued up or in progress,
}

// Per cpu structures
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpu_dbs_info {
    pub prev_cpu_idle: u64,
    pub prev_update_time: u64,
    pub prev_cpu_nice: u64,
//
// Used to keep track of load in the previous interval. However, when
// explicitly set to zero, it is used as a flag to ensure that we copy
// the previous load to the current interval only once, upon the first
// wake-up from idle.
//
    pub prev_load: c_uint,
    pub update_util: update_util_data,
    pub policy_dbs: *mut policy_dbs_info,
}

// Common Governor data across policies
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dbs_governor {
    pub gov: cpufreq_governor,
    pub kobj_type: kobj_type,
//
// Common data for platforms that don't set
// CPUFREQ_HAVE_GOVERNOR_PER_POLICY
//
    pub gdbs_data: *mut dbs_data,
    pub policy): *mut *mut unsigned int (gov_dbs_update)(struct cpufreq_policy,
    pub (*alloc)(void): *mut policy_dbs_info,
    pub policy_dbs): *mut *mut void (free)(struct policy_dbs_info,
    pub dbs_data): *mut *mut int (init)(struct dbs_data,
    pub dbs_data): *mut *mut void (exit)(struct dbs_data,
    pub policy): *mut *mut void (start)(struct cpufreq_policy,
    pub policy): *mut *mut void (limits)(struct cpufreq_policy,
}

extern "C" {
    pub fn container_of(_arg: policy->governor, dbs_governor: struct, _arg: gov) -> return;
}
// Governor callback routines
extern "C" {
    pub fn cpufreq_dbs_governor_init(policy: *mut cpufreq_policy) -> c_int;
}
extern "C" {
    pub fn cpufreq_dbs_governor_exit(policy: *mut cpufreq_policy);
}
extern "C" {
    pub fn cpufreq_dbs_governor_start(policy: *mut cpufreq_policy) -> c_int;
}
extern "C" {
    pub fn cpufreq_dbs_governor_stop(policy: *mut cpufreq_policy);
}
extern "C" {
    pub fn cpufreq_dbs_governor_limits(policy: *mut cpufreq_policy);
}

// Governor specific operations
#[repr(C)]
#[derive(Copy, Clone)]
pub struct od_ops {
    pub relation): unsigned int freq_next, unsigned int,
}

extern "C" {
    pub fn dbs_update(policy: *mut cpufreq_policy) -> c_uint;
}
extern "C" {
    pub fn od_unregister_powersave_bias_handler();
}
extern "C" {
    pub fn gov_update_cpu_data(dbs_data: *mut dbs_data);
}
