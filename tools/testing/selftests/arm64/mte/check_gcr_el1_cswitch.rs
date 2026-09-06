//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/arm64/mte/check_gcr_el1_cswitch.c
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

pub const NUM_ITERATIONS: c_int = 1024;
pub const MAX_THREADS: c_int = 5;
pub const THREAD_ITERATIONS: c_int = 1000;
    void *execute_thread(void *x)
    {
    let mut pid: pid_t = *((pid_t *)x);
    let mut tid: pid_t = gettid();
    uint64_t prctl_tag_mask;
    uint64_t prctl_set;
    uint64_t prctl_get;
    uint64_t prctl_tcf;
    srand(time(core::ptr::null_mut()) ^ (pid << 16) ^ (tid << 16));
    prctl_tag_mask = rand() & 0xffff;
    if (prctl_tag_mask % 2)
    prctl_tcf = PR_MTE_TCF_SYNC;
    else
    prctl_tcf = PR_MTE_TCF_ASYNC;
    prctl_set = PR_TAGGED_ADDR_ENABLE | prctl_tcf | (prctl_tag_mask << PR_MTE_TAG_SHIFT);
    for (int j = 0; j < THREAD_ITERATIONS; j++) {
    if (prctl(PR_SET_TAGGED_ADDR_CTRL, prctl_set, 0, 0, 0)) {
    perror("prctl() failed");
    goto fail;
    }
    prctl_get = prctl(PR_GET_TAGGED_ADDR_CTRL, 0, 0, 0, 0);
    if (prctl_set != prctl_get) {
    ksft_print_msg("Error: prctl_set: 0x%lx != prctl_get: 0x%lx\n",
    prctl_set, prctl_get);
    goto fail;
    }
    }
    return (void *)KSFT_PASS;
    fail:
    return (void *)KSFT_FAIL;
    }
#[no_mangle]
pub unsafe extern "C" fn execute_test(pid: pid_t) -> c_int {
    int execute_test(pid_t pid)
    {
    pthread_t thread_id[MAX_THREADS];
    int thread_data[MAX_THREADS];
    for (int i = 0; i < MAX_THREADS; i++)
    pthread_create(&thread_id[i], core::ptr::null_mut(),
    execute_thread, (void *)&pid);
    for (int i = 0; i < MAX_THREADS; i++)
    pthread_join(thread_id[i], (void *)&thread_data[i]);
    for (int i = 0; i < MAX_THREADS; i++)
    if (thread_data[i] == KSFT_FAIL)
    return KSFT_FAIL;
    return KSFT_PASS;
    }
#[no_mangle]
pub unsafe extern "C" fn mte_gcr_fork_test() -> c_int {
    int mte_gcr_fork_test(void)
    {
    pid_t pid;
    int results[NUM_ITERATIONS];
    pid_t cpid;
    int res;
    for (int i = 0; i < NUM_ITERATIONS; i++) {
    pid = fork();
    if (pid < 0)
    return KSFT_FAIL;
    if (pid == 0) {
    cpid = getpid();
    res = execute_test(cpid);
    exit(res);
    }
    }
    for (int i = 0; i < NUM_ITERATIONS; i++) {
    wait(&res);
    if (WIFEXITED(res))
    results[i] = WEXITSTATUS(res);
    else
    --i;
    }
    for (int i = 0; i < NUM_ITERATIONS; i++)
    if (results[i] == KSFT_FAIL)
    return KSFT_FAIL;
    return KSFT_PASS;
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv[]: *mut c_char) -> c_int {
    int main(int argc, char *argv[])
    {
    int err;
    err = mte_default_setup();
    if (err)
    return err;
    ksft_print_header();
    ksft_set_plan(1);
    evaluate_test(mte_gcr_fork_test(),
    "Verify that GCR_EL1 is set correctly on context switch\n");
    mte_restore_setup();
    ksft_print_cnts();
    return ksft_get_fail_cnt() == 0 ? KSFT_PASS : KSFT_FAIL;
    }
