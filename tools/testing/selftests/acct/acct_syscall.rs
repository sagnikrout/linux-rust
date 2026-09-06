//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/acct/acct_syscall.c
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
// kselftest for acct() system call
// The acct() system call enables or disables process accounting.
//

#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    char filename[] = "process_log";
    FILE *fp;
    pid_t child_pid;
    int sz;
// Setting up kselftest framework
    ksft_print_header();
    ksft_set_plan(1);
// Check if test is run a root
    if (geteuid()) {
    ksft_exit_skip("This test needs root to run!\n");
    return 1;
    }
// Create file to log closed processes
    fp = fopen(filename, "w");
    if (!fp) {
    ksft_test_result_error("%s.\n", strerror(errno));
    ksft_finished();
    return 1;
    }
    acct(filename);
// Handle error conditions
    if (errno) {
    ksft_test_result_error("%s.\n", strerror(errno));
    fclose(fp);
    ksft_finished();
    return 1;
    }
// Create child process and wait for it to terminate.
    child_pid = fork();
    if (child_pid < 0) {
    ksft_test_result_error("Creating a child process to log failed\n");
    acct(core::ptr::null_mut());
    return 1;
    } else if (child_pid > 0) {
    wait(core::ptr::null_mut());
    fseek(fp, 0L, SEEK_END);
    sz = ftell(fp);
    acct(core::ptr::null_mut());
    if (sz <= 0) {
    ksft_test_result_fail("Terminated child process not logged\n");
    ksft_exit_fail();
    return 1;
    }
    ksft_test_result_pass("Successfully logged terminated process.\n");
    fclose(fp);
    ksft_exit_pass();
    return 0;
    }
    return 1;
    }
