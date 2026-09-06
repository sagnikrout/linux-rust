//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/clone3/clone3_clear_sighand.c
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
unsafe extern "C" fn nop_handler(signo: c_int) {
    static void nop_handler(int signo)
    {
    }
#[no_mangle]
unsafe extern "C" fn wait_for_pid(pid: pid_t) -> c_int {
    static int wait_for_pid(pid_t pid)
    {
    int status, ret;
    again:
    ret = waitpid(pid, &status, 0);
    if (ret == -1) {
    if (errno == EINTR)
    goto again;
    return -1;
    }
    if (!WIFEXITED(status))
    return -1;
    return WEXITSTATUS(status);
    }
#[no_mangle]
unsafe extern "C" fn test_clone3_clear_sighand() {
    static void test_clone3_clear_sighand(void)
    {
    int ret;
    pid_t pid;
    let mut args: __clone_args = {};
    struct sigaction act;
//
// Check that CLONE_CLEAR_SIGHAND and CLONE_SIGHAND are mutually
// exclusive.
//
    args.flags |= CLONE_CLEAR_SIGHAND | CLONE_SIGHAND;
    args.exit_signal = SIGCHLD;
    pid = sys_clone3(&args, sizeof(args));
    if (pid > 0)
    ksft_exit_fail_msg(
    "clone3(CLONE_CLEAR_SIGHAND | CLONE_SIGHAND) succeeded\n");
    act.sa_handler = nop_handler;
    ret = sigemptyset(&act.sa_mask);
    if (ret < 0)
    ksft_exit_fail_msg("%s - sigemptyset() failed\n",
    strerror(errno));
    act.sa_flags = 0;
// Register signal handler for SIGUSR1
    ret = sigaction(SIGUSR1, &act, core::ptr::null_mut());
    if (ret < 0)
    ksft_exit_fail_msg(
    "%s - sigaction(SIGUSR1, &act, core::ptr::null_mut()) failed\n",
    strerror(errno));
// Register signal handler for SIGUSR2
    ret = sigaction(SIGUSR2, &act, core::ptr::null_mut());
    if (ret < 0)
    ksft_exit_fail_msg(
    "%s - sigaction(SIGUSR2, &act, core::ptr::null_mut()) failed\n",
    strerror(errno));
// Check that CLONE_CLEAR_SIGHAND works.
    args.flags = CLONE_CLEAR_SIGHAND;
    pid = sys_clone3(&args, sizeof(args));
    if (pid < 0)
    ksft_exit_fail_msg("%s - clone3(CLONE_CLEAR_SIGHAND) failed\n",
    strerror(errno));
    if (pid == 0) {
    ret = sigaction(SIGUSR1, core::ptr::null_mut(), &act);
    if (ret < 0)
    exit(EXIT_FAILURE);
    if (act.sa_handler != SIG_DFL)
    exit(EXIT_FAILURE);
    ret = sigaction(SIGUSR2, core::ptr::null_mut(), &act);
    if (ret < 0)
    exit(EXIT_FAILURE);
    if (act.sa_handler != SIG_DFL)
    exit(EXIT_FAILURE);
    exit(EXIT_SUCCESS);
    }
    ret = wait_for_pid(pid);
    if (ret)
    ksft_exit_fail_msg(
    "Failed to clear signal handler for child process\n");
    ksft_test_result_pass("Cleared signal handlers for child process\n");
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> c_int {
    int main(int argc, char **argv)
    {
    ksft_print_header();
    ksft_set_plan(1);
    test_clone3_supported();
    test_clone3_clear_sighand();
    ksft_exit_pass();
    }
