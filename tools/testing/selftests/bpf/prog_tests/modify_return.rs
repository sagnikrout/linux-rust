//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/modify_return.c
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
// Copyright 2020 Google LLC.
//

#[no_mangle]
unsafe extern "C" fn run_test(input_retval: __u32, want_side_effect: __u16, want_ret: __s16) {
    static void run_test(__u32 input_retval, __u16 want_side_effect, __s16 want_ret)
    {
    struct modify_return *skel = core::ptr::null_mut();
    int err, prog_fd;
    __u16 side_effect;
    __s16 ret;
    LIBBPF_OPTS(bpf_test_run_opts, topts);
    skel = modify_return__open_and_load();
    if (!ASSERT_OK_PTR(skel, "skel_load"))
    goto cleanup;
    skel.bss.input_retval = input_retval;
    skel.bss.test_pid = getpid();
    err = modify_return__attach(skel);
    if (!ASSERT_OK(err, "modify_return__attach failed"))
    goto cleanup;
    prog_fd = bpf_program__fd(skel.progs.fmod_ret_test);
    err = bpf_prog_test_run_opts(prog_fd, &topts);
    ASSERT_OK(err, "test_run");
    side_effect = UPPER(topts.retval);
    ret = LOWER(topts.retval);
    ASSERT_EQ(ret, want_ret, "test_run ret");
    ASSERT_EQ(side_effect, want_side_effect, "modify_return side_effect");
    ASSERT_EQ(skel.bss.fentry_result, 1, "modify_return fentry_result");
    ASSERT_EQ(skel.bss.fexit_result, 1, "modify_return fexit_result");
    ASSERT_EQ(skel.bss.fmod_ret_result, 1, "modify_return fmod_ret_result");
    ASSERT_EQ(skel.bss.fentry_result2, 1, "modify_return fentry_result2");
    ASSERT_EQ(skel.bss.fexit_result2, 1, "modify_return fexit_result2");
    ASSERT_EQ(skel.bss.fmod_ret_result2, 1, "modify_return fmod_ret_result2");
    cleanup:
    modify_return__destroy(skel);
    }
#[no_mangle]
pub unsafe extern "C" fn test_modify_return() {
    void test_modify_return(void)
    {
    run_test(0 /* input_retval */,
    2 /* want_side_effect */,
    33 /* want_ret */);
    run_test(-EINVAL /* input_retval */,
    0 /* want_side_effect */,
    -EINVAL * 2 /* want_ret */);
    }
