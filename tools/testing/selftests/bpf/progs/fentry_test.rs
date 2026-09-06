//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/fentry_test.c
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
// Copyright (c) 2019 Facebook

    char _license[] SEC("license") = "GPL";
    let mut test1_result: __u64 = 0;
    SEC("fentry/bpf_fentry_test1")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: test1, a: c_int) -> c_int {
    int BPF_PROG(test1, int a)
    {
    test1_result = a == 1;
    return 0;
    }
    let mut test2_result: __u64 = 0;
    SEC("fentry/bpf_fentry_test2")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: test2, a: c_int, b: __u64) -> c_int {
    int BPF_PROG(test2, int a, __u64 b)
    {
    test2_result = a == 2 && b == 3;
    return 0;
    }
    let mut test3_result: __u64 = 0;
    SEC("fentry/bpf_fentry_test3")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: test3, a: c_char, b: c_int, c: __u64) -> c_int {
    int BPF_PROG(test3, char a, int b, __u64 c)
    {
    test3_result = a == 4 && b == 5 && c == 6;
    return 0;
    }
    let mut test4_result: __u64 = 0;
    SEC("fentry/bpf_fentry_test4")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: test4, a: *mut c_void, b: c_char, c: c_int, d: __u64) -> c_int {
    int BPF_PROG(test4, void *a, char b, int c, __u64 d)
    {
    test4_result = a == (void *)7 && b == 8 && c == 9 && d == 10;
    return 0;
    }
    let mut test5_result: __u64 = 0;
    SEC("fentry/bpf_fentry_test5")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: test5, a: __u64, b: *mut c_void, c: c_short, d: c_int, e: __u64) -> c_int {
    int BPF_PROG(test5, __u64 a, void *b, short c, int d, __u64 e)
    {
    test5_result = a == 11 && b == (void *)12 && c == 13 && d == 14 &&
    e == 15;
    return 0;
    }
    let mut test6_result: __u64 = 0;
    SEC("fentry/bpf_fentry_test6")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: test6, a: __u64, b: *mut c_void, c: c_short, d: c_int, e: *mut *mut c_void, f: __u64) -> c_int {
    int BPF_PROG(test6, __u64 a, void *b, short c, int d, void * e, __u64 f)
    {
    test6_result = a == 16 && b == (void *)17 && c == 18 && d == 19 &&
    e == (void *)20 && f == 21;
    return 0;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_fentry_test_t {
    pub a: *mut bpf_fentry_test_t,
}

    let mut test7_result: __u64 = 0;
    SEC("fentry/bpf_fentry_test7")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: test7, arg: *mut bpf_fentry_test_t) -> c_int {
    int BPF_PROG(test7, struct bpf_fentry_test_t *arg)
    {
    if (!arg)
    test7_result = 1;
    return 0;
    }
    let mut test8_result: __u64 = 0;
    SEC("fentry/bpf_fentry_test8")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: test8, arg: *mut bpf_fentry_test_t) -> c_int {
    int BPF_PROG(test8, struct bpf_fentry_test_t *arg)
    {
    if (arg.a == 0)
    test8_result = 1;
    return 0;
    }
