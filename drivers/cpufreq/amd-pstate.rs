//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/cpufreq/amd-pstate.h
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
// Copyright (C) 2022 Advanced Micro Devices, Inc.
//
// Author: Meng Li <li.meng@amd.com>
//

// Macro flag: #define EXPORT_SYMBOL_FOR_PSTATE_UT(symbol)

//
// AMD P-state INTERFACE
//
// union perf_cached - A union to cache performance-related data.
// @highest_perf: the maximum performance an individual processor may reach,
// assuming ideal conditions
// For platforms that support the preferred core feature, the highest_perf value maybe
// configured to any value in the range 166-255 by the firmware (because the preferred
// core ranking is encoded in the highest_perf value). To maintain consistency across
// all platforms, we split the highest_perf and preferred core ranking values into
// cpudata->perf.highest_perf and cpudata->prefcore_ranking.
// @nominal_perf: the maximum sustained performance level of the processor,
// assuming ideal operating conditions
// @lowest_nonlinear_perf: the lowest performance level at which nonlinear power
// savings are achieved
// @lowest_perf: the absolute lowest performance level of the processor
// @min_limit_perf: Cached value of the performance corresponding to policy->min
// @max_limit_perf: Cached value of the performance corresponding to policy->max
// @bios_min_perf: Cached perf value corresponding to the "Requested CPU Min Frequency" BIOS option
// @val: Raw 64-bit value for atomic access via READ_ONCE()/WRITE_ONCE()
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union perf_cached {
    pub highest_perf: u8,
    pub nominal_perf: u8,
    pub lowest_nonlinear_perf: u8,
    pub lowest_perf: u8,
    pub min_limit_perf: u8,
    pub max_limit_perf: u8,
    pub bios_min_perf: u8,
}

//
// struct  amd_aperf_mperf
// @aperf: actual performance frequency clock count
// @mperf: maximum performance frequency clock count
// @tsc:   time stamp counter
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amd_aperf_mperf {
    pub aperf: u64,
    pub mperf: u64,
    pub tsc: u64,
}

//
// struct amd_cpudata - private CPU data for AMD P-State
// @cpu: CPU number
// @req: constraint request to apply
// @cppc_req_cached: cached performance request hints
// @cppc_req2_cached: cached value of MSR_AMD_CPPC_REQ2
// @perf: cached performance-related data
// @prefcore_ranking: the preferred core ranking, the higher value indicates a higher
// priority.
// @floor_perf_cnt: Cached value of the number of distinct floor
// performance levels supported
// @bios_floor_perf: Cached value of the boot-time floor performance level from
// MSR_AMD_CPPC_REQ2
// @min_limit_freq: Cached value of policy->min (in khz)
// @max_limit_freq: Cached value of policy->max (in khz)
// @nominal_freq: the frequency (in khz) that mapped to nominal_perf
// @max_freq: in ideal conditions the maximum frequency (in khz) possible frequency
// @lowest_nonlinear_freq: the frequency (in khz) that mapped to lowest_nonlinear_perf
// @floor_freq: Cached value of the user requested floor_freq
// @cur: Difference of Aperf/Mperf/tsc count between last and current sample
// @prev: Last Aperf/Mperf/tsc count value read from register
// @freq: current cpu frequency value (in khz)
// @boost_supported: check whether the Processor or SBIOS supports boost mode
// @hw_prefcore: check whether HW supports preferred core featue.
// Only when hw_prefcore and early prefcore param are true,
// AMD P-State driver supports preferred core featue.
// @policy: Cpufreq policy value
// @suspended: If CPU core if offlined
// @epp_default_ac: Default EPP value for AC power source
// @epp_default_dc: Default EPP value for DC power source
// @dynamic_epp: Whether dynamic EPP is enabled
// @raw_epp: Whether the last EPP write was a raw numeric value rather than a
// named preference
// @power_nb: Notifier block for power events
// @current_profile: Currently selected platform profile option
// @ppdev: Device registered with the platform profile handler
// @profile_name: Name under which @ppdev is registered
//
// The amd_cpudata is key private data for each CPU thread in AMD P-State, and
// represents all the attributes and goals that AMD P-State requests at runtime.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amd_cpudata {
    pub cpu: c_int,
    pub req: [freq_qos_request; 2],
    pub cppc_req_cached: u64,
    pub cppc_req2_cached: u64,
    pub perf: perf_cached,
    pub prefcore_ranking: u8,
    pub floor_perf_cnt: u8,
    pub bios_floor_perf: u8,
    pub min_limit_freq: u32,
    pub max_limit_freq: u32,
    pub nominal_freq: u32,
    pub max_freq: u32,
    pub lowest_nonlinear_freq: u32,
    pub floor_freq: u32,
    pub cur: amd_aperf_mperf,
    pub prev: amd_aperf_mperf,
    pub freq: u64,
    pub boost_supported: bool,
    pub hw_prefcore: bool,
// EPP feature related attributes
    pub policy: u32,
    pub suspended: bool,
    pub epp_default_ac: u8,
    pub epp_default_dc: u8,
    pub dynamic_epp: bool,
    pub raw_epp: bool,
    pub power_nb: notifier_block,
// platform profile
    pub current_profile: platform_profile_option,
    pub ppdev: *mut device,
    pub profile_name: *mut c_char,
}

//
// enum amd_pstate_mode - driver working mode of amd pstate
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amd_pstate_mode {
    AMD_PSTATE_UNDEFINED = 0,
    AMD_PSTATE_DISABLE,
    AMD_PSTATE_PASSIVE,
    AMD_PSTATE_ACTIVE,
    AMD_PSTATE_GUIDED,
    AMD_PSTATE_MAX,
}

extern "C" {
    pub fn amd_pstate_get_status() -> c_int;
}
extern "C" {
    pub fn amd_pstate_update_status(buf: *const c_char, size: usize) -> c_int;
}
extern "C" {
    pub fn show_energy_performance_preference(policy: *mut cpufreq_policy, buf: *mut c_char) -> isize;
}
extern "C" {
    pub fn amd_pstate_clear_dynamic_epp(policy: *mut cpufreq_policy);
}
extern "C" {
    pub fn store_amd_pstate_floor_freq(policy: *mut cpufreq_policy, buf: *const c_char, count: usize) -> isize;
}
extern "C" {
    pub fn show_amd_pstate_floor_freq(policy: *mut cpufreq_policy, buf: *mut c_char) -> isize;
}
