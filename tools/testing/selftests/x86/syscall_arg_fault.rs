//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/x86/syscall_arg_fault.c
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
// syscall_arg_fault.c - tests faults 32-bit fast syscall stack args
// Copyright (c) 2015 Andrew Lutomirski
//
// Macro flag: #define _GNU_SOURCE

    static sigjmp_buf jmpbuf;
    static volatile sig_atomic_t n_errs;

#[no_mangle]
unsafe extern "C" fn sigsegv_or_sigbus(sig: c_int, info: *mut siginfo_t, ctx_void: *mut c_void) {
    static void sigsegv_or_sigbus(int sig, siginfo_t *info, void *ctx_void)
    {
    ucontext_t *ctx = (ucontext_t*)ctx_void;
    let mut ax: c_long = (long)ctx.uc_mcontext.gregs[REG_AX];
    if (ax != -EFAULT && ax != -ENOSYS) {
    printf("[FAIL]\tAX had the wrong value: 0x%lx\n",
    (unsigned long)ax);
    printf("\tIP = 0x%lx\n", (unsigned long)ctx.uc_mcontext.gregs[REG_IP]);
    n_errs++;
    } else {
    printf("[OK]\tSeems okay\n");
    }
    siglongjmp(jmpbuf, 1);
    }
    static volatile sig_atomic_t sigtrap_consecutive_syscalls;
#[no_mangle]
unsafe extern "C" fn sigtrap(sig: c_int, info: *mut siginfo_t, ctx_void: *mut c_void) {
    static void sigtrap(int sig, siginfo_t *info, void *ctx_void)
    {
//
// KVM has some bugs that can cause us to stop making progress.
// detect them and complain, but don't infinite loop or fail the
// test.
//
    ucontext_t *ctx = (ucontext_t*)ctx_void;
    unsigned short *ip = (unsigned short *)ctx.uc_mcontext.gregs[REG_IP];
    if (*ip == 0x340f || *ip == 0x050f) {
// The trap was on SYSCALL or SYSENTER
    sigtrap_consecutive_syscalls++;
    if (sigtrap_consecutive_syscalls > 3) {
    printf("[WARN]\tGot stuck single-stepping -- you probably have a KVM bug\n");
    siglongjmp(jmpbuf, 1);
    }
    } else {
    sigtrap_consecutive_syscalls = 0;
    }
    }
#[no_mangle]
unsafe extern "C" fn sigill(sig: c_int, info: *mut siginfo_t, ctx_void: *mut c_void) {
    static void sigill(int sig, siginfo_t *info, void *ctx_void)
    {
    ucontext_t *ctx = (ucontext_t*)ctx_void;
    unsigned short *ip = (unsigned short *)ctx.uc_mcontext.gregs[REG_IP];
    if (*ip == 0x0b0f) {
// one of the ud2 instructions faulted
    printf("[OK]\tSYSCALL returned normally\n");
    } else {
    printf("[SKIP]\tIllegal instruction\n");
    }
    siglongjmp(jmpbuf, 1);
    }
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main()
    {
    stack_t stack = {
// Our sigaltstack scratch space.
    .ss_sp = malloc(sizeof(char) * SIGSTKSZ),
    .ss_size = SIGSTKSZ,
    };
    if (sigaltstack(&stack, core::ptr::null_mut()) != 0)
    err(1, "sigaltstack");
    sethandler(SIGSEGV, sigsegv_or_sigbus, SA_ONSTACK);
//
// The actual exception can vary.  On Atom CPUs, we get #SS
// instead of #PF when the vDSO fails to access the stack when
// ESP is too close to 2^32, and #SS causes SIGBUS.
//
    sethandler(SIGBUS, sigsegv_or_sigbus, SA_ONSTACK);
    sethandler(SIGILL, sigill, SA_ONSTACK);
//
// Exercise another nasty special case.  The 32-bit SYSCALL
// and SYSENTER instructions (even in compat mode) each
// clobber one register.  A Linux system call has a syscall
// number and six arguments, and the user stack pointer
// needs to live in some register on return.  That means
// that we need eight registers, but SYSCALL and SYSENTER
// only preserve seven registers.  As a result, one argument
// ends up on the stack.  The stack is user memory, which
// means that the kernel can fail to read it.
//
// The 32-bit fast system calls don't have a defined ABI:
// we're supposed to invoke them through the vDSO.  So we'll
// fudge it: we set all regs to invalid pointer values and
// invoke the entry instruction.  The return will fail no
// matter what, and we completely lose our program state,
// but we can fix it up with a signal handler.
//
    printf("[RUN]\tSYSENTER with invalid state\n");
    if (sigsetjmp(jmpbuf, 1) == 0) {
    asm volatile (
    "movl $-1, %%eax\n\t"
    "movl $-1, %%ebx\n\t"
    "movl $-1, %%ecx\n\t"
    "movl $-1, %%edx\n\t"
    "movl $-1, %%esi\n\t"
    "movl $-1, %%edi\n\t"
    "movl $-1, %%ebp\n\t"
    "movl $-1, %%esp\n\t"
    "sysenter"
    : : : "memory", "flags");
    }
    printf("[RUN]\tSYSCALL with invalid state\n");
    if (sigsetjmp(jmpbuf, 1) == 0) {
    asm volatile (
    "movl $-1, %%eax\n\t"
    "movl $-1, %%ebx\n\t"
    "movl $-1, %%ecx\n\t"
    "movl $-1, %%edx\n\t"
    "movl $-1, %%esi\n\t"
    "movl $-1, %%edi\n\t"
    "movl $-1, %%ebp\n\t"
    "movl $-1, %%esp\n\t"
    "syscall\n\t"
    "ud2"		/* make sure we recover cleanly */
    : : : "memory", "flags");
    }
    printf("[RUN]\tSYSENTER with TF and invalid state\n");
    sethandler(SIGTRAP, sigtrap, SA_ONSTACK);
    if (sigsetjmp(jmpbuf, 1) == 0) {
    sigtrap_consecutive_syscalls = 0;
    set_eflags(get_eflags() | X86_EFLAGS_TF);
    asm volatile (
    "movl $-1, %%eax\n\t"
    "movl $-1, %%ebx\n\t"
    "movl $-1, %%ecx\n\t"
    "movl $-1, %%edx\n\t"
    "movl $-1, %%esi\n\t"
    "movl $-1, %%edi\n\t"
    "movl $-1, %%ebp\n\t"
    "movl $-1, %%esp\n\t"
    "sysenter"
    : : : "memory", "flags");
    }
    set_eflags(get_eflags() & ~X86_EFLAGS_TF);
    printf("[RUN]\tSYSCALL with TF and invalid state\n");
    if (sigsetjmp(jmpbuf, 1) == 0) {
    sigtrap_consecutive_syscalls = 0;
    set_eflags(get_eflags() | X86_EFLAGS_TF);
    asm volatile (
    "movl $-1, %%eax\n\t"
    "movl $-1, %%ebx\n\t"
    "movl $-1, %%ecx\n\t"
    "movl $-1, %%edx\n\t"
    "movl $-1, %%esi\n\t"
    "movl $-1, %%edi\n\t"
    "movl $-1, %%ebp\n\t"
    "movl $-1, %%esp\n\t"
    "syscall\n\t"
    "ud2"		/* make sure we recover cleanly */
    : : : "memory", "flags");
    }
    set_eflags(get_eflags() & ~X86_EFLAGS_TF);

    printf("[RUN]\tSYSENTER with TF, invalid state, and GSBASE < 0\n");
    if (sigsetjmp(jmpbuf, 1) == 0) {
    sigtrap_consecutive_syscalls = 0;
    asm volatile ("wrgsbase %%rax\n\t"
    :: "a" (0xffffffffffff0000UL));
    set_eflags(get_eflags() | X86_EFLAGS_TF);
    asm volatile (
    "movl $-1, %%eax\n\t"
    "movl $-1, %%ebx\n\t"
    "movl $-1, %%ecx\n\t"
    "movl $-1, %%edx\n\t"
    "movl $-1, %%esi\n\t"
    "movl $-1, %%edi\n\t"
    "movl $-1, %%ebp\n\t"
    "movl $-1, %%esp\n\t"
    "sysenter"
    : : : "memory", "flags");
    }
    set_eflags(get_eflags() & ~X86_EFLAGS_TF);

    free(stack.ss_sp);
    return 0;
    }
