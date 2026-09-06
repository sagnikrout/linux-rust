//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/time_tai.c
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
// Copyright (C) 2022 Linutronix GmbH

#[no_mangle]
unsafe extern "C" fn ts_to_ns(ts: *const timespec) -> __u64 {
    static __u64 ts_to_ns(const struct timespec *ts)
    {
    return ts.tv_sec * NSEC_PER_SEC + ts.tv_nsec;
    }
#[no_mangle]
pub unsafe extern "C" fn test_time_tai() {
    void test_time_tai(void)
    {
    struct __sk_buff skb = {
    .cb[0] = 0,
    .cb[1] = 0,
    .tstamp = 0,
    };
    LIBBPF_OPTS(bpf_test_run_opts, topts,
    .data_in = &pkt_v4,
    .data_size_in = sizeof(pkt_v4),
    .ctx_in = &skb,
    .ctx_size_in = sizeof(skb),
    .ctx_out = &skb,
    .ctx_size_out = sizeof(skb),
    );
    struct test_time_tai *skel;
    struct timespec now_tai;
    __u64 ts1, ts2, now;
    int ret, prog_fd;
// Open and load
    skel = test_time_tai__open_and_load();
    if (!ASSERT_OK_PTR(skel, "tai_open"))
    return;
// Run test program
    prog_fd = bpf_program__fd(skel.progs.time_tai);
    ret = bpf_prog_test_run_opts(prog_fd, &topts);
    ASSERT_OK(ret, "test_run");
// Retrieve generated TAI timestamps
    ts1 = skb.tstamp;
    ts2 = skb.cb[0] | ((__u64)skb.cb[1] << 32);
// TAI != 0
    ASSERT_NEQ(ts1, 0, "tai_ts1");
    ASSERT_NEQ(ts2, 0, "tai_ts2");
// TAI is moving forward only
    ASSERT_GE(ts2, ts1, "tai_forward");
// Check for future
    ret = clock_gettime(CLOCK_TAI, &now_tai);
    ASSERT_EQ(ret, 0, "tai_gettime");
    now = ts_to_ns(&now_tai);
    ASSERT_TRUE(now > ts1, "tai_future_ts1");
    ASSERT_TRUE(now > ts2, "tai_future_ts2");
// Check for reasonable range
    ASSERT_TRUE(now - ts1 < TAI_THRESHOLD, "tai_range_ts1");
    ASSERT_TRUE(now - ts2 < TAI_THRESHOLD, "tai_range_ts2");
    test_time_tai__destroy(skel);
    }
