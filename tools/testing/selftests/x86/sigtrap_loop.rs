//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/x86/sigtrap_loop.c
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
// Copyright (C) 2025 Intel Corporation
//
// Macro flag: #define _GNU_SOURCE

#[no_mangle]
unsafe extern "C" fn sethandler(sig: c_int, (*handler)(int: *mut c_void, : *mut siginfo_t, ): *mut c_void, flags: c_int) {
    static void sethandler(int sig, void (*handler)(int, siginfo_t *, void *), int flags)
    {
    struct sigaction sa;
    memset(&sa, 0, sizeof(sa));
    sa.sa_sigaction = handler;
    sa.sa_flags = SA_SIGINFO | flags;
    sigemptyset(&sa.sa_mask);
    if (sigaction(sig, &sa, 0))
    err(1, "sigaction");
    return;
    }
#[no_mangle]
unsafe extern "C" fn sigtrap(sig: c_int, info: *mut siginfo_t, ctx_void: *mut c_void) {
    static void sigtrap(int sig, siginfo_t *info, void *ctx_void)
    {
    ucontext_t *ctx = (ucontext_t *)ctx_void;
    static unsigned int loop_count_on_same_ip;
    static unsigned long last_trap_ip;
    if (last_trap_ip == ctx.uc_mcontext.gregs[REG_IP]) {
    printf("\tTrapped at %016lx\n", last_trap_ip);
//
// If the same IP is hit more than 10 times in a row, it is
// _considered_ an infinite loop.
//
    if (++loop_count_on_same_ip > 10) {
    printf("[FAIL]\tDetected SIGTRAP infinite loop\n");
    exit(1);
    }
    return;
    }
    loop_count_on_same_ip = 0;
    last_trap_ip = ctx.uc_mcontext.gregs[REG_IP];
    printf("\tTrapped at %016lx\n", last_trap_ip);
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv[]: *mut c_char) -> c_int {
    int main(int argc, char *argv[])
    {
    sethandler(SIGTRAP, sigtrap, 0);
//
// Set the Trap Flag (TF) to single-step the test code, therefore to
// trigger a SIGTRAP signal after each instruction until the TF is
// cleared.
//
// Because the arithmetic flags are not significant here, the TF is
// set by pushing 0x302 onto the stack and then popping it into the
// flags register.
//
// Four instructions in the following asm code are executed with the
// TF set, thus the SIGTRAP handler is expected to run four times.
//
    printf("[RUN]\tSIGTRAP infinite loop detection\n");
    asm volatile(

//
// Avoid clobbering the redzone
//
// Equivalent to "sub $128, %rsp", however -128 can be encoded
// in a single byte immediate while 128 uses 4 bytes.
//
    "add $-128, %rsp\n\t"

    "push $0x302\n\t"
    "popf\n\t"
    "nop\n\t"
    "nop\n\t"
    "push $0x202\n\t"
    "popf\n\t"

    "sub $-128, %rsp\n\t"

    );
    printf("[OK]\tNo SIGTRAP infinite loop detected\n");
    return 0;
    }
