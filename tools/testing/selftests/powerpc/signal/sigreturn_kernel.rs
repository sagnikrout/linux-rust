//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/powerpc/signal/sigreturn_kernel.c
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
//
// Test that we can't sigreturn to kernel addresses, or to kernel mode.
//
// Macro flag: #define _GNU_SOURCE

    static volatile unsigned long long sigreturn_addr;
    static volatile unsigned long long sigreturn_msr_mask;
#[no_mangle]
unsafe extern "C" fn sigusr1_handler(signo: c_int, si: *mut siginfo_t, uc_ptr: *mut c_void) {
    static void sigusr1_handler(int signo, siginfo_t *si, void *uc_ptr)
    {
    ucontext_t *uc = (ucontext_t *)uc_ptr;
    if (sigreturn_addr)
    UCONTEXT_NIA(uc) = sigreturn_addr;
    if (sigreturn_msr_mask)
    UCONTEXT_MSR(uc) &= sigreturn_msr_mask;
    }
#[no_mangle]
unsafe extern "C" fn fork_child() -> pid_t {
    static pid_t fork_child(void)
    {
    pid_t pid;
    pid = fork();
    if (pid == 0) {
    raise(SIGUSR1);
    exit(0);
    }
    return pid;
    }
#[no_mangle]
unsafe extern "C" fn expect_segv(pid: pid_t) -> c_int {
    static int expect_segv(pid_t pid)
    {
    int child_ret;
    waitpid(pid, &child_ret, 0);
    FAIL_IF(WIFEXITED(child_ret));
    FAIL_IF(!WIFSIGNALED(child_ret));
    FAIL_IF(WTERMSIG(child_ret) != 11);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn test_sigreturn_kernel() -> c_int {
    int test_sigreturn_kernel(void)
    {
    struct sigaction act;
    int child_ret, i;
    pid_t pid;
    act.sa_sigaction = sigusr1_handler;
    act.sa_flags = SA_SIGINFO;
    sigemptyset(&act.sa_mask);
    FAIL_IF(sigaction(SIGUSR1, &act, core::ptr::null_mut()));
    for (i = 0; i < 2; i++) {
// Return to kernel
    sigreturn_addr = 0xcull << 60;
    pid = fork_child();
    expect_segv(pid);
// Return to kernel virtual
    sigreturn_addr = 0xc008ull << 48;
    pid = fork_child();
    expect_segv(pid);
// Return out of range
    sigreturn_addr = 0xc010ull << 48;
    pid = fork_child();
    expect_segv(pid);
// Return to no-man's land, just below PAGE_OFFSET
    sigreturn_addr = (0xcull << 60) - (64 * 1024);
    pid = fork_child();
    expect_segv(pid);
// Return to no-man's land, above TASK_SIZE_4PB
    sigreturn_addr = 0x1ull << 52;
    pid = fork_child();
    expect_segv(pid);
// Return to 0xd space
    sigreturn_addr = 0xdull << 60;
    pid = fork_child();
    expect_segv(pid);
// Return to 0xe space
    sigreturn_addr = 0xeull << 60;
    pid = fork_child();
    expect_segv(pid);
// Return to 0xf space
    sigreturn_addr = 0xfull << 60;
    pid = fork_child();
    expect_segv(pid);
// Attempt to set PR=0 for 2nd loop (should be blocked by kernel)
    sigreturn_msr_mask = ~MSR_PR;
    }
    printf("All children killed as expected\n");
// Don't change address, just MSR, should return to user as normal
    sigreturn_addr = 0;
    sigreturn_msr_mask = ~MSR_PR;
    pid = fork_child();
    waitpid(pid, &child_ret, 0);
    FAIL_IF(!WIFEXITED(child_ret));
    FAIL_IF(WIFSIGNALED(child_ret));
    FAIL_IF(WEXITSTATUS(child_ret) != 0);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    return test_harness(test_sigreturn_kernel, "sigreturn_kernel");
    }
