//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/timens/timerfd.c
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
unsafe extern "C" fn tclock_gettime(clockid: clockid_t, now: *mut timespec) -> c_int {
    static int tclock_gettime(clockid_t clockid, struct timespec *now)
    {
    if (clockid == CLOCK_BOOTTIME_ALARM)
    clockid = CLOCK_BOOTTIME;
    return clock_gettime(clockid, now);
    }
#[no_mangle]
unsafe extern "C" fn run_test(clockid: c_int, now: timespec) -> c_int {
    static int run_test(int clockid, struct timespec now)
    {
    struct itimerspec new_value;
    long long elapsed;
    int fd, i;
    if (check_skip(clockid))
    return 0;
    if (tclock_gettime(clockid, &now))
    return pr_perror("clock_gettime(%d)", clockid);
    for (i = 0; i < 2; i++) {
    let mut flags: c_int = 0;
    new_value.it_value.tv_sec = 3600;
    new_value.it_value.tv_nsec = 0;
    new_value.it_interval.tv_sec = 1;
    new_value.it_interval.tv_nsec = 0;
    if (i == 1) {
    new_value.it_value.tv_sec += now.tv_sec;
    new_value.it_value.tv_nsec += now.tv_nsec;
    }
    fd = timerfd_create(clockid, 0);
    if (fd == -1)
    return pr_perror("timerfd_create(%d)", clockid);
    if (i == 1)
    flags |= TFD_TIMER_ABSTIME;
    if (timerfd_settime(fd, flags, &new_value, core::ptr::null_mut()))
    return pr_perror("timerfd_settime(%d)", clockid);
    if (timerfd_gettime(fd, &new_value))
    return pr_perror("timerfd_gettime(%d)", clockid);
    elapsed = new_value.it_value.tv_sec;
    if (llabs(elapsed - 3600) > 60) {
    ksft_test_result_fail("clockid: %d elapsed: %lld\n",
    clockid, elapsed);
    return 1;
    }
    close(fd);
    }
    ksft_test_result_pass("clockid=%d\n", clockid);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv[]: *mut c_char) -> c_int {
    int main(int argc, char *argv[])
    {
    int ret, status, len, fd;
    char buf[4096];
    pid_t pid;
    struct timespec btime_now, mtime_now;
    ksft_print_header();
    nscheck();
    check_supported_timers();
    ksft_set_plan(3);
    clock_gettime(CLOCK_MONOTONIC, &mtime_now);
    clock_gettime(CLOCK_BOOTTIME, &btime_now);
    if (unshare_timens())
    return 1;
    len = snprintf(buf, sizeof(buf), "%d %d 0\n%d %d 0",
    CLOCK_MONOTONIC, 70 * 24 * 3600,
    CLOCK_BOOTTIME, 9 * 24 * 3600);
    fd = open("/proc/self/timens_offsets", O_WRONLY);
    if (fd < 0)
    return pr_perror("/proc/self/timens_offsets");
    if (write(fd, buf, len) != len)
    return pr_perror("/proc/self/timens_offsets");
    close(fd);
    mtime_now.tv_sec += 70 * 24 * 3600;
    btime_now.tv_sec += 9 * 24 * 3600;
    pid = fork();
    if (pid < 0)
    return pr_perror("Unable to fork");
    if (pid == 0) {
    ret = 0;
    ret |= run_test(CLOCK_BOOTTIME, btime_now);
    ret |= run_test(CLOCK_MONOTONIC, mtime_now);
    ret |= run_test(CLOCK_BOOTTIME_ALARM, btime_now);
    if (ret)
    ksft_exit_fail();
    ksft_exit_pass();
    return ret;
    }
    if (waitpid(pid, &status, 0) != pid)
    return pr_perror("Unable to wait the child process");
    if (WIFEXITED(status))
    return WEXITSTATUS(status);
    return 1;
    }
