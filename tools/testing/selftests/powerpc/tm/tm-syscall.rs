//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/powerpc/tm/tm-syscall.c
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
// Copyright 2015, Sam Bobroff, IBM Corp.
//
// Test the kernel's system call code to ensure that a system call
// made from within an active HTM transaction is aborted with the
// correct failure code.
// Conversely, ensure that a system call made from within a
// suspended transaction can succeed.
//

pub const PPC_FEATURE2_SCV: c_uint = 0x00100000 /* scv syscall */;

    extern int getppid_tm_active(void);
    extern int getppid_tm_suspended(void);
    extern int getppid_scv_tm_active(void);
    extern int getppid_scv_tm_suspended(void);
    let mut retries: unsigned = 0;

#[no_mangle]
pub unsafe extern "C" fn getppid_tm(scv: bool, suspend: bool) -> pid_t {
    pid_t getppid_tm(bool scv, bool suspend)
    {
    int i;
    pid_t pid;
    for (i = 0; i < TM_RETRIES; i++) {
    if (suspend) {
    if (scv)
    pid = getppid_scv_tm_suspended();
    else
    pid = getppid_tm_suspended();
    } else {
    if (scv)
    pid = getppid_scv_tm_active();
    else
    pid = getppid_tm_active();
    }
    if (pid >= 0)
    return pid;
    if (failure_is_persistent()) {
    if (failure_is_syscall())
    return -1;
    printf("Unexpected persistent transaction failure.\n");
    printf("TEXASR 0x%016lx, TFIAR 0x%016lx.\n",
    __builtin_get_texasr(), __builtin_get_tfiar());
    exit(-1);
    }
    retries++;
    }
    printf("Exceeded limit of %d temporary transaction failures.\n", TM_RETRIES);
    printf("TEXASR 0x%016lx, TFIAR 0x%016lx.\n",
    __builtin_get_texasr(), __builtin_get_tfiar());
    exit(-1);
    }
#[no_mangle]
pub unsafe extern "C" fn tm_syscall() -> c_int {
    int tm_syscall(void)
    {
    let mut count: unsigned = 0;
    struct timeval end, now;
    SKIP_IF(!have_htm_nosc());
    SKIP_IF(htm_is_synthetic());
    setbuf(stdout, core::ptr::null_mut());
    printf("Testing transactional syscalls for %d seconds...\n", TEST_DURATION);
    gettimeofday(&end, core::ptr::null_mut());
    now.tv_sec = TEST_DURATION;
    now.tv_usec = 0;
    timeradd(&end, &now, &end);
    for (count = 0; timercmp(&now, &end, <); count++) {
//
// Test a syscall within a suspended transaction and verify
// that it succeeds.
//
    FAIL_IF(getppid_tm(false, true) == -1); /* Should succeed. */
//
// Test a syscall within an active transaction and verify that
// it fails with the correct failure code.
//
    FAIL_IF(getppid_tm(false, false) != -1);  /* Should fail... */
    FAIL_IF(!failure_is_persistent()); /* ...persistently... */
    FAIL_IF(!failure_is_syscall());    /* ...with code syscall. */
// Now do it all again with scv if it is available.
    if (have_hwcap2(PPC_FEATURE2_SCV)) {
    FAIL_IF(getppid_tm(true, true) == -1); /* Should succeed. */
    FAIL_IF(getppid_tm(true, false) != -1);  /* Should fail... */
    FAIL_IF(!failure_is_persistent()); /* ...persistently... */
    FAIL_IF(!failure_is_syscall());    /* ...with code syscall. */
    }
    gettimeofday(&now, 0);
    }
    printf("%d active and suspended transactions behaved correctly.\n", count);
    printf("(There were %d transaction retries.)\n", retries);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    return test_harness(tm_syscall, "tm_syscall");
    }
