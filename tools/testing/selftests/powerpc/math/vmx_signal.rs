//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/powerpc/math/vmx_signal.c
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
// This test attempts to see if the VMX registers are correctly reported in a
// signal context. Each worker just spins checking its VMX registers, at some
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
    __thread vector int varray[] = {{1, 2, 3, 4}, {5, 6, 7, 8}, {9, 10,11,12},
    {13,14,15,16},{17,18,19,20},{21,22,23,24},
    {25,26,27,28},{29,30,31,32},{33,34,35,36},
    {37,38,39,40},{41,42,43,44},{45,46,47,48}};
    bool bad_context;
    int running;
    int threads_starting;
    extern int preempt_vmx(vector int *varray, int *threads_starting, int *sentinal);
#[no_mangle]
pub unsafe extern "C" fn signal_vmx_sig(sig: c_int, info: *mut siginfo_t, context: *mut c_void) {
    void signal_vmx_sig(int sig, siginfo_t *info, void *context)
    {
    int i;
    ucontext_t *uc = context;
    mcontext_t *mc = &uc.uc_mcontext;
// Only the non volatiles were loaded up
    for (i = 20; i < 32; i++) {
    if (memcmp(mc.v_regs.vrregs[i], &varray[i - 20], 16)) {
    int j;
//
// Shouldn't printf() in a signal handler, however, this is a
// test and we've detected failure. Understanding what failed
// is paramount. All that happens after this is tests exit with
// failure.
//
    printf("VMX mismatch at reg %d!\n", i);
    printf("Reg | Actual                  | Expected\n");
    for (j = 20; j < 32; j++) {
    printf("%d  | 0x%04x%04x%04x%04x      | 0x%04x%04x%04x%04x\n", j, mc.v_regs.vrregs[j][0],
    mc.v_regs.vrregs[j][1], mc.v_regs.vrregs[j][2], mc.v_regs.vrregs[j][3],
    varray[j - 20][0], varray[j - 20][1], varray[j - 20][2], varray[j - 20][3]);
    }
    bad_context = true;
    break;
    }
    }
    }
    void *signal_vmx_c(void *p)
    {
    int i, j;
    long rc;
    struct sigaction act;
    act.sa_sigaction = signal_vmx_sig;
    act.sa_flags = SA_SIGINFO;
    rc = sigaction(SIGUSR1, &act, core::ptr::null_mut());
    if (rc)
    return p;
    srand(pthread_self());
    for (i = 0; i < 12; i++)
    for (j = 0; j < 4; j++)
    varray[i][j] = rand();
    rc = preempt_vmx(varray, &threads_starting, &running);
    return (void *) rc;
    }
#[no_mangle]
pub unsafe extern "C" fn test_signal_vmx() -> c_int {
    int test_signal_vmx(void)
    {
    int i, j, rc, threads;
    void *rc_p;
    pthread_t *tids;
// vcmpequd used in vmx_asm.S is v2.07
    SKIP_IF(!have_hwcap2(PPC_FEATURE2_ARCH_2_07));
    threads = sysconf(_SC_NPROCESSORS_ONLN) * THREAD_FACTOR;
    tids = malloc(threads * sizeof(pthread_t));
    FAIL_IF(!tids);
    running = true;
    threads_starting = threads;
    for (i = 0; i < threads; i++) {
    rc = pthread_create(&tids[i], core::ptr::null_mut(), signal_vmx_c, core::ptr::null_mut());
    FAIL_IF(rc);
    }
    setbuf(stdout, core::ptr::null_mut());
    printf("\tWaiting for %d workers to start... %d", threads, threads_starting);
    while (threads_starting) {
    asm volatile("": : :"memory");
    usleep(1000);
    printf(", %d", threads_starting);
    }
    printf(" ...done\n");
    printf("\tSending signals to all threads %d times...", ITERATIONS);
    for (i = 0; i < ITERATIONS; i++) {
    for (j = 0; j < threads; j++) {
    pthread_kill(tids[j], SIGUSR1);
    }
    sleep(1);
    }
    printf("done\n");
    printf("\tKilling workers...");
    running = 0;
    for (i = 0; i < threads; i++) {
    pthread_join(tids[i], &rc_p);
//
// Harness will say the fail was here, look at why signal_vmx
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
    test_harness_set_timeout(360);
    return test_harness(test_signal_vmx, "vmx_signal");
    }
