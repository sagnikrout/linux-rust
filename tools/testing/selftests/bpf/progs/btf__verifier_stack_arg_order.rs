//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/btf__verifier_stack_arg_order.c
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
// Copyright (c) 2026 Meta Platforms, Inc. and affiliates.

    defined(__BPF_FEATURE_STACK_ARGUMENT)
#[no_mangle]
pub unsafe extern "C" fn subprog_bad_order_6args(a: c_int, b: c_int, c: c_int, d: c_int, e: c_int, f: c_int) -> c_int {
    int subprog_bad_order_6args(int a, int b, int c, int d, int e, int f)
    {
    return a + b + c + d + e + f;
    }
#[no_mangle]
pub unsafe extern "C" fn subprog_call_before_load_6args(a: c_int, b: c_int, c: c_int, d: c_int, e: c_int, f: c_int) -> c_int {
    int subprog_call_before_load_6args(int a, int b, int c, int d, int e, int f)
    {
    return a + b + c + d + e + f;
    }
#[no_mangle]
pub unsafe extern "C" fn subprog_pruning_call_before_load_6args(a: c_int, b: c_int, c: c_int, d: c_int, e: c_int, f: c_int) -> c_int {
    int subprog_pruning_call_before_load_6args(int a, int b, int c, int d, int e, int f)
    {
    return a + b + c + d + e + f;
    }
#[no_mangle]
pub unsafe extern "C" fn subprog_bad_ptr_7args(a: *mut c_long, b: c_int, c: c_int, d: c_int, e: c_int, f: c_int, g: c_int) {
    void subprog_bad_ptr_7args(long *a, int b, int c, int d, int e, int f, int g)
    {
    }

#[no_mangle]
pub unsafe extern "C" fn subprog_bad_order_6args() -> c_int {
    int subprog_bad_order_6args(void)
    {
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn subprog_call_before_load_6args() -> c_int {
    int subprog_call_before_load_6args(void)
    {
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn subprog_pruning_call_before_load_6args() -> c_int {
    int subprog_pruning_call_before_load_6args(void)
    {
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn subprog_bad_ptr_7args() {
    void subprog_bad_ptr_7args(void)
    {
    }
