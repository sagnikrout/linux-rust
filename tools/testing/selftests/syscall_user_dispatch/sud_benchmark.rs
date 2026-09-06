//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/syscall_user_dispatch/sud_benchmark.c
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
// Copyright (c) 2020 Collabora Ltd.
//
// Benchmark and test syscall user dispatch
//
// Macro flag: #define _GNU_SOURCE

//
// To test returning from a sigsys with selector blocked, the test
// requires some per-architecture support (i.e. knowledge about the
// signal trampoline address).  On i386, we know it is on the vdso, and
// a small trampoline is open-coded for x86_64.  Other architectures
// that have a trampoline in the vdso will support TEST_BLOCKED_RETURN
// out of the box, but don't enable them until they support syscall user
// dispatch.
//

// Macro flag: #define TEST_BLOCKED_RETURN

    void* (syscall_dispatcher_start)(void);
    void* (syscall_dispatcher_end)(void);

    let mut syscall_dispatcher_start: c_ulong = 0;
    let mut syscall_dispatcher_end: c_ulong = 0;

    let mut trapped_call_count: c_ulong = 0;
    let mut native_call_count: c_ulong = 0;
    char selector;

pub const CALIBRATION_STEP: c_int = 100000;
pub const CALIBRATE_TO_SECS: c_int = 5;
    int factor;
#[no_mangle]
unsafe extern "C" fn one_sysinfo_step() -> double {
    static double one_sysinfo_step(void)
    {
    struct timespec t1, t2;
    int i;
    struct sysinfo info;
    clock_gettime(CLOCK_MONOTONIC, &t1);
    for (i = 0; i < CALIBRATION_STEP; i++)
    sysinfo(&info);
    clock_gettime(CLOCK_MONOTONIC, &t2);
    return (t2.tv_sec - t1.tv_sec) + 1.0e-9 * (t2.tv_nsec - t1.tv_nsec);
    }
#[no_mangle]
unsafe extern "C" fn calibrate_set() {
    static void calibrate_set(void)
    {
    let mut elapsed: double = 0;
    printf("Calibrating test set to last ~%d seconds...\n", CALIBRATE_TO_SECS);
    while (elapsed < 1) {
    elapsed += one_sysinfo_step();
    factor += CALIBRATE_TO_SECS;
    }
    printf("test iterations = %d\n", CALIBRATION_STEP * factor);
    }
#[no_mangle]
unsafe extern "C" fn perf_syscall() -> double {
    static double perf_syscall(void)
    {
    unsigned int i;
    let mut partial: double = 0;
    for (i = 0; i < factor; ++i)
    partial += one_sysinfo_step()/(CALIBRATION_STEP*factor);
    return partial;
    }
#[no_mangle]
unsafe extern "C" fn handle_sigsys(sig: c_int, info: *mut siginfo_t, ucontext: *mut c_void) {
    static void handle_sigsys(int sig, siginfo_t *info, void *ucontext)
    {
    char buf[1024];
    int len;
    SYSCALL_UNBLOCK;
// printf and friends are not signal-safe.
    len = snprintf(buf, 1024, "Caught sys_%x\n", info.si_syscall);
    write(1, buf, len);
    if (info.si_syscall == MAGIC_SYSCALL_1)
    trapped_call_count++;
    else
    native_call_count++;

    SYSCALL_BLOCK;

    __asm__ volatile("movq $0xf, %rax");
    __asm__ volatile("leaveq");
    __asm__ volatile("add $0x8, %rsp");
    __asm__ volatile("syscall_dispatcher_start:");
    __asm__ volatile("syscall");
    __asm__ volatile("nop"); /* Landing pad within dispatcher area */
    __asm__ volatile("syscall_dispatcher_end:");

    }
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    struct sigaction act;
    double time1, time2;
    int ret;
    sigset_t mask;
    memset(&act, 0, sizeof(act));
    sigemptyset(&mask);
    act.sa_sigaction = handle_sigsys;
    act.sa_flags = SA_SIGINFO;
    act.sa_mask = mask;
    calibrate_set();
    time1 = perf_syscall();
    printf("Avg syscall time %.0lfns.\n", time1 * 1.0e9);
    ret = sigaction(SIGSYS, &act, core::ptr::null_mut());
    if (ret) {
    perror("Error sigaction:");
    exit(-1);
    }
    fprintf(stderr, "Enabling syscall trapping.\n");
    if (prctl(PR_SET_SYSCALL_USER_DISPATCH, PR_SYS_DISPATCH_ON,
    syscall_dispatcher_start,
    (syscall_dispatcher_end - syscall_dispatcher_start + 1),
    &selector)) {
    perror("prctl failed\n");
    exit(-1);
    }
    SYSCALL_BLOCK;
    syscall(MAGIC_SYSCALL_1);

    if (selector == SYSCALL_DISPATCH_FILTER_ALLOW) {
    fprintf(stderr, "Failed to return with selector blocked.\n");
    exit(-1);
    }

    SYSCALL_UNBLOCK;
    if (!trapped_call_count) {
    fprintf(stderr, "syscall trapping does not work.\n");
    exit(-1);
    }
    time2 = perf_syscall();
    if (native_call_count) {
    perror("syscall trapping intercepted more syscalls than expected\n");
    exit(-1);
    }
    printf("trapped_call_count %lu, native_call_count %lu.\n",
    trapped_call_count, native_call_count);
    printf("Avg syscall time %.0lfns.\n", time2 * 1.0e9);
    printf("Interception overhead: %.1lf%% (+%.0lfns).\n",
    100.0 * (time2 / time1 - 1.0), 1.0e9 * (time2 - time1));
    return 0;
    }
