//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_global_func_deep_stack.c
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
// Copyright (c) 2026 Meta Platforms, Inc and affiliates.

//
// Macro tricks to tersely define for long non-recursive call chains. Add
// computation to the functions prevent tail recursion from reducing the
// stack size to 0.
//

    __attribute__((noinline))         \
    int f0(unsigned long a)           \
    {                                 \
    volatile long b = a + 16; \
    if (a == 0)               \
    return 0;         \
    return b;                 \
    }

    __attribute__((noinline))                        \
    int XCAT(f, n)(unsigned long a)                  \
    {                                                \
    volatile long b = XCAT(f, prev)(a - 1);  \
    if (!b)                                  \
    return 0;                        \
    return b + 1;                            \
    }
// Call chain 33 levels deep.

    F(32)
// Ensure that even 32 levels deep, the function verifies.
    SEC("syscall")
    __success
#[no_mangle]
pub unsafe extern "C" fn global_func_deep_stack_success(skb: *mut __sk_buff) -> c_int {
    int global_func_deep_stack_success(struct __sk_buff *skb)
    {
    return f31(55);
    }
//
// Check we actually honor stack limits (33 * 16 = 528 > 512 = MAX_STACK_DEPTH).
// The stack depth is 16 because the verifier calls round_up_stack_depth() on
// the size.
//
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn __msg(calls": "combined stack size of 34) -> __failure {
    __failure __msg("combined stack size of 34 calls")
#[no_mangle]
pub unsafe extern "C" fn global_func_deep_stack_fail(skb: *mut __sk_buff) -> c_int {
    int global_func_deep_stack_fail(struct __sk_buff *skb)
    {
    return f32(123);
    }
