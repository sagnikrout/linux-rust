//! Automatically rewritten from C Header to Rust Module
//! Source: tools/power/x86/intel-speed-select/isst.h
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
// Intel Speed Select -- Enumerate and control features
// Copyright (c) 2019 Intel Corporation.
//

pub const CONFIG_TDP: c_uint = 0x7f;
pub const CONFIG_TDP_GET_LEVELS_INFO: c_uint = 0x00;
pub const CONFIG_TDP_GET_TDP_CONTROL: c_uint = 0x01;
pub const CONFIG_TDP_SET_TDP_CONTROL: c_uint = 0x02;
pub const CONFIG_TDP_GET_TDP_INFO: c_uint = 0x03;
pub const CONFIG_TDP_GET_PWR_INFO: c_uint = 0x04;
pub const CONFIG_TDP_GET_TJMAX_INFO: c_uint = 0x05;
pub const CONFIG_TDP_GET_CORE_MASK: c_uint = 0x06;
pub const CONFIG_TDP_GET_TURBO_LIMIT_RATIOS: c_uint = 0x07;
pub const CONFIG_TDP_SET_LEVEL: c_uint = 0x08;

pub const CONFIG_TDP_GET_P1_INFO: c_uint = 0x0a;
pub const CONFIG_TDP_GET_MEM_FREQ: c_uint = 0x0b;
pub const CONFIG_TDP_GET_RATIO_INFO: c_uint = 0x0c;
pub const CONFIG_TDP_GET_FACT_HP_TURBO_LIMIT_NUMCORES: c_uint = 0x10;
pub const CONFIG_TDP_GET_FACT_HP_TURBO_LIMIT_RATIOS: c_uint = 0x11;
pub const CONFIG_TDP_GET_FACT_LP_CLIPPING_RATIO: c_uint = 0x12;
pub const CONFIG_TDP_PBF_GET_CORE_MASK_INFO: c_uint = 0x20;
pub const CONFIG_TDP_PBF_GET_P1HI_P1LO_INFO: c_uint = 0x21;
pub const CONFIG_TDP_PBF_GET_TJ_MAX_INFO: c_uint = 0x22;

pub const CONFIG_CLOS: c_uint = 0xd0;
pub const CLOS_PQR_ASSOC: c_uint = 0x00;
pub const CLOS_PM_CLOS: c_uint = 0x01;
pub const CLOS_PM_QOS_CONFIG: c_uint = 0x02;
pub const CLOS_STATUS: c_uint = 0x03;
pub const MBOX_CMD_WRITE_BIT: c_uint = 0x08;
pub const PM_QOS_INFO_OFFSET: c_uint = 0x00;
pub const PM_QOS_CONFIG_OFFSET: c_uint = 0x04;
pub const PM_CLOS_OFFSET: c_uint = 0x08;
pub const PQR_ASSOC_OFFSET: c_uint = 0x20;
pub const READ_PM_CONFIG: c_uint = 0x94;
pub const WRITE_PM_CONFIG: c_uint = 0x95;
pub const PM_FEATURE: c_uint = 0x03;
pub const DISP_FREQ_MULTIPLIER: c_int = 100;
pub const MAX_PACKAGE_COUNT: c_int = 32;
pub const MAX_DIE_PER_PACKAGE: c_int = 16;
pub const MAX_PUNIT_PER_DIE: c_int = 8;
// Unified structure to specific a CPU or a Power Domain
#[repr(C)]
#[derive(Copy, Clone)]
pub struct isst_id {
    pub cpu: c_int,
    pub pkg: c_int,
    pub die: c_int,
    pub punit: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct isst_clos_config {
    pub clos_min: c_uint,
    pub clos_max: c_uint,
    pub epp: c_uchar,
    pub clos_prop_prio: c_uchar,
    pub clos_desired: c_uchar,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct isst_fact_bucket_info {
    pub hp_cores: c_int,
    pub hp_ratios: [c_int; TRL_MAX_LEVELS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct isst_pbf_info {
    pub pbf_acticated: c_int,
    pub pbf_available: c_int,
    pub core_cpumask_size: usize,
    pub core_cpumask: *mut cpu_set_t,
    pub p1_high: c_int,
    pub p1_low: c_int,
    pub t_control: c_int,
    pub t_prochot: c_int,
    pub tdp: c_int,
}

pub const ISST_TRL_MAX_ACTIVE_CORES: c_int = 8;
pub const ISST_FACT_MAX_BUCKETS: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct isst_fact_info {
    pub lp_ratios: [c_int; TRL_MAX_LEVELS],
    pub bucket_info: [isst_fact_bucket_info; ISST_FACT_MAX_BUCKETS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct isst_pkg_ctdp_level_info {
    pub processed: c_int,
    pub control_cpu: c_int,
    pub pkg_id: c_int,
    pub die_id: c_int,
    pub level: c_int,
    pub fact_support: c_int,
    pub pbf_support: c_int,
    pub fact_enabled: c_int,
    pub pbf_enabled: c_int,
    pub sst_cp_support: c_int,
    pub sst_cp_enabled: c_int,
    pub tdp_ratio: c_int,
    pub active: c_int,
    pub tdp_control: c_int,
    pub pkg_tdp: c_int,
    pub pkg_min_power: c_int,
    pub pkg_max_power: c_int,
    pub fact: c_int,
    pub t_proc_hot: c_int,
    pub cooling_type: c_int,
    pub uncore_p0: c_int,
    pub uncore_p1: c_int,
    pub uncore_pm: c_int,
    pub uncore1_p0: c_int,
    pub uncore1_p1: c_int,
    pub uncore1_pm: c_int,
    pub sse_p1: c_int,
    pub avx2_p1: c_int,
    pub avx512_p1: c_int,
    pub amx_p1: c_int,
    pub mem_freq: c_int,
    pub core_cpumask_size: usize,
    pub core_cpumask: *mut cpu_set_t,
    pub cpu_count: c_int,
    pub /: *mut *mut unsigned long long trl_cores; / Buckets info,
    pub trl_ratios: [c_int; TRL_MAX_LEVELS][ISST_TRL_MAX_ACTIVE_CORES],
    pub kobj_bucket_index: c_int,
    pub active_bucket: c_int,
    pub fact_max_index: c_int,
    pub fact_max_config: c_int,
    pub pbf_found: c_int,
    pub pbf_active: c_int,
    pub pbf_info: isst_pbf_info,
    pub fact_info: isst_fact_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct isst_pkg_ctdp {
    pub locked: c_int,
    pub version: c_int,
    pub processed: c_int,
    pub levels: c_int,
    pub current_level: c_int,
    pub enabled: c_int,
    pub ctdp_level: [isst_pkg_ctdp_level_info; ISST_MAX_TDP_LEVELS],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum isst_platform_param {
    ISST_PARAM_MBOX_DELAY,
    ISST_PARAM_MBOX_RETRIES,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct isst_platform_ops {
    pub (*get_disp_freq_multiplier)(void): *mut c_int,
    pub (*get_trl_max_levels)(void): *mut c_int,
    pub level): *mut *mut *mut char (get_trl_level_name)(int,
    pub value): *mut *mut void (update_platform_param)(enum isst_platform_param param, int,
    pub id): *mut *mut int (is_punit_valid)(struct isst_id,
    pub cp_cap): *mut *mut *mut *mut int (read_pm_config)(struct isst_id id, int cp_state, int,
    pub pkg_ctdp): *mut *mut *mut int (get_config_levels)(struct isst_id id, struct isst_pkg_ctdp,
    pub ctdp_level): *mut *mut *mut int (get_ctdp_control)(struct isst_id id, int config_index, struct isst_pkg_ctdp_level_info,
    pub ctdp_level): *mut *mut *mut int (get_tdp_info)(struct isst_id id, int config_index, struct isst_pkg_ctdp_level_info,
    pub ctdp_level): *mut *mut *mut int (get_pwr_info)(struct isst_id id, int config_index, struct isst_pkg_ctdp_level_info,
    pub ctdp_level): *mut *mut *mut int (get_coremask_info)(struct isst_id id, int config_index, struct isst_pkg_ctdp_level_info,
    pub trl): *mut *mut *mut int (get_get_trl)(struct isst_id id, int level, int avx_level, int,
    pub ctdp_level): *mut *mut *mut int (get_get_trls)(struct isst_id id, int level, struct isst_pkg_ctdp_level_info,
    pub buckets_info): *mut *mut *mut int (get_trl_bucket_info)(struct isst_id id, int level, unsigned long long,
    pub tdp_level): *mut *mut *mut int (set_tdp_level)(struct isst_id id, int,
    pub pbf_info): *mut *mut *mut int (get_pbf_info)(struct isst_id id, int level, struct isst_pbf_info,
    pub enable): *mut *mut *mut int (set_pbf_fact_status)(struct isst_id id, int pbf, int,
    pub fact_info): *mut *mut *mut int (get_fact_info)(struct isst_id id, int level, int fact_bucket, struct isst_fact_info,
    pub ctdp_level): *mut *mut *mut void (adjust_uncore_freq)(struct isst_id id, int config_index, struct isst_pkg_ctdp_level_info,
    pub type): *mut *mut *mut *mut int (get_clos_information)(struct isst_id id, int enable, int,
    pub priority_type): *mut *mut *mut int (pm_qos_config)(struct isst_id id, int enable_clos, int,
    pub clos_config): *mut *mut *mut int (pm_get_clos)(struct isst_id id, int clos, struct isst_clos_config,
    pub clos_config): *mut *mut *mut int (set_clos)(struct isst_id id, int clos, struct isst_clos_config,
    pub clos_id): *mut *mut *mut int (clos_get_assoc_status)(struct isst_id id, int,
    pub clos_id): *mut *mut *mut int (clos_associate)(struct isst_id id, int,
}

extern "C" {
    pub fn is_cpu_in_power_domain(cpu: c_int, id: *mut isst_id) -> c_int;
}
extern "C" {
    pub fn get_topo_max_cpus() -> c_int;
}
extern "C" {
    pub fn get_cpu_count(id: *mut isst_id) -> c_int;
}
extern "C" {
    pub fn get_max_punit_core_id(id: *mut isst_id) -> c_int;
}
extern "C" {
    pub fn api_version() -> c_int;
}
// Common interfaces
extern "C" {
    pub fn is_debug_enabled() -> c_int;
}
extern "C" {
    pub fn debug_printf(format: *const c_char, ...);
}
extern "C" {
    pub fn out_format_is_json() -> c_int;
}
extern "C" {
    pub fn set_isst_id(id: *mut isst_id, cpu: c_int);
}
extern "C" {
    pub fn alloc_cpu_set(cpu_set: *mut cpu_set_t) -> usize;
}
extern "C" {
    pub fn free_cpu_set(cpu_set: *mut cpu_set_t);
}
extern "C" {
    pub fn find_phy_core_num(logical_cpu: c_int) -> c_int;
}
extern "C" {
    pub fn isst_set_platform_ops(api_version: c_int) -> c_int;
}
extern "C" {
    pub fn isst_update_platform_param(isst_platform_param: enum, vale: c_int);
}
extern "C" {
    pub fn isst_get_disp_freq_multiplier() -> c_int;
}
extern "C" {
    pub fn isst_get_trl_max_levels() -> c_int;
}
extern "C" {
    pub fn isst_is_punit_valid(id: *mut isst_id) -> c_int;
}
extern "C" {
    pub fn isst_get_ctdp_levels(id: *mut isst_id, pkg_dev: *mut isst_pkg_ctdp) -> c_int;
}
extern "C" {
    pub fn isst_ctdp_display_information_start(outf: *mut FILE);
}
extern "C" {
    pub fn isst_ctdp_display_information_end(outf: *mut FILE);
}
extern "C" {
    pub fn isst_set_tdp_level(id: *mut isst_id, tdp_level: c_int) -> c_int;
}
extern "C" {
    pub fn isst_set_pbf_fact_status(id: *mut isst_id, pbf: c_int, enable: c_int) -> c_int;
}
extern "C" {
    pub fn isst_set_trl(id: *mut isst_id, trl: c_ulonglong) -> c_int;
}
extern "C" {
    pub fn isst_get_trl(id: *mut isst_id, trl: *mut c_ulonglong) -> c_int;
}
extern "C" {
    pub fn isst_set_trl_from_current_tdp(id: *mut isst_id, trl: c_ulonglong) -> c_int;
}
extern "C" {
    pub fn isst_get_config_tdp_lock_status(id: *mut isst_id) -> c_int;
}
extern "C" {
    pub fn isst_pm_qos_config(id: *mut isst_id, enable_clos: c_int, priority_type: c_int) -> c_int;
}
extern "C" {
    pub fn isst_clos_associate(id: *mut isst_id, clos: c_int) -> c_int;
}
extern "C" {
    pub fn isst_clos_get_assoc_status(id: *mut isst_id, clos_id: *mut c_int) -> c_int;
}
extern "C" {
    pub fn isst_clos_display_assoc_information(id: *mut isst_id, outf: *mut FILE, clos: c_int);
}
extern "C" {
    pub fn isst_clos_get_clos_information(id: *mut isst_id, enable: *mut c_int, type: *mut c_int) -> c_int;
}
extern "C" {
    pub fn is_clx_n_platform() -> c_int;
}
extern "C" {
    pub fn get_cpufreq_base_freq(cpu: c_int) -> c_int;
}
extern "C" {
    pub fn isst_read_pm_config(id: *mut isst_id, cp_state: *mut c_int, cp_cap: *mut c_int) -> c_int;
}
extern "C" {
    pub fn isst_display_error_info_message(error: c_int, msg: *mut c_char, arg_valid: c_int, arg: c_int);
}
extern "C" {
    pub fn is_skx_based_platform() -> c_int;
}
extern "C" {
    pub fn is_spr_platform() -> c_int;
}
extern "C" {
    pub fn is_emr_platform() -> c_int;
}
extern "C" {
    pub fn is_icx_platform() -> c_int;
}
extern "C" {
    pub fn isst_trl_display_information(id: *mut isst_id, outf: *mut FILE, trl: c_ulonglong);
}
extern "C" {
    pub fn set_cpu_online_offline(cpu: c_int, state: c_int);
}
extern "C" {
    pub fn isst_daemon(debug_mode: c_int, poll_interval: c_int, no_daemon: c_int) -> c_int;
}
extern "C" {
    pub fn process_level_change(id: *mut isst_id);
}
extern "C" {
    pub fn hfi_main() -> c_int;
}
extern "C" {
    pub fn hfi_exit();
}
// Interface specific callbacks
// Cgroup related interface
extern "C" {
    pub fn enable_cpuset_controller() -> c_int;
}
extern "C" {
    pub fn use_cgroupv2() -> c_int;
}
