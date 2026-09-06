//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/timens/gettime_perf.c
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
// Macro flag: #define _GNU_SOURCE

    typedef int (*vgettime_t)(clockid_t, struct timespec *);
    vgettime_t vdso_clock_gettime;
#[no_mangle]
unsafe extern "C" fn fill_function_pointers() {
    static void fill_function_pointers(void)
    {
    void *vdso = dlopen("linux-vdso.so.1",
    RTLD_LAZY | RTLD_LOCAL | RTLD_NOLOAD);
    if (!vdso)
    vdso = dlopen("linux-gate.so.1",
    RTLD_LAZY | RTLD_LOCAL | RTLD_NOLOAD);
    if (!vdso)
    vdso = dlopen("linux-vdso32.so.1",
    RTLD_LAZY | RTLD_LOCAL | RTLD_NOLOAD);
    if (!vdso)
    vdso = dlopen("linux-vdso64.so.1",
    RTLD_LAZY | RTLD_LOCAL | RTLD_NOLOAD);
    if (!vdso) {
    pr_err("[WARN]\tfailed to find vDSO\n");
    return;
    }
    vdso_clock_gettime = (vgettime_t)dlsym(vdso, "__vdso_clock_gettime");
    if (!vdso_clock_gettime)
    vdso_clock_gettime = (vgettime_t)dlsym(vdso, "__kernel_clock_gettime");
    if (!vdso_clock_gettime)
    pr_err("Warning: failed to find clock_gettime in vDSO\n");
    }
#[no_mangle]
unsafe extern "C" fn test(clockid: clock_t, clockstr: *mut c_char, in_ns: bool) {
    static void test(clock_t clockid, char *clockstr, bool in_ns)
    {
    struct timespec tp, start;
    let mut i: c_long = 0;
    let mut timeout: c_int = 3;
    vdso_clock_gettime(clockid, &start);
    tp = start;
    for (tp = start; start.tv_sec + timeout > tp.tv_sec ||
    (start.tv_sec + timeout == tp.tv_sec &&
    start.tv_nsec > tp.tv_nsec); i++) {
    vdso_clock_gettime(clockid, &tp);
    }
    ksft_test_result_pass("%s:\tclock: %10s\tcycles:\t%10ld\n",
    in_ns ? "ns" : "host", clockstr, i);
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv[]: *mut c_char) -> c_int {
    int main(int argc, char *argv[])
    {
    let mut offset: time_t = 10;
    int nsfd;
    ksft_print_header();
    ksft_set_plan(8);
    fill_function_pointers();
    test(CLOCK_MONOTONIC, "monotonic", false);
    test(CLOCK_MONOTONIC_COARSE, "monotonic-coarse", false);
    test(CLOCK_MONOTONIC_RAW, "monotonic-raw", false);
    test(CLOCK_BOOTTIME, "boottime", false);
    nscheck();
    if (unshare_timens())
    return 1;
    nsfd = open("/proc/self/ns/time_for_children", O_RDONLY);
    if (nsfd < 0)
    return pr_perror("Can't open a time namespace");
    if (_settime(CLOCK_MONOTONIC, offset))
    return 1;
    if (_settime(CLOCK_BOOTTIME, offset))
    return 1;
    if (setns(nsfd, CLONE_NEWTIME))
    return pr_perror("setns");
    test(CLOCK_MONOTONIC, "monotonic", true);
    test(CLOCK_MONOTONIC_COARSE, "monotonic-coarse", true);
    test(CLOCK_MONOTONIC_RAW, "monotonic-raw", true);
    test(CLOCK_BOOTTIME, "boottime", true);
    ksft_exit_pass();
    return 0;
    }
