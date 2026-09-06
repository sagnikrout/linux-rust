//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/arm64/mte/check_ksm_options.c
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
// Copyright (C) 2020 ARM Limited
// Macro flag: #define _GNU_SOURCE

pub const TEST_UNIT: c_int = 10;

pub const MAX_LOOP: c_int = 4;
    static size_t page_sz;
    static unsigned long ksm_sysfs[5];
    static bool has_merge_across_nodes;
#[no_mangle]
unsafe extern "C" fn merge_across_nodes_available() -> bool {
    static bool merge_across_nodes_available(void)
    {
    const char *path = PATH_KSM "merge_across_nodes";
    if (!access(path, R_OK | W_OK))
    return true;
    if (errno == ENOENT)
    return false;
    ksft_exit_skip("Unable to read and write %s: %s\n", path,
    strerror(errno));
    }
#[no_mangle]
unsafe extern "C" fn read_sysfs(str: *mut c_char) -> c_ulong {
    static unsigned long read_sysfs(char *str)
    {
    FILE *f;
    let mut val: c_ulong = 0;
    f = fopen(str, "r");
    if (!f) {
    ksft_print_msg("ERR: missing %s\n", str);
    return 0;
    }
    if (fscanf(f, "%lu", &val) != 1) {
    ksft_print_msg("ERR: parsing %s\n", str);
    val = 0;
    }
    fclose(f);
    return val;
    }
#[no_mangle]
unsafe extern "C" fn write_sysfs(str: *mut c_char, val: c_ulong) {
    static void write_sysfs(char *str, unsigned long val)
    {
    FILE *f;
    f = fopen(str, "w");
    if (!f) {
    ksft_print_msg("ERR: missing %s\n", str);
    return;
    }
    fprintf(f, "%lu", val);
    fclose(f);
    }
#[no_mangle]
unsafe extern "C" fn mte_ksm_setup() {
    static void mte_ksm_setup(void)
    {
    if (has_merge_across_nodes) {
    ksm_sysfs[0] = read_sysfs(PATH_KSM "merge_across_nodes");
    write_sysfs(PATH_KSM "merge_across_nodes", 1);
    }
    ksm_sysfs[1] = read_sysfs(PATH_KSM "sleep_millisecs");
    write_sysfs(PATH_KSM "sleep_millisecs", 0);
    ksm_sysfs[2] = read_sysfs(PATH_KSM "run");
    write_sysfs(PATH_KSM "run", 1);
    ksm_sysfs[3] = read_sysfs(PATH_KSM "max_page_sharing");
    write_sysfs(PATH_KSM "max_page_sharing", ksm_sysfs[3] + TEST_UNIT);
    ksm_sysfs[4] = read_sysfs(PATH_KSM "pages_to_scan");
    write_sysfs(PATH_KSM "pages_to_scan", ksm_sysfs[4] + TEST_UNIT);
    }
#[no_mangle]
unsafe extern "C" fn mte_ksm_restore() {
    static void mte_ksm_restore(void)
    {
    if (has_merge_across_nodes)
    write_sysfs(PATH_KSM "merge_across_nodes", ksm_sysfs[0]);
    write_sysfs(PATH_KSM "sleep_millisecs", ksm_sysfs[1]);
    write_sysfs(PATH_KSM "run", ksm_sysfs[2]);
    write_sysfs(PATH_KSM "max_page_sharing", ksm_sysfs[3]);
    write_sysfs(PATH_KSM "pages_to_scan", ksm_sysfs[4]);
    }
#[no_mangle]
unsafe extern "C" fn mte_ksm_scan() {
    static void mte_ksm_scan(void)
    {
    let mut cur_count: c_int = read_sysfs(PATH_KSM "full_scans");
    let mut scan_count: c_int = cur_count + 1;
    let mut max_loop_count: c_int = MAX_LOOP;
    while ((cur_count < scan_count) && max_loop_count) {
    sleep(1);
    cur_count = read_sysfs(PATH_KSM "full_scans");
    max_loop_count--;
    }

    ksft_print_msg("INFO: pages_shared=%lu pages_sharing=%lu\n",
    read_sysfs(PATH_KSM "pages_shared"),
    read_sysfs(PATH_KSM "pages_sharing"));

    }
#[no_mangle]
unsafe extern "C" fn check_madvise_options(mem_type: c_int, mode: c_int, mapping: c_int) -> c_int {
    static int check_madvise_options(int mem_type, int mode, int mapping)
    {
    char *ptr;
    int err, ret;
    err = KSFT_FAIL;
    if (access(PATH_KSM, F_OK) == -1) {
    ksft_print_msg("ERR: Kernel KSM config not enabled\n");
    return err;
    }
    mte_switch_mode(mode, MTE_ALLOW_NON_ZERO_TAG, false);
    ptr = mte_allocate_memory(TEST_UNIT * page_sz, mem_type, mapping, true);
    if (check_allocated_memory(ptr, TEST_UNIT * page_sz, mem_type, false) != KSFT_PASS)
    return KSFT_FAIL;
// Insert same data in all the pages
    memset(ptr, 'A', TEST_UNIT * page_sz);
    ret = madvise(ptr, TEST_UNIT * page_sz, MADV_MERGEABLE);
    if (ret) {
    ksft_print_msg("ERR: madvise failed to set MADV_UNMERGEABLE\n");
    goto madvise_err;
    }
    mte_ksm_scan();
// Tagged pages should not merge
    if ((read_sysfs(PATH_KSM "pages_shared") < 1) ||
    (read_sysfs(PATH_KSM "pages_sharing") < (TEST_UNIT - 1)))
    err = KSFT_PASS;
    madvise_err:
    mte_free_memory(ptr, TEST_UNIT * page_sz, mem_type, true);
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv[]: *mut c_char) -> c_int {
    int main(int argc, char *argv[])
    {
    int err;
    ksft_print_header();
    err = mte_default_setup();
    if (err)
    return err;
    if (geteuid() != 0)
    ksft_exit_skip("Please run the test as root\n");
    has_merge_across_nodes = merge_across_nodes_available();
    page_sz = getpagesize();
    if (!page_sz) {
    ksft_print_msg("ERR: Unable to get page size\n");
    return KSFT_FAIL;
    }
// Register signal handlers
    mte_register_signal(SIGBUS, mte_default_handler, false);
    mte_register_signal(SIGSEGV, mte_default_handler, false);
// Set test plan
    ksft_set_plan(4);
// Enable KSM
    mte_ksm_setup();
    evaluate_test(check_madvise_options(USE_MMAP, MTE_SYNC_ERR, MAP_PRIVATE),
    "Check KSM mte page merge for private mapping, sync mode and mmap memory\n");
    evaluate_test(check_madvise_options(USE_MMAP, MTE_ASYNC_ERR, MAP_PRIVATE),
    "Check KSM mte page merge for private mapping, async mode and mmap memory\n");
    evaluate_test(check_madvise_options(USE_MMAP, MTE_SYNC_ERR, MAP_SHARED),
    "Check KSM mte page merge for shared mapping, sync mode and mmap memory\n");
    evaluate_test(check_madvise_options(USE_MMAP, MTE_ASYNC_ERR, MAP_SHARED),
    "Check KSM mte page merge for shared mapping, async mode and mmap memory\n");
    mte_ksm_restore();
    mte_restore_setup();
    ksft_print_cnts();
    return ksft_get_fail_cnt() == 0 ? KSFT_PASS : KSFT_FAIL;
    }
