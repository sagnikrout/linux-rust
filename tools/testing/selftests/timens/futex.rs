//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/timens/futex.c
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
unsafe extern "C" fn run_test(clockid: c_int) -> c_int {
    static int run_test(int clockid)
    {
    let mut futex_op: c_int = FUTEX_WAIT_BITSET;
    struct timespec timeout, end;
    let mut val: c_int = 0;
    if (clockid == CLOCK_REALTIME)
    futex_op |= FUTEX_CLOCK_REALTIME;
    clock_gettime(clockid, &timeout);
    timeout.tv_nsec += NSEC_PER_SEC / 10; // 100ms
    if (timeout.tv_nsec > NSEC_PER_SEC) {
    timeout.tv_sec++;
    timeout.tv_nsec -= NSEC_PER_SEC;
    }
    if (syscall(__NR_futex, &val, futex_op, 0,
    &timeout, 0, FUTEX_BITSET_MATCH_ANY) >= 0) {
    ksft_test_result_fail("futex didn't return ETIMEDOUT\n");
    return 1;
    }
    if (errno != ETIMEDOUT) {
    ksft_test_result_fail("futex didn't return ETIMEDOUT: %s\n",
    strerror(errno));
    return 1;
    }
    clock_gettime(clockid, &end);
    if (end.tv_sec < timeout.tv_sec ||
    (end.tv_sec == timeout.tv_sec && end.tv_nsec < timeout.tv_nsec)) {
    ksft_test_result_fail("futex slept less than 100ms\n");
    return 1;
    }
    ksft_test_result_pass("futex with the %d clockid\n", clockid);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv[]: *mut c_char) -> c_int {
    int main(int argc, char *argv[])
    {
    int status, len, fd;
    char buf[4096];
    pid_t pid;
    struct timespec mtime_now;
    ksft_print_header();
    nscheck();
    ksft_set_plan(2);
    clock_gettime(CLOCK_MONOTONIC, &mtime_now);
    if (unshare_timens())
    return 1;
    len = snprintf(buf, sizeof(buf), "%d %d 0",
    CLOCK_MONOTONIC, 70 * 24 * 3600);
    fd = open("/proc/self/timens_offsets", O_WRONLY);
    if (fd < 0)
    return pr_perror("/proc/self/timens_offsets");
    if (write(fd, buf, len) != len)
    return pr_perror("/proc/self/timens_offsets");
    close(fd);
    pid = fork();
    if (pid < 0)
    return pr_perror("Unable to fork");
    if (pid == 0) {
    let mut ret: c_int = 0;
    ret |= run_test(CLOCK_REALTIME);
    ret |= run_test(CLOCK_MONOTONIC);
    if (ret)
    ksft_exit_fail();
    ksft_exit_pass();
    return 0;
    }
    if (waitpid(pid, &status, 0) != pid)
    return pr_perror("Unable to wait the child process");
    if (WIFEXITED(status))
    return WEXITSTATUS(status);
    return 1;
    }
