//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/x86/single_step_syscall.c
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
// single_step_syscall.c - single-steps various x86 syscalls
// Copyright (c) 2014-2015 Andrew Lutomirski
//
// This is a very simple series of tests that makes system calls with
// the TF flag set.  This exercises some nasty kernel code in the
// SYSENTER case: SYSENTER does not clear TF, so SYSENTER with TF set
// immediately issues #DB from CPL 0.  This requires special handling in
// the kernel.
//
// Macro flag: #define _GNU_SOURCE

    static volatile sig_atomic_t sig_traps, sig_eflags;
    sigjmp_buf jmpbuf;

#[no_mangle]
unsafe extern "C" fn sigtrap(sig: c_int, info: *mut siginfo_t, ctx_void: *mut c_void) {
    static void sigtrap(int sig, siginfo_t *info, void *ctx_void)
    {
    ucontext_t *ctx = (ucontext_t*)ctx_void;
    if (get_eflags() & X86_EFLAGS_TF) {
    set_eflags(get_eflags() & ~X86_EFLAGS_TF);
    printf("[WARN]\tSIGTRAP handler had TF set\n");
    _exit(1);
    }
    sig_traps++;
    if (sig_traps == 10000 || sig_traps == 10001) {
    printf("[WARN]\tHit %d SIGTRAPs with si_addr 0x%lx, ip 0x%lx\n",
    (int)sig_traps,
    (unsigned long)info.si_addr,
    (unsigned long)ctx.uc_mcontext.gregs[REG_IP]);
    }
    }
    static char const * const signames[] = {
    [SIGSEGV] = "SIGSEGV",
    [SIGBUS] = "SIBGUS",
    [SIGTRAP] = "SIGTRAP",
    [SIGILL] = "SIGILL",
    };
#[no_mangle]
unsafe extern "C" fn print_and_longjmp(sig: c_int, si: *mut siginfo_t, ctx_void: *mut c_void) {
    static void print_and_longjmp(int sig, siginfo_t *si, void *ctx_void)
    {
    ucontext_t *ctx = ctx_void;
    printf("\tGot %s with RIP=%lx, TF=%ld\n", signames[sig],
    (unsigned long)ctx.uc_mcontext.gregs[REG_IP],
    (unsigned long)ctx.uc_mcontext.gregs[REG_EFL] & X86_EFLAGS_TF);
    sig_eflags = (unsigned long)ctx.uc_mcontext.gregs[REG_EFL];
    siglongjmp(jmpbuf, 1);
    }
#[no_mangle]
unsafe extern "C" fn check_result() {
    static void check_result(void)
    {
    let mut new_eflags: c_ulong = get_eflags();
    set_eflags(new_eflags & ~X86_EFLAGS_TF);
    if (!sig_traps) {
    printf("[FAIL]\tNo SIGTRAP\n");
    exit(1);
    }
    if (!(new_eflags & X86_EFLAGS_TF)) {
    printf("[FAIL]\tTF was cleared\n");
    exit(1);
    }
    printf("[OK]\tSurvived with TF set and %d traps\n", (int)sig_traps);
    sig_traps = 0;
    }
#[no_mangle]
unsafe extern "C" fn fast_syscall_no_tf() {
    static void fast_syscall_no_tf(void)
    {
    sig_traps = 0;
    printf("[RUN]\tFast syscall with TF cleared\n");
    fflush(stdout);  /* Force a syscall */
    if (get_eflags() & X86_EFLAGS_TF) {
    printf("[FAIL]\tTF is now set\n");
    exit(1);
    }
    if (sig_traps) {
    printf("[FAIL]\tGot SIGTRAP\n");
    exit(1);
    }
    printf("[OK]\tNothing unexpected happened\n");
    }
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main()
    {

    int tmp;

    sethandler(SIGTRAP, sigtrap, 0);
    printf("[RUN]\tSet TF and check nop\n");
    set_eflags(get_eflags() | X86_EFLAGS_TF);
    asm volatile ("nop");
    check_result();

    printf("[RUN]\tSet TF and check syscall-less opportunistic sysret\n");
    set_eflags(get_eflags() | X86_EFLAGS_TF);
    extern unsigned char post_nop[];
    asm volatile ("pushf" WIDTH "\n\t"
    "pop" WIDTH " %%r11\n\t"
    "nop\n\t"
    "post_nop:"
    : : "c" (post_nop) : "r11");
    check_result();

    printf("[RUN]\tSet TF and check int80\n");
    set_eflags(get_eflags() | X86_EFLAGS_TF);
#[no_mangle]
pub unsafe extern "C" fn volatile((SYS_getpid: "int $0x80" : "=a" (tmp) : "a") -> asm {
    asm volatile ("int $0x80" : "=a" (tmp) : "a" (SYS_getpid)
    : INT80_CLOBBERS);
    check_result();

//
// This test is particularly interesting if fast syscalls use
// SYSENTER: it triggers a nasty design flaw in SYSENTER.
// Specifically, SYSENTER does not clear TF, so either SYSENTER
// or the next instruction traps at CPL0.  (Of course, Intel
// mostly forgot to document exactly what happens here.)  So we
// get a CPL0 fault with usergs (on 64-bit kernels) and possibly
// no stack.  The only sane way the kernel can possibly handle
// it is to clear TF on return from the #DB handler, but this
// happens way too early to set TF in the saved pt_regs, so the
// kernel has to do something clever to avoid losing track of
// the TF bit.
//
// Needless to say, we've had bugs in this area.
//
    syscall(SYS_getpid);  /* Force symbol binding without TF set. */
    printf("[RUN]\tSet TF and check a fast syscall\n");
    set_eflags(get_eflags() | X86_EFLAGS_TF);
    syscall(SYS_getpid);
    check_result();
// Now make sure that another fast syscall doesn't set TF again.
    fast_syscall_no_tf();
//
// And do a forced SYSENTER to make sure that this works even if
// fast syscalls don't use SYSENTER.
//
// Invoking SYSENTER directly breaks all the rules.  Just handle
// the SIGSEGV.
//
    if (sigsetjmp(jmpbuf, 1) == 0) {
    let mut nr: c_ulong = SYS_getpid;
    printf("[RUN]\tSet TF and check SYSENTER\n");
    stack_t stack = {
    .ss_sp = malloc(sizeof(char) * SIGSTKSZ),
    .ss_size = SIGSTKSZ,
    };
    if (sigaltstack(&stack, core::ptr::null_mut()) != 0)
    err(1, "sigaltstack");
    sethandler(SIGSEGV, print_and_longjmp,
    SA_RESETHAND | SA_ONSTACK);
    sethandler(SIGILL, print_and_longjmp, SA_RESETHAND);
    set_eflags(get_eflags() | X86_EFLAGS_TF);
    free(stack.ss_sp);
// Clear EBP first to make sure we segfault cleanly.
    asm volatile ("xorl %%ebp, %%ebp; SYSENTER" : "+a" (nr) :: "flags", "rcx"

    , "r11"

    );
// We're unreachable here.  SYSENTER forgets RIP.
    }
    clearhandler(SIGSEGV);
    clearhandler(SIGILL);
    if (!(sig_eflags & X86_EFLAGS_TF)) {
    printf("[FAIL]\tTF was cleared\n");
    exit(1);
    }
// Now make sure that another fast syscall doesn't set TF again.
    fast_syscall_no_tf();
    return 0;
    }
