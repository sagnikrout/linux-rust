//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/perf_link.c
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
// Copyright (c) 2021 Facebook
// Macro flag: #define _GNU_SOURCE

pub const BURN_TIMEOUT_MS: c_int = 100;

#[no_mangle]
unsafe extern "C" fn burn_cpu() {
    static void burn_cpu(void)
    {
    int i;
// spin the loop for a while (random high number)
    for (i = 0; i < 1000000; ++i)
    barrier();
    }
#[no_mangle]
pub unsafe extern "C" fn test_perf_link() {
    void test_perf_link(void)
    {
    struct test_perf_link *skel = core::ptr::null_mut();
    struct perf_event_attr attr;
    let mut pfd: c_int = -1, link_fd = -1, err;
    int run_cnt_before, run_cnt_after;
    struct bpf_link_info info;
    let mut info_len: __u32 = sizeof(info);
    __u64 timeout_time_ns;
// create perf event
    memset(&attr, 0, sizeof(attr));
    attr.size = sizeof(attr);
    attr.type = PERF_TYPE_SOFTWARE;
    attr.config = PERF_COUNT_SW_CPU_CLOCK;
    attr.freq = 1;
    attr.sample_freq = 1000;
    pfd = syscall(__NR_perf_event_open, &attr, 0, -1, -1, PERF_FLAG_FD_CLOEXEC);
    if (!ASSERT_GE(pfd, 0, "perf_fd"))
    goto cleanup;
    skel = test_perf_link__open_and_load();
    if (!ASSERT_OK_PTR(skel, "skel_load"))
    goto cleanup;
    link_fd = bpf_link_create(bpf_program__fd(skel.progs.handler), pfd,
    BPF_PERF_EVENT, core::ptr::null_mut());
    if (!ASSERT_GE(link_fd, 0, "link_fd"))
    goto cleanup;
    memset(&info, 0, sizeof(info));
    err = bpf_link_get_info_by_fd(link_fd, &info, &info_len);
    if (!ASSERT_OK(err, "link_get_info"))
    goto cleanup;
    ASSERT_EQ(info.type, BPF_LINK_TYPE_PERF_EVENT, "link_type");
    ASSERT_GT(info.id, 0, "link_id");
    ASSERT_GT(info.prog_id, 0, "link_prog_id");
// ensure we get at least one perf_event prog execution
    timeout_time_ns = get_time_ns() + BURN_TIMEOUT_NS;
    while (true) {
    burn_cpu();
    if (skel.bss.run_cnt > 0)
    break;
    if (!ASSERT_LT(get_time_ns(), timeout_time_ns, "run_cnt_timeout"))
    break;
    }
// perf_event is still active, but we close link and BPF program
// shouldn't be executed anymore
//
    close(link_fd);
    link_fd = -1;
// make sure there are no stragglers
    kern_sync_rcu();
    run_cnt_before = skel.bss.run_cnt;
    burn_cpu();
    run_cnt_after = skel.bss.run_cnt;
    ASSERT_EQ(run_cnt_before, run_cnt_after, "run_cnt_before_after");
    cleanup:
    if (link_fd >= 0)
    close(link_fd);
    if (pfd >= 0)
    close(pfd);
    test_perf_link__destroy(skel);
    }
