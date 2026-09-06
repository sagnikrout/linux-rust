//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/powerpc/math/vmx_preempt.c
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
// This test attempts to see if the VMX registers change across preemption.
// Two things should be noted here a) The check_vmx function in asm only checks
// the non volatile registers as it is reused from the syscall test b) There is
// no way to be sure preemption happened so this test just uses many threads
// and a long wait. As such, a successful test doesn't mean much but a failure
// is bad.
//

// Time to wait for workers to get preempted (seconds)
pub const PREEMPT_TIME: c_int = 20;
//
// Factor by which to multiply number of online CPUs for total number of
// worker threads
//
pub const THREAD_FACTOR: c_int = 8;
    __thread vector int varray[] = {{1, 2, 3, 4}, {5, 6, 7, 8}, {9, 10,11,12},
    {13,14,15,16},{17,18,19,20},{21,22,23,24},
    {25,26,27,28},{29,30,31,32},{33,34,35,36},
    {37,38,39,40},{41,42,43,44},{45,46,47,48}};
    int threads_starting;
    int running;
    extern int preempt_vmx(vector int *varray, int *threads_starting, int *running);
    void *preempt_vmx_c(void *p)
    {
    int i, j;
    long rc;
    srand(pthread_self());
    for (i = 0; i < 12; i++)
    for (j = 0; j < 4; j++)
    varray[i][j] = rand();
    rc = preempt_vmx(varray, &threads_starting, &running);
    return (void *)rc;
    }
#[no_mangle]
pub unsafe extern "C" fn test_preempt_vmx() -> c_int {
    int test_preempt_vmx(void)
    {
    int i, rc, threads;
    pthread_t *tids;
// vcmpequd used in vmx_asm.S is v2.07
    SKIP_IF(!have_hwcap2(PPC_FEATURE2_ARCH_2_07));
    threads = sysconf(_SC_NPROCESSORS_ONLN) * THREAD_FACTOR;
    tids = malloc(threads * sizeof(pthread_t));
    FAIL_IF(!tids);
    running = true;
    threads_starting = threads;
    for (i = 0; i < threads; i++) {
    rc = pthread_create(&tids[i], core::ptr::null_mut(), preempt_vmx_c, core::ptr::null_mut());
    FAIL_IF(rc);
    }
    setbuf(stdout, core::ptr::null_mut());
// Not really nessesary but nice to wait for every thread to start
    printf("\tWaiting for all workers to start...");
    while(threads_starting)
    asm volatile("": : :"memory");
    printf("done\n");
    printf("\tWaiting for %d seconds to let some workers get preempted...", PREEMPT_TIME);
    sleep(PREEMPT_TIME);
    printf("done\n");
    printf("\tStopping workers...");
//
// Working are checking this value every loop. In preempt_vmx 'cmpwi r5,0; bne 2b'.
// r5 will have loaded the value of running.
//
    running = 0;
    for (i = 0; i < threads; i++) {
    void *rc_p;
    pthread_join(tids[i], &rc_p);
//
// Harness will say the fail was here, look at why preempt_vmx
// returned
//
    if ((long) rc_p)
    printf("oops\n");
    FAIL_IF((long) rc_p);
    }
    printf("done\n");
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv[]: *mut c_char) -> c_int {
    int main(int argc, char *argv[])
    {
    return test_harness(test_preempt_vmx, "vmx_preempt");
    }
