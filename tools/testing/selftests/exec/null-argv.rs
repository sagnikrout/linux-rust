//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/exec/null-argv.c
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
// Test that empty argvs are swapped out for a single empty string.

    do {						\
    pid = fork();				\
    if (pid == 0) {				\
// Child */			\
    exec; /* Some kind of exec */	\
    perror("# " #exec);		\
    return 1;			\
    }					\
    check_result(pid, #exec);		\
    } while (0)
#[no_mangle]
pub unsafe extern "C" fn check_result(pid: pid_t, msg: *const c_char) {
    void check_result(pid_t pid, const char *msg)
    {
    int wstatus;
    if (pid == (pid_t)-1) {
    perror("# fork");
    ksft_test_result_fail("fork failed: %s\n", msg);
    return;
    }
    if (waitpid(pid, &wstatus, 0) < 0) {
    perror("# waitpid");
    ksft_test_result_fail("waitpid failed: %s\n", msg);
    return;
    }
    if (!WIFEXITED(wstatus)) {
    ksft_test_result_fail("child did not exit: %s\n", msg);
    return;
    }
    if (WEXITSTATUS(wstatus) != 0) {
    ksft_test_result_fail("non-zero exit: %s\n", msg);
    return;
    }
    ksft_test_result_pass("%s\n", msg);
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv[]: *mut c_char, envp[]: *mut c_char) -> c_int {
    int main(int argc, char *argv[], char *envp[])
    {
    pid_t pid;
    static char * const args[] = { core::ptr::null_mut() };
    static char * const str[] = { "", core::ptr::null_mut() };
// argc counting checks
    if (argc < 1) {
    fprintf(stderr, "# FAIL: saw argc == 0 (old kernel?)\n");
    return 1;
    }
    if (argc != 1) {
    fprintf(stderr, "# FAIL: unknown argc (%d)\n", argc);
    return 1;
    }
    if (argv[0][0] == '\0') {
// Good, we found a NULL terminated string at argv[0]!
    return 0;
    }
// Test runner.
    ksft_print_header();
    ksft_set_plan(5);
    FORK(execve(argv[0], str, core::ptr::null_mut()));
    FORK(execve(argv[0], core::ptr::null_mut(), core::ptr::null_mut()));
    FORK(execve(argv[0], core::ptr::null_mut(), envp));
    FORK(execve(argv[0], args, core::ptr::null_mut()));
    FORK(execve(argv[0], args, envp));
    ksft_exit(ksft_cnt.ksft_pass == ksft_plan);
    }
