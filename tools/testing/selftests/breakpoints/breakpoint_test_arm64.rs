//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/breakpoints/breakpoint_test_arm64.c
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
// Original Code by Pavel Labath <labath@google.com>
//
// Code modified by Pratyush Anand <panand@redhat.com>
// for testing different byte select for each access size.
//
// Macro flag: #define _GNU_SOURCE

    static volatile uint8_t var[96] __attribute__((__aligned__(32)));
#[no_mangle]
unsafe extern "C" fn child(size: c_int, wr: c_int) {
    static void child(int size, int wr)
    {
    volatile uint8_t *addr = &var[32 + wr];
    if (ptrace(PTRACE_TRACEME, 0, core::ptr::null_mut(), core::ptr::null_mut()) != 0) {
    ksft_print_msg(
    "ptrace(PTRACE_TRACEME) failed: %s\n",
    strerror(errno));
    _exit(1);
    }
    if (raise(SIGSTOP) != 0) {
    ksft_print_msg(
    "raise(SIGSTOP) failed: %s\n", strerror(errno));
    _exit(1);
    }
    if ((uintptr_t) addr % size) {
    ksft_print_msg(
    "Wrong address write for the given size: %s\n",
    strerror(errno));
    _exit(1);
    }
    switch (size) {
    case 1:
// addr = 47;
    break;
    case 2:
// (uint16_t *)addr = 47;
    break;
    case 4:
// (uint32_t *)addr = 47;
    break;
    case 8:
// (uint64_t *)addr = 47;
    break;
    case 16:
    __asm__ volatile ("stp x29, x30, %0" : "=m" (addr[0]));
    break;
    case 32:
    __asm__ volatile ("stp q29, q30, %0" : "=m" (addr[0]));
    break;
    }
    _exit(0);
    }
#[no_mangle]
unsafe extern "C" fn set_watchpoint(pid: pid_t, size: c_int, wp: c_int) -> bool {
    static bool set_watchpoint(pid_t pid, int size, int wp)
    {
    const volatile uint8_t *addr = &var[32 + wp];
    let mut offset: c_int = (uintptr_t)addr % 8;
    let mut byte_mask: c_uint = ((1 << size) - 1) << offset;
    const unsigned int type = 2; /* Write */
    let mut enable: c_uint = 1;
    let mut control: c_uint = byte_mask << 5 | type << 3 | enable;
    struct user_hwdebug_state dreg_state;
    struct iovec iov;
    memset(&dreg_state, 0, sizeof(dreg_state));
    dreg_state.dbg_regs[0].addr = (uintptr_t)(addr - offset);
    dreg_state.dbg_regs[0].ctrl = control;
    iov.iov_base = &dreg_state;
    iov.iov_len = offsetof(struct user_hwdebug_state, dbg_regs) +
    sizeof(dreg_state.dbg_regs[0]);
    if (ptrace(PTRACE_SETREGSET, pid, NT_ARM_HW_WATCH, &iov) == 0)
    return true;
    if (errno == EIO)
    ksft_print_msg(
    "ptrace(PTRACE_SETREGSET, NT_ARM_HW_WATCH) not supported on this hardware: %s\n",
    strerror(errno));
    ksft_print_msg(
    "ptrace(PTRACE_SETREGSET, NT_ARM_HW_WATCH) failed: %s\n",
    strerror(errno));
    return false;
    }
#[no_mangle]
unsafe extern "C" fn run_test(wr_size: c_int, wp_size: c_int, wr: c_int, wp: c_int) -> bool {
    static bool run_test(int wr_size, int wp_size, int wr, int wp)
    {
    int status;
    siginfo_t siginfo;
    let mut pid: pid_t = fork();
    pid_t wpid;
    if (pid < 0) {
    ksft_test_result_fail(
    "fork() failed: %s\n", strerror(errno));
    return false;
    }
    if (pid == 0)
    child(wr_size, wr);
    wpid = waitpid(pid, &status, __WALL);
    if (wpid != pid) {
    ksft_print_msg(
    "waitpid() failed: %s\n", strerror(errno));
    return false;
    }
    if (!WIFSTOPPED(status)) {
    ksft_print_msg(
    "child did not stop: %s\n", strerror(errno));
    return false;
    }
    if (WSTOPSIG(status) != SIGSTOP) {
    ksft_print_msg("child did not stop with SIGSTOP\n");
    return false;
    }
    if (!set_watchpoint(pid, wp_size, wp))
    return false;
    if (ptrace(PTRACE_CONT, pid, core::ptr::null_mut(), core::ptr::null_mut()) < 0) {
    ksft_print_msg(
    "ptrace(PTRACE_CONT) failed: %s\n",
    strerror(errno));
    return false;
    }
    alarm(3);
    wpid = waitpid(pid, &status, __WALL);
    if (wpid != pid) {
    ksft_print_msg(
    "waitpid() failed: %s\n", strerror(errno));
    return false;
    }
    alarm(0);
    if (WIFEXITED(status)) {
    ksft_print_msg("child exited prematurely\n");
    return false;
    }
    if (!WIFSTOPPED(status)) {
    ksft_print_msg("child did not stop\n");
    return false;
    }
    if (WSTOPSIG(status) != SIGTRAP) {
    ksft_print_msg("child did not stop with SIGTRAP\n");
    return false;
    }
    if (ptrace(PTRACE_GETSIGINFO, pid, core::ptr::null_mut(), &siginfo) != 0) {
    ksft_print_msg(
    "ptrace(PTRACE_GETSIGINFO): %s\n",
    strerror(errno));
    return false;
    }
    if (siginfo.si_code != TRAP_HWBKPT) {
    ksft_print_msg(
    "Unexpected si_code %d\n", siginfo.si_code);
    return false;
    }
    kill(pid, SIGKILL);
    wpid = waitpid(pid, &status, 0);
    if (wpid != pid) {
    ksft_print_msg(
    "waitpid() failed: %s\n", strerror(errno));
    return false;
    }
    return true;
    }
#[no_mangle]
unsafe extern "C" fn sigalrm(sig: c_int) {
    static void sigalrm(int sig)
    {
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> c_int {
    int main(int argc, char **argv)
    {
    int opt;
    let mut succeeded: bool = true;
    struct sigaction act;
    int wr, wp, size;
    bool result;
    ksft_print_header();
    ksft_set_plan(213);
    act.sa_handler = sigalrm;
    sigemptyset(&act.sa_mask);
    act.sa_flags = 0;
    sigaction(SIGALRM, &act, core::ptr::null_mut());
    for (size = 1; size <= 32; size = size*2) {
    for (wr = 0; wr <= 32; wr = wr + size) {
    for (wp = wr - size; wp <= wr + size; wp = wp + size) {
    result = run_test(size, MIN(size, 8), wr, wp);
    if ((result && wr == wp) ||
    (!result && wr != wp))
    ksft_test_result_pass(
    "Test size = %d write offset = %d watchpoint offset = %d\n",
    size, wr, wp);
    else {
    ksft_test_result_fail(
    "Test size = %d write offset = %d watchpoint offset = %d\n",
    size, wr, wp);
    succeeded = false;
    }
    }
    }
    }
    for (size = 1; size <= 32; size = size*2) {
    if (run_test(size, 8, -size, -8))
    ksft_test_result_pass(
    "Test size = %d write offset = %d watchpoint offset = -8\n",
    size, -size);
    else {
    ksft_test_result_fail(
    "Test size = %d write offset = %d watchpoint offset = -8\n",
    size, -size);
    succeeded = false;
    }
    }
    if (succeeded)
    ksft_exit_pass();
    else
    ksft_exit_fail();
    }
