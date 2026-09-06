//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/fentry_test.c
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
// Copyright (c) 2019 Facebook

#[no_mangle]
unsafe extern "C" fn fentry_test_common(fentry_skel: *mut fentry_test_lskel) -> c_int {
    static int fentry_test_common(struct fentry_test_lskel *fentry_skel)
    {
    int err, prog_fd, i;
    int link_fd;
    __u64 *result;
    LIBBPF_OPTS(bpf_test_run_opts, topts);
    err = fentry_test_lskel__attach(fentry_skel);
    if (!ASSERT_OK(err, "fentry_attach"))
    return err;
// Check that already linked program can't be attached again.
    link_fd = fentry_test_lskel__test1__attach(fentry_skel);
    if (!ASSERT_LT(link_fd, 0, "fentry_attach_link"))
    return -1;
    prog_fd = fentry_skel.progs.test1.prog_fd;
    err = bpf_prog_test_run_opts(prog_fd, &topts);
    ASSERT_OK(err, "test_run");
    ASSERT_EQ(topts.retval, 0, "test_run");
    result = (__u64 *)fentry_skel.bss;
    for (i = 0; i < sizeof(*fentry_skel.bss) / sizeof(__u64); i++) {
    if (!ASSERT_EQ(result[i], 1, "fentry_result"))
    return -1;
    }
    fentry_test_lskel__detach(fentry_skel);
// zero results for re-attach test
    memset(fentry_skel.bss, 0, sizeof(*fentry_skel.bss));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn fentry_test() {
    static void fentry_test(void)
    {
    struct fentry_test_lskel *fentry_skel = core::ptr::null_mut();
    int err;
    fentry_skel = fentry_test_lskel__open();
    if (!ASSERT_OK_PTR(fentry_skel, "fentry_skel_open"))
    goto cleanup;
    fentry_skel.keyring_id	= KEY_SPEC_SESSION_KEYRING;
    err = fentry_test_lskel__load(fentry_skel);
    if (!ASSERT_OK(err, "fentry_skel_load"))
    goto cleanup;
    err = fentry_test_common(fentry_skel);
    if (!ASSERT_OK(err, "fentry_first_attach"))
    goto cleanup;
    err = fentry_test_common(fentry_skel);
    ASSERT_OK(err, "fentry_second_attach");
    cleanup:
    fentry_test_lskel__destroy(fentry_skel);
    }
#[no_mangle]
unsafe extern "C" fn fentry_many_args() {
    static void fentry_many_args(void)
    {
    struct fentry_many_args *fentry_skel = core::ptr::null_mut();
    int err;
    fentry_skel = fentry_many_args__open_and_load();
    if (!ASSERT_OK_PTR(fentry_skel, "fentry_many_args_skel_load"))
    goto cleanup;
    err = fentry_many_args__attach(fentry_skel);
    if (!ASSERT_OK(err, "fentry_many_args_attach"))
    goto cleanup;
    ASSERT_OK(trigger_module_test_read(1), "trigger_read");
    ASSERT_EQ(fentry_skel.bss.test1_result, 1,
    "fentry_many_args_result1");
    ASSERT_EQ(fentry_skel.bss.test2_result, 1,
    "fentry_many_args_result2");
    ASSERT_EQ(fentry_skel.bss.test3_result, 1,
    "fentry_many_args_result3");
    cleanup:
    fentry_many_args__destroy(fentry_skel);
    }
#[no_mangle]
pub unsafe extern "C" fn test_fentry_test() {
    void test_fentry_test(void)
    {
    if (test__start_subtest("fentry"))
    fentry_test();
    if (test__start_subtest("fentry_many_args"))
    fentry_many_args();
    }
