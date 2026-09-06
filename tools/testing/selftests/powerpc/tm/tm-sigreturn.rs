//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/powerpc/tm/tm-sigreturn.c
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
// Copyright 2015, Laurent Dufour, IBM Corp.
//
// Test the kernel's signal returning code to check reclaim is done if the
// sigreturn() is called while in a transaction (suspended since active is
// already dropped trough the system call path).
//
// The kernel must discard the transaction when entering sigreturn, since
// restoring the potential TM SPRS from the signal frame is requiring to not be
// in a transaction.
//

#[no_mangle]
pub unsafe extern "C" fn handler(sig: c_int) {
    void handler(int sig)
    {
    uint64_t ret;
    asm __volatile__(
    "li             3,1             ;"
    "tbegin.                        ;"
    "beq            1f              ;"
    "li             3,0             ;"
    "tsuspend.                      ;"
    "1:                             ;"
    "std%X[ret]     3, %[ret]       ;"
    : [ret] "=m"(ret)
    :
    : "memory", "3", "cr0");
    if (ret)
    exit(1);
//
// We return from the signal handle while in a suspended transaction
//
    }
#[no_mangle]
pub unsafe extern "C" fn tm_sigreturn() -> c_int {
    int tm_sigreturn(void)
    {
    struct sigaction sa;
    let mut ret: u64 = 0;
    SKIP_IF(!have_htm());
    SKIP_IF(htm_is_synthetic());
    SKIP_IF(!is_ppc64le());
    memset(&sa, 0, sizeof(sa));
    sa.sa_handler = handler;
    sigemptyset(&sa.sa_mask);
    if (sigaction(SIGSEGV, &sa, core::ptr::null_mut()))
    exit(1);
    asm __volatile__(
    "tbegin.                        ;"
    "beq            1f              ;"
    "li             3,0             ;"
    "std            3,0(3)          ;" /* trigger SEGV */
    "li             3,1             ;"
    "std%X[ret]     3,%[ret]        ;"
    "tend.                          ;"
    "b              2f              ;"
    "1:                             ;"
    "li             3,2             ;"
    "std%X[ret]     3,%[ret]        ;"
    "2:                             ;"
    : [ret] "=m"(ret)
    :
    : "memory", "3", "cr0");
    if (ret != 2)
    exit(1);
    exit(0);
    }
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    return test_harness(tm_sigreturn, "tm_sigreturn");
    }
