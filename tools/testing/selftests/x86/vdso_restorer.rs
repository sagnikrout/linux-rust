//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/x86/vdso_restorer.c
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
// vdso_restorer.c - tests vDSO-based signal restore
// Copyright (c) 2015 Andrew Lutomirski
//
// This makes sure that sa_restorer == NULL keeps working on 32-bit
// configurations.  Modern glibc doesn't use it under any circumstances,
// so it's easy to overlook breakage.
//
// 64-bit userspace has never supported sa_restorer == NULL, so this is
// 32-bit only.
//
// Macro flag: #define _GNU_SOURCE

// Open-code this -- the headers are too messy to easily use them.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct real_sigaction {
    pub handler: *mut c_void,
    pub flags: c_ulong,
    pub restorer: *mut c_void,
    pub mask: [c_uint; 2],
}

    static volatile sig_atomic_t handler_called;
#[no_mangle]
unsafe extern "C" fn handler_with_siginfo(sig: c_int, info: *mut siginfo_t, ctx_void: *mut c_void) {
    static void handler_with_siginfo(int sig, siginfo_t *info, void *ctx_void)
    {
    handler_called = 1;
    }
#[no_mangle]
unsafe extern "C" fn handler_without_siginfo(sig: c_int) {
    static void handler_without_siginfo(int sig)
    {
    handler_called = 1;
    }
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main()
    {
    let mut nerrs: c_int = 0;
    struct real_sigaction sa;
    void *vdso = dlopen("linux-vdso.so.1",
    RTLD_LAZY | RTLD_LOCAL | RTLD_NOLOAD);
    if (!vdso)
    vdso = dlopen("linux-gate.so.1",
    RTLD_LAZY | RTLD_LOCAL | RTLD_NOLOAD);
    if (!vdso) {
    printf("[SKIP]\tFailed to find vDSO.  Tests are not expected to work.\n");
    return 0;
    }
    memset(&sa, 0, sizeof(sa));
    sa.handler = handler_with_siginfo;
    sa.flags = SA_SIGINFO;
    sa.restorer = core::ptr::null_mut();	/* request kernel-provided restorer */
    printf("[RUN]\tRaise a signal, SA_SIGINFO, sa.restorer == core::ptr::null_mut()\n");
    if (syscall(SYS_rt_sigaction, SIGUSR1, &sa, core::ptr::null_mut(), 8) != 0)
    err(1, "raw rt_sigaction syscall");
    raise(SIGUSR1);
    if (handler_called) {
    printf("[OK]\tSA_SIGINFO handler returned successfully\n");
    } else {
    printf("[FAIL]\tSA_SIGINFO handler was not called\n");
    nerrs++;
    }
    printf("[RUN]\tRaise a signal, !SA_SIGINFO, sa.restorer == core::ptr::null_mut()\n");
    sa.flags = 0;
    sa.handler = handler_without_siginfo;
    if (syscall(SYS_sigaction, SIGUSR1, &sa, 0) != 0)
    err(1, "raw sigaction syscall");
    handler_called = 0;
    raise(SIGUSR1);
    if (handler_called) {
    printf("[OK]\t!SA_SIGINFO handler returned successfully\n");
    } else {
    printf("[FAIL]\t!SA_SIGINFO handler was not called\n");
    nerrs++;
    }
    return nerrs;
    }
