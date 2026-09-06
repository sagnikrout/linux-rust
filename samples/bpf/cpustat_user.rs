//! Automatically rewritten from C to Rust
//! Source: samples/bpf/cpustat_user.c
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
// Macro flag: #define _GNU_SOURCE

    static int cstate_map_fd, pstate_map_fd;
pub const MAX_CPU: c_int = 8;
pub const MAX_PSTATE_ENTRIES: c_int = 5;
pub const MAX_CSTATE_ENTRIES: c_int = 3;
pub const MAX_STARS: c_int = 40;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpu_stat_data {
    pub cstate: [c_ulong; MAX_CSTATE_ENTRIES],
    pub pstate: [c_ulong; MAX_PSTATE_ENTRIES],
}

    static struct cpu_stat_data stat_data[MAX_CPU];
#[no_mangle]
unsafe extern "C" fn cpu_stat_print() {
    static void cpu_stat_print(void)
    {
    int i, j;
    char state_str[sizeof("cstate-9")];
    struct cpu_stat_data *data;
// Clear screen
    printf("\033[2J");
// Header
    printf("\nCPU states statistics:\n");
    printf("%-10s ", "state(ms)");
    for (i = 0; i < MAX_CSTATE_ENTRIES; i++) {
    sprintf(state_str, "cstate-%d", i);
    printf("%-11s ", state_str);
    }
    for (i = 0; i < MAX_PSTATE_ENTRIES; i++) {
    sprintf(state_str, "pstate-%d", i);
    printf("%-11s ", state_str);
    }
    printf("\n");
    for (j = 0; j < MAX_CPU; j++) {
    data = &stat_data[j];
    printf("CPU-%-6d ", j);
    for (i = 0; i < MAX_CSTATE_ENTRIES; i++)
    printf("%-11lu ", data.cstate[i] / 1000000);
    for (i = 0; i < MAX_PSTATE_ENTRIES; i++)
    printf("%-11lu ", data.pstate[i] / 1000000);
    printf("\n");
    }
    }
#[no_mangle]
unsafe extern "C" fn cpu_stat_update(cstate_fd: c_int, pstate_fd: c_int) {
    static void cpu_stat_update(int cstate_fd, int pstate_fd)
    {
    unsigned long key, value;
    int c, i;
    for (c = 0; c < MAX_CPU; c++) {
    for (i = 0; i < MAX_CSTATE_ENTRIES; i++) {
    key = c * MAX_CSTATE_ENTRIES + i;
    bpf_map_lookup_elem(cstate_fd, &key, &value);
    stat_data[c].cstate[i] = value;
    }
    for (i = 0; i < MAX_PSTATE_ENTRIES; i++) {
    key = c * MAX_PSTATE_ENTRIES + i;
    bpf_map_lookup_elem(pstate_fd, &key, &value);
    stat_data[c].pstate[i] = value;
    }
    }
    }
//
// This function is copied from 'idlestat' tool function
// idlestat_wake_all() in idlestate.c.
//
// It sets the self running task affinity to cpus one by one so can wake up
// the specific CPU to handle scheduling; this results in all cpus can be
// waken up once and produce ftrace event 'trace_cpu_idle'.
//
#[no_mangle]
unsafe extern "C" fn cpu_stat_inject_cpu_idle_event() -> c_int {
    static int cpu_stat_inject_cpu_idle_event(void)
    {
    int rcpu, i, ret;
    cpu_set_t cpumask;
    cpu_set_t original_cpumask;
    ret = sysconf(_SC_NPROCESSORS_CONF);
    if (ret < 0)
    return -1;
    rcpu = sched_getcpu();
    if (rcpu < 0)
    return -1;
// Keep track of the CPUs we will run on
    sched_getaffinity(0, sizeof(original_cpumask), &original_cpumask);
    for (i = 0; i < ret; i++) {
// Pointless to wake up ourself
    if (i == rcpu)
    continue;
// Pointless to wake CPUs we will not run on
    if (!CPU_ISSET(i, &original_cpumask))
    continue;
    CPU_ZERO(&cpumask);
    CPU_SET(i, &cpumask);
    sched_setaffinity(0, sizeof(cpumask), &cpumask);
    }
// Enable all the CPUs of the original mask
    sched_setaffinity(0, sizeof(original_cpumask), &original_cpumask);
    return 0;
    }
//
// It's possible to have no any frequency change for long time and cannot
// get ftrace event 'trace_cpu_frequency' for long period, this introduces
// big deviation for pstate statistics.
//
// To solve this issue, below code forces to set 'scaling_max_freq' to 208MHz
// for triggering ftrace event 'trace_cpu_frequency' and then recovery back to
// the maximum frequency value 1.2GHz.
//
#[no_mangle]
unsafe extern "C" fn cpu_stat_inject_cpu_frequency_event() -> c_int {
    static int cpu_stat_inject_cpu_frequency_event(void)
    {
    int len, fd;
    fd = open(CPUFREQ_MAX_SYSFS_PATH, O_WRONLY);
    if (fd < 0) {
    printf("failed to open scaling_max_freq, errno=%d\n", errno);
    return fd;
    }
    len = write(fd, CPUFREQ_LOWEST_FREQ, strlen(CPUFREQ_LOWEST_FREQ));
    if (len < 0) {
    printf("failed to open scaling_max_freq, errno=%d\n", errno);
    goto err;
    }
    len = write(fd, CPUFREQ_HIGHEST_FREQ, strlen(CPUFREQ_HIGHEST_FREQ));
    if (len < 0) {
    printf("failed to open scaling_max_freq, errno=%d\n", errno);
    goto err;
    }
    err:
    close(fd);
    return len;
    }
#[no_mangle]
unsafe extern "C" fn int_exit(sig: c_int) {
    static void int_exit(int sig)
    {
    cpu_stat_inject_cpu_idle_event();
    cpu_stat_inject_cpu_frequency_event();
    cpu_stat_update(cstate_map_fd, pstate_map_fd);
    cpu_stat_print();
    exit(0);
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> c_int {
    int main(int argc, char **argv)
    {
    struct bpf_link *link = core::ptr::null_mut();
    struct bpf_program *prog;
    struct bpf_object *obj;
    char filename[256];
    int ret;
    snprintf(filename, sizeof(filename), "%s_kern.o", argv[0]);
    obj = bpf_object__open_file(filename, core::ptr::null_mut());
    if (libbpf_get_error(obj)) {
    fprintf(stderr, "ERROR: opening BPF object file failed\n");
    return 0;
    }
    prog = bpf_object__find_program_by_name(obj, "bpf_prog1");
    if (!prog) {
    printf("finding a prog in obj file failed\n");
    goto cleanup;
    }
// load BPF program
    if (bpf_object__load(obj)) {
    fprintf(stderr, "ERROR: loading BPF object file failed\n");
    goto cleanup;
    }
    cstate_map_fd = bpf_object__find_map_fd_by_name(obj, "cstate_duration");
    pstate_map_fd = bpf_object__find_map_fd_by_name(obj, "pstate_duration");
    if (cstate_map_fd < 0 || pstate_map_fd < 0) {
    fprintf(stderr, "ERROR: finding a map in obj file failed\n");
    goto cleanup;
    }
    link = bpf_program__attach(prog);
    if (libbpf_get_error(link)) {
    fprintf(stderr, "ERROR: bpf_program__attach failed\n");
    link = core::ptr::null_mut();
    goto cleanup;
    }
    ret = cpu_stat_inject_cpu_idle_event();
    if (ret < 0)
    return 1;
    ret = cpu_stat_inject_cpu_frequency_event();
    if (ret < 0)
    return 1;
    signal(SIGINT, int_exit);
    signal(SIGTERM, int_exit);
    while (1) {
    cpu_stat_update(cstate_map_fd, pstate_map_fd);
    cpu_stat_print();
    sleep(5);
    }
    cleanup:
    bpf_link__destroy(link);
    bpf_object__close(obj);
    return 0;
    }
