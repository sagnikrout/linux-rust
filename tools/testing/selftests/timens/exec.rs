//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/timens/exec.c
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
pub unsafe extern "C" fn main(argc: c_int, argv[]: *mut c_char) -> c_int {
    int main(int argc, char *argv[])
    {
    struct timespec now, tst;
    int status, i;
    pid_t pid;
    if (argc > 1) {
    if (sscanf(argv[1], "%ld", &now.tv_sec) != 1)
    return pr_perror("sscanf");
    for (i = 0; i < 2; i++) {
    _gettime(CLOCK_MONOTONIC, &tst, i);
    if (labs(tst.tv_sec - now.tv_sec) > 5)
    return pr_fail("%ld %ld\n", now.tv_sec, tst.tv_sec);
    }
    return 0;
    }
    ksft_print_header();
    nscheck();
    ksft_set_plan(1);
    clock_gettime(CLOCK_MONOTONIC, &now);
    if (unshare_timens())
    return 1;
    if (_settime(CLOCK_MONOTONIC, OFFSET))
    return 1;
    for (i = 0; i < 2; i++) {
    _gettime(CLOCK_MONOTONIC, &tst, i);
    if (labs(tst.tv_sec - now.tv_sec) > 5)
    return pr_fail("%ld %ld\n",
    now.tv_sec, tst.tv_sec);
    }
    if (argc > 1)
    return 0;
    pid = fork();
    if (pid < 0)
    return pr_perror("fork");
    if (pid == 0) {
    char now_str[64];
    char *cargv[] = {"exec", now_str, core::ptr::null_mut()};
    char *cenv[] = {core::ptr::null_mut()};
// Check that a child process is in the new timens.
    for (i = 0; i < 2; i++) {
    _gettime(CLOCK_MONOTONIC, &tst, i);
    if (labs(tst.tv_sec - now.tv_sec - OFFSET) > 5)
    return pr_fail("%ld %ld\n",
    now.tv_sec + OFFSET, tst.tv_sec);
    }
// Check for proper vvar offsets after execve.
    snprintf(now_str, sizeof(now_str), "%ld", now.tv_sec + OFFSET);
    execve("/proc/self/exe", cargv, cenv);
    return pr_perror("execve");
    }
    if (waitpid(pid, &status, 0) != pid)
    return pr_perror("waitpid");
    if (status)
    ksft_exit_fail();
    ksft_test_result_pass("exec\n");
    ksft_exit_pass();
    return 0;
    }
