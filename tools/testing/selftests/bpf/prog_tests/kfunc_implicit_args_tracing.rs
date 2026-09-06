//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/kfunc_implicit_args_tracing.c
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
// Copyright (c) 2026 Meta Platforms, Inc. and affiliates.

#[no_mangle]
pub unsafe extern "C" fn test_kfunc_implicit_args_tracing() {
    void test_kfunc_implicit_args_tracing(void)
    {
    struct kfunc_implicit_args_tracing *skel;
    LIBBPF_OPTS(bpf_test_run_opts, topts);
    int err, fd;
    skel = kfunc_implicit_args_tracing__open_and_load();
    if (!ASSERT_OK_PTR(skel, "open_and_load"))
    return;
    err = kfunc_implicit_args_tracing__attach(skel);
    if (!ASSERT_OK(err, "attach"))
    goto cleanup;
    fd = bpf_program__fd(skel.progs.trigger_implicit_arg);
    err = bpf_prog_test_run_opts(fd, &topts);
    if (!ASSERT_OK(err, "test_run"))
    goto cleanup;
    ASSERT_EQ(topts.retval, 5, "kfunc_retval");
    ASSERT_EQ(skel.bss.fentry_arg_cnt, 2, "fentry_arg_cnt");
    ASSERT_NEQ(skel.bss.fentry_aux_arg, 0, "fentry_aux_arg");
    ASSERT_EQ(skel.bss.fentry_result, 1, "fentry_result");
    ASSERT_EQ(skel.bss.fexit_arg_cnt, 2, "fexit_arg_cnt");
    ASSERT_NEQ(skel.bss.fexit_aux_arg, 0, "fexit_aux_arg");
    ASSERT_EQ(skel.bss.fexit_result, 1, "fexit_result");
    cleanup:
    kfunc_implicit_args_tracing__destroy(skel);
    }
