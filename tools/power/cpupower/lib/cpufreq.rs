//! Automatically rewritten from C Header to Rust Module
//! Source: tools/power/cpupower/lib/cpufreq.h
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
// cpufreq.h - definitions for libcpufreq
//
// Copyright (C) 2004-2009  Dominik Brodowski <linux@dominikbrodowski.de>
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpufreq_policy {
    pub min: c_ulong,
    pub max: c_ulong,
    pub governor: *mut c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpufreq_available_governors {
    pub governor: *mut c_char,
    pub next: *mut cpufreq_available_governors,
    pub first: *mut cpufreq_available_governors,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpufreq_available_frequencies {
    pub frequency: c_ulong,
    pub next: *mut cpufreq_available_frequencies,
    pub first: *mut cpufreq_available_frequencies,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpufreq_affected_cpus {
    pub cpu: c_uint,
    pub next: *mut cpufreq_affected_cpus,
    pub first: *mut cpufreq_affected_cpus,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpufreq_stats {
    pub frequency: c_ulong,
    pub time_in_state: c_ulonglong,
    pub next: *mut cpufreq_stats,
    pub first: *mut cpufreq_stats,
}

// determine current CPU frequency
// - _kernel variant means kernel's opinion of CPU frequency
// - _hardware variant means actual hardware CPU frequency,
// which is only available to root.
//
// returns 0 on failure, else frequency in kHz.
//
extern "C" {
    pub fn cpufreq_get_freq_kernel(cpu: c_uint) -> c_ulong;
}
extern "C" {
    pub fn cpufreq_get_freq_hardware(cpu: c_uint) -> c_ulong;
}

// determine CPU transition latency
//
// returns 0 on failure, else transition latency in 10^(-9) s = nanoseconds
//
extern "C" {
    pub fn cpufreq_get_transition_latency(cpu: c_uint) -> c_ulong;
}
// determine energy performance preference
//
// returns NULL on failure, else the string that represents the energy performance
// preference requested.
//
extern "C" {
    pub fn cpufreq_put_energy_performance_preference(ptr: *mut c_char);
}
// determine hardware CPU frequency limits
//
// These may be limited further by thermal, energy or other
// considerations by cpufreq policy notifiers in the kernel.
//
// determine CPUfreq driver used
//
// Remember to call cpufreq_put_driver when no longer needed
// to avoid memory leakage, please.
//
extern "C" {
    pub fn cpufreq_put_driver(ptr: *mut c_char);
}
// determine CPUfreq policy currently used
//
// Remember to call cpufreq_put_policy when no longer needed
// to avoid memory leakage, please.
//
extern "C" {
    pub fn cpufreq_put_policy(policy: *mut cpufreq_policy);
}
// determine CPUfreq governors currently available
//
// may be modified by modprobe'ing or rmmod'ing other governors. Please
// free allocated memory by calling cpufreq_put_available_governors
// after use.
//
// cpufreq_get_available_governors(unsigned int cpu);
// determine CPU frequency states available
//
// Only present on _some_ ->target() cpufreq drivers. For information purposes
// only. Please free allocated memory by calling
// cpufreq_put_frequencies after use.
//
// cpufreq_get_available_frequencies(unsigned int cpu);
// cpufreq_get_boost_frequencies(unsigned int cpu);
// determine affected CPUs
//
// Remember to call cpufreq_put_affected_cpus when no longer needed
// to avoid memory leakage, please.
//
extern "C" {
    pub fn cpufreq_put_affected_cpus(first: *mut cpufreq_affected_cpus);
}
// determine related CPUs
//
// Remember to call cpufreq_put_related_cpus when no longer needed
// to avoid memory leakage, please.
//
extern "C" {
    pub fn cpufreq_put_related_cpus(first: *mut cpufreq_affected_cpus);
}
// determine stats for cpufreq subsystem
//
// This is not available in all kernel versions or configurations.
//
extern "C" {
    pub fn cpufreq_put_stats(stats: *mut cpufreq_stats);
}
extern "C" {
    pub fn cpufreq_get_transitions(cpu: c_uint) -> c_ulong;
}
// set new cpufreq policy
//
// Tries to set the passed policy as new policy as close as possible,
// but results may differ depending e.g. on governors being available.
//
extern "C" {
    pub fn cpufreq_set_policy(cpu: c_uint, policy: *mut cpufreq_policy) -> c_int;
}
// modify a policy by only changing min/max freq or governor
//
// Does not check whether result is what was intended.
//
extern "C" {
    pub fn cpufreq_modify_policy_min(cpu: c_uint, min_freq: c_ulong) -> c_int;
}
extern "C" {
    pub fn cpufreq_modify_policy_max(cpu: c_uint, max_freq: c_ulong) -> c_int;
}
extern "C" {
    pub fn cpufreq_modify_policy_governor(cpu: c_uint, governor: *mut c_char) -> c_int;
}
// set a specific frequency
//
// Does only work if userspace governor can be used and no external
// interference (other calls to this function or to set/modify_policy)
// occurs. Also does not work on ->range() cpufreq drivers.
//
// get the sysfs value from specific table
//
// Read the value with the sysfs file name from specific table. Does
// only work if the cpufreq driver has the specific sysfs interfaces.
//

