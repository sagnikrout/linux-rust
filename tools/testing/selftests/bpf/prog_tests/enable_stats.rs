//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/enable_stats.c
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

#[no_mangle]
pub unsafe extern "C" fn test_enable_stats() {
    void test_enable_stats(void)
    {
    struct test_enable_stats *skel;
    int stats_fd, err, prog_fd;
    struct bpf_prog_info info;
    let mut info_len: __u32 = sizeof(info);
    let mut duration: c_int = 0;
    skel = test_enable_stats__open_and_load();
    if (CHECK(!skel, "skel_open_and_load", "skeleton open/load failed\n"))
    return;
    stats_fd = bpf_enable_stats(BPF_STATS_RUN_TIME);
    if (CHECK(stats_fd < 0, "get_stats_fd", "failed %d\n", errno)) {
    test_enable_stats__destroy(skel);
    return;
    }
    err = test_enable_stats__attach(skel);
    if (CHECK(err, "attach_raw_tp", "err %d\n", err))
    goto cleanup;
    test_enable_stats__detach(skel);
    prog_fd = bpf_program__fd(skel.progs.test_enable_stats);
    memset(&info, 0, info_len);
    err = bpf_prog_get_info_by_fd(prog_fd, &info, &info_len);
    if (CHECK(err, "get_prog_info",
    "failed to get bpf_prog_info for fd %d\n", prog_fd))
    goto cleanup;
    if (CHECK(info.run_time_ns == 0, "check_stats_enabled",
    "failed to enable run_time_ns stats\n"))
    goto cleanup;
    CHECK(info.run_cnt != skel.bss.count, "check_run_cnt_valid",
    "invalid run_cnt stats\n");
    cleanup:
    test_enable_stats__destroy(skel);
    close(stats_fd);
    }
