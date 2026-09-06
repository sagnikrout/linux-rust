//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/powerpc/tm/tm-exec.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright 2016, Cyril Bur, IBM Corp.
//
// Syscalls can be performed provided the transactions are suspended.
// The exec() class of syscall is unique as a new process is loaded.
//
// It makes little sense for after an exec() call for the previously
// suspended transaction to still exist.
//
// Macro flag: #define _GNU_SOURCE

    static char *path;
#[no_mangle]
unsafe extern "C" fn test_exec() -> c_int {
    static int test_exec(void)
    {
    SKIP_IF(!have_htm());
    SKIP_IF(htm_is_synthetic());
    asm __volatile__(
    "tbegin.;"
    "blt    1f; "
    "tsuspend.;"
    "1: ;"
    : : : "memory");
    execl(path, "tm-exec", "--child", core::ptr::null_mut());
// Shouldn't get here
    perror("execl() failed");
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn after_exec() -> c_int {
    static int after_exec(void)
    {
    asm __volatile__(
    "tbegin.;"
    "blt    1f;"
    "tsuspend.;"
    "1: ;"
    : : : "memory");
    FAIL_IF(failure_is_nesting());
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv[]: *mut c_char) -> c_int {
    int main(int argc, char *argv[])
    {
    path = argv[0];
    if (argc > 1 && strcmp(argv[1], "--child") == 0)
    return after_exec();
    return test_harness(test_exec, "tm_exec");
    }
