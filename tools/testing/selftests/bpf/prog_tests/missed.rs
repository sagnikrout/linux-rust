//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/missed.c
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
// Putting kprobe on bpf_fentry_test1 that calls bpf_kfunc_common_test
// kfunc, which has also kprobe on. The latter won't get triggered due
// to kprobe recursion check and kprobe missed counter is incremented.
//
#[no_mangle]
unsafe extern "C" fn test_missed_perf_kprobe() {
    static void test_missed_perf_kprobe(void)
    {
    LIBBPF_OPTS(bpf_test_run_opts, topts);
    let mut info: bpf_link_info = {};
    struct missed_kprobe *skel;
    let mut len: __u32 = sizeof(info);
    int err, prog_fd;
    skel = missed_kprobe__open_and_load();
    if (!ASSERT_OK_PTR(skel, "missed_kprobe__open_and_load"))
    goto cleanup;
    err = missed_kprobe__attach(skel);
    if (!ASSERT_OK(err, "missed_kprobe__attach"))
    goto cleanup;
    prog_fd = bpf_program__fd(skel.progs.trigger);
    err = bpf_prog_test_run_opts(prog_fd, &topts);
    ASSERT_OK(err, "test_run");
    ASSERT_EQ(topts.retval, 0, "test_run");
    err = bpf_link_get_info_by_fd(bpf_link__fd(skel.links.test2), &info, &len);
    if (!ASSERT_OK(err, "bpf_link_get_info_by_fd"))
    goto cleanup;
    ASSERT_EQ(info.type, BPF_LINK_TYPE_PERF_EVENT, "info.type");
    ASSERT_EQ(info.perf_event.type, BPF_PERF_EVENT_KPROBE, "info.perf_event.type");
    ASSERT_EQ(info.perf_event.kprobe.missed, 1, "info.perf_event.kprobe.missed");
    cleanup:
    missed_kprobe__destroy(skel);
    }
#[no_mangle]
unsafe extern "C" fn get_missed_count(fd: c_int) -> __u64 {
    static __u64 get_missed_count(int fd)
    {
    let mut info: bpf_prog_info = {};
    let mut len: __u32 = sizeof(info);
    int err;
    err = bpf_prog_get_info_by_fd(fd, &info, &len);
    if (!ASSERT_OK(err, "bpf_prog_get_info_by_fd"))
    return (__u64) -1;
    return info.recursion_misses;
    }
//
// Putting kprobe.multi on bpf_fentry_test1 that calls bpf_kfunc_common_test
// kfunc which has 3 perf event kprobes and 1 kprobe.multi attached.
//
// Because fprobe (kprobe.multi attach layear) does not have strict recursion
// check the kprobe's bpf_prog_active check is hit for test2-5.
//
#[no_mangle]
unsafe extern "C" fn test_missed_kprobe_recursion() {
    static void test_missed_kprobe_recursion(void)
    {
    LIBBPF_OPTS(bpf_test_run_opts, topts);
    struct missed_kprobe_recursion *skel;
    int err, prog_fd;
    skel = missed_kprobe_recursion__open_and_load();
    if (!ASSERT_OK_PTR(skel, "missed_kprobe_recursion__open_and_load"))
    goto cleanup;
    err = missed_kprobe_recursion__attach(skel);
    if (!ASSERT_OK(err, "missed_kprobe_recursion__attach"))
    goto cleanup;
    prog_fd = bpf_program__fd(skel.progs.trigger);
    err = bpf_prog_test_run_opts(prog_fd, &topts);
    ASSERT_OK(err, "test_run");
    ASSERT_EQ(topts.retval, 0, "test_run");
    ASSERT_EQ(get_missed_count(bpf_program__fd(skel.progs.test1)), 0, "test1_recursion_misses");
    ASSERT_GE(get_missed_count(bpf_program__fd(skel.progs.test2)), 1, "test2_recursion_misses");
    ASSERT_GE(get_missed_count(bpf_program__fd(skel.progs.test3)), 1, "test3_recursion_misses");
    ASSERT_GE(get_missed_count(bpf_program__fd(skel.progs.test4)), 1, "test4_recursion_misses");
    ASSERT_GE(get_missed_count(bpf_program__fd(skel.progs.test5)), 1, "test5_recursion_misses");
    ASSERT_EQ(get_missed_count(bpf_program__fd(skel.progs.test6)), 1, "test6_recursion_misses");
    cleanup:
    missed_kprobe_recursion__destroy(skel);
    }
//
// Putting kprobe on bpf_fentry_test1 that calls bpf_printk and invokes
// bpf_trace_printk tracepoint. The bpf_trace_printk tracepoint has test[234]
// programs attached to it.
//
// Because kprobe execution goes through bpf_prog_active check, programs
// attached to the tracepoint will fail the recursion check and increment
// the recursion_misses stats.
//
#[no_mangle]
unsafe extern "C" fn test_missed_tp_recursion() {
    static void test_missed_tp_recursion(void)
    {
    LIBBPF_OPTS(bpf_test_run_opts, topts);
    struct missed_tp_recursion *skel;
    int err, prog_fd;
    skel = missed_tp_recursion__open_and_load();
    if (!ASSERT_OK_PTR(skel, "missed_tp_recursion__open_and_load"))
    goto cleanup;
    err = missed_tp_recursion__attach(skel);
    if (!ASSERT_OK(err, "missed_tp_recursion__attach"))
    goto cleanup;
    prog_fd = bpf_program__fd(skel.progs.trigger);
    err = bpf_prog_test_run_opts(prog_fd, &topts);
    ASSERT_OK(err, "test_run");
    ASSERT_EQ(topts.retval, 0, "test_run");
    ASSERT_EQ(get_missed_count(bpf_program__fd(skel.progs.test1)), 0, "test1_recursion_misses");
    ASSERT_EQ(get_missed_count(bpf_program__fd(skel.progs.test2)), 1, "test2_recursion_misses");
    ASSERT_EQ(get_missed_count(bpf_program__fd(skel.progs.test3)), 1, "test3_recursion_misses");
    ASSERT_EQ(get_missed_count(bpf_program__fd(skel.progs.test4)), 1, "test4_recursion_misses");
    cleanup:
    missed_tp_recursion__destroy(skel);
    }
#[no_mangle]
pub unsafe extern "C" fn test_missed() {
    void test_missed(void)
    {
    if (test__start_subtest("perf_kprobe"))
    test_missed_perf_kprobe();
    if (test__start_subtest("kprobe_recursion"))
    test_missed_kprobe_recursion();
    if (test__start_subtest("tp_recursion"))
    test_missed_tp_recursion();
    }
