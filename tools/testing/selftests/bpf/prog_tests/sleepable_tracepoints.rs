//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/sleepable_tracepoints.c
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
// Copyright (c) 2025 Meta Platforms, Inc. and affiliates.

#[no_mangle]
unsafe extern "C" fn run_test(skel: *mut test_sleepable_tracepoints) {
    static void run_test(struct test_sleepable_tracepoints *skel)
    {
    char buf[PATH_MAX] = "/";
    skel.bss.target_pid = getpid();
    skel.bss.prog_triggered = 0;
    skel.bss.err = 0;
    skel.bss.copied_byte = 0;
    syscall(__NR_getcwd, buf, sizeof(buf));
    ASSERT_EQ(skel.bss.prog_triggered, 1, "prog_triggered");
    ASSERT_EQ(skel.bss.err, 0, "err");
    ASSERT_EQ(skel.bss.copied_byte, '/', "copied_byte");
    }
    static void run_auto_attach_test(struct bpf_program *prog,
    struct test_sleepable_tracepoints *skel)
    {
    struct bpf_link *link;
    link = bpf_program__attach(prog);
    if (!ASSERT_OK_PTR(link, "prog_attach"))
    return;
    run_test(skel);
    bpf_link__destroy(link);
    }
#[no_mangle]
unsafe extern "C" fn test_attach_only(prog: *mut bpf_program) {
    static void test_attach_only(struct bpf_program *prog)
    {
    struct bpf_link *link;
    link = bpf_program__attach(prog);
    if (ASSERT_OK_PTR(link, "attach"))
    bpf_link__destroy(link);
    }
#[no_mangle]
unsafe extern "C" fn test_attach_reject(prog: *mut bpf_program) {
    static void test_attach_reject(struct bpf_program *prog)
    {
    struct bpf_link *link;
    link = bpf_program__attach(prog);
    if (!ASSERT_ERR_PTR(link, "attach_should_fail"))
    bpf_link__destroy(link);
    }
#[no_mangle]
unsafe extern "C" fn test_raw_tp_bare(skel: *mut test_sleepable_tracepoints) {
    static void test_raw_tp_bare(struct test_sleepable_tracepoints *skel)
    {
    struct bpf_link *link;
    link = bpf_program__attach_raw_tracepoint(skel.progs.handle_raw_tp_bare,
    "sys_enter");
    if (ASSERT_OK_PTR(link, "attach"))
    bpf_link__destroy(link);
    }
#[no_mangle]
unsafe extern "C" fn test_tp_bare(skel: *mut test_sleepable_tracepoints) {
    static void test_tp_bare(struct test_sleepable_tracepoints *skel)
    {
    struct bpf_link *link;
    link = bpf_program__attach_tracepoint(skel.progs.handle_tp_bare,
    "syscalls", "sys_enter_getcwd");
    if (ASSERT_OK_PTR(link, "attach"))
    bpf_link__destroy(link);
    }
#[no_mangle]
unsafe extern "C" fn test_test_run(skel: *mut test_sleepable_tracepoints) {
    static void test_test_run(struct test_sleepable_tracepoints *skel)
    {
    __u64 args[2] = {0x1234ULL, 0x5678ULL};
    LIBBPF_OPTS(bpf_test_run_opts, topts,
    .ctx_in = args,
    .ctx_size_in = sizeof(args),
    );
    int fd, err;
    fd = bpf_program__fd(skel.progs.handle_test_run);
    err = bpf_prog_test_run_opts(fd, &topts);
    ASSERT_OK(err, "test_run");
    ASSERT_EQ(topts.retval, args[0] + args[1], "test_run_retval");
    }
#[no_mangle]
unsafe extern "C" fn test_test_run_on_cpu_reject(skel: *mut test_sleepable_tracepoints) {
    static void test_test_run_on_cpu_reject(struct test_sleepable_tracepoints *skel)
    {
    __u64 args[2] = {};
    LIBBPF_OPTS(bpf_test_run_opts, topts,
    .ctx_in = args,
    .ctx_size_in = sizeof(args),
    .flags = BPF_F_TEST_RUN_ON_CPU,
    );
    int fd, err;
    fd = bpf_program__fd(skel.progs.handle_test_run);
    err = bpf_prog_test_run_opts(fd, &topts);
    ASSERT_ERR(err, "test_run_on_cpu_reject");
    }
#[no_mangle]
pub unsafe extern "C" fn test_sleepable_tracepoints() {
    void test_sleepable_tracepoints(void)
    {
    struct test_sleepable_tracepoints *skel;
    skel = test_sleepable_tracepoints__open_and_load();
    if (!ASSERT_OK_PTR(skel, "open_and_load"))
    return;
    if (test__start_subtest("tp_btf"))
    run_auto_attach_test(skel.progs.handle_sys_enter_tp_btf, skel);
    if (test__start_subtest("raw_tp"))
    run_auto_attach_test(skel.progs.handle_sys_enter_raw_tp, skel);
    if (test__start_subtest("tracepoint"))
    run_auto_attach_test(skel.progs.handle_sys_enter_tp, skel);
    if (test__start_subtest("sys_exit"))
    run_auto_attach_test(skel.progs.handle_sys_exit_tp, skel);
    if (test__start_subtest("tracepoint_alias"))
    test_attach_only(skel.progs.handle_sys_enter_tp_alias);
    if (test__start_subtest("raw_tracepoint_alias"))
    test_attach_only(skel.progs.handle_sys_enter_raw_tp_alias);
    if (test__start_subtest("raw_tp_bare"))
    test_raw_tp_bare(skel);
    if (test__start_subtest("tp_bare"))
    test_tp_bare(skel);
    if (test__start_subtest("test_run"))
    test_test_run(skel);
    if (test__start_subtest("test_run_on_cpu_reject"))
    test_test_run_on_cpu_reject(skel);
    if (test__start_subtest("raw_tp_non_faultable"))
    test_attach_reject(skel.progs.handle_raw_tp_non_faultable);
    if (test__start_subtest("tp_non_syscall"))
    test_attach_reject(skel.progs.handle_tp_non_syscall);
    if (test__start_subtest("tp_btf_non_faultable_reject"))
    RUN_TESTS(test_sleepable_tracepoints_fail);
    test_sleepable_tracepoints__destroy(skel);
    }
