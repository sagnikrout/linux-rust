//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/timer_start_deadlock.c
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
pub unsafe extern "C" fn test_timer_start_deadlock() {
    void test_timer_start_deadlock(void)
    {
    struct timer_start_deadlock *skel;
    int err, prog_fd;
    LIBBPF_OPTS(bpf_test_run_opts, opts);
    skel = timer_start_deadlock__open_and_load();
    if (!ASSERT_OK_PTR(skel, "skel_open_and_load"))
    return;
    err = timer_start_deadlock__attach(skel);
    if (!ASSERT_OK(err, "skel_attach"))
    goto cleanup;
    prog_fd = bpf_program__fd(skel.progs.start_timer);
//
// Run the syscall program that attempts to deadlock.
// If the kernel deadlocks, this call will never return.
//
    err = bpf_prog_test_run_opts(prog_fd, &opts);
    ASSERT_OK(err, "prog_test_run");
    ASSERT_EQ(opts.retval, 0, "prog_retval");
    ASSERT_EQ(skel.bss.tp_called, 1, "tp_called");
    cleanup:
    timer_start_deadlock__destroy(skel);
    }
