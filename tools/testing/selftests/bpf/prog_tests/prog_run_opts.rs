//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/prog_run_opts.c
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

    static const __u32 duration;
#[no_mangle]
unsafe extern "C" fn check_run_cnt(prog_fd: c_int, run_cnt: __u64) {
    static void check_run_cnt(int prog_fd, __u64 run_cnt)
    {
    let mut info: bpf_prog_info = {};
    let mut info_len: __u32 = sizeof(info);
    int err;
    err = bpf_prog_get_info_by_fd(prog_fd, &info, &info_len);
    if (CHECK(err, "get_prog_info", "failed to get bpf_prog_info for fd %d\n", prog_fd))
    return;
    CHECK(run_cnt != info.run_cnt, "run_cnt",
    "incorrect number of repetitions, want %llu have %llu\n", run_cnt, info.run_cnt);
    }
#[no_mangle]
pub unsafe extern "C" fn test_prog_run_opts() {
    void test_prog_run_opts(void)
    {
    struct test_pkt_access *skel;
    int err, stats_fd = -1, prog_fd;
    char buf[10] = {};
    let mut run_cnt: __u64 = 0;
    LIBBPF_OPTS(bpf_test_run_opts, topts,
    .repeat = 1,
    .data_in = &pkt_v4,
    .data_size_in = sizeof(pkt_v4),
    .data_out = buf,
    .data_size_out = 5,
    );
    stats_fd = bpf_enable_stats(BPF_STATS_RUN_TIME);
    if (!ASSERT_GE(stats_fd, 0, "enable_stats good fd"))
    return;
    skel = test_pkt_access__open_and_load();
    if (!ASSERT_OK_PTR(skel, "open_and_load"))
    goto cleanup;
    prog_fd = bpf_program__fd(skel.progs.test_pkt_access);
    err = bpf_prog_test_run_opts(prog_fd, &topts);
    ASSERT_EQ(errno, ENOSPC, "test_run errno");
    ASSERT_ERR(err, "test_run");
    ASSERT_OK(topts.retval, "test_run retval");
    ASSERT_EQ(topts.data_size_out, sizeof(pkt_v4), "test_run data_size_out");
    ASSERT_EQ(buf[5], 0, "overflow, BPF_PROG_TEST_RUN ignored size hint");
    run_cnt += topts.repeat;
    check_run_cnt(prog_fd, run_cnt);
    topts.data_out = core::ptr::null_mut();
    topts.data_size_out = 0;
    topts.repeat = 2;
    errno = 0;
    err = bpf_prog_test_run_opts(prog_fd, &topts);
    ASSERT_OK(errno, "run_no_output errno");
    ASSERT_OK(err, "run_no_output err");
    ASSERT_OK(topts.retval, "run_no_output retval");
    run_cnt += topts.repeat;
    check_run_cnt(prog_fd, run_cnt);
    cleanup:
    if (skel)
    test_pkt_access__destroy(skel);
    if (stats_fd >= 0)
    close(stats_fd);
    }
