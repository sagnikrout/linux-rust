//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/stream.c
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
pub unsafe extern "C" fn test_stream_failure() {
    void test_stream_failure(void)
    {
    RUN_TESTS(stream_fail);
    }
#[no_mangle]
pub unsafe extern "C" fn test_stream_success() {
    void test_stream_success(void)
    {
    RUN_TESTS(stream);
    return;
    }
#[no_mangle]
pub unsafe extern "C" fn test_stream_syscall() {
    void test_stream_syscall(void)
    {
    LIBBPF_OPTS(bpf_test_run_opts, opts);
    LIBBPF_OPTS(bpf_prog_stream_read_opts, ropts);
    struct stream *skel;
    int ret, prog_fd;
    char buf[64];
    skel = stream__open_and_load();
    if (!ASSERT_OK_PTR(skel, "stream__open_and_load"))
    return;
    prog_fd = bpf_program__fd(skel.progs.stream_syscall);
    ret = bpf_prog_test_run_opts(prog_fd, &opts);
    ASSERT_OK(ret, "ret");
    ASSERT_OK(opts.retval, "retval");
    ASSERT_LT(bpf_prog_stream_read(0, BPF_STREAM_STDOUT, buf, sizeof(buf), &ropts), 0, "error");
    ret = -errno;
    ASSERT_EQ(ret, -EINVAL, "bad prog_fd");
    ASSERT_LT(bpf_prog_stream_read(prog_fd, 0, buf, sizeof(buf), &ropts), 0, "error");
    ret = -errno;
    ASSERT_EQ(ret, -ENOENT, "bad stream id");
    ASSERT_LT(bpf_prog_stream_read(prog_fd, BPF_STREAM_STDOUT, core::ptr::null_mut(), sizeof(buf), core::ptr::null_mut()), 0, "error");
    ret = -errno;
    ASSERT_EQ(ret, -EFAULT, "bad stream buf");
    ret = bpf_prog_stream_read(prog_fd, BPF_STREAM_STDOUT, buf, 2, core::ptr::null_mut());
    ASSERT_EQ(ret, 2, "bytes");
    ret = bpf_prog_stream_read(prog_fd, BPF_STREAM_STDOUT, buf, 2, core::ptr::null_mut());
    ASSERT_EQ(ret, 1, "bytes");
    ret = bpf_prog_stream_read(prog_fd, BPF_STREAM_STDOUT, buf, 1, &ropts);
    ASSERT_EQ(ret, 0, "no bytes stdout");
    ret = bpf_prog_stream_read(prog_fd, BPF_STREAM_STDERR, buf, 1, &ropts);
    ASSERT_EQ(ret, 0, "no bytes stderr");
    stream__destroy(skel);
    }
#[no_mangle]
unsafe extern "C" fn test_address(prog: *mut bpf_program, fault_addr_p: *mut c_ulong) {
    static void test_address(struct bpf_program *prog, unsigned long *fault_addr_p)
    {
    LIBBPF_OPTS(bpf_test_run_opts, opts);
    LIBBPF_OPTS(bpf_prog_stream_read_opts, ropts);
    int ret, prog_fd;
    char fault_addr[64];
    char buf[1024];
    prog_fd = bpf_program__fd(prog);
    ret = bpf_prog_test_run_opts(prog_fd, &opts);
    ASSERT_OK(ret, "ret");
    ASSERT_OK(opts.retval, "retval");
    sprintf(fault_addr, "0x%lx", *fault_addr_p);
    ret = bpf_prog_stream_read(prog_fd, BPF_STREAM_STDERR, buf, sizeof(buf), &ropts);
    ASSERT_GT(ret, 0, "stream read");
    ASSERT_LE(ret, 1023, "len for buf");
    buf[ret] = '\0';
    if (!ASSERT_HAS_SUBSTR(buf, fault_addr, "fault_addr")) {
    fprintf(stderr, "Output from stream:\n%s\n", buf);
    fprintf(stderr, "Fault Addr: %s\n", fault_addr);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn test_stream_arena_fault_address() {
    void test_stream_arena_fault_address(void)
    {
    struct stream *skel;

    printf("%s:SKIP: arena fault reporting not supported\n", __func__);
    test__skip();
    return;

    skel = stream__open_and_load();
    if (!ASSERT_OK_PTR(skel, "stream__open_and_load"))
    return;
    if (test__start_subtest("read_fault"))
    test_address(skel.progs.stream_arena_read_fault, &skel.bss.fault_addr);
    if (test__start_subtest("write_fault"))
    test_address(skel.progs.stream_arena_write_fault, &skel.bss.fault_addr);
    if (test__start_subtest("load_acquire_fault"))
    test_address(skel.progs.stream_arena_load_acquire_fault, &skel.bss.fault_addr);
    if (test__start_subtest("xchg_fault"))
    test_address(skel.progs.stream_arena_xchg_fault, &skel.bss.fault_addr);
    if (test__start_subtest("cmpxchg_fault"))
    test_address(skel.progs.stream_arena_cmpxchg_fault, &skel.bss.fault_addr);
    stream__destroy(skel);
    }
