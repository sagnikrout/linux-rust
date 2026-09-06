//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/fexit_test.c
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
unsafe extern "C" fn fexit_test_common(fexit_skel: *mut fexit_test_lskel) -> c_int {
    static int fexit_test_common(struct fexit_test_lskel *fexit_skel)
    {
    int err, prog_fd, i;
    int link_fd;
    __u64 *result;
    LIBBPF_OPTS(bpf_test_run_opts, topts);
    err = fexit_test_lskel__attach(fexit_skel);
    if (!ASSERT_OK(err, "fexit_attach"))
    return err;
// Check that already linked program can't be attached again.
    link_fd = fexit_test_lskel__test1__attach(fexit_skel);
    if (!ASSERT_LT(link_fd, 0, "fexit_attach_link"))
    return -1;
    prog_fd = fexit_skel.progs.test1.prog_fd;
    err = bpf_prog_test_run_opts(prog_fd, &topts);
    ASSERT_OK(err, "test_run");
    ASSERT_EQ(topts.retval, 0, "test_run");
    result = (__u64 *)fexit_skel.bss;
    for (i = 0; i < sizeof(*fexit_skel.bss) / sizeof(__u64); i++) {
    if (!ASSERT_EQ(result[i], 1, "fexit_result"))
    return -1;
    }
    fexit_test_lskel__detach(fexit_skel);
// zero results for re-attach test
    memset(fexit_skel.bss, 0, sizeof(*fexit_skel.bss));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn fexit_test() {
    static void fexit_test(void)
    {
    struct fexit_test_lskel *fexit_skel = core::ptr::null_mut();
    int err;
    fexit_skel = fexit_test_lskel__open();
    if (!ASSERT_OK_PTR(fexit_skel, "fexit_skel_open"))
    goto cleanup;
    fexit_skel.keyring_id	= KEY_SPEC_SESSION_KEYRING;
    err = fexit_test_lskel__load(fexit_skel);
    if (!ASSERT_OK(err, "fexit_skel_load"))
    goto cleanup;
    err = fexit_test_common(fexit_skel);
    if (!ASSERT_OK(err, "fexit_first_attach"))
    goto cleanup;
    err = fexit_test_common(fexit_skel);
    ASSERT_OK(err, "fexit_second_attach");
    cleanup:
    fexit_test_lskel__destroy(fexit_skel);
    }
#[no_mangle]
unsafe extern "C" fn fexit_many_args() {
    static void fexit_many_args(void)
    {
    struct fexit_many_args *fexit_skel = core::ptr::null_mut();
    int err;
    fexit_skel = fexit_many_args__open_and_load();
    if (!ASSERT_OK_PTR(fexit_skel, "fexit_many_args_skel_load"))
    goto cleanup;
    err = fexit_many_args__attach(fexit_skel);
    if (!ASSERT_OK(err, "fexit_many_args_attach"))
    goto cleanup;
    ASSERT_OK(trigger_module_test_read(1), "trigger_read");
    ASSERT_EQ(fexit_skel.bss.test1_result, 1,
    "fexit_many_args_result1");
    ASSERT_EQ(fexit_skel.bss.test2_result, 1,
    "fexit_many_args_result2");
    ASSERT_EQ(fexit_skel.bss.test3_result, 1,
    "fexit_many_args_result3");
    cleanup:
    fexit_many_args__destroy(fexit_skel);
    }
#[no_mangle]
pub unsafe extern "C" fn test_fexit_test() {
    void test_fexit_test(void)
    {
    if (test__start_subtest("fexit"))
    fexit_test();
    if (test__start_subtest("fexit_many_args"))
    fexit_many_args();
    }
