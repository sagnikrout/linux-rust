//! Automatically rewritten from C to Rust
//! Source: tools/power/cpupower/utils/idle_monitor/hsw_ext_idle.c
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
// Based on SandyBridge monitor. Implements the new package C-states
// (PC8, PC9, PC10) coming with a specific Haswell (family 0x45) CPU.
//

pub const MSR_PKG_C8_RESIDENCY: c_uint = 0x00000630;
pub const MSR_PKG_C9_RESIDENCY: c_uint = 0x00000631;
pub const MSR_PKG_C10_RESIDENCY: c_uint = 0x00000632;
pub const MSR_TSC: c_uint = 0x10;
    enum intel_hsw_ext_id { PC8 = 0, PC9, PC10, HSW_EXT_CSTATE_COUNT,
    TSC = 0xFFFF };
    static int hsw_ext_get_count_percent(unsigned int self_id, double *percent,
    unsigned int cpu);
    static cstate_t hsw_ext_cstates[HSW_EXT_CSTATE_COUNT] = {
    {
    .name			= "PC8",
    .desc			= N_("Processor Package C8"),
    .id			= PC8,
    .range			= RANGE_PACKAGE,
    .get_count_percent	= hsw_ext_get_count_percent,
    },
    {
    .name			= "PC9",
    .desc			= N_("Processor Package C9"),
    .id			= PC9,
    .range			= RANGE_PACKAGE,
    .get_count_percent	= hsw_ext_get_count_percent,
    },
    {
    .name			= "PC10",
    .desc			= N_("Processor Package C10"),
    .id			= PC10,
    .range			= RANGE_PACKAGE,
    .get_count_percent	= hsw_ext_get_count_percent,
    },
    };
    static unsigned long long tsc_at_measure_start;
    static unsigned long long tsc_at_measure_end;
    static unsigned long long *previous_count[HSW_EXT_CSTATE_COUNT];
    static unsigned long long *current_count[HSW_EXT_CSTATE_COUNT];
// valid flag for all CPUs. If a MSR read failed it will be zero
    static int *is_valid;
    static int hsw_ext_get_count(enum intel_hsw_ext_id id, unsigned long long *val,
    unsigned int cpu)
    {
    int msr;
    switch (id) {
    case PC8:
    msr = MSR_PKG_C8_RESIDENCY;
    break;
    case PC9:
    msr = MSR_PKG_C9_RESIDENCY;
    break;
    case PC10:
    msr = MSR_PKG_C10_RESIDENCY;
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
    static int hsw_ext_get_count_percent(unsigned int id, double *percent,
    unsigned int cpu)
    {
// percent = 0.0;
    if (!is_valid[cpu])
    return -1;
// percent = (100.0
    (current_count[id][cpu] - previous_count[id][cpu])) /
    (tsc_at_measure_end - tsc_at_measure_start);
    dprint("%s: previous: %llu - current: %llu - (%u)\n",
    hsw_ext_cstates[id].name, previous_count[id][cpu],
    current_count[id][cpu], cpu);
    dprint("%s: tsc_diff: %llu - count_diff: %llu - percent: %2.f (%u)\n",
    hsw_ext_cstates[id].name,
    (unsigned long long) tsc_at_measure_end - tsc_at_measure_start,
    current_count[id][cpu] - previous_count[id][cpu],
// percent, cpu);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hsw_ext_start() -> c_int {
    static int hsw_ext_start(void)
    {
    int num, cpu;
    unsigned long long val;
    for (num = 0; num < HSW_EXT_CSTATE_COUNT; num++) {
    for (cpu = 0; cpu < cpu_count; cpu++) {
    is_valid[cpu] = !hsw_ext_get_count(num, &val, cpu);
    previous_count[num][cpu] = val;
    }
    }
    hsw_ext_get_count(TSC, &tsc_at_measure_start, base_cpu);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hsw_ext_stop() -> c_int {
    static int hsw_ext_stop(void)
    {
    unsigned long long val;
    int num, cpu;
    hsw_ext_get_count(TSC, &tsc_at_measure_end, base_cpu);
    for (num = 0; num < HSW_EXT_CSTATE_COUNT; num++) {
    for (cpu = 0; cpu < cpu_count; cpu++) {
    is_valid[cpu] |= !hsw_ext_get_count(num, &val, cpu);
    current_count[num][cpu] = val;
    }
    }
    return 0;
    }
    struct cpuidle_monitor intel_hsw_ext_monitor;
    static struct cpuidle_monitor *hsw_ext_register(void)
    {
    int num;
    if (cpupower_cpu_info.vendor != X86_VENDOR_INTEL
    || cpupower_cpu_info.family != 6)
    return core::ptr::null_mut();
    switch (cpupower_cpu_info.model) {
    case 0x45: /* HSW */
    break;
    default:
    return core::ptr::null_mut();
    }
    is_valid = calloc(cpu_count, sizeof(int));
    for (num = 0; num < HSW_EXT_CSTATE_COUNT; num++) {
    previous_count[num] = calloc(cpu_count,
    sizeof(unsigned long long));
    current_count[num]  = calloc(cpu_count,
    sizeof(unsigned long long));
    }
    intel_hsw_ext_monitor.name_len = strlen(intel_hsw_ext_monitor.name);
    return &intel_hsw_ext_monitor;
    }
#[no_mangle]
pub unsafe extern "C" fn hsw_ext_unregister() {
    void hsw_ext_unregister(void)
    {
    int num;
    free(is_valid);
    for (num = 0; num < HSW_EXT_CSTATE_COUNT; num++) {
    free(previous_count[num]);
    free(current_count[num]);
    }
    }
    struct cpuidle_monitor intel_hsw_ext_monitor = {
    .name			= "HaswellExtended",
    .hw_states		= hsw_ext_cstates,
    .hw_states_num		= HSW_EXT_CSTATE_COUNT,
    .start			= hsw_ext_start,
    .stop			= hsw_ext_stop,
    .do_register		= hsw_ext_register,
    .unregister		= hsw_ext_unregister,
    .flags.needs_root	= 1,
    .overflow_s		= 922000000 /* 922337203 seconds TSC overflow
    at 20GHz */
    };
