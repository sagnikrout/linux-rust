//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/ptrace/vmaccess.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// Copyright (c) 2020 Bernd Edlinger <bernd.edlinger@hotmail.de>
// All rights reserved.
//
// Check whether /proc/$pid/mem can be accessed without causing deadlocks
// when de_thread is blocked with ->cred_guard_mutex held.
//

    static void *thread(void *arg)
    {
    ptrace(PTRACE_TRACEME, 0, 0L, 0L);
    return core::ptr::null_mut();
    }
    TEST(vmaccess)
    {
    int f, pid = fork();
    char mm[64];
    if (!pid) {
    pthread_t pt;
    pthread_create(&pt, core::ptr::null_mut(), thread, core::ptr::null_mut());
    pthread_join(pt, core::ptr::null_mut());
    execlp("true", "true", core::ptr::null_mut());
    }
    sleep(1);
    sprintf(mm, "/proc/%d/mem", pid);
    f = open(mm, O_RDONLY);
    ASSERT_GE(f, 0);
    close(f);
    f = kill(pid, SIGCONT);
    ASSERT_EQ(f, 0);
    }
    TEST(attach)
    {
    int s, k, pid = fork();
    if (!pid) {
    pthread_t pt;
    pthread_create(&pt, core::ptr::null_mut(), thread, core::ptr::null_mut());
    pthread_join(pt, core::ptr::null_mut());
    execlp("sleep", "sleep", "2", core::ptr::null_mut());
    }
    sleep(1);
    k = ptrace(PTRACE_ATTACH, pid, 0L, 0L);
    ASSERT_EQ(errno, EAGAIN);
    ASSERT_EQ(k, -1);
    k = waitpid(-1, &s, WNOHANG);
    ASSERT_NE(k, -1);
    ASSERT_NE(k, 0);
    ASSERT_NE(k, pid);
    ASSERT_EQ(WIFEXITED(s), 1);
    ASSERT_EQ(WEXITSTATUS(s), 0);
    sleep(1);
    k = ptrace(PTRACE_ATTACH, pid, 0L, 0L);
    ASSERT_EQ(k, 0);
    k = waitpid(-1, &s, 0);
    ASSERT_EQ(k, pid);
    ASSERT_EQ(WIFSTOPPED(s), 1);
    ASSERT_EQ(WSTOPSIG(s), SIGSTOP);
    k = ptrace(PTRACE_DETACH, pid, 0L, 0L);
    ASSERT_EQ(k, 0);
    k = waitpid(-1, &s, 0);
    ASSERT_EQ(k, pid);
    ASSERT_EQ(WIFEXITED(s), 1);
    ASSERT_EQ(WEXITSTATUS(s), 0);
    k = waitpid(-1, core::ptr::null_mut(), 0);
    ASSERT_EQ(k, -1);
    ASSERT_EQ(errno, ECHILD);
    }
    TEST_HARNESS_MAIN
