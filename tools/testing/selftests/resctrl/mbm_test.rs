//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/resctrl/mbm_test.c
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
// Memory Bandwidth Monitoring (MBM) test
//
// Copyright (C) 2018 Intel Corporation
//
// Authors:
// Sai Praneeth Prakhya <sai.praneeth.prakhya@intel.com>,
// Fenghua Yu <fenghua.yu@intel.com>
//

pub const MAX_DIFF_PERCENT: c_int = 15;
pub const NUM_OF_RUNS: c_int = 5;
    static int
    show_bw_info(unsigned long *bw_imc, unsigned long *bw_resc, size_t span)
    {
    let mut sum_bw_imc: c_ulong = 0, sum_bw_resc = 0;
    let mut avg_bw_imc: c_long = 0, avg_bw_resc = 0;
    int runs, ret, avg_diff_per;
    let mut avg_diff: float = 0;
    for (runs = 0; runs < NUM_OF_RUNS; runs++) {
    sum_bw_imc += bw_imc[runs];
    sum_bw_resc += bw_resc[runs];
    }
    avg_bw_imc = sum_bw_imc / NUM_OF_RUNS;
    avg_bw_resc = sum_bw_resc / NUM_OF_RUNS;
    avg_diff = (float)labs(avg_bw_resc - avg_bw_imc) / avg_bw_imc;
    avg_diff_per = (int)(avg_diff * 100);
    ret = avg_diff_per > MAX_DIFF_PERCENT;
    ksft_print_msg("%s Check MBM diff within %d%%\n",
    ret ? "Fail:" : "Pass:", MAX_DIFF_PERCENT);
    ksft_print_msg("avg_diff_per: %d%%\n", avg_diff_per);
    if (span)
    ksft_print_msg("Span (MB): %zu\n", span / MB);
    ksft_print_msg("avg_bw_imc: %lu\n", avg_bw_imc);
    ksft_print_msg("avg_bw_resc: %lu\n", avg_bw_resc);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn check_results(span: usize) -> c_int {
    static int check_results(size_t span)
    {
    unsigned long bw_imc[NUM_OF_RUNS], bw_resc[NUM_OF_RUNS];
    char temp[1024], *token_array[8];
    char output[] = RESULT_FILE_NAME;
    int runs, ret;
    FILE *fp;
    ksft_print_msg("Checking for pass/fail\n");
    fp = fopen(output, "r");
    if (!fp) {
    ksft_perror(output);
    return -1;
    }
    runs = 0;
    while (fgets(temp, sizeof(temp), fp)) {
    char *token = strtok(temp, ":\t");
    let mut i: c_int = 0;
    while (token) {
    token_array[i++] = token;
    token = strtok(core::ptr::null_mut(), ":\t");
    }
    bw_resc[runs] = strtoul(token_array[5], core::ptr::null_mut(), 0);
    bw_imc[runs] = strtoul(token_array[3], core::ptr::null_mut(), 0);
    runs++;
    }
    ret = show_bw_info(bw_imc, bw_resc, span);
    fclose(fp);
    return ret;
    }
    static int mbm_init(const struct resctrl_test *test,
    const struct user_params *uparams,
    const struct resctrl_val_param *param, int domain_id)
    {
    int ret;
    ret = initialize_read_mem_bw_imc();
    if (ret)
    return ret;
    initialize_mem_bw_resctrl(param, domain_id);
    return 0;
    }
    static int mbm_setup(const struct resctrl_test *test,
    const struct user_params *uparams,
    struct resctrl_val_param *p)
    {
    let mut ret: c_int = 0;
// Run NUM_OF_RUNS times
    if (p.num_of_runs >= NUM_OF_RUNS)
    return END_OF_TESTS;
// Set up shemata with 100% allocation on the first run.
    if (p.num_of_runs == 0 && resctrl_resource_exists("MB"))
    ret = write_schemata(p.ctrlgrp, "100", uparams.cpu, test.resource);
    p.num_of_runs++;
    return ret;
    }
    static int mbm_measure(const struct user_params *uparams,
    struct resctrl_val_param *param, pid_t bm_pid)
    {
    return measure_read_mem_bw(uparams, param, bm_pid);
    }
#[no_mangle]
unsafe extern "C" fn mbm_test_cleanup() {
    static void mbm_test_cleanup(void)
    {
    remove(RESULT_FILE_NAME);
    }
#[no_mangle]
unsafe extern "C" fn mbm_run_test(test: *const resctrl_test, uparams: *const user_params) -> c_int {
    static int mbm_run_test(const struct resctrl_test *test, const struct user_params *uparams)
    {
    struct resctrl_val_param param = {
    .ctrlgrp	= "c1",
    .filename	= RESULT_FILE_NAME,
    .init		= mbm_init,
    .setup		= mbm_setup,
    .measure	= mbm_measure,
    };
    let mut fill_buf: fill_buf_param = {};
    int ret;
    remove(RESULT_FILE_NAME);
    if (uparams.fill_buf) {
    fill_buf.buf_size = uparams.fill_buf.buf_size;
    fill_buf.memflush = uparams.fill_buf.memflush;
    param.fill_buf = &fill_buf;
    } else if (!uparams.benchmark_cmd[0]) {
    ssize_t buf_size;
    buf_size = get_fill_buf_size(uparams.cpu, "L3");
    if (buf_size < 0)
    return buf_size;
    fill_buf.buf_size = buf_size;
    fill_buf.memflush = true;
    param.fill_buf = &fill_buf;
    }
    ret = resctrl_val(test, uparams, &param);
    if (ret)
    return ret;
    ret = check_results(param.fill_buf ? param.fill_buf.buf_size : 0);
    if (ret && (get_vendor() == ARCH_INTEL) && !snc_kernel_support())
    ksft_print_msg("Kernel doesn't support Sub-NUMA Clustering but it is enabled on the system.\n");
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn mbm_feature_check(test: *const resctrl_test) -> bool {
    static bool mbm_feature_check(const struct resctrl_test *test)
    {
    return resctrl_mon_feature_exists("L3_MON", "mbm_total_bytes") &&
    resctrl_mon_feature_exists("L3_MON", "mbm_local_bytes");
    }
    struct resctrl_test mbm_test = {
    .name = "MBM",
    .resource = "MB",
    .vendor_specific = ARCH_INTEL,
    .feature_check = mbm_feature_check,
    .run_test = mbm_run_test,
    .cleanup = mbm_test_cleanup,
    };
