//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/powerpc/signal/sigfuz.c
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
//
// Copyright 2018, Breno Leitao, IBM Corp.
// Licensed under GPLv2.
//
// Sigfuz(tm): A PowerPC TM-aware signal fuzzer.
//
// This is a new selftest that raises SIGUSR1 signals and handles it in a set
// of different ways, trying to create different scenario for testing
// purpose.
//
// This test works raising a signal and calling sigreturn interleaved with
// TM operations, as starting, suspending and terminating a transaction. The
// test depends on random numbers, and, based on them, it sets different TM
// states.
//
// Other than that, the test fills out the user context struct that is passed
// to the sigreturn system call with random data, in order to make sure that
// the signal handler syscall can handle different and invalid states
// properly.
//
// This selftest has command line parameters to control what kind of tests the
// user wants to run, as for example, if a transaction should be started prior
// to signal being raised, or, after the signal being raised and before the
// sigreturn. If no parameter is given, the default is enabling all options.
//
// This test does not check if the user context is being read and set
// properly by the kernel. Its purpose, at this time, is basically
// guaranteeing that the kernel does not crash on invalid scenarios.
//

// Selftest defaults

// Arguments options
pub const ARG_MESS_WITH_TM_AT: c_uint = 0x1;
pub const ARG_MESS_WITH_TM_BEFORE: c_uint = 0x2;
pub const ARG_MESS_WITH_MSR_AT: c_uint = 0x4;
pub const ARG_FOREVER: c_uint = 0x10;

    ARG_MESS_WITH_TM_BEFORE |	\
    ARG_MESS_WITH_MSR_AT)
    static int args;
    let mut nthread: static int = THREADS;
    let mut count_max: static int = COUNT_MAX;
// checkpoint context
    static ucontext_t *tmp_uc;
// Return true with 1/x probability
#[no_mangle]
unsafe extern "C" fn one_in_chance(x: c_int) -> c_int {
    static int one_in_chance(int x)
    {
    return rand() % x == 0;
    }
// Change TM states
#[no_mangle]
unsafe extern "C" fn mess_with_tm() {
    static void mess_with_tm(void)
    {
// Starts a transaction 33% of the time
    if (one_in_chance(3)) {
    asm ("tbegin.	;"
    "beq 8	;");
// And suspended half of them
    if (one_in_chance(2))
    asm("tsuspend.	;");
    }
// Call 'tend' in 5% of the runs
    if (one_in_chance(20))
    asm("tend.	;");
    }
// Signal handler that will be invoked with raise()
#[no_mangle]
unsafe extern "C" fn trap_signal_handler(signo: c_int, si: *mut siginfo_t, uc: *mut c_void) {
    static void trap_signal_handler(int signo, siginfo_t *si, void *uc)
    {
    ucontext_t *ucp = uc;
    ucp.uc_link = tmp_uc;
//
// Set uc_link in three possible ways:
// - Setting a single 'int' in the whole chunk
// - Cloning ucp into uc_link
// - Allocating a new memory chunk
//
    if (one_in_chance(3)) {
    memset(ucp.uc_link, rand(), sizeof(ucontext_t));
    } else if (one_in_chance(2)) {
    memcpy(ucp.uc_link, uc, sizeof(ucontext_t));
    } else if (one_in_chance(2)) {
    if (tmp_uc) {
    free(tmp_uc);
    tmp_uc = core::ptr::null_mut();
    }
    tmp_uc = malloc(sizeof(ucontext_t));
    ucp.uc_link = tmp_uc;
// Trying to cause a major page fault at Kernel level
    madvise(ucp.uc_link, sizeof(ucontext_t), MADV_DONTNEED);
    }
    if (args & ARG_MESS_WITH_MSR_AT) {
// Changing the checkpointed registers
    if (one_in_chance(4)) {
    ucp.uc_link.uc_mcontext.gp_regs[PT_MSR] |= MSR_TS_S;
    } else {
    if (one_in_chance(2)) {
    ucp.uc_link.uc_mcontext.gp_regs[PT_MSR] |=
    MSR_TS_T;
    } else if (one_in_chance(2)) {
    ucp.uc_link.uc_mcontext.gp_regs[PT_MSR] |=
    MSR_TS_T | MSR_TS_S;
    }
    }
// Checking the current register context
    if (one_in_chance(2)) {
    ucp.uc_mcontext.gp_regs[PT_MSR] |= MSR_TS_S;
    } else if (one_in_chance(2)) {
    if (one_in_chance(2))
    ucp.uc_mcontext.gp_regs[PT_MSR] |=
    MSR_TS_T;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: one_in_chance(2)) -> else {
    else if (one_in_chance(2))
    ucp.uc_mcontext.gp_regs[PT_MSR] |=
    MSR_TS_T | MSR_TS_S;
    }
    }
    if (one_in_chance(20)) {
// Nested transaction start
    if (one_in_chance(5))
    mess_with_tm();
// Return without changing any other context info
    return;
    }
    if (one_in_chance(10))
    ucp.uc_mcontext.gp_regs[PT_MSR] = random();
    if (one_in_chance(10))
    ucp.uc_mcontext.gp_regs[PT_NIP] = random();
    if (one_in_chance(10))
    ucp.uc_link.uc_mcontext.gp_regs[PT_MSR] = random();
    if (one_in_chance(10))
    ucp.uc_link.uc_mcontext.gp_regs[PT_NIP] = random();
    ucp.uc_mcontext.gp_regs[PT_TRAP] = random();
    ucp.uc_mcontext.gp_regs[PT_DSISR] = random();
    ucp.uc_mcontext.gp_regs[PT_DAR] = random();
    ucp.uc_mcontext.gp_regs[PT_ORIG_R3] = random();
    ucp.uc_mcontext.gp_regs[PT_XER] = random();
    ucp.uc_mcontext.gp_regs[PT_RESULT] = random();
    ucp.uc_mcontext.gp_regs[PT_SOFTE] = random();
    ucp.uc_mcontext.gp_regs[PT_DSCR] = random();
    ucp.uc_mcontext.gp_regs[PT_CTR] = random();
    ucp.uc_mcontext.gp_regs[PT_LNK] = random();
    ucp.uc_mcontext.gp_regs[PT_CCR] = random();
    ucp.uc_mcontext.gp_regs[PT_REGS_COUNT] = random();
    ucp.uc_link.uc_mcontext.gp_regs[PT_TRAP] = random();
    ucp.uc_link.uc_mcontext.gp_regs[PT_DSISR] = random();
    ucp.uc_link.uc_mcontext.gp_regs[PT_DAR] = random();
    ucp.uc_link.uc_mcontext.gp_regs[PT_ORIG_R3] = random();
    ucp.uc_link.uc_mcontext.gp_regs[PT_XER] = random();
    ucp.uc_link.uc_mcontext.gp_regs[PT_RESULT] = random();
    ucp.uc_link.uc_mcontext.gp_regs[PT_SOFTE] = random();
    ucp.uc_link.uc_mcontext.gp_regs[PT_DSCR] = random();
    ucp.uc_link.uc_mcontext.gp_regs[PT_CTR] = random();
    ucp.uc_link.uc_mcontext.gp_regs[PT_LNK] = random();
    ucp.uc_link.uc_mcontext.gp_regs[PT_CCR] = random();
    ucp.uc_link.uc_mcontext.gp_regs[PT_REGS_COUNT] = random();
    if (args & ARG_MESS_WITH_TM_BEFORE) {
    if (one_in_chance(2))
    mess_with_tm();
    }
    }
#[no_mangle]
unsafe extern "C" fn seg_signal_handler(signo: c_int, si: *mut siginfo_t, uc: *mut c_void) {
    static void seg_signal_handler(int signo, siginfo_t *si, void *uc)
    {
// Clear exit for process that segfaults
    exit(0);
    }
    static void *sigfuz_test(void *thrid)
    {
    struct sigaction trap_sa, seg_sa;
    int ret, i = 0;
    pid_t t;
    tmp_uc = malloc(sizeof(ucontext_t));
// Main signal handler
    trap_sa.sa_flags = SA_SIGINFO;
    trap_sa.sa_sigaction = trap_signal_handler;
// SIGSEGV signal handler
    seg_sa.sa_flags = SA_SIGINFO;
    seg_sa.sa_sigaction = seg_signal_handler;
// The signal handler will enable MSR_TS
    sigaction(SIGUSR1, &trap_sa, core::ptr::null_mut());
// If it does not crash, it will segfault, avoid it to retest
    sigaction(SIGSEGV, &seg_sa, core::ptr::null_mut());
    while (i < count_max) {
    t = fork();
    if (t == 0) {
// Once seed per process
    srand(time(core::ptr::null_mut()) + getpid());
    if (args & ARG_MESS_WITH_TM_AT) {
    if (one_in_chance(2))
    mess_with_tm();
    }
    raise(SIGUSR1);
    exit(0);
    } else {
    waitpid(t, &ret, 0);
    }
    if (!(args & ARG_FOREVER))
    i++;
    }
// If not freed already, free now
    if (tmp_uc) {
    free(tmp_uc);
    tmp_uc = core::ptr::null_mut();
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn signal_fuzzer() -> c_int {
    static int signal_fuzzer(void)
    {
    int t, rc;
    pthread_t *threads;
    threads = malloc(nthread * sizeof(pthread_t));
    for (t = 0; t < nthread; t++) {
    rc = pthread_create(&threads[t], core::ptr::null_mut(), sigfuz_test,
    (void *)&t);
    if (rc)
    perror("Thread creation error\n");
    }
    for (t = 0; t < nthread; t++) {
    rc = pthread_join(threads[t], core::ptr::null_mut());
    if (rc)
    perror("Thread join error\n");
    }
    free(threads);
    return EXIT_SUCCESS;
    }
#[no_mangle]
unsafe extern "C" fn show_help(name: *mut c_char) {
    static void show_help(char *name)
    {
    printf("%s: Sigfuzzer for powerpc\n", name);
    printf("Usage:\n");
    printf("\t-b\t Mess with TM before raising a SIGUSR1 signal\n");
    printf("\t-a\t Mess with TM after raising a SIGUSR1 signal\n");
    printf("\t-m\t Mess with MSR[TS] bits at mcontext\n");
    printf("\t-x\t Mess with everything above\n");
    printf("\t-f\t Run forever (Press ^C to Quit)\n");
    printf("\t-i\t Amount of interactions.	(Default = %d)\n", COUNT_MAX);
    printf("\t-t\t Amount of threads.	(Default = %d)\n", THREADS);
    exit(-1);
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> c_int {
    int main(int argc, char **argv)
    {
    int opt;
    while ((opt = getopt(argc, argv, "bamxt:fi:h")) != -1) {
    if (opt == 'b') {
    printf("Mess with TM before signal\n");
    args |= ARG_MESS_WITH_TM_BEFORE;
    } else if (opt == 'a') {
    printf("Mess with TM at signal handler\n");
    args |= ARG_MESS_WITH_TM_AT;
    } else if (opt == 'm') {
    printf("Mess with MSR[TS] bits in mcontext\n");
    args |= ARG_MESS_WITH_MSR_AT;
    } else if (opt == 'x') {
    printf("Running with all options enabled\n");
    args |= ARG_COMPLETE;
    } else if (opt == 't') {
    nthread = atoi(optarg);
    printf("Threads = %d\n", nthread);
    } else if (opt == 'f') {
    args |= ARG_FOREVER;
    printf("Press ^C to stop\n");
    test_harness_set_timeout(-1);
    } else if (opt == 'i') {
    count_max = atoi(optarg);
    printf("Running for %d interactions\n", count_max);
    } else if (opt == 'h') {
    show_help(argv[0]);
    }
    }
// Default test suite
    if (!args)
    args = ARG_COMPLETE;
    return test_harness(signal_fuzzer, "signal_fuzzer");
    }
