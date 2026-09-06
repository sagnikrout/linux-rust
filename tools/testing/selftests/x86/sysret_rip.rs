//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/x86/sysret_rip.c
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
// sigreturn.c - tests that x86 avoids Intel SYSRET pitfalls
// Copyright (c) 2014-2016 Andrew Lutomirski
//
// Macro flag: #define _GNU_SOURCE

//
// These items are in clang_helpers_64.S, in order to avoid clang inline asm
// limitations:
//
    void test_syscall_ins(void);
    extern const char test_page[];
    static const void *current_test_page_addr = test_page;
// State used by our signal handlers.
    static gregset_t initial_regs;
    static volatile unsigned long rip;
#[no_mangle]
unsafe extern "C" fn sigsegv_for_sigreturn_test(sig: c_int, info: *mut siginfo_t, ctx_void: *mut c_void) {
    static void sigsegv_for_sigreturn_test(int sig, siginfo_t *info, void *ctx_void)
    {
    ucontext_t *ctx = (ucontext_t *)ctx_void;
    if (rip != ctx.uc_mcontext.gregs[REG_RIP]) {
    printf("[FAIL]\tRequested RIP=0x%lx but got RIP=0x%lx\n",
    rip, (unsigned long)ctx.uc_mcontext.gregs[REG_RIP]);
    fflush(stdout);
    _exit(1);
    }
    memcpy(&ctx.uc_mcontext.gregs, &initial_regs, sizeof(gregset_t));
    printf("[OK]\tGot SIGSEGV at RIP=0x%lx\n", rip);
    }
#[no_mangle]
unsafe extern "C" fn sigusr1(sig: c_int, info: *mut siginfo_t, ctx_void: *mut c_void) {
    static void sigusr1(int sig, siginfo_t *info, void *ctx_void)
    {
    ucontext_t *ctx = (ucontext_t *)ctx_void;
    memcpy(&initial_regs, &ctx.uc_mcontext.gregs, sizeof(gregset_t));
// Set IP and CX to match so that SYSRET can happen.
    ctx.uc_mcontext.gregs[REG_RIP] = rip;
    ctx.uc_mcontext.gregs[REG_RCX] = rip;
// R11 and EFLAGS should already match.
    assert(ctx.uc_mcontext.gregs[REG_EFL] ==
    ctx.uc_mcontext.gregs[REG_R11]);
    sethandler(SIGSEGV, sigsegv_for_sigreturn_test, SA_RESETHAND);
    }
#[no_mangle]
unsafe extern "C" fn test_sigreturn_to(ip: c_ulong) {
    static void test_sigreturn_to(unsigned long ip)
    {
    rip = ip;
    printf("[RUN]\tsigreturn to 0x%lx\n", ip);
    raise(SIGUSR1);
    }
    static jmp_buf jmpbuf;
#[no_mangle]
unsafe extern "C" fn sigsegv_for_fallthrough(sig: c_int, info: *mut siginfo_t, ctx_void: *mut c_void) {
    static void sigsegv_for_fallthrough(int sig, siginfo_t *info, void *ctx_void)
    {
    ucontext_t *ctx = (ucontext_t *)ctx_void;
    if (rip != ctx.uc_mcontext.gregs[REG_RIP]) {
    printf("[FAIL]\tExpected SIGSEGV at 0x%lx but got RIP=0x%lx\n",
    rip, (unsigned long)ctx.uc_mcontext.gregs[REG_RIP]);
    fflush(stdout);
    _exit(1);
    }
    siglongjmp(jmpbuf, 1);
    }
#[no_mangle]
unsafe extern "C" fn test_syscall_fallthrough_to(ip: c_ulong) {
    static void test_syscall_fallthrough_to(unsigned long ip)
    {
    void *new_address = (void *)(ip - 4096);
    void *ret;
    printf("[RUN]\tTrying a SYSCALL that falls through to 0x%lx\n", ip);
    ret = mremap((void *)current_test_page_addr, 4096, 4096,
    MREMAP_MAYMOVE | MREMAP_FIXED, new_address);
    if (ret == MAP_FAILED) {
    if (ip <= (1UL << 47) - PAGE_SIZE) {
    err(1, "mremap to %p", new_address);
    } else {
    printf("[OK]\tmremap to %p failed\n", new_address);
    return;
    }
    }
    if (ret != new_address)
    errx(1, "mremap malfunctioned: asked for %p but got %p\n",
    new_address, ret);
    current_test_page_addr = new_address;
    rip = ip;
    if (sigsetjmp(jmpbuf, 1) == 0) {
    asm volatile ("call *%[syscall_insn]" :: "a" (SYS_getpid),
    [syscall_insn] "rm" (ip - 2));
    errx(1, "[FAIL]\tSyscall trampoline returned");
    }
    printf("[OK]\tWe survived\n");
    }
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
//
// When the kernel returns from a slow-path syscall, it will
// detect whether SYSRET is appropriate.  If it incorrectly
// thinks that SYSRET is appropriate when RIP is noncanonical,
// it'll crash on Intel CPUs.
//
    sethandler(SIGUSR1, sigusr1, 0);
    for (int i = 47; i < 64; i++)
    test_sigreturn_to(1UL<<i);
    clearhandler(SIGUSR1);
    sethandler(SIGSEGV, sigsegv_for_fallthrough, 0);
// One extra test to check that we didn't screw up the mremap logic.
    test_syscall_fallthrough_to((1UL << 47) - 2*PAGE_SIZE);
// These are the interesting cases.
    for (int i = 47; i < 64; i++) {
    test_syscall_fallthrough_to((1UL<<i) - PAGE_SIZE);
    test_syscall_fallthrough_to(1UL<<i);
    }
    return 0;
    }
