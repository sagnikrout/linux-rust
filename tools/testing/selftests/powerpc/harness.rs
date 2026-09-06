//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/powerpc/harness.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright 2013, Michael Ellerman, IBM Corp.
//

pub const KILL_TIMEOUT: c_int = 5;
// Setting timeout to -1 disables the alarm
    let mut timeout: static uint64_t = 120;
#[no_mangle]
pub unsafe extern "C" fn run_test((test_function)(void): c_int, name: *const c_char) -> c_int {
    int run_test(int (test_function)(void), const char *name)
    {
    bool terminated;
    int rc, status;
    pid_t pid;
// Make sure output is flushed before forking
    fflush(stdout);
    pid = fork();
    if (pid == 0) {
    setpgid(0, 0);
    exit(test_function());
    } else if (pid == -1) {
    perror("fork");
    return 1;
    }
    setpgid(pid, pid);
    if (timeout != -1)
// Wake us up in timeout seconds
    alarm(timeout);
    terminated = false;
    wait:
    rc = waitpid(pid, &status, 0);
    if (rc == -1) {
    if (errno != EINTR) {
    printf("unknown error from waitpid\n");
    return 1;
    }
    if (terminated) {
    printf("!! force killing %s\n", name);
    kill(-pid, SIGKILL);
    return 1;
    } else {
    printf("!! killing %s\n", name);
    kill(-pid, SIGTERM);
    terminated = true;
    alarm(KILL_TIMEOUT);
    goto wait;
    }
    }
// Kill anything else in the process group that is still running
    kill(-pid, SIGTERM);
    if (WIFEXITED(status))
    status = WEXITSTATUS(status);
    else {
    if (WIFSIGNALED(status))
    printf("!! child died by signal %d\n", WTERMSIG(status));
    else
    printf("!! child died by unknown cause\n");
    status = 1; /* Signal or other */
    }
    return status;
    }
#[no_mangle]
unsafe extern "C" fn sig_handler(signum: c_int) {
    static void sig_handler(int signum)
    {
// Just wake us up from waitpid
    }
    static struct sigaction sig_action = {
    .sa_handler = sig_handler,
    };
#[no_mangle]
pub unsafe extern "C" fn test_harness_set_timeout(time: u64) {
    void test_harness_set_timeout(uint64_t time)
    {
    timeout = time;
    }
#[no_mangle]
pub unsafe extern "C" fn test_harness((test_function)(void): c_int, name: *const c_char) -> c_int {
    int test_harness(int (test_function)(void), const char *name)
    {
    int rc;
    test_start(name);
    test_set_git_version(GIT_VERSION);
    if (sigaction(SIGINT, &sig_action, core::ptr::null_mut())) {
    perror("sigaction (sigint)");
    test_error(name);
    return 1;
    }
    if (sigaction(SIGALRM, &sig_action, core::ptr::null_mut())) {
    perror("sigaction (sigalrm)");
    test_error(name);
    return 1;
    }
    rc = run_test(test_function, name);
    if (rc == MAGIC_SKIP_RETURN_VALUE) {
    test_skip(name);
// so that skipped test is not marked as failed
    rc = 0;
    } else
    test_finish(name, rc);
    return rc;
    }
