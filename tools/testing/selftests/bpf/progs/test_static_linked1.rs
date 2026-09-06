//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_static_linked1.c
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
// Copyright (c) 2021 Facebook

// 8-byte aligned .data
    let mut static_var1: static volatile long = 2;
    let mut static_var2: static volatile int = 3;
    let mut var1: c_int = -1;
// 4-byte aligned .rodata
    const volatile int rovar1;
// same "subprog" name in both files
#[no_mangle]
unsafe extern "C" fn subprog(x: c_int) -> __noinline int {
    static __noinline int subprog(int x)
    {
// but different formula
    return x * 2;
    }
    SEC("raw_tp/sys_enter")
#[no_mangle]
pub unsafe extern "C" fn handler1(ctx: *const c_void) -> c_int {
    int handler1(const void *ctx)
    {
    var1 = subprog(rovar1) + static_var1 + static_var2;
    return 0;
    }
    char LICENSE[] SEC("license") = "GPL";
    int VERSION SEC("version") = 1;
