//! Automatically rewritten from C to Rust
//! Source: tools/power/cpupower/utils/idle_monitor/nhm_idle.c
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
// (C) 2010,2011       Thomas Renninger <trenn@suse.de>, Novell Inc.
//
// Based on Len Brown's <lenb@kernel.org> turbostat tool.
//

pub const MSR_PKG_C3_RESIDENCY: c_uint = 0x3F8;
pub const MSR_PKG_C6_RESIDENCY: c_uint = 0x3F9;
pub const MSR_CORE_C3_RESIDENCY: c_uint = 0x3FC;
pub const MSR_CORE_C6_RESIDENCY: c_uint = 0x3FD;
pub const MSR_TSC: c_uint = 0x10;
pub const NHM_CSTATE_COUNT: c_int = 4;
    enum intel_nhm_id { C3 = 0, C6, PC3, PC6, TSC = 0xFFFF };
    static int nhm_get_count_percent(unsigned int self_id, double *percent,
    unsigned int cpu);
    static cstate_t nhm_cstates[NHM_CSTATE_COUNT] = {
    {
    .name			= "C3",
    .desc			= N_("Processor Core C3"),
    .id			= C3,
    .range			= RANGE_CORE,
    .get_count_percent	= nhm_get_count_percent,
    },
    {
    .name			= "C6",
    .desc			= N_("Processor Core C6"),
    .id			= C6,
    .range			= RANGE_CORE,
    .get_count_percent	= nhm_get_count_percent,
    },
    {
    .name			= "PC3",
    .desc			= N_("Processor Package C3"),
    .id			= PC3,
    .range			= RANGE_PACKAGE,
    .get_count_percent	= nhm_get_count_percent,
    },
    {
    .name			= "PC6",
    .desc			= N_("Processor Package C6"),
    .id			= PC6,
    .range			= RANGE_PACKAGE,
    .get_count_percent	= nhm_get_count_percent,
    },
    };
    static unsigned long long tsc_at_measure_start;
    static unsigned long long tsc_at_measure_end;
    static unsigned long long *previous_count[NHM_CSTATE_COUNT];
    static unsigned long long *current_count[NHM_CSTATE_COUNT];
// valid flag for all CPUs. If a MSR read failed it will be zero
    static int *is_valid;
    static int nhm_get_count(enum intel_nhm_id id, unsigned long long *val,
    unsigned int cpu)
    {
    int msr;
    switch (id) {
    case C3:
    msr = MSR_CORE_C3_RESIDENCY;
    break;
    case C6:
    msr = MSR_CORE_C6_RESIDENCY;
    break;
    case PC3:
    msr = MSR_PKG_C3_RESIDENCY;
    break;
    case PC6:
    msr = MSR_PKG_C6_RESIDENCY;
    break;
    case TSC:
    msr = MSR_TSC;
    break;
    default:
    return -1;
    }
    if (read_msr(cpu, msr, val))
    return -1;
    return 0;
    }
    static int nhm_get_count_percent(unsigned int id, double *percent,
    unsigned int cpu)
    {
// percent = 0.0;
    if (!is_valid[cpu])
    return -1;
// percent = (100.0
    (current_count[id][cpu] - previous_count[id][cpu])) /
    (tsc_at_measure_end - tsc_at_measure_start);
    dprint("%s: previous: %llu - current: %llu - (%u)\n",
    nhm_cstates[id].name, previous_count[id][cpu],
    current_count[id][cpu], cpu);
    dprint("%s: tsc_diff: %llu - count_diff: %llu - percent: %2.f (%u)\n",
    nhm_cstates[id].name,
    (unsigned long long) tsc_at_measure_end - tsc_at_measure_start,
    current_count[id][cpu] - previous_count[id][cpu],
// percent, cpu);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn nhm_start() -> c_int {
    static int nhm_start(void)
    {
    int num, cpu;
    unsigned long long dbg, val;
    nhm_get_count(TSC, &tsc_at_measure_start, base_cpu);
    for (num = 0; num < NHM_CSTATE_COUNT; num++) {
    for (cpu = 0; cpu < cpu_count; cpu++) {
    is_valid[cpu] = !nhm_get_count(num, &val, cpu);
    previous_count[num][cpu] = val;
    }
    }
    nhm_get_count(TSC, &dbg, base_cpu);
    dprint("TSC diff: %llu\n", dbg - tsc_at_measure_start);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn nhm_stop() -> c_int {
    static int nhm_stop(void)
    {
    unsigned long long val;
    unsigned long long dbg;
    int num, cpu;
    nhm_get_count(TSC, &tsc_at_measure_end, base_cpu);
    for (num = 0; num < NHM_CSTATE_COUNT; num++) {
    for (cpu = 0; cpu < cpu_count; cpu++) {
    is_valid[cpu] |= !nhm_get_count(num, &val, cpu);
    current_count[num][cpu] = val;
    }
    }
    nhm_get_count(TSC, &dbg, base_cpu);
    dprint("TSC diff: %llu\n", dbg - tsc_at_measure_end);
    return 0;
    }
    struct cpuidle_monitor intel_nhm_monitor;
    struct cpuidle_monitor *intel_nhm_register(void)
    {
    int num;
    if (cpupower_cpu_info.vendor != X86_VENDOR_INTEL)
    return core::ptr::null_mut();
    if (!(cpupower_cpu_info.caps & CPUPOWER_CAP_INV_TSC))
    return core::ptr::null_mut();
    if (!(cpupower_cpu_info.caps & CPUPOWER_CAP_APERF))
    return core::ptr::null_mut();
// Free this at program termination
    is_valid = calloc(cpu_count, sizeof(int));
    for (num = 0; num < NHM_CSTATE_COUNT; num++) {
    previous_count[num] = calloc(cpu_count,
    sizeof(unsigned long long));
    current_count[num]  = calloc(cpu_count,
    sizeof(unsigned long long));
    }
    intel_nhm_monitor.name_len = strlen(intel_nhm_monitor.name);
    return &intel_nhm_monitor;
    }
#[no_mangle]
pub unsafe extern "C" fn intel_nhm_unregister() {
    void intel_nhm_unregister(void)
    {
    int num;
    for (num = 0; num < NHM_CSTATE_COUNT; num++) {
    free(previous_count[num]);
    free(current_count[num]);
    }
    free(is_valid);
    }
    struct cpuidle_monitor intel_nhm_monitor = {
    .name			= "Nehalem",
    .hw_states_num		= NHM_CSTATE_COUNT,
    .hw_states		= nhm_cstates,
    .start			= nhm_start,
    .stop			= nhm_stop,
    .do_register		= intel_nhm_register,
    .unregister		= intel_nhm_unregister,
    .flags.needs_root	= 1,
    .overflow_s		= 922000000 /* 922337203 seconds TSC overflow
    at 20GHz */
    };
