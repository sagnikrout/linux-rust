//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/syscall.c
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct args {
    pub log_buf: __u64,
    pub log_size: __u32,
    pub max_entries: c_int,
    pub map_fd: c_int,
    pub prog_fd: c_int,
    pub btf_fd: c_int,
}

#[no_mangle]
unsafe extern "C" fn test_syscall_load_prog() {
    static void test_syscall_load_prog(void)
    {
    static char verifier_log[8192];
    struct args ctx = {
    .max_entries = 1024,
    .log_buf = (uintptr_t) verifier_log,
    .log_size = sizeof(verifier_log),
    };
    LIBBPF_OPTS(bpf_test_run_opts, tattr,
    .ctx_in = &ctx,
    .ctx_size_in = sizeof(ctx),
    );
    struct syscall *skel = core::ptr::null_mut();
    let mut key: __u64 = 12, value = 0;
    int err, prog_fd;
    skel = syscall__open_and_load();
    if (!ASSERT_OK_PTR(skel, "skel_load"))
    goto cleanup;
    prog_fd = bpf_program__fd(skel.progs.load_prog);
    err = bpf_prog_test_run_opts(prog_fd, &tattr);
    ASSERT_EQ(err, 0, "err");
    ASSERT_EQ(tattr.retval, 1, "retval");
    ASSERT_GT(ctx.map_fd, 0, "ctx.map_fd");
    ASSERT_GT(ctx.prog_fd, 0, "ctx.prog_fd");
    ASSERT_OK(memcmp(verifier_log, "processed", sizeof("processed") - 1),
    "verifier_log");
    err = bpf_map_lookup_elem(ctx.map_fd, &key, &value);
    ASSERT_EQ(err, 0, "map_lookup");
    ASSERT_EQ(value, 34, "map lookup value");
    cleanup:
    syscall__destroy(skel);
    if (ctx.prog_fd > 0)
    close(ctx.prog_fd);
    if (ctx.map_fd > 0)
    close(ctx.map_fd);
    if (ctx.btf_fd > 0)
    close(ctx.btf_fd);
    }
#[no_mangle]
unsafe extern "C" fn test_syscall_update_outer_map() {
    static void test_syscall_update_outer_map(void)
    {
    LIBBPF_OPTS(bpf_test_run_opts, opts);
    struct syscall *skel;
    int err, prog_fd;
    skel = syscall__open_and_load();
    if (!ASSERT_OK_PTR(skel, "skel_load"))
    goto cleanup;
    prog_fd = bpf_program__fd(skel.progs.update_outer_map);
    err = bpf_prog_test_run_opts(prog_fd, &opts);
    ASSERT_EQ(err, 0, "err");
    ASSERT_EQ(opts.retval, 1, "retval");
    cleanup:
    syscall__destroy(skel);
    }
#[no_mangle]
pub unsafe extern "C" fn test_syscall() {
    void test_syscall(void)
    {
    if (test__start_subtest("load_prog"))
    test_syscall_load_prog();
    if (test__start_subtest("update_outer_map"))
    test_syscall_update_outer_map();
    }
