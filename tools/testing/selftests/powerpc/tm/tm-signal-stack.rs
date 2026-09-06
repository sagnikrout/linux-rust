//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/powerpc/tm/tm-signal-stack.c
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
// Copyright 2015, Michael Neuling, IBM Corp.
//
// Test the kernel's signal delievery code to ensure that we don't
// trelaim twice in the kernel signal delivery code.  This can happen
// if we trigger a signal when in a transaction and the stack pointer
// is bogus.
//
// This test case registers a SEGV handler, sets the stack pointer
// (r1) to NULL, starts a transaction and then generates a SEGV.  The
// SEGV should be handled but we exit here as the stack pointer is
// invalid and hance we can't sigreturn.  We only need to check that
// this flow doesn't crash the kernel.
//

#[no_mangle]
pub unsafe extern "C" fn signal_segv(signum: c_int) {
    void signal_segv(int signum)
    {
// This should never actually run since stack is foobar
    exit(1);
    }
#[no_mangle]
pub unsafe extern "C" fn tm_signal_stack() -> c_int {
    int tm_signal_stack()
    {
    int pid;
    SKIP_IF(!have_htm());
    SKIP_IF(htm_is_synthetic());
    pid = fork();
    if (pid < 0)
    exit(1);
    if (pid) { /* Parent */
//
// It's likely the whole machine will crash here so if
// the child ever exits, we are good.
//
    wait(core::ptr::null_mut());
    return 0;
    }
//
// The flow here is:
// 1) register a signal handler (so signal delievery occurs)
// 2) make stack pointer (r1) = NULL
// 3) start transaction
// 4) cause segv
//
    if (signal(SIGSEGV, signal_segv) == SIG_ERR)
    exit(1);
    asm volatile("li 1, 0 ;"		/* stack ptr == core::ptr::null_mut() */
    "1:"
    "tbegin.;"
    "beq 1b ;"			/* retry forever */
    "tsuspend.;"
    "ld 2, 0(1) ;"		/* trigger segv" */
    : : : "memory");
// This should never get here due to above segv
    return 1;
    }
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    return test_harness(tm_signal_stack, "tm_signal_stack");
    }
