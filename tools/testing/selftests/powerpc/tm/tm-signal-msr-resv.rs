//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/powerpc/tm/tm-signal-msr-resv.c
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
// Test the kernel's signal return code to ensure that it doesn't
// crash when both the transactional and suspend MSR bits are set in
// the signal context.
//
// For this test, we send ourselves a SIGUSR1.  In the SIGUSR1 handler
// we modify the signal context to set both MSR TM S and T bits (which
// is "reserved" by the PowerISA). When we return from the signal
// handler (implicit sigreturn), the kernel should detect reserved MSR
// value and send us with a SIGSEGV.
//

    let mut segv_expected: c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn signal_segv(signum: c_int) {
    void signal_segv(int signum)
    {
    if (segv_expected && (signum == SIGSEGV))
    _exit(0);
    _exit(1);
    }
#[no_mangle]
pub unsafe extern "C" fn signal_usr1(signum: c_int, info: *mut siginfo_t, uc: *mut c_void) {
    void signal_usr1(int signum, siginfo_t *info, void *uc)
    {
    ucontext_t *ucp = uc;
// Link tm checkpointed context to normal context
    ucp.uc_link = ucp;
// Set all TM bits so that the context is now invalid

    ucp.uc_mcontext.gp_regs[PT_MSR] |= (7ULL << 32);

    ucp.uc_mcontext.uc_regs.gregs[PT_MSR] |= (7ULL);

// Should segv on return becuase of invalid context
    segv_expected = 1;
    }
#[no_mangle]
pub unsafe extern "C" fn tm_signal_msr_resv() -> c_int {
    int tm_signal_msr_resv()
    {
    struct sigaction act;
    SKIP_IF(!have_htm());
    act.sa_sigaction = signal_usr1;
    sigemptyset(&act.sa_mask);
    act.sa_flags = SA_SIGINFO;
    if (sigaction(SIGUSR1, &act, core::ptr::null_mut()) < 0) {
    perror("sigaction sigusr1");
    exit(1);
    }
    if (signal(SIGSEGV, signal_segv) == SIG_ERR)
    exit(1);
    raise(SIGUSR1);
// We shouldn't get here as we exit in the segv handler
    return 1;
    }
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    return test_harness(tm_signal_msr_resv, "tm_signal_msr_resv");
    }
