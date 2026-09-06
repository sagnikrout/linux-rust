//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/test_profiler.c
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
// Copyright (c) 2020 Facebook

#[no_mangle]
unsafe extern "C" fn sanity_run(prog: *mut bpf_program) -> c_int {
    static int sanity_run(struct bpf_program *prog)
    {
    LIBBPF_OPTS(bpf_test_run_opts, test_attr);
    __u64 args[] = {1, 2, 3};
    int err, prog_fd;
    prog_fd = bpf_program__fd(prog);
    test_attr.ctx_in = args;
    test_attr.ctx_size_in = sizeof(args);
    err = bpf_prog_test_run_opts(prog_fd, &test_attr);
    if (!ASSERT_OK(err, "test_run"))
    return -1;
    if (!ASSERT_OK(test_attr.retval, "test_run retval"))
    return -1;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn test_test_profiler() {
    void test_test_profiler(void)
    {
    struct profiler1 *profiler1_skel = core::ptr::null_mut();
    struct profiler2 *profiler2_skel = core::ptr::null_mut();
    struct profiler3 *profiler3_skel = core::ptr::null_mut();
    let mut duration: __u32 = 0;
    int err;
    profiler1_skel = profiler1__open_and_load();
    if (CHECK(!profiler1_skel, "profiler1_skel_load", "profiler1 skeleton failed\n"))
    goto cleanup;
    err = profiler1__attach(profiler1_skel);
    if (CHECK(err, "profiler1_attach", "profiler1 attach failed: %d\n", err))
    goto cleanup;
    if (sanity_run(profiler1_skel.progs.raw_tracepoint__sched_process_exec))
    goto cleanup;
    profiler2_skel = profiler2__open_and_load();
    if (CHECK(!profiler2_skel, "profiler2_skel_load", "profiler2 skeleton failed\n"))
    goto cleanup;
    err = profiler2__attach(profiler2_skel);
    if (CHECK(err, "profiler2_attach", "profiler2 attach failed: %d\n", err))
    goto cleanup;
    if (sanity_run(profiler2_skel.progs.raw_tracepoint__sched_process_exec))
    goto cleanup;
    profiler3_skel = profiler3__open_and_load();
    if (CHECK(!profiler3_skel, "profiler3_skel_load", "profiler3 skeleton failed\n"))
    goto cleanup;
    err = profiler3__attach(profiler3_skel);
    if (CHECK(err, "profiler3_attach", "profiler3 attach failed: %d\n", err))
    goto cleanup;
    if (sanity_run(profiler3_skel.progs.raw_tracepoint__sched_process_exec))
    goto cleanup;
    cleanup:
    profiler1__destroy(profiler1_skel);
    profiler2__destroy(profiler2_skel);
    profiler3__destroy(profiler3_skel);
    }
