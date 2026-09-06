//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/get_func_args_test.c
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
pub unsafe extern "C" fn test_get_func_args_test() {
    void test_get_func_args_test(void)
    {
    struct get_func_args_test *skel = core::ptr::null_mut();
    int err, prog_fd;
    LIBBPF_OPTS(bpf_test_run_opts, topts);
    skel = get_func_args_test__open_and_load();
    if (!ASSERT_OK_PTR(skel, "get_func_args_test__open_and_load"))
    return;
    err = get_func_args_test__attach(skel);
    if (!ASSERT_OK(err, "get_func_args_test__attach"))
    goto cleanup;
// This runs bpf_fentry_test* functions and triggers
// fentry/fexit programs.
//
    prog_fd = bpf_program__fd(skel.progs.test1);
    err = bpf_prog_test_run_opts(prog_fd, &topts);
    ASSERT_OK(err, "test_run");
    ASSERT_EQ(topts.retval, 0, "test_run");
// This runs bpf_modify_return_test function and triggers
// fmod_ret_test and fexit_test programs.
//
    prog_fd = bpf_program__fd(skel.progs.fmod_ret_test);
    err = bpf_prog_test_run_opts(prog_fd, &topts);
    ASSERT_OK(err, "test_run");
    ASSERT_EQ(topts.retval >> 16, 1, "test_run");
    ASSERT_EQ(topts.retval & 0xffff, 1234 + 29, "test_run");
    ASSERT_OK(trigger_module_test_read(1), "trigger_read");
    ASSERT_EQ(skel.bss.test1_result, 1, "test1_result");
    ASSERT_EQ(skel.bss.test2_result, 1, "test2_result");
    ASSERT_EQ(skel.bss.test3_result, 1, "test3_result");
    ASSERT_EQ(skel.bss.test4_result, 1, "test4_result");
    ASSERT_EQ(skel.bss.test5_result, 1, "test5_result");
    ASSERT_EQ(skel.bss.test6_result, 1, "test6_result");
    cleanup:
    get_func_args_test__destroy(skel);
    }
#[no_mangle]
pub unsafe extern "C" fn test_get_func_args_fsession_test() {
    void test_get_func_args_fsession_test(void)
    {
    struct get_func_args_fsession_test *skel = core::ptr::null_mut();
    int err;
    LIBBPF_OPTS(bpf_test_run_opts, topts);
    skel = get_func_args_fsession_test__open_and_load();
    if (!ASSERT_OK_PTR(skel, "get_func_args_fsession_test__open_and_load"))
    return;
    err = get_func_args_fsession_test__attach(skel);
    if (!ASSERT_OK(err, "get_func_args_fsession_test__attach"))
    goto cleanup;
    err = bpf_prog_test_run_opts(bpf_program__fd(skel.progs.test1), &topts);
    ASSERT_OK(err, "test_run");
    ASSERT_EQ(topts.retval, 0, "test_run");
    ASSERT_EQ(skel.bss.test1_result, 1, "test1_result");
    cleanup:
    get_func_args_fsession_test__destroy(skel);
    }
