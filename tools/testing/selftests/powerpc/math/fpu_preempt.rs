//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/powerpc/math/fpu_preempt.c
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
// Copyright 2023, Michael Ellerman, IBM Corp.
//
// This test attempts to see if the FPU registers change across preemption.
// There is no way to be sure preemption happened so this test just uses many
// threads and a long wait. As such, a successful test doesn't mean much but
// a failure is bad.
//

// Time to wait for workers to get preempted (seconds)
pub const PREEMPT_TIME: c_int = 60;
//
// Factor by which to multiply number of online CPUs for total number of
// worker threads
//
pub const THREAD_FACTOR: c_int = 8;
    __thread double darray[32];
    int threads_starting;
    int running;
    extern int preempt_fpu(double *darray, int *threads_starting, int *running);
    void *preempt_fpu_c(void *p)
    {
    long rc;
    srand(pthread_self());
    randomise_darray(darray, ARRAY_SIZE(darray));
    rc = preempt_fpu(darray, &threads_starting, &running);
    return (void *)rc;
    }
#[no_mangle]
pub unsafe extern "C" fn test_preempt_fpu() -> c_int {
    int test_preempt_fpu(void)
    {
    int i, rc, threads;
    pthread_t *tids;
    threads = sysconf(_SC_NPROCESSORS_ONLN) * THREAD_FACTOR;
    tids = malloc((threads) * sizeof(pthread_t));
    FAIL_IF(!tids);
    running = true;
    threads_starting = threads;
    for (i = 0; i < threads; i++) {
    rc = pthread_create(&tids[i], core::ptr::null_mut(), preempt_fpu_c, core::ptr::null_mut());
    FAIL_IF(rc);
    }
    setbuf(stdout, core::ptr::null_mut());
// Not really necessary but nice to wait for every thread to start
    printf("\tWaiting for all workers to start...");
    while(threads_starting)
    asm volatile("": : :"memory");
    printf("done\n");
    printf("\tWaiting for %d seconds to let some workers get preempted...", PREEMPT_TIME);
    sleep(PREEMPT_TIME);
    printf("done\n");
    printf("\tStopping workers...");
//
// Working are checking this value every loop. In preempt_fpu 'cmpwi r5,0; bne 2b'.
// r5 will have loaded the value of running.
//
    running = 0;
    for (i = 0; i < threads; i++) {
    void *rc_p;
    pthread_join(tids[i], &rc_p);
//
// Harness will say the fail was here, look at why preempt_fpu
// returned
//
    if ((long) rc_p)
    printf("oops\n");
    FAIL_IF((long) rc_p);
    }
    printf("done\n");
    free(tids);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv[]: *mut c_char) -> c_int {
    int main(int argc, char *argv[])
    {
    return test_harness(test_preempt_fpu, "fpu_preempt");
    }
