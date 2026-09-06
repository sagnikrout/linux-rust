//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/powerpc/benchmarks/null_syscall.c
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
// Test null syscall performance
//
// Copyright (C) 2009-2015 Anton Blanchard, IBM
//
pub const NR_LOOPS: c_int = 10000000;

    static volatile int soak_done;
    unsigned long long clock_frequency;
    unsigned long long timebase_frequency;
    double timebase_multiplier;
#[no_mangle]
pub unsafe extern "C" fn mftb() -> c_ulong {
    static inline unsigned long mftb(void)
    {
    unsigned long low;
    asm volatile("mftb %0" : "=r" (low));
    return low;
    }
#[no_mangle]
unsafe extern "C" fn sigalrm_handler(unused: c_int) {
    static void sigalrm_handler(int unused)
    {
    soak_done = 1;
    }
//
// Use a timer instead of busy looping on clock_gettime() so we don't
// pollute profiles with glibc and VDSO hits.
//
#[no_mangle]
unsafe extern "C" fn cpu_soak_usecs(usecs: c_ulong) {
    static void cpu_soak_usecs(unsigned long usecs)
    {
    struct itimerval val;
    memset(&val, 0, sizeof(val));
    val.it_value.tv_usec = usecs;
    signal(SIGALRM, sigalrm_handler);
    setitimer(ITIMER_REAL, &val, core::ptr::null_mut());
    while (1) {
    if (soak_done)
    break;
    }
    signal(SIGALRM, SIG_DFL);
    }
//
// This only works with recent kernels where cpufreq modifies
// /proc/cpuinfo dynamically.
//
#[no_mangle]
unsafe extern "C" fn get_proc_frequency() {
    static void get_proc_frequency(void)
    {
    FILE *f;
    char line[128];
    char *p, *end;
    unsigned long v;
    double d;
    char *override;
// Try to get out of low power/low frequency mode
    cpu_soak_usecs(0.25 * 1000000);
    f = fopen("/proc/cpuinfo", "r");
    if (f == core::ptr::null_mut())
    return;
    timebase_frequency = 0;
    while (fgets(line, sizeof(line), f) != core::ptr::null_mut()) {
    if (strncmp(line, "timebase", 8) == 0) {
    p = strchr(line, ':');
    if (p != core::ptr::null_mut()) {
    v = strtoull(p + 1, &end, 0);
    if (end != p + 1)
    timebase_frequency = v;
    }
    }
    if (((strncmp(line, "clock", 5) == 0) ||
    (strncmp(line, "cpu MHz", 7) == 0))) {
    p = strchr(line, ':');
    if (p != core::ptr::null_mut()) {
    d = strtod(p + 1, &end);
    if (end != p + 1) {
// Find fastest clock frequency
    if ((d * 1000000ULL) > clock_frequency)
    clock_frequency = d * 1000000ULL;
    }
    }
    }
    }
    fclose(f);
    override = getenv("FREQUENCY");
    if (override)
    clock_frequency = strtoull(override, core::ptr::null_mut(), 10);
    if (timebase_frequency)
    timebase_multiplier = (double)clock_frequency
    / timebase_frequency;
    else
    timebase_multiplier = 1;
    }
#[no_mangle]
unsafe extern "C" fn do_null_syscall(nr: c_ulong) {
    static void do_null_syscall(unsigned long nr)
    {
    unsigned long i;
    for (i = 0; i < nr; i++)
    syscall(__NR_gettid);
    }

#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    unsigned long tb_start, tb_now;
    struct timespec tv_start, tv_now;
    unsigned long long elapsed_ns, elapsed_tb;
    get_proc_frequency();
    clock_gettime(CLOCK_MONOTONIC, &tv_start);
    tb_start = mftb();
    do_null_syscall(NR_LOOPS);
    clock_gettime(CLOCK_MONOTONIC, &tv_now);
    tb_now = mftb();
    elapsed_ns = (tv_now.tv_sec - tv_start.tv_sec) * 1000000000ULL +
    (tv_now.tv_nsec - tv_start.tv_nsec);
    elapsed_tb = tb_now - tb_start;
    printf("%10.2f ns %10.2f cycles\n", (float)elapsed_ns / NR_LOOPS,
    (float)elapsed_tb * timebase_multiplier / NR_LOOPS);
    return 0;
    }
