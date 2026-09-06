//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/pidfd/pidfd_poll_test.c
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

    static bool timeout;
#[no_mangle]
unsafe extern "C" fn handle_alarm(sig: c_int) {
    static void handle_alarm(int sig)
    {
    timeout = true;
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> c_int {
    int main(int argc, char **argv)
    {
    struct pollfd fds;
    int iter, nevents;
    let mut nr_iterations: c_int = 10000;
    fds.events = POLLIN;
    if (argc > 2)
    ksft_exit_fail_msg("Unexpected command line argument\n");
    if (argc == 2) {
    nr_iterations = atoi(argv[1]);
    if (nr_iterations <= 0)
    ksft_exit_fail_msg("invalid input parameter %s\n",
    argv[1]);
    }
    ksft_print_msg("running pidfd poll test for %d iterations\n",
    nr_iterations);
    for (iter = 0; iter < nr_iterations; iter++) {
    int pidfd;
    let mut child_pid: c_int = fork();
    if (child_pid < 0) {
    if (errno == EAGAIN) {
    iter--;
    continue;
    }
    ksft_exit_fail_msg(
    "%s - failed to fork a child process\n",
    strerror(errno));
    }
    if (child_pid == 0) {
// Child process just sleeps for a min and exits
    sleep(60);
    exit(EXIT_SUCCESS);
    }
// Parent kills the child and waits for its death
    pidfd = sys_pidfd_open(child_pid, 0);
    if (pidfd < 0)
    ksft_exit_fail_msg("%s - pidfd_open failed\n",
    strerror(errno));
// Setup 3 sec alarm - plenty of time
    if (signal(SIGALRM, handle_alarm) == SIG_ERR)
    ksft_exit_fail_msg("%s - signal failed\n",
    strerror(errno));
    alarm(3);
// Send SIGKILL to the child
    if (sys_pidfd_send_signal(pidfd, SIGKILL, core::ptr::null_mut(), 0))
    ksft_exit_fail_msg("%s - pidfd_send_signal failed\n",
    strerror(errno));
// Wait for the death notification
    fds.fd = pidfd;
    nevents = poll(&fds, 1, -1);
// Check for error conditions
    if (nevents < 0)
    ksft_exit_fail_msg("%s - poll failed\n",
    strerror(errno));
    if (nevents != 1)
    ksft_exit_fail_msg("unexpected poll result: %d\n",
    nevents);
    if (!(fds.revents & POLLIN))
    ksft_exit_fail_msg(
    "unexpected event type received: 0x%x\n",
    fds.revents);
    if (timeout)
    ksft_exit_fail_msg(
    "death notification wait timeout\n");
    close(pidfd);
// Wait for child to prevent zombies
    if (waitpid(child_pid, core::ptr::null_mut(), 0) < 0)
    ksft_exit_fail_msg("%s - waitpid failed\n",
    strerror(errno));
    }
    ksft_test_result_pass("pidfd poll test: pass\n");
    ksft_exit_pass();
    }
