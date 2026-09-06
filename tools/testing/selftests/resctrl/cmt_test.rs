//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/resctrl/cmt_test.c
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
// Cache Monitoring Technology (CMT) test
//
// Copyright (C) 2018 Intel Corporation
//
// Authors:
// Sai Praneeth Prakhya <sai.praneeth.prakhya@intel.com>,
// Fenghua Yu <fenghua.yu@intel.com>
//

pub const NUM_OF_RUNS: c_int = 5;
pub const MAX_DIFF: c_int = 2000000;
pub const MAX_DIFF_PERCENT: c_int = 15;

    "%s/%s/mon_data/mon_L3_%02d/llc_occupancy"
//
// Initialize capacity bitmasks (CBMs) of:
// - control group being tested per test parameters,
// - default resource group as inverse of control group being tested to prevent
// other tasks from interfering with test,
// - L2 resource of control group being tested to minimize allocations into
// L2 if possible to better predict L3 occupancy.
//
    static int cmt_init(const struct resctrl_test *test,
    const struct user_params *uparams,
    const struct resctrl_val_param *param, int domain_id)
    {
    unsigned long full_mask;
    char schemata[64];
    int ret;
    sprintf(llc_occup_path, CON_MON_LCC_OCCUP_PATH, RESCTRL_PATH,
    param.ctrlgrp, domain_id);
    ret = get_full_cbm(test.resource, &full_mask);
    if (ret)
    return ret;
    snprintf(schemata, sizeof(schemata), "%lx", ~param.mask & full_mask);
    ret = write_schemata("", schemata, uparams.cpu, test.resource);
    if (ret)
    return ret;
    snprintf(schemata, sizeof(schemata), "%lx", param.mask);
    ret = write_schemata(param.ctrlgrp, schemata, uparams.cpu, test.resource);
    if (ret)
    return ret;
    return minimize_l2_occupancy(test, uparams, param);
    }
    static int cmt_setup(const struct resctrl_test *test,
    const struct user_params *uparams,
    struct resctrl_val_param *p)
    {
// Run NUM_OF_RUNS times
    if (p.num_of_runs >= NUM_OF_RUNS)
    return END_OF_TESTS;
    p.num_of_runs++;
    return 0;
    }
    static int cmt_measure(const struct user_params *uparams,
    struct resctrl_val_param *param, pid_t bm_pid)
    {
    sleep(1);
    return measure_llc_resctrl(param.filename, bm_pid);
    }
    static int show_results_info(unsigned long sum_llc_val, int no_of_bits,
    unsigned long cache_span, unsigned long max_diff,
    unsigned long max_diff_percent, unsigned long num_of_runs,
    bool platform)
    {
    let mut avg_llc_val: c_ulong = 0;
    float diff_percent;
    let mut avg_diff: c_long = 0;
    int ret;
    avg_llc_val = sum_llc_val / num_of_runs;
    avg_diff = (long)(cache_span - avg_llc_val);
    diff_percent = ((float)cache_span - avg_llc_val) / cache_span * 100;
    ret = platform && abs((int)diff_percent) > max_diff_percent &&
    labs(avg_diff) > max_diff;
    ksft_print_msg("%s Check cache miss rate within %lu%%\n",
    ret ? "Fail:" : "Pass:", max_diff_percent);
    ksft_print_msg("Percent diff=%d\n", abs((int)diff_percent));
    show_cache_info(no_of_bits, avg_llc_val, cache_span, false);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn check_results(param: *mut resctrl_val_param, span: usize, no_of_bits: c_int) -> c_int {
    static int check_results(struct resctrl_val_param *param, size_t span, int no_of_bits)
    {
    char *token_array[8], temp[512];
    let mut sum_llc_occu_resc: c_ulong = 0;
    let mut runs: c_int = 0;
    FILE *fp;
    ksft_print_msg("Checking for pass/fail\n");
    fp = fopen(param.filename, "r");
    if (!fp) {
    ksft_perror("Error in opening file");
    return -1;
    }
    while (fgets(temp, sizeof(temp), fp)) {
    char *token = strtok(temp, ":\t");
    let mut fields: c_int = 0;
    while (token) {
    token_array[fields++] = token;
    token = strtok(core::ptr::null_mut(), ":\t");
    }
// Field 3 is llc occ resc value
    sum_llc_occu_resc += strtoul(token_array[3], core::ptr::null_mut(), 0);
    runs++;
    }
    fclose(fp);
    return show_results_info(sum_llc_occu_resc, no_of_bits, span,
    MAX_DIFF, MAX_DIFF_PERCENT, runs, true);
    }
#[no_mangle]
unsafe extern "C" fn cmt_test_cleanup() {
    static void cmt_test_cleanup(void)
    {
    remove(RESULT_FILE_NAME);
    }
#[no_mangle]
unsafe extern "C" fn cmt_run_test(test: *const resctrl_test, uparams: *const user_params) -> c_int {
    static int cmt_run_test(const struct resctrl_test *test, const struct user_params *uparams)
    {
    let mut fill_buf: fill_buf_param = {};
    let mut cache_total_size: c_ulong = 0;
    let mut n: c_int = uparams.bits ? : 5;
    unsigned long long_mask;
    int count_of_bits;
    size_t span;
    int ret;
    ret = get_full_cbm("L3", &long_mask);
    if (ret)
    return ret;
    ret = get_cache_size(uparams.cpu, "L3", &cache_total_size);
    if (ret)
    return ret;
    ksft_print_msg("Cache size :%lu\n", cache_total_size);
    count_of_bits = count_bits(long_mask);
    if (n < 1 || n > count_of_bits) {
    ksft_print_msg("Invalid input value for numbr_of_bits n!\n");
    ksft_print_msg("Please enter value in range 1 to %d\n", count_of_bits);
    return -1;
    }
    struct resctrl_val_param param = {
    .ctrlgrp	= "c1",
    .filename	= RESULT_FILE_NAME,
    .mask		= ~(long_mask << n) & long_mask,
    .num_of_runs	= 0,
    .init		= cmt_init,
    .setup		= cmt_setup,
    .measure	= cmt_measure,
    };
    span = cache_portion_size(cache_total_size, param.mask, long_mask);
    if (uparams.fill_buf) {
    fill_buf.buf_size = span * 2;
    fill_buf.memflush = uparams.fill_buf.memflush;
    param.fill_buf = &fill_buf;
    } else if (!uparams.benchmark_cmd[0]) {
    fill_buf.buf_size = span * 2;
    fill_buf.memflush = true;
    param.fill_buf = &fill_buf;
    }
    remove(RESULT_FILE_NAME);
    ret = resctrl_val(test, uparams, &param);
    if (ret)
    return ret;
    ret = check_results(&param, span, n);
    if (ret && (get_vendor() == ARCH_INTEL) && !snc_kernel_support())
    ksft_print_msg("Kernel doesn't support Sub-NUMA Clustering but it is enabled on the system.\n");
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn cmt_feature_check(test: *const resctrl_test) -> bool {
    static bool cmt_feature_check(const struct resctrl_test *test)
    {
    return test_resource_feature_check(test) &&
    resctrl_mon_feature_exists("L3_MON", "llc_occupancy");
    }
    struct resctrl_test cmt_test = {
    .name = "CMT",
    .resource = "L3",
    .feature_check = cmt_feature_check,
    .run_test = cmt_run_test,
    .cleanup = cmt_test_cleanup,
    };
