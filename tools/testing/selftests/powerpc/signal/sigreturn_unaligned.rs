//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/powerpc/signal/sigreturn_unaligned.c
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
// Test sigreturn to an unaligned address, ie. low 2 bits set.
// Nothing bad should happen.
// This was able to trigger warnings with CONFIG_PPC_RFI_SRR_DEBUG=y.
//

#[no_mangle]
unsafe extern "C" fn sigusr1_handler(signo: c_int, info: *mut siginfo_t, ptr: *mut c_void) {
    static void sigusr1_handler(int signo, siginfo_t *info, void *ptr)
    {
    ucontext_t *uc = ptr;
    UCONTEXT_NIA(uc) |= 3;
    }
#[no_mangle]
unsafe extern "C" fn test_sigreturn_unaligned() -> c_int {
    static int test_sigreturn_unaligned(void)
    {
    struct sigaction action;
    memset(&action, 0, sizeof(action));
    action.sa_sigaction = sigusr1_handler;
    action.sa_flags = SA_SIGINFO;
    FAIL_IF(sigaction(SIGUSR1, &action, core::ptr::null_mut()) == -1);
    raise(SIGUSR1);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    return test_harness(test_sigreturn_unaligned, "sigreturn_unaligned");
    }
