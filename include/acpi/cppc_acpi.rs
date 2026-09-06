//! Automatically rewritten from C Header to Rust Module
//! Source: include/acpi/cppc_acpi.h
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
// CPPC (Collaborative Processor Performance Control) methods used
// by CPUfreq drivers.
//
// (C) Copyright 2014, 2015 Linaro Ltd.
// Author: Ashwin Chaugule <ashwin.chaugule@linaro.org>
//

// CPPCv2, CPPCv3 and CPPCv4 support
pub const CPPC_V2_REV: c_int = 2;
pub const CPPC_V3_REV: c_int = 3;
pub const CPPC_V4_REV: c_int = 4;
pub const CPPC_V2_NUM_ENT: c_int = 21;
pub const CPPC_V3_NUM_ENT: c_int = 23;
pub const CPPC_V4_NUM_ENT: c_int = 25;

pub const MAX_CPC_REG_ENT: c_int = 23;
// CPPC specific PCC commands.
pub const CMD_READ: c_int = 0;
pub const CMD_WRITE: c_int = 1;

// CPPC_AUTO_ACT_WINDOW_MAX_SIG is 127, so 128 and 129 will decay to 127 when writing
pub const CPPC_AUTO_ACT_WINDOW_SIG_CARRY_THRESH: c_int = 129;
pub const CPPC_EPP_PERFORMANCE_PREF: c_uint = 0x00;
pub const CPPC_EPP_ENERGY_EFFICIENCY_PREF: c_uint = 0xFF;

// Each register has the folowing format.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpc_reg {
    pub descriptor: u8,
    pub length: u16,
    pub space_id: u8,
    pub bit_width: u8,
    pub bit_offset: u8,
    pub access_width: u8,
    pub address: u64,
    pub __packed: },
//
// Each entry in the CPC table is either
// of type ACPI_TYPE_BUFFER or
// ACPI_TYPE_INTEGER.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpc_register_resource {
    pub type: acpi_object_type,
    pub sys_mem_vaddr: *mut u64 __iomem,
    pub reg: cpc_reg,
    pub use_rmw_lock: bool,
}

// Container to hold the CPC details for each CPU
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpc_desc {
    pub num_entries: c_int,
    pub version: c_int,
    pub cpu_id: c_int,
    pub write_cmd_status: c_int,
    pub write_cmd_id: c_int,
// Lock used for RMW operations in cpc_write()
    pub rmw_lock: raw_spinlock_t,
    pub cpc_regs: [cpc_register_resource; MAX_CPC_REG_ENT],
    pub domain_info: acpi_psd_package,
    pub kobj: kobject,
}

// These are indexes into the per-cpu cpc_regs[]. Order is important.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cppc_regs {
    HIGHEST_PERF,
    NOMINAL_PERF,
    LOW_NON_LINEAR_PERF,
    LOWEST_PERF,
    GUARANTEED_PERF,
    DESIRED_PERF,
    MIN_PERF,
    MAX_PERF,
    PERF_REDUC_TOLERANCE,
    TIME_WINDOW,
    CTR_WRAP_TIME,
    REFERENCE_CTR,
    DELIVERED_CTR,
    PERF_LIMITED,
    ENABLE,
    AUTO_SEL_ENABLE,
    AUTO_ACT_WINDOW,
    ENERGY_PERF,
    REFERENCE_PERF,
    LOWEST_FREQ,
    NOMINAL_FREQ,
    OSPM_NOMINAL_PERF,
    RESOURCE_PRIORITY,
}

//
// Categorization of registers as described
// in the ACPI v.5.1 spec.
// XXX: Only filling up ones which are used by governors
// today.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cppc_perf_caps {
    pub guaranteed_perf: u32,
    pub highest_perf: u32,
    pub nominal_perf: u32,
    pub reference_perf: u32,
    pub lowest_perf: u32,
    pub lowest_nonlinear_perf: u32,
    pub lowest_freq: u32,
    pub nominal_freq: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cppc_perf_ctrls {
    pub max_perf: u32,
    pub min_perf: u32,
    pub desired_perf: u32,
    pub energy_perf: u32,
    pub auto_sel: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cppc_perf_fb_ctrs {
    pub reference: u64,
    pub delivered: u64,
    pub wraparound_time: u64,
}

// Per CPU container for runtime CPPC management.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cppc_cpudata {
    pub perf_caps: cppc_perf_caps,
    pub perf_ctrls: cppc_perf_ctrls,
    pub perf_fb_ctrs: cppc_perf_fb_ctrs,
    pub shared_type: c_uint,
    pub shared_cpu_map: cpumask_var_t,
}

extern "C" {
    pub fn cppc_get_desired_perf(cpunum: c_int, desired_perf: *mut u64) -> c_int;
}
extern "C" {
    pub fn cppc_get_nominal_perf(cpunum: c_int, nominal_perf: *mut u64) -> c_int;
}
extern "C" {
    pub fn cppc_get_highest_perf(cpunum: c_int, highest_perf: *mut u64) -> c_int;
}
extern "C" {
    pub fn cppc_get_perf_ctrs(cpu: c_int, perf_fb_ctrs: *mut cppc_perf_fb_ctrs) -> c_int;
}
extern "C" {
    pub fn cppc_get_perf(cpu: c_int, perf_ctrls: *mut cppc_perf_ctrls) -> c_int;
}
extern "C" {
    pub fn cppc_set_perf(cpu: c_int, perf_ctrls: *mut cppc_perf_ctrls) -> c_int;
}
extern "C" {
    pub fn cppc_set_enable(cpu: c_int, enable: bool) -> c_int;
}
extern "C" {
    pub fn cppc_get_perf_caps(cpu: c_int, caps: *mut cppc_perf_caps) -> c_int;
}
extern "C" {
    pub fn cppc_perf_ctrs_in_pcc_cpu(cpu: c_uint) -> bool;
}
extern "C" {
    pub fn cppc_perf_ctrs_in_pcc() -> bool;
}
extern "C" {
    pub fn cppc_get_dmi_max_khz() -> u64;
}
extern "C" {
    pub fn cppc_perf_to_khz(caps: *mut cppc_perf_caps, perf: c_uint) -> c_uint;
}
extern "C" {
    pub fn cppc_khz_to_perf(caps: *mut cppc_perf_caps, freq: c_uint) -> c_uint;
}
extern "C" {
    pub fn acpi_cpc_valid() -> bool;
}
extern "C" {
    pub fn cppc_allow_fast_switch(cpus: *const cpumask) -> bool;
}
extern "C" {
    pub fn acpi_get_psd_map(cpu: c_uint, cpu_data: *mut cppc_cpudata) -> c_int;
}
extern "C" {
    pub fn cppc_get_transition_latency(cpu: c_int) -> c_int;
}
extern "C" {
    pub fn cpc_ffh_supported() -> bool;
}
extern "C" {
    pub fn cpc_supported_by_cpu() -> bool;
}
extern "C" {
    pub fn cpc_read_ffh(cpunum: c_int, reg: *mut cpc_reg, val: *mut u64) -> c_int;
}
extern "C" {
    pub fn cpc_write_ffh(cpunum: c_int, reg: *mut cpc_reg, val: u64) -> c_int;
}
extern "C" {
    pub fn cppc_get_epp_perf(cpunum: c_int, epp_perf: *mut u64) -> c_int;
}
extern "C" {
    pub fn cppc_set_epp_perf(cpu: c_int, perf_ctrls: *mut cppc_perf_ctrls, enable: bool) -> c_int;
}
extern "C" {
    pub fn cppc_set_epp(cpu: c_int, epp_val: u64) -> c_int;
}
extern "C" {
    pub fn cppc_get_auto_act_window(cpu: c_int, auto_act_window: *mut u64) -> c_int;
}
extern "C" {
    pub fn cppc_set_auto_act_window(cpu: c_int, auto_act_window: u64) -> c_int;
}
extern "C" {
    pub fn cppc_get_auto_sel(cpu: c_int, enable: *mut bool) -> c_int;
}
extern "C" {
    pub fn cppc_set_auto_sel(cpu: c_int, enable: bool) -> c_int;
}
extern "C" {
    pub fn cppc_get_perf_limited(cpu: c_int, perf_limited: *mut u64) -> c_int;
}
extern "C" {
    pub fn cppc_set_perf_limited(cpu: c_int, bits_to_clear: u64) -> c_int;
}
extern "C" {
    pub fn amd_get_highest_perf(cpu: c_uint, highest_perf: *mut u32) -> c_int;
}
extern "C" {
    pub fn amd_get_boost_ratio_numerator(cpu: c_uint, numerator: *mut u64) -> c_int;
}
extern "C" {
    pub fn amd_detect_prefcore(detected: *mut bool) -> c_int;
}

