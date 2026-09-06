//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/powerpc/tm/tm-fork.c
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
// Edited: Rashmica Gupta, Nov 2015
//
// This test does a fork syscall inside a transaction. Basic sniff test
// to see if we can enter the kernel during a transaction.
//

#[no_mangle]
pub unsafe extern "C" fn test_fork() -> c_int {
    int test_fork(void)
    {
    SKIP_IF(!have_htm());
    SKIP_IF(htm_is_synthetic());
    asm __volatile__(
    "tbegin.;"
    "blt    1f; "
    "li     0, 2;"  /* fork syscall */
    "sc  ;"
    "tend.;"
    "1: ;"
    : : : "memory", "r0");
// If we reach here, we've passed.  Otherwise we've probably crashed
// the kernel
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv[]: *mut c_char) -> c_int {
    int main(int argc, char *argv[])
    {
    return test_harness(test_fork, "tm_fork");
    }
