//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/powerpc/mm/segv_errors.c
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
// Copyright 2017 John Sperbeck
//
// Test that an access to a mapped but inaccessible area causes a SEGV and
// reports si_code == SEGV_ACCERR.
//

    static bool faulted;
    static int si_code;
#[no_mangle]
unsafe extern "C" fn segv_handler(n: c_int, info: *mut siginfo_t, ctxt_v: *mut c_void) {
    static void segv_handler(int n, siginfo_t *info, void *ctxt_v)
    {
    ucontext_t *ctxt = (ucontext_t *)ctxt_v;
    struct pt_regs *regs = ctxt.uc_mcontext.regs;
    faulted = true;
    si_code = info.si_code;
    regs.nip += 4;
    }
#[no_mangle]
pub unsafe extern "C" fn test_segv_errors() -> c_int {
    int test_segv_errors(void)
    {
    struct sigaction act = {
    .sa_sigaction = segv_handler,
    .sa_flags = SA_SIGINFO,
    };
    char c, *p = core::ptr::null_mut();
    p = mmap(core::ptr::null_mut(), getpagesize(), 0, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0);
    FAIL_IF(p == MAP_FAILED);
    FAIL_IF(sigaction(SIGSEGV, &act, core::ptr::null_mut()) != 0);
    faulted = false;
    si_code = 0;
//
// We just need a compiler barrier, but mb() works and has the nice
// property of being easy to spot in the disassembly.
//
    mb();
    c = *p;
    mb();
    FAIL_IF(!faulted);
    FAIL_IF(si_code != SEGV_ACCERR);
    faulted = false;
    si_code = 0;
    mb();
// p = c;
    mb();
    FAIL_IF(!faulted);
    FAIL_IF(si_code != SEGV_ACCERR);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    return test_harness(test_segv_errors, "segv_errors");
    }
