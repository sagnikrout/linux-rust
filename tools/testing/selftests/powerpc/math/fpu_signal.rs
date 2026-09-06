//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/powerpc/math/fpu_signal.c
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
// Copyright 2015, Cyril Bur, IBM Corp.
//
// This test attempts to see if the FPU registers are correctly reported in a
// signal context. Each worker just spins checking its FPU registers, at some
// point a signal will interrupt it and C code will check the signal context
// ensuring it is also the same.
//

// Number of times each thread should receive the signal
pub const ITERATIONS: c_int = 10;
//
// Factor by which to multiply number of online CPUs for total number of
// worker threads
//
pub const THREAD_FACTOR: c_int = 8;
    __thread double darray[32];
    bool bad_context;
    int threads_starting;
    int running;
    extern long preempt_fpu(double *darray, int *threads_starting, int *running);
#[no_mangle]
pub unsafe extern "C" fn signal_fpu_sig(sig: c_int, info: *mut siginfo_t, context: *mut c_void) {
    void signal_fpu_sig(int sig, siginfo_t *info, void *context)
    {
    int i;
    ucontext_t *uc = context;
    mcontext_t *mc = &uc.uc_mcontext;
// Don't check f30/f31, they're used as scratches in check_all_fprs()
    for (i = 0; i < 30; i++) {
    if (mc.fp_regs[i] != darray[i]) {
    bad_context = true;
    break;
    }
    }
    }
    void *signal_fpu_c(void *p)
    {
    long rc;
    struct sigaction act;
    act.sa_sigaction = signal_fpu_sig;
    act.sa_flags = SA_SIGINFO;
    rc = sigaction(SIGUSR1, &act, core::ptr::null_mut());
    if (rc)
    return p;
    srand(pthread_self());
    randomise_darray(darray, ARRAY_SIZE(darray));
    rc = preempt_fpu(darray, &threads_starting, &running);
    return (void *) rc;
    }
#[no_mangle]
pub unsafe extern "C" fn test_signal_fpu() -> c_int {
    int test_signal_fpu(void)
    {
    int i, j, rc, threads;
    void *rc_p;
    pthread_t *tids;
    threads = sysconf(_SC_NPROCESSORS_ONLN) * THREAD_FACTOR;
    tids = malloc(threads * sizeof(pthread_t));
    FAIL_IF(!tids);
    running = true;
    threads_starting = threads;
    for (i = 0; i < threads; i++) {
    rc = pthread_create(&tids[i], core::ptr::null_mut(), signal_fpu_c, core::ptr::null_mut());
    FAIL_IF(rc);
    }
    setbuf(stdout, core::ptr::null_mut());
    printf("\tWaiting for all workers to start...");
    while (threads_starting)
    asm volatile("": : :"memory");
    printf("done\n");
    printf("\tSending signals to all threads %d times...", ITERATIONS);
    for (i = 0; i < ITERATIONS; i++) {
    for (j = 0; j < threads; j++) {
    pthread_kill(tids[j], SIGUSR1);
    }
    sleep(1);
    }
    printf("done\n");
    printf("\tStopping workers...");
    running = 0;
    for (i = 0; i < threads; i++) {
    pthread_join(tids[i], &rc_p);
//
// Harness will say the fail was here, look at why signal_fpu
// returned
//
    if ((long) rc_p || bad_context)
    printf("oops\n");
    if (bad_context)
    fprintf(stderr, "\t!! bad_context is true\n");
    FAIL_IF((long) rc_p || bad_context);
    }
    printf("done\n");
    free(tids);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv[]: *mut c_char) -> c_int {
    int main(int argc, char *argv[])
    {
    return test_harness(test_signal_fpu, "fpu_signal");
    }
