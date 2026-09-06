//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/timens/clock_nanosleep.c
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

#[no_mangle]
pub unsafe extern "C" fn test_sig(sig: c_int) {
    void test_sig(int sig)
    {
    if (sig == SIGUSR2)
    pthread_exit(core::ptr::null_mut());
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct thread_args {
    pub rem: *mut *mut timespec now,,
    pub lock: *mut pthread_mutex_t,
    pub clockid: c_int,
    pub abs: c_int,
}

    void *call_nanosleep(void *_args)
    {
    struct thread_args *args = _args;
    clock_nanosleep(args.clockid, args.abs ? TIMER_ABSTIME : 0, args.now, args.rem);
    pthread_mutex_unlock(args.lock);
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn run_test(clockid: c_int, abs: c_int) -> c_int {
    static int run_test(int clockid, int abs)
    {
    let mut now: timespec = {}, rem;
    let mut args: thread_args = { .now = &now, .rem = &rem, .clockid = clockid};
    struct timespec start;
    pthread_mutex_t lock;
    pthread_t thread;
    int j, ok, ret;
    signal(SIGUSR1, test_sig);
    signal(SIGUSR2, test_sig);
    pthread_mutex_init(&lock, core::ptr::null_mut());
    pthread_mutex_lock(&lock);
    if (clock_gettime(clockid, &start) == -1) {
    if (errno == EINVAL && check_skip(clockid))
    return 0;
    return pr_perror("clock_gettime");
    }
    if (abs) {
    now.tv_sec = start.tv_sec;
    now.tv_nsec = start.tv_nsec;
    }
    now.tv_sec += 3600;
    args.abs = abs;
    args.lock = &lock;
    ret = pthread_create(&thread, core::ptr::null_mut(), call_nanosleep, &args);
    if (ret != 0) {
    pr_err("Unable to create a thread: %s", strerror(ret));
    return 1;
    }
// Wait when the thread will call clock_nanosleep().
    ok = 0;
    for (j = 0; j < 8; j++) {
// The maximum timeout is about 5 seconds.
    usleep(10000 << j);
// Try to interrupt clock_nanosleep().
    pthread_kill(thread, SIGUSR1);
    usleep(10000 << j);
// Check whether clock_nanosleep() has been interrupted or not.
    if (pthread_mutex_trylock(&lock) == 0) {
//
    ok = 1;
    break;
    }
    }
    if (!ok)
    pthread_kill(thread, SIGUSR2);
    pthread_join(thread, core::ptr::null_mut());
    pthread_mutex_destroy(&lock);
    if (!ok) {
    ksft_test_result_pass("clockid: %d abs:%d timeout\n", clockid, abs);
    return 1;
    }
    if (rem.tv_sec < 3300 || rem.tv_sec > 3900) {
    pr_fail("clockid: %d abs: %d remain: %ld\n",
    clockid, abs, rem.tv_sec);
    return 1;
    }
    ksft_test_result_pass("clockid: %d abs:%d\n", clockid, abs);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv[]: *mut c_char) -> c_int {
    int main(int argc, char *argv[])
    {
    int ret, nsfd;
    ksft_print_header();
    nscheck();
    ksft_set_plan(4);
    check_supported_timers();
    if (unshare_timens())
    return 1;
    if (_settime(CLOCK_MONOTONIC, 7 * 24 * 3600))
    return 1;
    if (_settime(CLOCK_BOOTTIME, 9 * 24 * 3600))
    return 1;
    nsfd = open("/proc/self/ns/time_for_children", O_RDONLY);
    if (nsfd < 0)
    return pr_perror("Unable to open timens_for_children");
    if (setns(nsfd, CLONE_NEWTIME))
    return pr_perror("Unable to set timens");
    ret = 0;
    ret |= run_test(CLOCK_MONOTONIC, 0);
    ret |= run_test(CLOCK_MONOTONIC, 1);
    ret |= run_test(CLOCK_BOOTTIME_ALARM, 0);
    ret |= run_test(CLOCK_BOOTTIME_ALARM, 1);
    if (ret)
    ksft_exit_fail();
    ksft_exit_pass();
    return ret;
    }
