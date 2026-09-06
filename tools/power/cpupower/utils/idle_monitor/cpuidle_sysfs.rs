//! Automatically rewritten from C to Rust
//! Source: tools/power/cpupower/utils/idle_monitor/cpuidle_sysfs.c
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
// (C) 2010,2011       Thomas Renninger <trenn@suse.de>, Novell Inc
//

pub const CPUIDLE_STATES_MAX: c_int = 10;
    static cstate_t cpuidle_cstates[CPUIDLE_STATES_MAX];
    struct cpuidle_monitor cpuidle_sysfs_monitor;
    static unsigned long long **previous_count;
    static unsigned long long **current_count;
    static struct timespec start_time;
    static unsigned long long timediff;
    static int cpuidle_get_count_percent(unsigned int id, double *percent,
    unsigned int cpu)
    {
    unsigned long long statediff = current_count[cpu][id]
    - previous_count[cpu][id];
    dprint("%s: - diff: %llu - percent: %f (%u)\n",
    cpuidle_cstates[id].name, timediff, *percent, cpu);
    if (timediff == 0)
// percent = 0.0;
    else
// percent = ((100.0 * statediff) / timediff);
    dprint("%s: - timediff: %llu - statediff: %llu - percent: %f (%u)\n",
    cpuidle_cstates[id].name, timediff, statediff, *percent, cpu);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cpuidle_start() -> c_int {
    static int cpuidle_start(void)
    {
    int cpu, state;
    clock_gettime(CLOCK_REALTIME, &start_time);
    for (cpu = 0; cpu < cpu_count; cpu++) {
    for (state = 0; state < cpuidle_sysfs_monitor.hw_states_num;
    state++) {
    previous_count[cpu][state] =
    cpuidle_state_time(cpu, state);
    dprint("CPU %d - State: %d - Val: %llu\n",
    cpu, state, previous_count[cpu][state]);
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cpuidle_stop() -> c_int {
    static int cpuidle_stop(void)
    {
    int cpu, state;
    struct timespec end_time;
    clock_gettime(CLOCK_REALTIME, &end_time);
    timediff = timespec_diff_us(start_time, end_time);
    for (cpu = 0; cpu < cpu_count; cpu++) {
    for (state = 0; state < cpuidle_sysfs_monitor.hw_states_num;
    state++) {
    current_count[cpu][state] =
    cpuidle_state_time(cpu, state);
    dprint("CPU %d - State: %d - Val: %llu\n",
    cpu, state, current_count[cpu][state]);
    }
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn fix_up_intel_idle_driver_name(tmp: *mut c_char, num: c_int) {
    void fix_up_intel_idle_driver_name(char *tmp, int num)
    {
// fix up cpuidle name for intel idle driver
    if (!strncmp(tmp, "NHM-", 4)) {
    switch (num) {
    case 1:
    strcpy(tmp, "C1");
    break;
    case 2:
    strcpy(tmp, "C3");
    break;
    case 3:
    strcpy(tmp, "C6");
    break;
    }
    } else if (!strncmp(tmp, "SNB-", 4)) {
    switch (num) {
    case 1:
    strcpy(tmp, "C1");
    break;
    case 2:
    strcpy(tmp, "C3");
    break;
    case 3:
    strcpy(tmp, "C6");
    break;
    case 4:
    strcpy(tmp, "C7");
    break;
    }
    } else if (!strncmp(tmp, "ATM-", 4)) {
    switch (num) {
    case 1:
    strcpy(tmp, "C1");
    break;
    case 2:
    strcpy(tmp, "C2");
    break;
    case 3:
    strcpy(tmp, "C4");
    break;
    case 4:
    strcpy(tmp, "C6");
    break;
    }
    }
    }

#[no_mangle]
pub unsafe extern "C" fn map_power_idle_state_name(tmp: *mut c_char) {
    void map_power_idle_state_name(char *tmp)
    {
    if (!strncmp(tmp, "stop0_lite", CSTATE_NAME_LEN))
    strcpy(tmp, "stop0L");
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strncmp(tmp, _arg: "stop1_lite", _arg: CSTATE_NAME_LEN)) -> else {
    else if (!strncmp(tmp, "stop1_lite", CSTATE_NAME_LEN))
    strcpy(tmp, "stop1L");
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strncmp(tmp, _arg: "stop2_lite", _arg: CSTATE_NAME_LEN)) -> else {
    else if (!strncmp(tmp, "stop2_lite", CSTATE_NAME_LEN))
    strcpy(tmp, "stop2L");
    }

    void map_power_idle_state_name(char *tmp) { }

    static struct cpuidle_monitor *cpuidle_register(void)
    {
    int num;
    char *tmp;
    int this_cpu;
    this_cpu = sched_getcpu();
// Assume idle state count is the same for all CPUs
    cpuidle_sysfs_monitor.hw_states_num = cpuidle_state_count(this_cpu);
    if (cpuidle_sysfs_monitor.hw_states_num <= 0)
    return core::ptr::null_mut();
    for (num = 0; num < cpuidle_sysfs_monitor.hw_states_num; num++) {
    tmp = cpuidle_state_name(this_cpu, num);
    if (tmp == core::ptr::null_mut())
    continue;
    map_power_idle_state_name(tmp);
    fix_up_intel_idle_driver_name(tmp, num);
    strncpy(cpuidle_cstates[num].name, tmp, CSTATE_NAME_LEN - 1);
    free(tmp);
    tmp = cpuidle_state_desc(this_cpu, num);
    if (tmp == core::ptr::null_mut())
    continue;
    strncpy(cpuidle_cstates[num].desc, tmp,	CSTATE_DESC_LEN - 1);
    free(tmp);
    cpuidle_cstates[num].range = RANGE_THREAD;
    cpuidle_cstates[num].id = num;
    cpuidle_cstates[num].get_count_percent =
    cpuidle_get_count_percent;
    }
// Free this at program termination
    previous_count = malloc(sizeof(long long *) * cpu_count);
    current_count = malloc(sizeof(long long *) * cpu_count);
    for (num = 0; num < cpu_count; num++) {
    previous_count[num] = malloc(sizeof(long long) *
    cpuidle_sysfs_monitor.hw_states_num);
    current_count[num] = malloc(sizeof(long long) *
    cpuidle_sysfs_monitor.hw_states_num);
    }
    cpuidle_sysfs_monitor.name_len = strlen(cpuidle_sysfs_monitor.name);
    return &cpuidle_sysfs_monitor;
    }
#[no_mangle]
pub unsafe extern "C" fn cpuidle_unregister() {
    void cpuidle_unregister(void)
    {
    int num;
    for (num = 0; num < cpu_count; num++) {
    free(previous_count[num]);
    free(current_count[num]);
    }
    free(previous_count);
    free(current_count);
    }
    struct cpuidle_monitor cpuidle_sysfs_monitor = {
    .name			= "Idle_Stats",
    .hw_states		= cpuidle_cstates,
    .start			= cpuidle_start,
    .stop			= cpuidle_stop,
    .do_register		= cpuidle_register,
    .unregister		= cpuidle_unregister,
    .flags.needs_root	= 0,
    .overflow_s		= UINT_MAX,
    };
