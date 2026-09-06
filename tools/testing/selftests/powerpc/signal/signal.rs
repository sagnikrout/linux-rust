//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/powerpc/signal/signal.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright 2016, Cyril Bur, IBM Corp.
//
// Sending one self a signal should always get delivered.
//

pub const MAX_ATTEMPT: c_int = 500000;
pub const TIMEOUT: c_int = 5;
    extern long signal_self(pid_t pid, int sig);
    static sig_atomic_t signaled;
    static sig_atomic_t fail;
#[no_mangle]
unsafe extern "C" fn signal_handler(sig: c_int) {
    static void signal_handler(int sig)
    {
    if (sig == SIGUSR1)
    signaled = 1;
    else
    fail = 1;
    }
#[no_mangle]
unsafe extern "C" fn test_signal() -> c_int {
    static int test_signal()
    {
    int i;
    struct sigaction act;
    let mut ppid: pid_t = getpid();
    pid_t pid;
    act.sa_handler = signal_handler;
    act.sa_flags = 0;
    sigemptyset(&act.sa_mask);
    if (sigaction(SIGUSR1, &act, core::ptr::null_mut()) < 0) {
    perror("sigaction SIGUSR1");
    exit(1);
    }
    if (sigaction(SIGALRM, &act, core::ptr::null_mut()) < 0) {
    perror("sigaction SIGALRM");
    exit(1);
    }
// Don't do this for MAX_ATTEMPT, its simply too long
    for(i  = 0; i < 1000; i++) {
    pid = fork();
    if (pid == -1) {
    perror("fork");
    exit(1);
    }
    if (pid == 0) {
    signal_self(ppid, SIGUSR1);
    exit(1);
    } else {
    alarm(0); /* Disable any pending */
    alarm(2);
    while (!signaled && !fail)
    asm volatile("": : :"memory");
    if (!signaled) {
    fprintf(stderr, "Didn't get signal from child\n");
    FAIL_IF(1); /* For the line number */
    }
// Otherwise we'll loop too fast and fork() will eventually fail
    waitpid(pid, core::ptr::null_mut(), 0);
    }
    }
    for (i = 0; i < MAX_ATTEMPT; i++) {
    long rc;
    alarm(0); /* Disable any pending */
    signaled = 0;
    alarm(TIMEOUT);
    rc = signal_self(ppid, SIGUSR1);
    if (rc) {
    fprintf(stderr, "(%d) Fail reason: %d rc=0x%lx",
    i, fail, rc);
    FAIL_IF(1); /* For the line number */
    }
    while (!signaled && !fail)
    asm volatile("": : :"memory");
    if (!signaled) {
    fprintf(stderr, "(%d) Fail reason: %d rc=0x%lx",
    i, fail, rc);
    FAIL_IF(1); /* For the line number */
    }
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    test_harness_set_timeout(300);
    return test_harness(test_signal, "signal");
    }
