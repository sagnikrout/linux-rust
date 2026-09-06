//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/x86/sigaltstack.c
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
// Macro flag: #define _GNU_SOURCE

// sigaltstack()-enforced minimum stack
pub const ENFORCED_MINSIGSTKSZ: c_int = 2048;

    static int nerrs;
    static bool sigalrm_expected;
    static unsigned long at_minstack_size;
#[no_mangle]
unsafe extern "C" fn setup_altstack(start: *mut c_void, size: c_ulong) -> c_int {
    static int setup_altstack(void *start, unsigned long size)
    {
    stack_t ss;
    memset(&ss, 0, sizeof(ss));
    ss.ss_size = size;
    ss.ss_sp = start;
    return sigaltstack(&ss, core::ptr::null_mut());
    }
    static jmp_buf jmpbuf;
#[no_mangle]
unsafe extern "C" fn sigsegv(sig: c_int, info: *mut siginfo_t, ctx_void: *mut c_void) {
    static void sigsegv(int sig, siginfo_t *info, void *ctx_void)
    {
    if (sigalrm_expected) {
    printf("[FAIL]\tWrong signal delivered: SIGSEGV (expected SIGALRM).");
    nerrs++;
    } else {
    printf("[OK]\tSIGSEGV signal delivered.\n");
    }
    siglongjmp(jmpbuf, 1);
    }
#[no_mangle]
unsafe extern "C" fn sigalrm(sig: c_int, info: *mut siginfo_t, ctx_void: *mut c_void) {
    static void sigalrm(int sig, siginfo_t *info, void *ctx_void)
    {
    if (!sigalrm_expected) {
    printf("[FAIL]\tWrong signal delivered: SIGALRM (expected SIGSEGV).");
    nerrs++;
    } else {
    printf("[OK]\tSIGALRM signal delivered.\n");
    }
    }
#[no_mangle]
unsafe extern "C" fn test_sigaltstack(altstack: *mut c_void, size: c_ulong) {
    static void test_sigaltstack(void *altstack, unsigned long size)
    {
    if (setup_altstack(altstack, size))
    err(1, "sigaltstack()");
    sigalrm_expected = (size > at_minstack_size) ? true : false;
    sethandler(SIGSEGV, sigsegv, 0);
    sethandler(SIGALRM, sigalrm, SA_ONSTACK);
    if (!sigsetjmp(jmpbuf, 1)) {
    printf("[RUN]\tTest an alternate signal stack of %ssufficient size.\n",
    sigalrm_expected ? "" : "in");
    printf("\tRaise SIGALRM. %s is expected to be delivered.\n",
    sigalrm_expected ? "It" : "SIGSEGV");
    raise(SIGALRM);
    }
    clearhandler(SIGALRM);
    clearhandler(SIGSEGV);
    }
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    void *altstack;
    at_minstack_size = getauxval(AT_MINSIGSTKSZ);
    altstack = mmap(core::ptr::null_mut(), at_minstack_size + SIGSTKSZ, PROT_READ | PROT_WRITE,
    MAP_PRIVATE | MAP_ANONYMOUS | MAP_STACK, -1, 0);
    if (altstack == MAP_FAILED)
    err(1, "mmap()");
    if ((ENFORCED_MINSIGSTKSZ + 1) < at_minstack_size)
    test_sigaltstack(altstack, ENFORCED_MINSIGSTKSZ + 1);
    test_sigaltstack(altstack, at_minstack_size + SIGSTKSZ);
    let mut nerrs: return = = 0 ? 0 : 1;
    }
