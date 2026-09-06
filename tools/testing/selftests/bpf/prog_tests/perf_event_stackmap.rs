//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/perf_event_stackmap.c
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
// Copyright (c) 2020 Facebook
// Macro flag: #define _GNU_SOURCE

#[no_mangle]
pub unsafe extern "C" fn func_1() -> noinline int {
    noinline int func_1(void)
    {
    let mut val: static int = 1;
    val += 1;
    usleep(100);
    return val;
    }
#[no_mangle]
pub unsafe extern "C" fn func_2() -> noinline int {
    noinline int func_2(void)
    {
    return func_1();
    }
#[no_mangle]
pub unsafe extern "C" fn func_3() -> noinline int {
    noinline int func_3(void)
    {
    return func_2();
    }
#[no_mangle]
pub unsafe extern "C" fn func_4() -> noinline int {
    noinline int func_4(void)
    {
    return func_3();
    }
#[no_mangle]
pub unsafe extern "C" fn func_5() -> noinline int {
    noinline int func_5(void)
    {
    return func_4();
    }
#[no_mangle]
pub unsafe extern "C" fn func_6() -> noinline int {
    noinline int func_6(void)
    {
    int i, val = 1;
    for (i = 0; i < 100; i++)
    val += func_5();
    return val;
    }
#[no_mangle]
pub unsafe extern "C" fn test_perf_event_stackmap() {
    void test_perf_event_stackmap(void)
    {
    struct perf_event_attr attr = {
// .type = PERF_TYPE_SOFTWARE,
    .type = PERF_TYPE_HARDWARE,
    .config = PERF_COUNT_HW_CPU_CYCLES,
    .precise_ip = 2,
    .sample_type = PERF_SAMPLE_IP | PERF_SAMPLE_BRANCH_STACK |
    PERF_SAMPLE_CALLCHAIN,
    .branch_sample_type = PERF_SAMPLE_BRANCH_USER |
    PERF_SAMPLE_BRANCH_NO_FLAGS |
    PERF_SAMPLE_BRANCH_NO_CYCLES |
    PERF_SAMPLE_BRANCH_CALL_STACK,
    .freq = 1,
    .sample_freq = read_perf_max_sample_freq(),
    .size = sizeof(struct perf_event_attr),
    };
    struct perf_event_stackmap *skel;
    let mut duration: __u32 = 0;
    cpu_set_t cpu_set;
    int pmu_fd, err;
    skel = perf_event_stackmap__open();
    if (CHECK(!skel, "skel_open", "skeleton open failed\n"))
    return;
    err = perf_event_stackmap__load(skel);
    if (CHECK(err, "skel_load", "skeleton load failed: %d\n", err))
    goto cleanup;
    CPU_ZERO(&cpu_set);
    CPU_SET(0, &cpu_set);
    err = pthread_setaffinity_np(pthread_self(), sizeof(cpu_set), &cpu_set);
    if (CHECK(err, "set_affinity", "err %d, errno %d\n", err, errno))
    goto cleanup;
    pmu_fd = syscall(__NR_perf_event_open, &attr, -1 /* pid */,
    0 /* cpu 0 */, -1 /* group id */,
    0 /* flags */);
    if (pmu_fd < 0) {
    printf("%s:SKIP:cpu doesn't support the event\n", __func__);
    test__skip();
    goto cleanup;
    }
    skel.links.oncpu = bpf_program__attach_perf_event(skel.progs.oncpu,
    pmu_fd);
    if (!ASSERT_OK_PTR(skel.links.oncpu, "attach_perf_event")) {
    close(pmu_fd);
    goto cleanup;
    }
// create kernel and user stack traces for testing
    func_6();
    CHECK(skel.data.stackid_kernel != 2, "get_stackid_kernel", "failed\n");
    CHECK(skel.data.stackid_user != 2, "get_stackid_user", "failed\n");
    CHECK(skel.data.stack_kernel != 2, "get_stack_kernel", "failed\n");
    CHECK(skel.data.stack_user != 2, "get_stack_user", "failed\n");
    cleanup:
    perf_event_stackmap__destroy(skel);
    }
