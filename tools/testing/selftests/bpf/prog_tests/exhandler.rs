//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/exhandler.c
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
// Copyright (c) 2021, Oracle and/or its affiliates.

// Test that verifies exception handling is working. fork()
// triggers task_newtask tracepoint; that new task will have a
// NULL pointer task_works, and the associated task->task_works->func
// should not be NULL if task_works itself is non-NULL.
//
// So to verify exception handling we want to see a NULL task_works
// and task_works->func; if we see this we can conclude that the
// exception handler ran when we attempted to dereference task->task_works
// and zeroed the destination register.
//

#[no_mangle]
pub unsafe extern "C" fn test_exhandler() {
    void test_exhandler(void)
    {
    let mut err: c_int = 0, duration = 0, status;
    struct exhandler_kern *skel;
    pid_t cpid;
    skel = exhandler_kern__open_and_load();
    if (CHECK(!skel, "skel_load", "skeleton failed: %d\n", err))
    goto cleanup;
    skel.bss.test_pid = getpid();
    err = exhandler_kern__attach(skel);
    if (!ASSERT_OK(err, "attach"))
    goto cleanup;
    cpid = fork();
    if (!ASSERT_GT(cpid, -1, "fork failed"))
    goto cleanup;
    if (cpid == 0)
    _exit(0);
    waitpid(cpid, &status, 0);
    ASSERT_NEQ(skel.bss.exception_triggered, 0, "verify exceptions occurred");
    cleanup:
    exhandler_kern__destroy(skel);
    }
