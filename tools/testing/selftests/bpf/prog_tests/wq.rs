//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/wq.c
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
// Copyright (c) 2024 Benjamin Tissoires

#[no_mangle]
pub unsafe extern "C" fn serial_test_wq() {
    void serial_test_wq(void)
    {
    struct wq *wq_skel = core::ptr::null_mut();
    int err, prog_fd;
    LIBBPF_OPTS(bpf_test_run_opts, topts);
    RUN_TESTS(wq);
// re-run the success test to check if the timer was actually executed
    wq_skel = wq__open_and_load();
    if (!ASSERT_OK_PTR(wq_skel, "wq__open_and_load"))
    return;
    err = wq__attach(wq_skel);
    if (!ASSERT_OK(err, "wq_attach"))
    goto clean_up;
    prog_fd = bpf_program__fd(wq_skel.progs.test_syscall_array_sleepable);
    err = bpf_prog_test_run_opts(prog_fd, &topts);
    ASSERT_OK(err, "test_run");
    ASSERT_EQ(topts.retval, 0, "test_run");
    usleep(50); /* 10 usecs should be enough, but give it extra */
    ASSERT_EQ(wq_skel.bss.ok_sleepable, (1 << 1), "ok_sleepable");
    clean_up:
    wq__destroy(wq_skel);
    }
#[no_mangle]
pub unsafe extern "C" fn serial_test_failures_wq() {
    void serial_test_failures_wq(void)
    {
    RUN_TESTS(wq_failures);
    }
#[no_mangle]
unsafe extern "C" fn test_failure_map_no_btf() {
    static void test_failure_map_no_btf(void)
    {
    struct wq *skel = core::ptr::null_mut();
    char log[8192];
    const struct bpf_insn *insns;
    size_t insn_cnt;
    int ret, err, map_fd;
    LIBBPF_OPTS(bpf_prog_load_opts, opts, .log_size = sizeof(log), .log_buf = log,
    .log_level = 2);
    skel = wq__open();
    if (!ASSERT_OK_PTR(skel, "skel_open"))
    return;
    err = bpf_object__prepare(skel.obj);
    if (!ASSERT_OK(err, "skel__prepare"))
    goto out;
    map_fd = bpf_map_create(BPF_MAP_TYPE_ARRAY, "map_no_btf", sizeof(__u32), sizeof(__u64), 100,
    core::ptr::null_mut());
    if (!ASSERT_GT(map_fd, -1, "map create"))
    goto out;
    err = bpf_map__reuse_fd(skel.maps.array, map_fd);
    if (!ASSERT_OK(err, "map reuse fd")) {
    close(map_fd);
    goto out;
    }
    insns = bpf_program__insns(skel.progs.test_map_no_btf);
    if (!ASSERT_OK_PTR(insns, "insns ptr"))
    goto out;
    insn_cnt = bpf_program__insn_cnt(skel.progs.test_map_no_btf);
    if (!ASSERT_GT(insn_cnt, 0u, "insn cnt"))
    goto out;
    ret = bpf_prog_load(BPF_PROG_TYPE_TRACEPOINT, core::ptr::null_mut(), "GPL", insns, insn_cnt, &opts);
    if (!ASSERT_LT(ret, 0, "prog load failed")) {
    if (ret > 0)
    close(ret);
    goto out;
    }
    ASSERT_HAS_SUBSTR(log, "map 'map_no_btf' has to have BTF in order to use bpf_wq",
    "log complains no map BTF");
    out:
    wq__destroy(skel);
    }
#[no_mangle]
pub unsafe extern "C" fn test_wq_custom() {
    void test_wq_custom(void)
    {
    if (test__start_subtest("test_failure_map_no_btf"))
    test_failure_map_no_btf();
    }
