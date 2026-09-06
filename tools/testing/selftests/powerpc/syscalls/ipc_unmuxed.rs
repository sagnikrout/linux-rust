//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/powerpc/syscalls/ipc_unmuxed.c
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
// Copyright 2015, Michael Ellerman, IBM Corp.
//
// This test simply tests that certain syscalls are implemented. It doesn't
// actually exercise their logic in any way.
//
// Macro flag: #define _GNU_SOURCE

    static int test_##_name(void)			\
    {						\
    int rc;					\
    printf("Testing " #_name);		\
    errno = 0;				\
    rc = syscall(_num, -1, 0, 0, 0, 0, 0);	\
    printf("\treturned %d, errno %d\n", rc, errno); \
    return errno == ENOSYS;			\
    }

#[no_mangle]
unsafe extern "C" fn ipc_unmuxed() -> c_int {
    static int ipc_unmuxed(void)
    {
    let mut tests_done: c_int = 0;

    FAIL_IF(test_##_name());	\
    tests_done++;

//
// If we ran no tests then it means none of the syscall numbers were
// defined, possibly because we were built against old headers. But it
// means we didn't really test anything, so instead of passing mark it
// as a skip to give the user a clue.
//
    SKIP_IF(tests_done == 0);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    return test_harness(ipc_unmuxed, "ipc_unmuxed");
    }
