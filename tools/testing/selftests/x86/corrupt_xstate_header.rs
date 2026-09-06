//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/x86/corrupt_xstate_header.c
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
// Corrupt the XSTATE header in a signal frame
//
// Based on analysis and a test case from Thomas Gleixner.
//
// Macro flag: #define _GNU_SOURCE

#[no_mangle]
pub unsafe extern "C" fn xsave_enabled() -> c_int {
    static inline int xsave_enabled(void)
    {
    unsigned int eax, ebx, ecx, edx;
    __cpuid_count(0x1, 0x0, eax, ebx, ecx, edx);
// Is CR4.OSXSAVE enabled ?
    return ecx & (1U << 27);
    }
#[no_mangle]
unsafe extern "C" fn sigusr1(sig: c_int, info: *mut siginfo_t, uc_void: *mut c_void) {
    static void sigusr1(int sig, siginfo_t *info, void *uc_void)
    {
    ucontext_t *uc = uc_void;
    uint8_t *fpstate = (uint8_t *)uc.uc_mcontext.fpregs;
    uint64_t *xfeatures = (uint64_t *)(fpstate + 512);
    printf("\tWreck XSTATE header\n");
// Wreck the first reserved bytes in the header
// (xfeatures + 2) = 0xfffffff;
    }
#[no_mangle]
unsafe extern "C" fn sigsegv(sig: c_int, info: *mut siginfo_t, uc_void: *mut c_void) {
    static void sigsegv(int sig, siginfo_t *info, void *uc_void)
    {
    printf("\tGot SIGSEGV\n");
    }
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    cpu_set_t set;
    sethandler(SIGUSR1, sigusr1, 0);
    sethandler(SIGSEGV, sigsegv, 0);
    if (!xsave_enabled()) {
    printf("[SKIP] CR4.OSXSAVE disabled.\n");
    return 0;
    }
    CPU_ZERO(&set);
    CPU_SET(0, &set);
//
// Enforce that the child runs on the same CPU
// which in turn forces a schedule.
//
    sched_setaffinity(getpid(), sizeof(set), &set);
    printf("[RUN]\tSend ourselves a signal\n");
    raise(SIGUSR1);
    printf("[OK]\tBack from the signal.  Now schedule.\n");
    let mut child: pid_t = fork();
    if (child < 0)
    err(1, "fork");
    if (child == 0)
    return 0;
    if (child)
    waitpid(child, core::ptr::null_mut(), 0);
    printf("[OK]\tBack in the main thread.\n");
//
// We could try to confirm that extended state is still preserved
// when we schedule.  For now, the only indication of failure is
// a warning in the kernel logs.
//
    return 0;
    }
