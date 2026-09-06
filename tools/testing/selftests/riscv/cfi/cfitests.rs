//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/riscv/cfi/cfitests.c
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

// do not optimize cfi related test functions

#[no_mangle]
pub unsafe extern "C" fn sigsegv_handler(signum: c_int, si: *mut siginfo_t, uc: *mut c_void) {
    void sigsegv_handler(int signum, siginfo_t *si, void *uc)
    {
    struct ucontext *ctx = (struct ucontext *)uc;
    if (si.si_code == SEGV_CPERR) {
    ksft_print_msg("Control flow violation happened somewhere\n");
    ksft_print_msg("PC where violation happened %lx\n", ctx.uc_mcontext.gregs[0]);
    exit(-1);
    }
// all other cases are expected to be of shadow stack write case
    exit(CHILD_EXIT_CODE_SSWRITE);
    }
#[no_mangle]
pub unsafe extern "C" fn register_signal_handler() -> bool {
    bool register_signal_handler(void)
    {
    let mut sa: sigaction = {};
    sa.sa_sigaction = sigsegv_handler;
    sa.sa_flags = SA_SIGINFO;
    if (sigaction(SIGSEGV, &sa, core::ptr::null_mut())) {
    ksft_print_msg("Registering signal handler for landing pad violation failed\n");
    return false;
    }
    return true;
    }
    long ptrace(int request, pid_t pid, void *addr, void *data);
#[no_mangle]
pub unsafe extern "C" fn cfi_ptrace_test() -> bool {
    bool cfi_ptrace_test(void)
    {
    pid_t pid;
    int status, ret = 0;
    let mut ptrace_test_num: c_ulong = 0, total_ptrace_tests = 2;
    struct user_cfi_state cfi_reg;
    struct iovec iov;
    pid = fork();
    if (pid == -1) {
    ksft_exit_fail_msg("%s: fork failed\n", __func__);
    exit(1);
    }
    if (pid == 0) {
// allow to be traced
    ptrace(PTRACE_TRACEME, 0, core::ptr::null_mut(), core::ptr::null_mut());
    raise(SIGSTOP);
    asm volatile ("la a5, 1f\n"
    "jalr a5\n"
    "nop\n"
    "nop\n"
    "1: nop\n"
    : : : "a5");
    exit(11);
// child shouldn't go beyond here
    }
// parent's code goes here
    iov.iov_base = &cfi_reg;
    iov.iov_len = sizeof(cfi_reg);
    while (ptrace_test_num < total_ptrace_tests) {
    memset(&cfi_reg, 0, sizeof(cfi_reg));
    waitpid(pid, &status, 0);
    if (WIFSTOPPED(status)) {
    errno = 0;
    ret = ptrace(PTRACE_GETREGSET, pid, (void *)NT_RISCV_USER_CFI, &iov);
    if (ret == -1 && errno)
    ksft_exit_fail_msg("%s: PTRACE_GETREGSET failed\n", __func__);
    } else {
    ksft_exit_fail_msg("%s: child didn't stop, failed\n", __func__);
    }
    switch (ptrace_test_num) {

    PTRACE_CFI_SHADOW_STACK_EN_STATE |		\
    PTRACE_CFI_SHADOW_STACK_PTR_STATE)
    case 0:
    if ((cfi_reg.cfi_status.cfi_state & CFI_ENABLE_MASK) != CFI_ENABLE_MASK)
    ksft_exit_fail_msg("%s: ptrace_getregset failed, %llu\n", __func__,
    cfi_reg.cfi_status.cfi_state);
    if (!cfi_reg.shstk_ptr)
    ksft_exit_fail_msg("%s: core::ptr::null_mut() shadow stack pointer, test failed\n",
    __func__);
    break;
    case 1:
    if (!(cfi_reg.cfi_status.cfi_state &
    PTRACE_CFI_BRANCH_EXPECTED_LANDING_PAD_STATE))
    ksft_exit_fail_msg("%s: elp must have been set\n", __func__);
// clear elp state. not interested in anything else
    cfi_reg.cfi_status.cfi_state = 0;
    ret = ptrace(PTRACE_SETREGSET, pid, (void *)NT_RISCV_USER_CFI, &iov);
    if (ret == -1 && errno)
    ksft_exit_fail_msg("%s: PTRACE_GETREGSET failed\n", __func__);
    break;
    default:
    ksft_exit_fail_msg("%s: unreachable switch case\n", __func__);
    break;
    }
    ptrace(PTRACE_CONT, pid, core::ptr::null_mut(), core::ptr::null_mut());
    ptrace_test_num++;
    }
    waitpid(pid, &status, 0);
    if (WEXITSTATUS(status) != 11)
    ksft_print_msg("%s, bad return code from child\n", __func__);
    ksft_print_msg("%s, ptrace test succeeded\n", __func__);
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv[]: *mut c_char) -> c_int {
    int main(int argc, char *argv[])
    {
    let mut ret: c_int = 0;
    let mut lpad_status: c_ulong = 0, ss_status = 0;
    ksft_print_header();
    ksft_print_msg("Starting risc-v tests\n");
// Test unknown PR_CFI bits
    ret = my_syscall5(__NR_prctl, PR_SET_CFI, PR_CFI_BRANCH_LANDING_PADS,
    PR_CFI_ENABLE | 0xffff0, 0, 0);
    if (!ret)
    ksft_exit_fail_msg("PR_SET_CFI accepted reserved branch landing pad bits\n");
//
// Landing pad test. Not a lot of kernel changes to support landing
// pads for user mode except lighting up a bit in senvcfg via a prctl.
// Enable landing pad support throughout the execution of the test binary.
//
    ret = my_syscall5(__NR_prctl, PR_GET_CFI, PR_CFI_BRANCH_LANDING_PADS, &lpad_status, 0, 0);
    if (ret)
    ksft_exit_fail_msg("Get landing pad status failed with %d\n", ret);
    if (!(lpad_status & PR_CFI_ENABLE))
    ksft_exit_fail_msg("Landing pad is not enabled, should be enabled via glibc\n");
    ret = my_syscall5(__NR_prctl, PR_GET_SHADOW_STACK_STATUS, &ss_status, 0, 0, 0);
    if (ret)
    ksft_exit_fail_msg("Get shadow stack failed with %d\n", ret);
    if (!(ss_status & PR_SHADOW_STACK_ENABLE))
    ksft_exit_fail_msg("Shadow stack is not enabled, should be enabled via glibc\n");
    if (!register_signal_handler())
    ksft_exit_fail_msg("Registering signal handler for SIGSEGV failed\n");
    ksft_print_msg("Landing pad and shadow stack are enabled for binary\n");
    cfi_ptrace_test();
    execute_shadow_stack_tests();
    return 0;
    }
