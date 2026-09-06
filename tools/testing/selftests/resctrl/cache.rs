//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/resctrl/cache.c
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

    char llc_occup_path[1024];
#[no_mangle]
pub unsafe extern "C" fn perf_event_attr_initialize(pea: *mut perf_event_attr, config: __u64) {
    void perf_event_attr_initialize(struct perf_event_attr *pea, __u64 config)
    {
    memset(pea, 0, sizeof(*pea));
    pea.type = PERF_TYPE_HARDWARE;
    pea.size = sizeof(*pea);
    pea.exclude_kernel = 1;
    pea.exclude_hv = 1;
    pea.exclude_idle = 1;
    pea.exclude_callchain_kernel = 1;
    pea.inherit = 1;
    pea.exclude_guest = 1;
    pea.disabled = 1;
    pea.config = config;
    }
// Start counters to log values
#[no_mangle]
pub unsafe extern "C" fn perf_event_reset_enable(pe_fd: c_int) -> c_int {
    int perf_event_reset_enable(int pe_fd)
    {
    int ret;
    ret = ioctl(pe_fd, PERF_EVENT_IOC_RESET, 0);
    if (ret < 0)
    return ret;
    ret = ioctl(pe_fd, PERF_EVENT_IOC_ENABLE, 0);
    if (ret < 0)
    return ret;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn perf_open(pea: *mut perf_event_attr, pid: pid_t, cpu_no: c_int) -> c_int {
    int perf_open(struct perf_event_attr *pea, pid_t pid, int cpu_no)
    {
    int pe_fd;
    pe_fd = perf_event_open(pea, pid, cpu_no, -1, PERF_FLAG_FD_CLOEXEC);
    if (pe_fd == -1) {
    ksft_perror("Unable to set up performance monitoring");
    return -1;
    }
    perf_event_reset_enable(pe_fd);
    return pe_fd;
    }
//
// Get LLC Occupancy as reported by RESCTRL FS
// For CMT,
// 1. If con_mon grp and mon grp given, then read from mon grp in
// con_mon grp
// 2. If only con_mon grp given, then read from con_mon grp
// 3. If both not given, then read from root con_mon grp
// For CAT,
// 1. If con_mon grp given, then read from it
// 2. If con_mon grp not given, then read from root con_mon grp
//
// Return: =0 on success.  <0 on failure.
//
#[no_mangle]
unsafe extern "C" fn get_llc_occu_resctrl(llc_occupancy: *mut c_ulong) -> c_int {
    static int get_llc_occu_resctrl(unsigned long *llc_occupancy)
    {
    FILE *fp;
    fp = fopen(llc_occup_path, "r");
    if (!fp) {
    ksft_perror("Failed to open results file");
    return -1;
    }
    if (fscanf(fp, "%lu", llc_occupancy) <= 0) {
    ksft_perror("Could not get llc occupancy");
    fclose(fp);
    return -1;
    }
    fclose(fp);
    return 0;
    }
//
// print_results_cache:	the cache results are stored in a file
// @filename:		file that stores the results
// @bm_pid:		child pid that runs benchmark
// @llc_value:		perf miss value
// llc occupancy value reported by resctrl FS
//
// Return:		0 on success, < 0 on error.
//
#[no_mangle]
unsafe extern "C" fn print_results_cache(filename: *const c_char, bm_pid: pid_t, llc_value: __u64) -> c_int {
    static int print_results_cache(const char *filename, pid_t bm_pid, __u64 llc_value)
    {
    FILE *fp;
    if (strcmp(filename, "stdio") == 0 || strcmp(filename, "stderr") == 0) {
    printf("Pid: %d \t LLC_value: %llu\n", (int)bm_pid, llc_value);
    } else {
    fp = fopen(filename, "a");
    if (!fp) {
    ksft_perror("Cannot open results file");
    return -1;
    }
    fprintf(fp, "Pid: %d \t llc_value: %llu\n", (int)bm_pid, llc_value);
    fclose(fp);
    }
    return 0;
    }
//
// perf_event_measure - Measure perf events
// @filename:	Filename for writing the results
// @bm_pid:	PID that runs the benchmark
//
// Measures perf events (e.g., cache misses) and writes the results into
// @filename. @bm_pid is written to the results file along with the measured
// value.
//
// Return: =0 on success. <0 on failure.
//
#[no_mangle]
pub unsafe extern "C" fn perf_event_measure(pe_fd: c_int, filename: *const c_char, bm_pid: pid_t) -> c_int {
    int perf_event_measure(int pe_fd, const char *filename, pid_t bm_pid)
    {
    __u64 value;
    int ret;
// Stop counters after one span to get miss rate
    ret = ioctl(pe_fd, PERF_EVENT_IOC_DISABLE, 0);
    if (ret < 0)
    return ret;
    ret = read(pe_fd, &value, sizeof(value));
    if (ret == -1) {
    ksft_perror("Could not get perf value");
    return -1;
    }
    return print_results_cache(filename, bm_pid, value);
    }
//
// measure_llc_resctrl - Measure resctrl LLC value from resctrl
// @filename:	Filename for writing the results
// @bm_pid:	PID that runs the benchmark
//
// Measures LLC occupancy from resctrl and writes the results into @filename.
// @bm_pid is written to the results file along with the measured value.
//
// Return: =0 on success. <0 on failure.
//
#[no_mangle]
pub unsafe extern "C" fn measure_llc_resctrl(filename: *const c_char, bm_pid: pid_t) -> c_int {
    int measure_llc_resctrl(const char *filename, pid_t bm_pid)
    {
    let mut llc_occu_resc: c_ulong = 0;
    int ret;
    ret = get_llc_occu_resctrl(&llc_occu_resc);
    if (ret < 0)
    return ret;
    return print_results_cache(filename, bm_pid, llc_occu_resc);
    }
//
// Reduce L2 allocation to minimum when testing L3 cache allocation.
//
    int minimize_l2_occupancy(const struct resctrl_test *test,
    const struct user_params *uparams,
    const struct resctrl_val_param *param)
    {
    if (!strcmp(test.resource, "L3") && resctrl_resource_exists("L2"))
    return write_schemata(param.ctrlgrp, "0x1", uparams.cpu, "L2");
    return 0;
    }
//
// show_cache_info - Show generic cache test information
// @no_of_bits:		Number of bits
// @avg_llc_val:	Average of LLC cache result data
// @cache_span:		Cache span
// @lines:		@cache_span in lines or bytes
//
#[no_mangle]
pub unsafe extern "C" fn show_cache_info(no_of_bits: c_int, avg_llc_val: __u64, cache_span: usize, lines: bool) {
    void show_cache_info(int no_of_bits, __u64 avg_llc_val, size_t cache_span, bool lines)
    {
    ksft_print_msg("Number of bits: %d\n", no_of_bits);
    ksft_print_msg("Average LLC val: %llu\n", avg_llc_val);
    ksft_print_msg("Cache span (%s): %zu\n", lines ? "lines" : "bytes",
    cache_span);
    }
