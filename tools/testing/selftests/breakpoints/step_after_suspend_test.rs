//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/breakpoints/step_after_suspend_test.c
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
// Copyright (C) 2016 Google, Inc.
//
// Macro flag: #define _GNU_SOURCE

#[no_mangle]
pub unsafe extern "C" fn child(cpu: c_int) {
    void child(int cpu)
    {
    cpu_set_t set;
    CPU_ZERO(&set);
    CPU_SET(cpu, &set);
    if (sched_setaffinity(0, sizeof(set), &set) != 0) {
    ksft_print_msg("sched_setaffinity() failed: %s\n",
    strerror(errno));
    _exit(1);
    }
    if (ptrace(PTRACE_TRACEME, 0, core::ptr::null_mut(), core::ptr::null_mut()) != 0) {
    ksft_print_msg("ptrace(PTRACE_TRACEME) failed: %s\n",
    strerror(errno));
    _exit(1);
    }
    if (raise(SIGSTOP) != 0) {
    ksft_print_msg("raise(SIGSTOP) failed: %s\n", strerror(errno));
    _exit(1);
    }
    _exit(0);
    }
#[no_mangle]
pub unsafe extern "C" fn run_test(cpu: c_int) -> c_int {
    int run_test(int cpu)
    {
    int status;
    let mut pid: pid_t = fork();
    pid_t wpid;
    if (pid < 0) {
    ksft_print_msg("fork() failed: %s\n", strerror(errno));
    return KSFT_FAIL;
    }
    if (pid == 0)
    child(cpu);
    wpid = waitpid(pid, &status, __WALL);
    if (wpid != pid) {
    ksft_print_msg("waitpid() failed: %s\n", strerror(errno));
    return KSFT_FAIL;
    }
    if (!WIFSTOPPED(status)) {
    ksft_print_msg("child did not stop: %s\n", strerror(errno));
    return KSFT_FAIL;
    }
    if (WSTOPSIG(status) != SIGSTOP) {
    ksft_print_msg("child did not stop with SIGSTOP: %s\n",
    strerror(errno));
    return KSFT_FAIL;
    }
    if (ptrace(PTRACE_SINGLESTEP, pid, core::ptr::null_mut(), core::ptr::null_mut()) < 0) {
    if (errno == EIO) {
    ksft_print_msg(
    "ptrace(PTRACE_SINGLESTEP) not supported on this architecture: %s\n",
    strerror(errno));
    return KSFT_SKIP;
    }
    ksft_print_msg("ptrace(PTRACE_SINGLESTEP) failed: %s\n",
    strerror(errno));
    return KSFT_FAIL;
    }
    wpid = waitpid(pid, &status, __WALL);
    if (wpid != pid) {
    ksft_print_msg("waitpid() failed: %s\n", strerror(errno));
    return KSFT_FAIL;
    }
    if (WIFEXITED(status)) {
    ksft_print_msg("child did not single-step: %s\n",
    strerror(errno));
    return KSFT_FAIL;
    }
    if (!WIFSTOPPED(status)) {
    ksft_print_msg("child did not stop: %s\n", strerror(errno));
    return KSFT_FAIL;
    }
    if (WSTOPSIG(status) != SIGTRAP) {
    ksft_print_msg("child did not stop with SIGTRAP: %s\n",
    strerror(errno));
    return KSFT_FAIL;
    }
    if (ptrace(PTRACE_CONT, pid, core::ptr::null_mut(), core::ptr::null_mut()) < 0) {
    ksft_print_msg("ptrace(PTRACE_CONT) failed: %s\n",
    strerror(errno));
    return KSFT_FAIL;
    }
    wpid = waitpid(pid, &status, __WALL);
    if (wpid != pid) {
    ksft_print_msg("waitpid() failed: %s\n", strerror(errno));
    return KSFT_FAIL;
    }
    if (!WIFEXITED(status)) {
    ksft_print_msg("child did not exit after PTRACE_CONT: %s\n",
    strerror(errno));
    return KSFT_FAIL;
    }
    return KSFT_PASS;
    }
//
// Reads the suspend success count from sysfs.
// Returns the count on success or exits on failure.
//
#[no_mangle]
unsafe extern "C" fn get_suspend_success_count_or_fail() -> c_int {
    static int get_suspend_success_count_or_fail(void)
    {
    FILE *fp;
    int val;
    fp = fopen("/sys/power/suspend_stats/success", "r");
    if (!fp)
    ksft_exit_fail_msg(
    "Failed to open suspend_stats/success: %s\n",
    strerror(errno));
    if (fscanf(fp, "%d", &val) != 1) {
    fclose(fp);
    ksft_exit_fail_msg(
    "Failed to read suspend success count\n");
    }
    fclose(fp);
    return val;
    }
#[no_mangle]
pub unsafe extern "C" fn suspend() {
    void suspend(void)
    {
    int timerfd;
    int err;
    int count_before;
    int count_after;
    let mut spec: itimerspec = {};
    if (getuid() != 0)
    ksft_exit_skip("Please run the test as root - Exiting.\n");
    timerfd = timerfd_create(CLOCK_BOOTTIME_ALARM, 0);
    if (timerfd < 0)
    ksft_exit_fail_msg("timerfd_create() failed\n");
    spec.it_value.tv_sec = 5;
    err = timerfd_settime(timerfd, 0, &spec, core::ptr::null_mut());
    if (err < 0)
    ksft_exit_fail_msg("timerfd_settime() failed\n");
    count_before = get_suspend_success_count_or_fail();
    system("(echo mem > /sys/power/state) 2> /dev/null");
    count_after = get_suspend_success_count_or_fail();
    if (count_after <= count_before)
    ksft_exit_fail_msg("Failed to enter Suspend state\n");
    close(timerfd);
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> c_int {
    int main(int argc, char **argv)
    {
    int opt;
    let mut do_suspend: bool = true;
    let mut succeeded: bool = true;
    let mut tests: c_uint = 0;
    cpu_set_t available_cpus;
    int err;
    int cpu;
    ksft_print_header();
    while ((opt = getopt(argc, argv, "n")) != -1) {
    switch (opt) {
    case 'n':
    do_suspend = false;
    break;
    default:
    printf("Usage: %s [-n]\n", argv[0]);
    printf("        -n: do not trigger a suspend/resume cycle before the test\n");
    return -1;
    }
    }
    err = sched_getaffinity(0, sizeof(available_cpus), &available_cpus);
    if (err < 0)
    ksft_exit_fail_msg("sched_getaffinity() failed\n");
    for (cpu = 0; cpu < CPU_SETSIZE; cpu++) {
    if (!CPU_ISSET(cpu, &available_cpus))
    continue;
    tests++;
    }
    if (do_suspend)
    suspend();
    ksft_set_plan(tests);
    for (cpu = 0; cpu < CPU_SETSIZE; cpu++) {
    int test_success;
    if (!CPU_ISSET(cpu, &available_cpus))
    continue;
    test_success = run_test(cpu);
    switch (test_success) {
    case KSFT_PASS:
    ksft_test_result_pass("CPU %d\n", cpu);
    break;
    case KSFT_SKIP:
    ksft_test_result_skip("CPU %d\n", cpu);
    break;
    case KSFT_FAIL:
    ksft_test_result_fail("CPU %d\n", cpu);
    succeeded = false;
    break;
    }
    }
    if (succeeded)
    ksft_exit_pass();
    else
    ksft_exit_fail();
    }
