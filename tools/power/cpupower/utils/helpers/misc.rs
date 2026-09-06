//! Automatically rewritten from C to Rust
//! Source: tools/power/cpupower/utils/helpers/misc.c
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

pub const MSR_AMD_HWCR: c_uint = 0xc0010015;
    int cpufreq_has_x86_boost_support(unsigned int cpu, int *support, int *active,
    int *states)
    {
    int ret;
    unsigned long long val;
    char linebuf[MAX_LINE_LEN];
    char path[SYSFS_PATH_MAX];
    char *endp;
// support = *active = *states = 0;
    if (cpupower_cpu_info.caps & CPUPOWER_CAP_AMD_CPB) {
// support = 1;
// AMD Family 0x17 does not utilize PCI D18F4 like prior
// families and has no fixed discrete boost states but
// has Hardware determined variable increments instead.
//
    if (cpupower_cpu_info.caps & CPUPOWER_CAP_AMD_CPB_MSR) {
    if (!read_msr(cpu, MSR_AMD_HWCR, &val)) {
    if (!(val & CPUPOWER_AMD_CPBDIS))
// active = 1;
    }
    } else {
    ret = amd_pci_get_num_boost_states(active, states);
    if (ret)
    return ret;
    }
    } else if (cpupower_cpu_info.caps & CPUPOWER_CAP_AMD_PSTATE) {
    amd_pstate_boost_init(cpu, support, active);
    } else if (cpupower_cpu_info.caps & CPUPOWER_CAP_INTEL_IDA) {
// support = *active = 1;
    snprintf(path, sizeof(path), PATH_TO_CPU "intel_pstate/no_turbo");
    if (!is_valid_path(path))
    return 0;
    if (cpupower_read_sysfs(path, linebuf, MAX_LINE_LEN) == 0)
    return -1;
    val = strtol(linebuf, &endp, 0);
    if (endp == linebuf || errno == ERANGE)
    return -1;
// active = !val;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn cpupower_set_intel_turbo_boost(turbo_boost: c_int) -> c_int {
    int cpupower_set_intel_turbo_boost(int turbo_boost)
    {
    char path[SYSFS_PATH_MAX];
    char linebuf[2] = {};
    snprintf(path, sizeof(path), PATH_TO_CPU "intel_pstate/no_turbo");
// Fallback to generic solution when intel_pstate driver not running
    if (!is_valid_path(path))
    return cpupower_set_generic_turbo_boost(turbo_boost);
    snprintf(linebuf, sizeof(linebuf), "%d", !turbo_boost);
    if (cpupower_write_sysfs(path, linebuf, 2) <= 0)
    return -1;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn cpupower_intel_get_perf_bias(cpu: c_uint) -> c_int {
    int cpupower_intel_get_perf_bias(unsigned int cpu)
    {
    char linebuf[MAX_LINE_LEN];
    char path[SYSFS_PATH_MAX];
    unsigned long val;
    char *endp;
    if (!(cpupower_cpu_info.caps & CPUPOWER_CAP_PERF_BIAS))
    return -1;
    snprintf(path, sizeof(path), PATH_TO_CPU "cpu%u/power/energy_perf_bias", cpu);
    if (cpupower_read_sysfs(path, linebuf, MAX_LINE_LEN) == 0)
    return -1;
    val = strtol(linebuf, &endp, 0);
    if (endp == linebuf || errno == ERANGE)
    return -1;
    return val;
    }
#[no_mangle]
pub unsafe extern "C" fn cpupower_intel_set_perf_bias(cpu: c_uint, val: c_uint) -> c_int {
    int cpupower_intel_set_perf_bias(unsigned int cpu, unsigned int val)
    {
    char path[SYSFS_PATH_MAX];
    char linebuf[3] = {};
    if (!(cpupower_cpu_info.caps & CPUPOWER_CAP_PERF_BIAS))
    return -1;
    snprintf(path, sizeof(path), PATH_TO_CPU "cpu%u/power/energy_perf_bias", cpu);
    snprintf(linebuf, sizeof(linebuf), "%d", val);
    if (cpupower_write_sysfs(path, linebuf, 3) <= 0)
    return -1;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn cpupower_set_epp(cpu: c_uint, epp: *mut c_char) -> c_int {
    int cpupower_set_epp(unsigned int cpu, char *epp)
    {
    char path[SYSFS_PATH_MAX];
    char linebuf[30] = {};
    snprintf(path, sizeof(path),
    PATH_TO_CPU "cpu%u/cpufreq/energy_performance_preference", cpu);
    if (!is_valid_path(path))
    return -1;
    snprintf(linebuf, sizeof(linebuf), "%s", epp);
    if (cpupower_write_sysfs(path, linebuf, 30) <= 0)
    return -1;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn cpupower_set_amd_pstate_mode(mode: *mut c_char) -> c_int {
    int cpupower_set_amd_pstate_mode(char *mode)
    {
    char path[SYSFS_PATH_MAX];
    char linebuf[20] = {};
    snprintf(path, sizeof(path), PATH_TO_CPU "amd_pstate/status");
    if (!is_valid_path(path))
    return -1;
    snprintf(linebuf, sizeof(linebuf), "%s\n", mode);
    if (cpupower_write_sysfs(path, linebuf, 20) <= 0)
    return -1;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn cpupower_amd_pstate_enabled() -> bool {
    bool cpupower_amd_pstate_enabled(void)
    {
    char *driver = cpufreq_get_driver(0);
    let mut ret: bool = false;
    if (!driver)
    return ret;
    if (!strncmp(driver, "amd", 3))
    ret = true;
    cpufreq_put_driver(driver);
    return ret;
    }

#[no_mangle]
pub unsafe extern "C" fn cpufreq_has_generic_boost_support(active: *mut bool) -> c_int {
    int cpufreq_has_generic_boost_support(bool *active)
    {
    char path[SYSFS_PATH_MAX];
    char linebuf[2] = {};
    unsigned long val;
    char *endp;
    snprintf(path, sizeof(path), PATH_TO_CPU "cpufreq/boost");
    if (!is_valid_path(path))
    return -EACCES;
    if (cpupower_read_sysfs(path, linebuf, 2) <= 0)
    return -EINVAL;
    val = strtoul(linebuf, &endp, 0);
    if (endp == linebuf || errno == ERANGE)
    return -EINVAL;
    switch (val) {
    case 0:
// active = false;
    break;
    case 1:
// active = true;
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
// get_cpustate
//
// Gather the information of all online CPUs into bitmask struct
//
#[no_mangle]
pub unsafe extern "C" fn get_cpustate() {
    void get_cpustate(void)
    {
    let mut cpu: c_uint = 0;
    bitmask_clearall(online_cpus);
    bitmask_clearall(offline_cpus);
    for (cpu = bitmask_first(cpus_chosen);
    cpu <= bitmask_last(cpus_chosen); cpu++) {
    if (cpupower_is_cpu_online(cpu) == 1)
    bitmask_setbit(online_cpus, cpu);
    else
    bitmask_setbit(offline_cpus, cpu);
    continue;
    }
    }
// print_online_cpus
//
// Print the CPU numbers of all CPUs that are online currently
//
#[no_mangle]
pub unsafe extern "C" fn print_online_cpus() {
    void print_online_cpus(void)
    {
    let mut str_len: c_int = 0;
    char *online_cpus_str = core::ptr::null_mut();
    str_len = online_cpus.size * 5;
    online_cpus_str = (void *)malloc(sizeof(char) * str_len);
    if (!bitmask_isallclear(online_cpus)) {
    bitmask_displaylist(online_cpus_str, str_len, online_cpus);
    printf(_("Following CPUs are online:\n%s\n"), online_cpus_str);
    }
    }
// print_offline_cpus
//
// Print the CPU numbers of all CPUs that are offline currently
//
#[no_mangle]
pub unsafe extern "C" fn print_offline_cpus() {
    void print_offline_cpus(void)
    {
    let mut str_len: c_int = 0;
    char *offline_cpus_str = core::ptr::null_mut();
    str_len = offline_cpus.size * 5;
    offline_cpus_str = (void *)malloc(sizeof(char) * str_len);
    if (!bitmask_isallclear(offline_cpus)) {
    bitmask_displaylist(offline_cpus_str, str_len, offline_cpus);
    printf(_("Following CPUs are offline:\n%s\n"), offline_cpus_str);
    printf(_("cpupower set operation was not performed on them\n"));
    }
    }
//
// print_speed
//
// Print the exact CPU frequency with appropriate unit
//
#[no_mangle]
pub unsafe extern "C" fn print_speed(speed: c_ulong, no_rounding: c_int) {
    void print_speed(unsigned long speed, int no_rounding)
    {
    unsigned long tmp;
    if (no_rounding) {
    if (speed > 1000000)
    printf("%u.%06u GHz", ((unsigned int)speed / 1000000),
    ((unsigned int)speed % 1000000));
#[no_mangle]
pub unsafe extern "C" fn if(1000: speed >) -> else {
    else if (speed > 1000)
    printf("%u.%03u MHz", ((unsigned int)speed / 1000),
    (unsigned int)(speed % 1000));
    else
    printf("%lu kHz", speed);
    } else {
    if (speed > 1000000) {
    tmp = speed % 10000;
    if (tmp >= 5000)
    speed += 10000;
    printf("%u.%02u GHz", ((unsigned int)speed / 1000000),
    ((unsigned int)(speed % 1000000) / 10000));
    } else if (speed > 100000) {
    tmp = speed % 1000;
    if (tmp >= 500)
    speed += 1000;
    printf("%u MHz", ((unsigned int)speed / 1000));
    } else if (speed > 1000) {
    tmp = speed % 100;
    if (tmp >= 50)
    speed += 100;
    printf("%u.%01u MHz", ((unsigned int)speed / 1000),
    ((unsigned int)(speed % 1000) / 100));
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn cpupower_set_generic_turbo_boost(turbo_boost: c_int) -> c_int {
    int cpupower_set_generic_turbo_boost(int turbo_boost)
    {
    char path[SYSFS_PATH_MAX];
    char linebuf[2] = {};
    snprintf(path, sizeof(path), PATH_TO_CPU "cpufreq/boost");
    if (!is_valid_path(path))
    return -1;
    snprintf(linebuf, sizeof(linebuf), "%d", turbo_boost);
    if (cpupower_write_sysfs(path, linebuf, 2) <= 0)
    return -1;
    return 0;
    }
