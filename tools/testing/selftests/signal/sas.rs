//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/signal/sas.c
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
// Stas Sergeev <stsp@users.sourceforge.net>
//
// test sigaltstack(SS_ONSTACK | SS_AUTODISARM)
// If that succeeds, then swapcontext() can be used inside sighandler safely.
//
// Macro flag: #define _GNU_SOURCE

pub const AT_MINSIGSTKSZ: c_int = 51;

    static unsigned int stack_size;
    static void *sstack, *ustack;
    static ucontext_t uc, sc;
    static const char *msg = "[OK]\tStack preserved";
    static const char *msg2 = "[FAIL]\tStack corrupted";
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stk_data {
    pub msg: [c_char; 128],
    pub flag: c_int,
}

#[no_mangle]
pub unsafe extern "C" fn my_usr1(sig: c_int, si: *mut siginfo_t, u: *mut c_void) {
    void my_usr1(int sig, siginfo_t *si, void *u)
    {
    char *aa;
    int err;
    stack_t stk;
    struct stk_data *p;
    if (sp < (unsigned long)sstack ||
    sp >= (unsigned long)sstack + stack_size) {
    ksft_exit_fail_msg("SP is not on sigaltstack\n");
    }
// put some data on stack. other sighandler will try to overwrite it
    aa = alloca(1024);
    assert(aa);
    p = (struct stk_data *)(aa + 512);
    strcpy(p.msg, msg);
    p.flag = 1;
    ksft_print_msg("[RUN]\tsignal USR1\n");
    err = sigaltstack(core::ptr::null_mut(), &stk);
    if (err) {
    ksft_exit_fail_msg("sigaltstack() - %s\n", strerror(errno));
    exit(EXIT_FAILURE);
    }
    if (stk.ss_flags != SS_DISABLE)
    ksft_test_result_fail("tss_flags=%x, should be SS_DISABLE\n",
    stk.ss_flags);
    else
    ksft_test_result_pass(
    "sigaltstack is disabled in sighandler\n");
    swapcontext(&sc, &uc);
    ksft_print_msg("%s\n", p.msg);
    if (!p.flag) {
    ksft_exit_fail_msg("[RUN]\tAborting\n");
    exit(EXIT_FAILURE);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn my_usr2(sig: c_int, si: *mut siginfo_t, u: *mut c_void) {
    void my_usr2(int sig, siginfo_t *si, void *u)
    {
    char *aa;
    struct stk_data *p;
    ksft_print_msg("[RUN]\tsignal USR2\n");
    aa = alloca(1024);
// dont run valgrind on this
// try to find the data stored by previous sighandler
    p = memmem(aa, 1024, msg, strlen(msg));
    if (p) {
    ksft_test_result_fail("sigaltstack re-used\n");
// corrupt the data
    strcpy(p.msg, msg2);
// tell other sighandler that his data is corrupted
    p.flag = 0;
    }
    }
#[no_mangle]
unsafe extern "C" fn switch_fn() {
    static void switch_fn(void)
    {
    ksft_print_msg("[RUN]\tswitched to user ctx\n");
    raise(SIGUSR2);
    setcontext(&sc);
    }
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    struct sigaction act;
    stack_t stk;
    int err;
// Make sure more than the required minimum.
    stack_size = getauxval(AT_MINSIGSTKSZ) + SIGSTKSZ;
    ksft_print_msg("[NOTE]\tthe stack size is %u\n", stack_size);
    ksft_print_header();
    ksft_set_plan(3);
    sigemptyset(&act.sa_mask);
    act.sa_flags = SA_ONSTACK | SA_SIGINFO;
    act.sa_sigaction = my_usr1;
    sigaction(SIGUSR1, &act, core::ptr::null_mut());
    act.sa_sigaction = my_usr2;
    sigaction(SIGUSR2, &act, core::ptr::null_mut());
    sstack = mmap(core::ptr::null_mut(), stack_size, PROT_READ | PROT_WRITE,
    MAP_PRIVATE | MAP_ANONYMOUS | MAP_STACK, -1, 0);
    if (sstack == MAP_FAILED) {
    ksft_exit_fail_msg("mmap() - %s\n", strerror(errno));
    return EXIT_FAILURE;
    }
    err = sigaltstack(core::ptr::null_mut(), &stk);
    if (err) {
    ksft_exit_fail_msg("sigaltstack() - %s\n", strerror(errno));
    exit(EXIT_FAILURE);
    }
    if (stk.ss_flags == SS_DISABLE) {
    ksft_test_result_pass(
    "Initial sigaltstack state was SS_DISABLE\n");
    } else {
    ksft_exit_fail_msg("Initial sigaltstack state was %x; "
    "should have been SS_DISABLE\n", stk.ss_flags);
    return EXIT_FAILURE;
    }
    stk.ss_sp = sstack;
    stk.ss_size = stack_size;
    stk.ss_flags = SS_ONSTACK | SS_AUTODISARM;
    err = sigaltstack(&stk, core::ptr::null_mut());
    if (err) {
    if (errno == EINVAL) {
    ksft_test_result_skip(
    "[NOTE]\tThe running kernel doesn't support SS_AUTODISARM\n");
//
// If test cases for the !SS_AUTODISARM variant were
// added, we could still run them.  We don't have any
// test cases like that yet, so just exit and report
// success.
//
    return 0;
    } else {
    ksft_exit_fail_msg(
    "sigaltstack(SS_ONSTACK | SS_AUTODISARM)  %s\n",
    strerror(errno));
    return EXIT_FAILURE;
    }
    }
    ustack = mmap(core::ptr::null_mut(), stack_size, PROT_READ | PROT_WRITE,
    MAP_PRIVATE | MAP_ANONYMOUS | MAP_STACK, -1, 0);
    if (ustack == MAP_FAILED) {
    ksft_exit_fail_msg("mmap() - %s\n", strerror(errno));
    return EXIT_FAILURE;
    }
    getcontext(&uc);
    uc.uc_link = core::ptr::null_mut();
    uc.uc_stack.ss_sp = ustack;
    uc.uc_stack.ss_size = stack_size;
    makecontext(&uc, switch_fn, 0);
    raise(SIGUSR1);
    err = sigaltstack(core::ptr::null_mut(), &stk);
    if (err) {
    ksft_exit_fail_msg("sigaltstack() - %s\n", strerror(errno));
    exit(EXIT_FAILURE);
    }
    if (stk.ss_flags != SS_AUTODISARM) {
    ksft_exit_fail_msg("ss_flags=%x, should be SS_AUTODISARM\n",
    stk.ss_flags);
    exit(EXIT_FAILURE);
    }
    ksft_test_result_pass(
    "sigaltstack is still SS_AUTODISARM after signal\n");
    ksft_exit_pass();
    return 0;
    }
