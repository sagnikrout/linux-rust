//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/powerpc/tm/tm-signal-sigreturn-nt.c
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
// Copyright 2018, Breno Leitao, Gustavo Romero, IBM Corp.
//
// A test case that creates a signal and starts a suspended transaction
// inside the signal handler.
//
// It returns from the signal handler with the CPU at suspended state, but
// without setting usercontext MSR Transaction State (TS) fields.
//
// Macro flag: #define _GNU_SOURCE

#[no_mangle]
pub unsafe extern "C" fn trap_signal_handler(signo: c_int, si: *mut siginfo_t, uc: *mut c_void) {
    void trap_signal_handler(int signo, siginfo_t *si, void *uc)
    {
    ucontext_t *ucp = (ucontext_t *) uc;
    asm("tbegin.; tsuspend.;");
// Skip 'trap' instruction if it succeed
    ucp.uc_mcontext.regs.nip += 4;
    }
#[no_mangle]
pub unsafe extern "C" fn tm_signal_sigreturn_nt() -> c_int {
    int tm_signal_sigreturn_nt(void)
    {
    struct sigaction trap_sa;
    SKIP_IF(!have_htm());
    SKIP_IF(htm_is_synthetic());
    trap_sa.sa_flags = SA_SIGINFO;
    trap_sa.sa_sigaction = trap_signal_handler;
    sigaction(SIGTRAP, &trap_sa, core::ptr::null_mut());
    raise(SIGTRAP);
    return EXIT_SUCCESS;
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> c_int {
    int main(int argc, char **argv)
    {
    return test_harness(tm_signal_sigreturn_nt, "tm_signal_sigreturn_nt");
    }
