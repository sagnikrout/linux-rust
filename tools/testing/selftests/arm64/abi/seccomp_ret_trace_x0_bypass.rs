//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/arm64/abi/seccomp_ret_trace_x0_bypass.c
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
// Test for SECCOMP_RET_TRACE argument modification bypass
// via stale orig_x0 during filter re-evaluation.
//
// On arm64, syscall_get_arguments() reads the first argument from
// regs->orig_x0.  When a seccomp filter returns SECCOMP_RET_TRACE,
// ptrace may modify regs->regs[0] while orig_x0 remains unchanged.
// The kernel then re-evaluates the filter; if it sees the stale
// orig_x0, it may incorrectly allow a syscall that the tracer intended
// to block.
//
// This test installs a filter that:
// - TRACEs write() when fd == 2
// - returns ERRNO(EPERM) when fd == 1
// - allows all other syscalls
//
// The child calls write(2, ...).  The parent catches the SECCOMP stop,
// changes x0 (fd) from 2 to 1, and resumes the child.
//
// If re-evaluation sees the old fd=2 (stale orig_x0), the filter
// returns TRACE again; because recheck_after_trace is true, the kernel
// allows the syscall to proceed.  write(1, ...) succeeds, child exits 0.
// -> test FAIL (bypass detected).
//
// If re-evaluation sees the new fd=1 (synced orig_x0), the filter
// returns ERRNO(EPERM), write fails, child exits 1.
// -> test PASS (no bypass).
//
// No special privileges required beyond CAP_SYS_PTRACE.
//

#[no_mangle]
unsafe extern "C" fn do_child() -> c_int {
    static int do_child(void)
    {
    long ret;
    if (ptrace(PTRACE_TRACEME, 0, core::ptr::null_mut(), core::ptr::null_mut()))
    _exit(2);
    raise(SIGSTOP);	/* synchronize with parent */
//
// Filter:
// if syscall == write:
// if fd == 2 -> TRACE
// if fd == 1 -> ERRNO(EPERM)
// else -> ALLOW
//
    struct sock_filter filter[] = {
// Load syscall number
    BPF_STMT(BPF_LD | BPF_W | BPF_ABS, offsetof(struct seccomp_data, nr)),
// If not write, allow
    BPF_JUMP(BPF_JMP | BPF_JEQ | BPF_K, __NR_write, 0, 5),
// Load first argument (fd)
    BPF_STMT(BPF_LD | BPF_W | BPF_ABS, ARG0_OFFSET),
// fd == 2 ?
    BPF_JUMP(BPF_JMP | BPF_JEQ | BPF_K, 2, 0, 1),
// Yes: TRACE
    BPF_STMT(BPF_RET | BPF_K, SECCOMP_RET_TRACE),
// fd == 1 ?
    BPF_JUMP(BPF_JMP | BPF_JEQ | BPF_K, 1, 0, 1),
// Yes: ERRNO(EPERM)
    BPF_STMT(BPF_RET | BPF_K, SECCOMP_RET_ERRNO | (EPERM & SECCOMP_RET_DATA)),
// Other fd: ALLOW
    BPF_STMT(BPF_RET | BPF_K, SECCOMP_RET_ALLOW),
    };
    struct sock_fprog prog = {
    .len = ARRAY_SIZE(filter),
    .filter = filter,
    };
    if (prctl(PR_SET_NO_NEW_PRIVS, 1, 0, 0, 0))
    _exit(3);
    if (prctl(PR_SET_SECCOMP, SECCOMP_MODE_FILTER, &prog))
    _exit(4);
//
// write(2, ...) triggers TRACE, parent changes fd to 1.
// If re-eval sees fd=1 -> ERRNO -> write fails, ret = -EPERM.
// If re-eval sees fd=2 -> TRACE again -> allowed -> write succeeds.
//
    ret = syscall(__NR_write, 2, "", 0);
    _exit(ret == 0 ? 0 : 1);
    }
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    struct user_pt_regs regs;
    let mut iov: iovec = { .iov_base = &regs, .iov_len = sizeof(regs) };
    pid_t child;
    int status;
    ksft_print_header();
    ksft_set_plan(1);
    child = fork();
    if (child < 0)
    ksft_exit_fail_msg("fork failed: %s", strerror(errno));
    if (!child)
    return do_child();
// 1. Wait for initial SIGSTOP
    if (waitpid(child, &status, 0) != child)
    ksft_exit_fail_msg("waitpid SIGSTOP");
    if (!WIFSTOPPED(status) || WSTOPSIG(status) != SIGSTOP)
    ksft_exit_fail_msg("unexpected initial stop");
// 2. Enable SECCOMP ptrace events
    if (ptrace(PTRACE_SETOPTIONS, child, 0, PTRACE_O_TRACESECCOMP))
    ksft_exit_fail_msg("PTRACE_SETOPTIONS");
// 3. Continue child to hit SECCOMP stop
    if (ptrace(PTRACE_CONT, child, 0, 0))
    ksft_exit_fail_msg("PTRACE_CONT");
// 4. Wait for SECCOMP stop
    while (1) {
    if (waitpid(child, &status, 0) != child)
    ksft_exit_fail_msg("waitpid SECCOMP");
    if (WIFEXITED(status)) {
    ksft_test_result_fail("child exited before SECCOMP stop\n");
    goto out;
    }
    if (WIFSIGNALED(status)) {
    ksft_test_result_fail("child killed unexpectedly\n");
    goto out;
    }
    if (WIFSTOPPED(status) &&
    WSTOPSIG(status) == SIGTRAP &&
    PTRACE_EVENT_MASK(status) == PTRACE_EVENT_SECCOMP)
    break;
    ptrace(PTRACE_CONT, child, 0, WSTOPSIG(status));
    }
// 5. Modify x0 (fd) from 2 to 1
    if (ptrace(PTRACE_GETREGSET, child, NT_PRSTATUS, &iov))
    ksft_exit_fail_perror("GETREGSET");
    if (regs.regs[8] != __NR_write || regs.regs[0] != 2) {
    ksft_test_result_fail("unexpected regs: syscall=%llu, x0=%llu\n",
    regs.regs[8], regs.regs[0]);
    goto out;
    }
    regs.regs[0] = 1;
    if (ptrace(PTRACE_SETREGSET, child, NT_PRSTATUS, &iov))
    ksft_exit_fail_perror("SETREGSET");
// 6. Resume child
    if (ptrace(PTRACE_CONT, child, 0, 0))
    ksft_exit_fail_perror("PTRACE_CONT");
// 7. Reap child – must exit normally
    if (waitpid(child, &status, 0) != child)
    ksft_exit_fail_msg("final waitpid");
    if (!WIFEXITED(status)) {
    ksft_test_result_fail("child did not exit normally\n");
    goto out;
    }
    if (WEXITSTATUS(status) != 0)
    ksft_test_result_pass("seccomp correctly denied modified syscall\n");
    else
    ksft_test_result_fail("write succeeded, orig_x0 bypass likely\n");
    out:
    if (child > 0) {
    kill(child, SIGKILL);
    waitpid(child, core::ptr::null_mut(), 0);
    }
    ksft_print_cnts();
    return ksft_get_fail_cnt() ? EXIT_FAILURE : EXIT_SUCCESS;
    }
