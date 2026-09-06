//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_global_func5.c
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
// Copyright (c) 2020 Facebook

    __attribute__ ((noinline))
#[no_mangle]
pub unsafe extern "C" fn f1(skb: *mut __sk_buff) -> c_int {
    int f1(struct __sk_buff *skb)
    {
    return skb.len;
    }
    int f3(int, struct __sk_buff *skb);
    __attribute__ ((noinline))
#[no_mangle]
pub unsafe extern "C" fn f2(val: c_int, skb: *mut __sk_buff) -> c_int {
    int f2(int val, struct __sk_buff *skb)
    {
    return f1(skb) + f3(val, (void *)&val); /* type mismatch */
    }
    __attribute__ ((noinline))
#[no_mangle]
pub unsafe extern "C" fn f3(val: c_int, skb: *mut __sk_buff) -> c_int {
    int f3(int val, struct __sk_buff *skb)
    {
    return skb.ifindex * val;
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn __msg(ctx": "expects pointer to) -> __failure {
    __failure __msg("expects pointer to ctx")
#[no_mangle]
pub unsafe extern "C" fn global_func5(skb: *mut __sk_buff) -> c_int {
    int global_func5(struct __sk_buff *skb)
    {
    return f1(skb) + f2(2, skb) + f3(3, skb);
    }
