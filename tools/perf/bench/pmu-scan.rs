//! Automatically rewritten from C to Rust
//! Source: tools/perf/bench/pmu-scan.c
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
// Benchmark scanning sysfs files for PMU information.
//
// Copyright 2023 Google LLC.
//

    let mut iterations: static unsigned int = 100;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pmu_scan_result {
    pub name: *mut c_char,
    pub nr_aliases: c_int,
    pub nr_formats: c_int,
    pub nr_caps: c_int,
    pub is_core: bool,
}

    static const struct option options[] = {
    OPT_UINTEGER('i', "iterations", &iterations,
    "Number of iterations used to compute average"),
    OPT_END()
    };
    static const char *const bench_usage[] = {
    "perf bench internals pmu-scan <options>",
    core::ptr::null_mut()
    };
    static int nr_pmus;
    static struct pmu_scan_result *results;
#[no_mangle]
unsafe extern "C" fn save_result() -> c_int {
    static int save_result(void)
    {
    struct perf_pmu *pmu = core::ptr::null_mut();
    struct list_head *list;
    struct pmu_scan_result *r;
    while ((pmu = perf_pmus__scan(pmu)) != core::ptr::null_mut()) {
    r = realloc(results, (nr_pmus + 1) * sizeof(*r));
    if (r == core::ptr::null_mut())
    return -ENOMEM;
    results = r;
    r = results + nr_pmus;
    r.name = strdup(pmu.name);
    r.is_core = pmu.is_core;
    r.nr_caps = pmu.nr_caps;
    r.nr_aliases = perf_pmu__num_events(pmu);
    r.nr_formats = 0;
    list_for_each(list, &pmu.format)
    r.nr_formats++;
    pr_debug("pmu[%d] name=%s, nr_caps=%d, nr_aliases=%d, nr_formats=%d\n",
    nr_pmus, r.name, r.nr_caps, r.nr_aliases, r.nr_formats);
    nr_pmus++;
    }
    perf_pmus__destroy();
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn check_result(core_only: bool) -> c_int {
    static int check_result(bool core_only)
    {
    struct pmu_scan_result *r;
    struct perf_pmu *pmu;
    struct list_head *list;
    int nr;
    for (int i = 0; i < nr_pmus; i++) {
    r = &results[i];
    if (core_only && !r.is_core)
    continue;
    pmu = perf_pmus__find(r.name);
    if (pmu == core::ptr::null_mut()) {
    pr_err("Cannot find PMU %s\n", r.name);
    return -1;
    }
    if (pmu.nr_caps != (u32)r.nr_caps) {
    pr_err("Unmatched number of event caps in %s: expect %d vs got %d\n",
    pmu.name, r.nr_caps, pmu.nr_caps);
    return -1;
    }
    nr = perf_pmu__num_events(pmu);
    if (nr != r.nr_aliases) {
    pr_err("Unmatched number of event aliases in %s: expect %d vs got %d\n",
    pmu.name, r.nr_aliases, nr);
    return -1;
    }
    nr = 0;
    list_for_each(list, &pmu.format)
    nr++;
    if (nr != r.nr_formats) {
    pr_err("Unmatched number of event formats in %s: expect %d vs got %d\n",
    pmu.name, r.nr_formats, nr);
    return -1;
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn delete_result() {
    static void delete_result(void)
    {
    for (int i = 0; i < nr_pmus; i++)
    free(results[i].name);
    free(results);
    results = core::ptr::null_mut();
    nr_pmus = 0;
    }
#[no_mangle]
unsafe extern "C" fn run_pmu_scan() -> c_int {
    static int run_pmu_scan(void)
    {
    struct stats stats;
    struct timeval start, end, diff;
    double time_average, time_stddev;
    u64 runtime_us;
    int ret;
    init_stats(&stats);
    pr_info("Computing performance of sysfs PMU event scan for %u times\n",
    iterations);
    if (save_result() < 0) {
    pr_err("Failed to initialize PMU scan result\n");
    return -1;
    }
    for (int j = 0; j < 2; j++) {
    let mut core_only: bool = (j == 0);
    for (unsigned int i = 0; i < iterations; i++) {
    gettimeofday(&start, core::ptr::null_mut());
    if (core_only)
    perf_pmus__scan_core(core::ptr::null_mut());
    else
    perf_pmus__scan(core::ptr::null_mut());
    gettimeofday(&end, core::ptr::null_mut());
    timersub(&end, &start, &diff);
    runtime_us = diff.tv_sec * USEC_PER_SEC + diff.tv_usec;
    update_stats(&stats, runtime_us);
    ret = check_result(core_only);
    perf_pmus__destroy();
    if (ret < 0)
    break;
    }
    time_average = avg_stats(&stats);
    time_stddev = stddev_stats(&stats);
    pr_info("  Average%s PMU scanning took: %.3f usec (+- %.3f usec)\n",
    core_only ? " core" : "", time_average, time_stddev);
    }
    delete_result();
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn bench_pmu_scan(argc: c_int, argv: *const c_char) -> c_int {
    int bench_pmu_scan(int argc, const char **argv)
    {
    let mut err: c_int = 0;
    argc = parse_options(argc, argv, options, bench_usage, 0);
    if (argc) {
    usage_with_options(bench_usage, options);
    exit(EXIT_FAILURE);
    }
    err = run_pmu_scan();
    return err;
    }
