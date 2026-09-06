//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_global_func2.c
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

#[no_mangle]
pub unsafe extern "C" fn __attribute__(_arg: (noinline)) -> static {
    static __attribute__ ((noinline))
#[no_mangle]
pub unsafe extern "C" fn f0(var: c_int, skb: *mut __sk_buff) -> c_int {
    int f0(int var, struct __sk_buff *skb)
    {
    return skb.len;
    }
    __attribute__ ((noinline))
#[no_mangle]
pub unsafe extern "C" fn f1(skb: *mut __sk_buff) -> c_int {
    int f1(struct __sk_buff *skb)
    {
    volatile char buf[MAX_STACK] = {};
    __sink(buf[MAX_STACK - 1]);
    return f0(0, skb) + skb.len;
    }
    int f3(int, struct __sk_buff *skb, int);
    __attribute__ ((noinline))
#[no_mangle]
pub unsafe extern "C" fn f2(val: c_int, skb: *mut __sk_buff) -> c_int {
    int f2(int val, struct __sk_buff *skb)
    {
    return f1(skb) + f3(val, skb, 1);
    }
    __attribute__ ((noinline))
#[no_mangle]
pub unsafe extern "C" fn f3(val: c_int, skb: *mut __sk_buff, var: c_int) -> c_int {
    int f3(int val, struct __sk_buff *skb, int var)
    {
    volatile char buf[MAX_STACK] = {};
    __sink(buf[MAX_STACK - 1]);
    return skb.ifindex * val * var;
    }
    SEC("tc")
    __success
#[no_mangle]
pub unsafe extern "C" fn global_func2(skb: *mut __sk_buff) -> c_int {
    int global_func2(struct __sk_buff *skb)
    {
    return f0(1, skb) + f1(skb) + f2(2, skb) + f3(3, skb, 4);
    }
