//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/powerpc/signal/signal_tm.c
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
pub const TIMEOUT: c_int = 10;
    extern long tm_signal_self(pid_t pid, int sig, long *ret);
    static sig_atomic_t signaled;
    static sig_atomic_t fail;
#[no_mangle]
unsafe extern "C" fn signal_handler(sig: c_int) {
    static void signal_handler(int sig)
    {
    if (tcheck_active()) {
    fail = 2;
    return;
    }
    if (sig == SIGUSR1)
    signaled = 1;
    else
    fail = 1;
    }
#[no_mangle]
unsafe extern "C" fn test_signal_tm() -> c_int {
    static int test_signal_tm()
    {
    int i;
    struct sigaction act;
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
    SKIP_IF(!have_htm());
    SKIP_IF(htm_is_synthetic());
    for (i = 0; i < MAX_ATTEMPT; i++) {
//
// If anything bad happens in ASM and we fail to set ret
// because *handwave* TM this will cause failure
//
    let mut ret: c_long = 0xdead;
    let mut rc: c_long = 0xbeef;
    alarm(0); /* Disable any pending */
    signaled = 0;
    alarm(TIMEOUT);
    FAIL_IF(tcheck_transactional());
    rc = tm_signal_self(getpid(), SIGUSR1, &ret);
    if (ret == 0xdead)
//
// This basically means the transaction aborted before we
// even got to the suspend... this is crazy but it
// happens.
// Yes this also means we might never make forward
// progress... the alarm() will trip eventually...
//
    continue;
    if (rc || ret) {
// Ret is actually an errno
    printf("TEXASR 0x%016lx, TFIAR 0x%016lx\n",
    __builtin_get_texasr(), __builtin_get_tfiar());
    fprintf(stderr, "(%d) Fail reason: %d rc=0x%lx ret=0x%lx\n",
    i, fail, rc, ret);
    FAIL_IF(ret);
    }
    while(!signaled && !fail)
    asm volatile("": : :"memory");
    if (!signaled) {
    fprintf(stderr, "(%d) Fail reason: %d rc=0x%lx ret=0x%lx\n",
    i, fail, rc, ret);
    FAIL_IF(fail); /* For the line number */
    }
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    return test_harness(test_signal_tm, "signal_tm");
    }
