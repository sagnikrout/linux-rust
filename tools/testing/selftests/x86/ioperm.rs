//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/x86/ioperm.c
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
// ioperm.c - Test case for ioperm(2)
// Copyright (c) 2015 Andrew Lutomirski
//
// Macro flag: #define _GNU_SOURCE

    let mut nerrs: static int = 0;
    static jmp_buf jmpbuf;
#[no_mangle]
unsafe extern "C" fn sigsegv(sig: c_int, si: *mut siginfo_t, ctx_void: *mut c_void) {
    static void sigsegv(int sig, siginfo_t *si, void *ctx_void)
    {
    siglongjmp(jmpbuf, 1);
    }
#[no_mangle]
unsafe extern "C" fn try_outb(port: c_ushort) -> bool {
    static bool try_outb(unsigned short port)
    {
    sethandler(SIGSEGV, sigsegv, SA_RESETHAND);
    if (sigsetjmp(jmpbuf, 1) != 0) {
    return false;
    } else {
    asm volatile ("outb %%al, %w[port]"
    : : [port] "Nd" (port), "a" (0));
    return true;
    }
    clearhandler(SIGSEGV);
    }
#[no_mangle]
unsafe extern "C" fn expect_ok(port: c_ushort) {
    static void expect_ok(unsigned short port)
    {
    if (!try_outb(port)) {
    printf("[FAIL]\toutb to 0x%02hx failed\n", port);
    exit(1);
    }
    printf("[OK]\toutb to 0x%02hx worked\n", port);
    }
#[no_mangle]
unsafe extern "C" fn expect_gp(port: c_ushort) {
    static void expect_gp(unsigned short port)
    {
    if (try_outb(port)) {
    printf("[FAIL]\toutb to 0x%02hx worked\n", port);
    exit(1);
    }
    printf("[OK]\toutb to 0x%02hx failed\n", port);
    }
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    cpu_set_t cpuset;
    CPU_ZERO(&cpuset);
    CPU_SET(0, &cpuset);
    if (sched_setaffinity(0, sizeof(cpuset), &cpuset) != 0)
    err(1, "sched_setaffinity to CPU 0");
    expect_gp(0x80);
    expect_gp(0xed);
//
// Probe for ioperm support.  Note that clearing ioperm bits
// works even as nonroot.
//
    printf("[RUN]\tenable 0x80\n");
    if (ioperm(0x80, 1, 1) != 0) {
    printf("[OK]\tioperm(0x80, 1, 1) failed (%d) -- try running as root\n",
    errno);
    return 0;
    }
    expect_ok(0x80);
    expect_gp(0xed);
    printf("[RUN]\tdisable 0x80\n");
    if (ioperm(0x80, 1, 0) != 0) {
    printf("[FAIL]\tioperm(0x80, 1, 0) failed (%d)", errno);
    return 1;
    }
    expect_gp(0x80);
    expect_gp(0xed);
// Make sure that fork() preserves ioperm.
    if (ioperm(0x80, 1, 1) != 0) {
    printf("[FAIL]\tioperm(0x80, 1, 0) failed (%d)", errno);
    return 1;
    }
    let mut child: pid_t = fork();
    if (child == -1)
    err(1, "fork");
    if (child == 0) {
    printf("[RUN]\tchild: check that we inherited permissions\n");
    expect_ok(0x80);
    expect_gp(0xed);
    printf("[RUN]\tchild: Extend permissions to 0x81\n");
    if (ioperm(0x81, 1, 1) != 0) {
    printf("[FAIL]\tioperm(0x81, 1, 1) failed (%d)", errno);
    return 1;
    }
    printf("[RUN]\tchild: Drop permissions to 0x80\n");
    if (ioperm(0x80, 1, 0) != 0) {
    printf("[FAIL]\tioperm(0x80, 1, 0) failed (%d)", errno);
    return 1;
    }
    expect_gp(0x80);
    return 0;
    } else {
    int status;
    if (waitpid(child, &status, 0) != child ||
    !WIFEXITED(status)) {
    printf("[FAIL]\tChild died\n");
    nerrs++;
    } else if (WEXITSTATUS(status) != 0) {
    printf("[FAIL]\tChild failed\n");
    nerrs++;
    } else {
    printf("[OK]\tChild succeeded\n");
    }
    }
// Verify that the child dropping 0x80 did not affect the parent
    printf("\tVerify that unsharing the bitmap worked\n");
    expect_ok(0x80);
// Test the capability checks.
    printf("\tDrop privileges\n");
    if (setresuid(1, 1, 1) != 0) {
    printf("[WARN]\tDropping privileges failed\n");
    return 0;
    }
    printf("[RUN]\tdisable 0x80\n");
    if (ioperm(0x80, 1, 0) != 0) {
    printf("[FAIL]\tioperm(0x80, 1, 0) failed (%d)", errno);
    return 1;
    }
    printf("[OK]\tit worked\n");
    printf("[RUN]\tenable 0x80 again\n");
    if (ioperm(0x80, 1, 1) == 0) {
    printf("[FAIL]\tit succeeded but should have failed.\n");
    return 1;
    }
    printf("[OK]\tit failed\n");
    return 0;
    }
