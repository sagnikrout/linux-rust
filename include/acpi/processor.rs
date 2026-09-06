//! Automatically rewritten from C Header to Rust Module
//! Source: include/acpi/processor.h
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

pub const ACPI_PROCESSOR_BUSY_METRIC: c_int = 10;
pub const ACPI_PROCESSOR_MAX_POWER: c_int = 8;
pub const ACPI_PROCESSOR_MAX_C2_LATENCY: c_int = 100;
pub const ACPI_PROCESSOR_MAX_C3_LATENCY: c_int = 1000;
pub const ACPI_PROCESSOR_MAX_THROTTLING: c_int = 16;

pub const ACPI_PROCESSOR_MAX_DUTY_WIDTH: c_int = 4;
pub const ACPI_PDC_REVISION_ID: c_uint = 0x1;

pub const ACPI_PSD_REV0_ENTRIES: c_int = 5;

pub const ACPI_TSD_REV0_ENTRIES: c_int = 5;
//
// Types of coordination defined in ACPI 3.0. Same macros can be used across
// P, C and T states
//
pub const DOMAIN_COORD_TYPE_SW_ALL: c_uint = 0xfc;
pub const DOMAIN_COORD_TYPE_SW_ANY: c_uint = 0xfd;
pub const DOMAIN_COORD_TYPE_HW_ALL: c_uint = 0xfe;
pub const ACPI_CSTATE_SYSTEMIO: c_int = 0;
pub const ACPI_CSTATE_FFH: c_int = 1;
pub const ACPI_CSTATE_HALT: c_int = 2;
pub const ACPI_CSTATE_INTEGER: c_int = 3;
pub const ACPI_CX_DESC_LEN: c_int = 32;
// Power Management
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_power_register {
    pub descriptor: u8,
    pub length: u16,
    pub space_id: u8,
    pub bit_width: u8,
    pub bit_offset: u8,
    pub access_size: u8,
    pub address: u64,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_processor_cx {
    pub valid: u8,
    pub type: u8,
    pub address: u32,
    pub entry_method: u8,
    pub index: u8,
    pub latency: u32,
    pub bm_sts_skip: u8,
    pub desc: [c_char; ACPI_CX_DESC_LEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_lpi_state {
    pub min_residency: u32,
    pub /: *mut *mut u32 wake_latency; / worst case,
    pub flags: u32,
    pub arch_flags: u32,
    pub res_cnt_freq: u32,
    pub enable_parent_state: u32,
    pub address: u64,
    pub index: u8,
    pub entry_method: u8,
    pub desc: [c_char; ACPI_CX_DESC_LEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_processor_power {
    pub count: c_int,
    pub states: [acpi_processor_cx; ACPI_PROCESSOR_MAX_POWER],
    pub lpi_states: [acpi_lpi_state; ACPI_PROCESSOR_MAX_POWER],
}

// Performance Management
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_psd_package {
    pub num_entries: u64,
    pub revision: u64,
    pub domain: u64,
    pub coord_type: u64,
    pub num_processors: u64,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_pct_register {
    pub descriptor: u8,
    pub length: u16,
    pub space_id: u8,
    pub bit_width: u8,
    pub bit_offset: u8,
    pub reserved: u8,
    pub address: u64,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_processor_px {
    pub /: *mut *mut u64 core_frequency; / megahertz,
    pub /: *mut *mut u64 power; / milliWatts,
    pub /: *mut *mut u64 transition_latency; / microseconds,
    pub /: *mut *mut u64 bus_master_latency; / microseconds,
    pub /: *mut *mut u64 control; / control value,
    pub /: *mut *mut u64 status; / success indicator,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_processor_performance {
    pub state: c_uint,
    pub platform_limit: c_uint,
    pub control_register: acpi_pct_register,
    pub status_register: acpi_pct_register,
    pub state_count: c_uint,
    pub states: *mut acpi_processor_px,
    pub domain_info: acpi_psd_package,
    pub shared_cpu_map: cpumask_var_t,
    pub shared_type: c_uint,
}

// Throttling Control
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_tsd_package {
    pub num_entries: u64,
    pub revision: u64,
    pub domain: u64,
    pub coord_type: u64,
    pub num_processors: u64,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_ptc_register {
    pub descriptor: u8,
    pub length: u16,
    pub space_id: u8,
    pub bit_width: u8,
    pub bit_offset: u8,
    pub reserved: u8,
    pub address: u64,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_processor_tx_tss {
    pub /: *mut *mut u64 freqpercentage; /,
    pub /: *mut *mut u64 power; / milliWatts,
    pub /: *mut *mut u64 transition_latency; / microseconds,
    pub /: *mut *mut u64 control; / control value,
    pub /: *mut *mut u64 status; / success indicator,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_processor_tx {
    pub power: u16,
    pub performance: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_processor_throttling {
    pub state: c_uint,
    pub platform_limit: c_uint,
    pub control_register: acpi_pct_register,
    pub status_register: acpi_pct_register,
    pub state_count: c_uint,
    pub states_tss: *mut acpi_processor_tx_tss,
    pub domain_info: acpi_tsd_package,
    pub shared_cpu_map: cpumask_var_t,
    pub pr): *mut *mut *mut int (acpi_processor_get_throttling) (struct acpi_processor,
    pub force): int state, bool,
    pub address: u32,
    pub duty_offset: u8,
    pub duty_width: u8,
    pub tsd_valid_flag: u8,
    pub shared_type: c_uint,
    pub states: [acpi_processor_tx; ACPI_PROCESSOR_MAX_THROTTLING],
}

// Limit Interface
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_processor_lx {
    pub /: *mut *mut int px; / performance state,
    pub /: *mut *mut int tx; / throttle level,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_processor_limit {
    pub /: *mut *mut acpi_processor_lx state; / current limit,
    pub /: *mut *mut acpi_processor_lx thermal; / thermal limit,
    pub /: *mut *mut acpi_processor_lx user; / user limit,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_processor_flags {
    pub power:1: u8,
    pub performance:1: u8,
    pub throttling:1: u8,
    pub limit:1: u8,
    pub bm_control:1: u8,
    pub bm_check:1: u8,
    pub has_cst:1: u8,
    pub has_lpi:1: u8,
    pub power_setup_done:1: u8,
    pub bm_rld_set:1: u8,
    pub previously_online:1: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_processor {
    pub handle: acpi_handle,
    pub acpi_id: u32,
    pub /: *mut *mut phys_cpuid_t phys_id; / CPU hardware ID such as APIC ID for x86,
    pub /: *mut *mut u32 id; / CPU logical ID allocated by OS,
    pub pblk: u32,
    pub performance_platform_limit: c_int,
    pub throttling_platform_limit: c_int,
// 0 - states 0..n-th state available
    pub flags: acpi_processor_flags,
    pub power: acpi_processor_power,
    pub performance: *mut acpi_processor_performance,
    pub throttling: acpi_processor_throttling,
    pub limit: acpi_processor_limit,
    pub cdev: *mut thermal_cooling_device,
    pub /: *mut *mut *mut device dev; / Processor device.,
    pub perflib_req: freq_qos_request,
    pub thermal_req: freq_qos_request,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_processor_errata {
    pub smp: u8,
    pub throttle:1: u8,
    pub fdma:1: u8,
    pub reserved:6: u8,
    pub bmisx: u32,
    pub piix4: },
}

// performance, unsigned int cpu);
extern "C" {
    pub fn acpi_processor_unregister_performance(cpu: c_uint);
}
extern "C" {
    pub fn acpi_processor_pstate_control() -> c_int;
}
// note: this locks both the calling module and the processor module
extern "C" {
    pub fn acpi_processor_notify_smm(calling_module: *mut module) -> c_int;
}
// parsing the _P* objects.
extern "C" {
    pub fn acpi_processor_get_performance_info(pr: *mut acpi_processor) -> c_int;
}
// for communication between multiple parts of the processor kernel module

extern "C" {
    pub fn acpi_processor_ffh_cstate_enter(cstate: *mut acpi_processor_cx);
}
extern "C" {
    pub fn acpi_processor_ffh_play_dead(cx: *mut acpi_processor_cx) -> void __noreturn;
}

// flags, unsigned int cpu)
// reg)
// cstate)

extern "C" {
    pub fn fn(_arg: arg) -> return;
}
extern "C" {
    pub fn work_on_cpu(_arg: cpu, _arg: fn, _arg: arg) -> return;
}
// in processor_perflib.c

extern "C" {
    pub fn acpi_processor_ignore_ppc_init();
}
extern "C" {
    pub fn acpi_processor_ppc_init(policy: *mut cpufreq_policy);
}
extern "C" {
    pub fn acpi_processor_ppc_exit(policy: *mut cpufreq_policy);
}
extern "C" {
    pub fn acpi_processor_ppc_has_changed(pr: *mut acpi_processor, event_flag: c_int);
}
extern "C" {
    pub fn acpi_processor_get_bios_limit(cpu: c_int, limit: *mut c_uint) -> c_int;
}

// in processor_core.c
extern "C" {
    pub fn acpi_get_phys_id(_arg: acpi_handle, type: c_int, acpi_id: u32) -> phys_cpuid_t;
}
extern "C" {
    pub fn acpi_map_madt_entry(acpi_id: u32) -> phys_cpuid_t;
}
extern "C" {
    pub fn acpi_map_cpuid(phys_id: phys_cpuid_t, acpi_id: u32) -> c_int;
}
extern "C" {
    pub fn acpi_get_cpuid(_arg: acpi_handle, type: c_int, acpi_id: u32) -> c_int;
}

extern "C" {
    pub fn acpi_cppc_processor_probe(pr: *mut acpi_processor) -> c_int;
}
extern "C" {
    pub fn acpi_cppc_processor_exit(pr: *mut acpi_processor);
}

// in processor_pdc.c
extern "C" {
    pub fn acpi_processor_set_pdc(handle: acpi_handle);
}
// in processor_throttling.c

extern "C" {
    pub fn acpi_processor_tstate_has_changed(pr: *mut acpi_processor) -> c_int;
}
extern "C" {
    pub fn acpi_processor_get_throttling_info(pr: *mut acpi_processor) -> c_int;
}
//
// Reevaluate whether the T-state is invalid after one cpu is
// onlined/offlined. In such case the flags.throttling will be updated.
//
extern "C" {
    pub fn acpi_processor_throttling_init();
}

// in processor_idle.c

extern "C" {
    pub fn acpi_processor_power_init(pr: *mut acpi_processor);
}
extern "C" {
    pub fn acpi_processor_power_exit(pr: *mut acpi_processor);
}
extern "C" {
    pub fn acpi_processor_power_state_has_changed(pr: *mut acpi_processor) -> c_int;
}
extern "C" {
    pub fn acpi_processor_hotplug(pr: *mut acpi_processor) -> c_int;
}
extern "C" {
    pub fn acpi_processor_register_idle_driver();
}
extern "C" {
    pub fn acpi_processor_unregister_idle_driver();
}
extern "C" {
    pub fn acpi_processor_ffh_lpi_probe(cpu: c_uint) -> c_int;
}
extern "C" {
    pub fn acpi_processor_ffh_lpi_enter(lpi: *mut acpi_lpi_state) -> c_int;
}

// in processor_thermal.c

extern "C" {
    pub fn acpi_thermal_cpufreq_init(policy: *mut cpufreq_policy);
}
extern "C" {
    pub fn acpi_thermal_cpufreq_exit(policy: *mut cpufreq_policy);
}

extern "C" {
    pub fn acpi_processor_init_invariance_cppc();
}
