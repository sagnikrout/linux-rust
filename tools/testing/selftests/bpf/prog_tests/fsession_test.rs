//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/fsession_test.c
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
// Copyright (c) 2025 ChinaTelecom

#[no_mangle]
unsafe extern "C" fn check_result(skel: *mut fsession_test) -> c_int {
    static int check_result(struct fsession_test *skel)
    {
    LIBBPF_OPTS(bpf_test_run_opts, topts);
    int err, prog_fd;
// Trigger test function calls
    prog_fd = bpf_program__fd(skel.progs.test1);
    err = bpf_prog_test_run_opts(prog_fd, &topts);
    if (!ASSERT_OK(err, "test_run_opts err"))
    return err;
    if (!ASSERT_OK(topts.retval, "test_run_opts retval"))
    return topts.retval;
    for (int i = 0; i < sizeof(*skel.bss) / sizeof(__u64); i++) {
    if (!ASSERT_EQ(((__u64 *)skel.bss)[i], 1, "test_result"))
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn test_fsession_basic() {
    static void test_fsession_basic(void)
    {
    struct fsession_test *skel = core::ptr::null_mut();
    int err;
    skel = fsession_test__open();
    if (!ASSERT_OK_PTR(skel, "fsession_test__open"))
    return;
    err = fsession_test__load(skel);
    if (err == -EOPNOTSUPP) {
    test__skip();
    goto cleanup;
    }
    if (!ASSERT_OK(err, "fsession_test__load"))
    goto cleanup;
    err = fsession_test__attach(skel);
    if (!ASSERT_OK(err, "fsession_attach"))
    goto cleanup;
    check_result(skel);
    cleanup:
    fsession_test__destroy(skel);
    }
#[no_mangle]
unsafe extern "C" fn test_fsession_reattach() {
    static void test_fsession_reattach(void)
    {
    struct fsession_test *skel = core::ptr::null_mut();
    int err;
    skel = fsession_test__open();
    if (!ASSERT_OK_PTR(skel, "fsession_test__open"))
    return;
    err = fsession_test__load(skel);
    if (err == -EOPNOTSUPP) {
    test__skip();
    goto cleanup;
    }
    if (!ASSERT_OK(err, "fsession_test__load"))
    goto cleanup;
// first attach
    err = fsession_test__attach(skel);
    if (!ASSERT_OK(err, "fsession_first_attach"))
    goto cleanup;
    if (check_result(skel))
    goto cleanup;
// detach
    fsession_test__detach(skel);
// reset counters
    memset(skel.bss, 0, sizeof(*skel.bss));
// second attach
    err = fsession_test__attach(skel);
    if (!ASSERT_OK(err, "fsession_second_attach"))
    goto cleanup;
    if (check_result(skel))
    goto cleanup;
    cleanup:
    fsession_test__destroy(skel);
    }
#[no_mangle]
unsafe extern "C" fn test_fsession_cookie() {
    static void test_fsession_cookie(void)
    {
    struct fsession_test *skel = core::ptr::null_mut();
    int err;
    skel = fsession_test__open();
    if (!ASSERT_OK_PTR(skel, "fsession_test__open"))
    goto cleanup;
//
// The test_fsession_basic() will test the session cookie with
// bpf_get_func_ip() case, so we need only check
// the cookie without bpf_get_func_ip() case here
//
    bpf_program__set_autoload(skel.progs.test6, false);
    err = fsession_test__load(skel);
    if (err == -EOPNOTSUPP) {
    test__skip();
    goto cleanup;
    }
    if (!ASSERT_OK(err, "fsession_test__load"))
    goto cleanup;
    err = fsession_test__attach(skel);
    if (!ASSERT_OK(err, "fsession_attach"))
    goto cleanup;
    skel.bss.test6_entry_result = 1;
    skel.bss.test6_exit_result = 1;
    check_result(skel);
    cleanup:
    fsession_test__destroy(skel);
    }
#[no_mangle]
pub unsafe extern "C" fn test_fsession_test() {
    void test_fsession_test(void)
    {
    if (test__start_subtest("fsession_test"))
    test_fsession_basic();
    if (test__start_subtest("fsession_reattach"))
    test_fsession_reattach();
    if (test__start_subtest("fsession_cookie"))
    test_fsession_cookie();
    }
