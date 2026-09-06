//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/intel_rapl.h
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


// SPDX-License-Identifier: GPL-2.0
//
// Data types and headers for RAPL support
//
// Copyright (C) 2019  Intel Corporation.
//
// Author: Zhang Rui <rui.zhang@intel.com>
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rapl_if_type {
    RAPL_IF_MSR,	/* RAPL I/F using MSR registers */
    RAPL_IF_MMIO,	/* RAPL I/F using MMIO registers */
    RAPL_IF_TPMI,	/* RAPL I/F using TPMI registers */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rapl_domain_type {
    RAPL_DOMAIN_PACKAGE,	/* entire package/socket */
    RAPL_DOMAIN_PP0,	/* core power plane */
    RAPL_DOMAIN_PP1,	/* graphics uncore */
    RAPL_DOMAIN_DRAM,	/* DRAM control_type */
    RAPL_DOMAIN_PLATFORM,	/* PSys control_type */
    RAPL_DOMAIN_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rapl_domain_reg_id {
    RAPL_DOMAIN_REG_LIMIT,
    RAPL_DOMAIN_REG_STATUS,
    RAPL_DOMAIN_REG_PERF,
    RAPL_DOMAIN_REG_POLICY,
    RAPL_DOMAIN_REG_INFO,
    RAPL_DOMAIN_REG_PL4,
    RAPL_DOMAIN_REG_UNIT,
    RAPL_DOMAIN_REG_PL2,
    RAPL_DOMAIN_REG_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rapl_primitives {
    POWER_LIMIT1,
    POWER_LIMIT2,
    POWER_LIMIT4,
    ENERGY_COUNTER,
    FW_LOCK,
    FW_HIGH_LOCK,
    PL1_LOCK,
    PL2_LOCK,
    PL4_LOCK,

    PL1_ENABLE,		/* power limit 1, aka long term */
    PL1_CLAMP,		/* allow frequency to go below OS request */
    PL2_ENABLE,		/* power limit 2, aka short term, instantaneous */
    PL2_CLAMP,
    PL4_ENABLE,		/* power limit 4, aka max peak power */

    TIME_WINDOW1,		/* long term */
    TIME_WINDOW2,		/* short term */
    THERMAL_SPEC_POWER,
    MAX_POWER,

    MIN_POWER,
    MAX_TIME_WINDOW,
    THROTTLED_TIME,
    PRIORITY_LEVEL,

    PSYS_POWER_LIMIT1,
    PSYS_POWER_LIMIT2,
    PSYS_PL1_ENABLE,
    PSYS_PL2_ENABLE,
    PSYS_TIME_WINDOW1,
    PSYS_TIME_WINDOW2,
// below are not raw primitive data
    NR_RAPL_PRIMITIVES,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rapl_domain_data {
    pub primitives: [u64; NR_RAPL_PRIMITIVES],
    pub timestamp: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rapl_power_limit {
    pub constraint: *mut powercap_zone_constraint,
    pub domain: *mut rapl_domain,
    pub name: *const c_char,
    pub locked: bool,
    pub last_power_limit: u64,
}

pub const RAPL_DOMAIN_NAME_LENGTH: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub union rapl_reg {
    pub mmio: *mut void __iomem,
    pub msr: u32,
    pub val: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rapl_domain {
    pub name: [c_char; RAPL_DOMAIN_NAME_LENGTH],
    pub id: rapl_domain_type,
    pub regs: [rapl_reg; RAPL_DOMAIN_REG_MAX],
    pub power_zone: powercap_zone,
    pub rdd: rapl_domain_data,
    pub rpl: [rapl_power_limit; NR_POWER_LIMITS],
    pub /: *mut *mut u64 attr_map; / track capabilities,
    pub state: c_uint,
    pub power_unit: c_uint,
    pub energy_unit: c_uint,
    pub time_unit: c_uint,
    pub rp: *mut rapl_package,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct reg_action {
    pub reg: rapl_reg,
    pub mask: u64,
    pub value: u64,
    pub err: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rapl_defaults {
    pub floor_freq_reg_addr: u8,
    pub rd): *mut *mut int (check_unit)(struct rapl_domain,
    pub mode): *mut *mut *mut void (set_floor_freq)(struct rapl_domain rd, bool,
    pub to_raw): *mut *mut *mut u64 (compute_time_window)(struct rapl_domain rd, u64 val, bool,
    pub dram_domain_energy_unit: c_uint,
    pub psys_domain_energy_unit: c_uint,
    pub spr_psys_bits: bool,
    pub msr_pl4_support: bool,
    pub msr_pmu_support: bool,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum unit_type {
    ARBITRARY_UNIT,		/* no translation */
    POWER_UNIT,
    ENERGY_UNIT,
    TIME_UNIT,
}

// per domain data. used to describe individual knobs such that access function
// can be consolidated into one instead of many inline functions.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rapl_primitive_info {
    pub name: *const c_char,
    pub mask: u64,
    pub shift: c_int,
    pub id: rapl_domain_reg_id,
    pub unit: unit_type,
    pub flag: u32,
}

//
// struct rapl_if_priv: private data for different RAPL interfaces
// @control_type:		Each RAPL interface must have its own powercap
// control type.
// @platform_rapl_domain:	Optional. Some RAPL interface may have platform
// level RAPL control.
// @pcap_rapl_online:		CPU hotplug state for each RAPL interface.
// @reg_unit:			Register for getting energy/power/time unit.
// @regs:			Register sets for different RAPL Domains.
// @limits:			Number of power limits supported by each domain.
// @read_raw:			Callback for reading RAPL interface specific
// registers.
// @write_raw:			Callback for writing RAPL interface specific
// registers.
// @defaults:			pointer to default settings
// @rpi:			pointer to interface primitive info
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rapl_if_priv {
    pub type: rapl_if_type,
    pub control_type: *mut powercap_control_type,
    pub pcap_rapl_online: cpuhp_state,
    pub reg_unit: rapl_reg,
    pub regs: [rapl_reg; RAPL_DOMAIN_MAX][RAPL_DOMAIN_REG_MAX],
    pub limits: [c_int; RAPL_DOMAIN_MAX],
    pub pmu_ctx): *mut *mut *mut int (read_raw)(int id, struct reg_action ra, bool,
    pub ra): *mut *mut int (write_raw)(int id, struct reg_action,
    pub defaults: *const rapl_defaults,
    pub rpi: *mut rapl_primitive_info,
}

//
// struct rapl_package_pmu_data: Per package data for PMU support
// @scale:		Scale of 2^-32 Joules for each energy counter increase.
// @lock:		Lock to protect n_active and active_list.
// @n_active:		Number of active events.
// @active_list:	List of active events.
// @timer_interval:	Maximum timer expiration time before counter overflow.
// @hrtimer:		Periodically update the counter to prevent overflow.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rapl_package_pmu_data {
    pub scale: [u64; RAPL_DOMAIN_MAX],
    pub lock: raw_spinlock_t,
    pub n_active: c_int,
    pub active_list: list_head,
    pub timer_interval: ktime_t,
    pub hrtimer: hrtimer,
}

// maximum rapl package domain name: package-%d-die-%d
pub const PACKAGE_DOMAIN_NAME_LENGTH: c_int = 30;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rapl_package {
    pub /: *mut *mut unsigned int id; / logical die id, equals physical 1-die systems,
    pub nr_domains: c_uint,
    pub /: *mut *mut unsigned long domain_map; / bit map of active domains,
    pub /: *mut *mut *mut rapl_domain domains; / array of domains, sized at runtime,
    pub /: *mut *mut *mut powercap_zone power_zone; / keep track of parent zone,
    pub limit: *mut *mut unsigned long power_limit_irq; / keep track of package power,
// notify interrupt enable status.
//
    pub plist: list_head,
    pub /: *mut *mut int lead_cpu; / one active cpu per package for access,
// Track active cpus
    pub cpumask: cpumask,
    pub name: [c_char; PACKAGE_DOMAIN_NAME_LENGTH],
    pub priv: *mut rapl_if_priv,

    pub has_pmu: bool,
    pub pmu_data: rapl_package_pmu_data,

}

extern "C" {
    pub fn rapl_remove_package_cpuslocked(rp: *mut rapl_package);
}
extern "C" {
    pub fn rapl_remove_package(rp: *mut rapl_package);
}
extern "C" {
    pub fn rapl_default_check_unit(rd: *mut rapl_domain) -> c_int;
}
extern "C" {
    pub fn rapl_default_set_floor_freq(rd: *mut rapl_domain, mode: bool);
}
extern "C" {
    pub fn rapl_default_compute_time_window(rd: *mut rapl_domain, value: u64, to_raw: bool) -> u64;
}

extern "C" {
    pub fn rapl_package_add_pmu(rp: *mut rapl_package) -> c_int;
}
extern "C" {
    pub fn rapl_package_add_pmu_locked(rp: *mut rapl_package) -> c_int;
}
extern "C" {
    pub fn rapl_package_remove_pmu(rp: *mut rapl_package);
}
extern "C" {
    pub fn rapl_package_remove_pmu_locked(rp: *mut rapl_package);
}

