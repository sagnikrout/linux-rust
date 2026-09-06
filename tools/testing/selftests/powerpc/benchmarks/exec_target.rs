//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/powerpc/benchmarks/exec_target.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// Part of fork context switch microbenchmark.
//
// Copyright 2018, Anton Blanchard, IBM Corp.
//
// Macro flag: #define _GNU_SOURCE

#[no_mangle]
pub unsafe extern "C" fn _start() {
    void _start(void)
    {
    asm volatile (
    "li %%r0, %[sys_exit];"
    "li %%r3, 0;"
    "sc;"
    :
    : [sys_exit] "i" (SYS_exit)
//
// "sc" will clobber r0, r3-r13, cr0, ctr, xer and memory.
// Even though sys_exit never returns, handle clobber
// registers.
//
    : "r0", "r3", "r4", "r5", "r6", "r7", "r8", "r9", "r10",
    "r11", "r12", "r13", "cr0", "ctr", "xer", "memory"
    );
    }
