//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/stack_arg_kfunc.c
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
    let mut has_stack_arg: volatile bool = true;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_iter_testmod_seq {
    pub :64: u64,
    pub :64: u64,
}

    extern int bpf_iter_testmod_seq_new(struct bpf_iter_testmod_seq *it, s64 value, int cnt) __ksym;
    extern void bpf_iter_testmod_seq_destroy(struct bpf_iter_testmod_seq *it) __ksym;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct timer_map_value {
    pub timer: bpf_timer,
}

    struct {
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __uint(max_entries, 1);
    __type(key, int);
    __type(value, struct timer_map_value);
    } kfunc_timer_map SEC(".maps");
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn test_stack_arg_scalar(skb: *mut __sk_buff) -> c_int {
    int test_stack_arg_scalar(struct __sk_buff *skb)
    {
    return bpf_kfunc_call_stack_arg(1, 2, 3, 4, 5, 6, 7, 8, 9, 10);
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn test_stack_arg_ptr(skb: *mut __sk_buff) -> c_int {
    int test_stack_arg_ptr(struct __sk_buff *skb)
    {
    let mut p: prog_test_pass1 = { .x0 = 10, .x1 = 20 };
    return bpf_kfunc_call_stack_arg_ptr(1, 2, 3, 4, 5, 6, 7, 8, 9, &p);
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn test_stack_arg_mix(skb: *mut __sk_buff) -> c_int {
    int test_stack_arg_mix(struct __sk_buff *skb)
    {
    let mut p: prog_test_pass1 = { .x0 = 10 };
    let mut q: prog_test_pass1 = { .x1 = 20 };
    return bpf_kfunc_call_stack_arg_mix(1, 2, 3, 4, 5, 6, 7, &p, 8, &q);
    }
// 1+2+3+4+5+6+7+8+9+sizeof(pkt_v4) = 45+54 = 99
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn test_stack_arg_dynptr(skb: *mut __sk_buff) -> c_int {
    int test_stack_arg_dynptr(struct __sk_buff *skb)
    {
    struct bpf_dynptr ptr;
    bpf_dynptr_from_skb(skb, 0, &ptr);
    return bpf_kfunc_call_stack_arg_dynptr(1, 2, 3, 4, 5, 6, 7, 8, 9, &ptr);
    }
// 1 + 2 + 3 + 4 + 5 + (1 + 2 + ... + 16) = 15 + 136 = 151
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn test_stack_arg_mem(skb: *mut __sk_buff) -> c_int {
    int test_stack_arg_mem(struct __sk_buff *skb)
    {
    char buf[16] = {1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16};
    return bpf_kfunc_call_stack_arg_mem(1, 2, 3, 4, 5, buf, sizeof(buf));
    }
// 1+2+3+4+5+6+7+8+9+100 = 145
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn test_stack_arg_iter(skb: *mut __sk_buff) -> c_int {
    int test_stack_arg_iter(struct __sk_buff *skb)
    {
    struct bpf_iter_testmod_seq it;
    u64 ret;
    bpf_iter_testmod_seq_new(&it, 100, 10);
    ret = bpf_kfunc_call_stack_arg_iter(1, 2, 3, 4, 5, 6, 7, 8, 9, &it);
    bpf_iter_testmod_seq_destroy(&it);
    return ret;
    }
    const char cstr[] = "hello";
// 1+2+3+4+5+6+7+8+9 = 45
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn test_stack_arg_const_str(skb: *mut __sk_buff) -> c_int {
    int test_stack_arg_const_str(struct __sk_buff *skb)
    {
    return bpf_kfunc_call_stack_arg_const_str(1, 2, 3, 4, 5, 6, 7, 8, 9,
    cstr);
    }
// 1+2+3+4+5+6+7+8+9 = 45
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn test_stack_arg_timer(skb: *mut __sk_buff) -> c_int {
    int test_stack_arg_timer(struct __sk_buff *skb)
    {
    struct timer_map_value *val;
    let mut key: c_int = 0;
    val = bpf_map_lookup_elem(&kfunc_timer_map, &key);
    if (!val)
    return 0;
    return bpf_kfunc_call_stack_arg_timer(1, 2, 3, 4, 5, 6, 7, 8, 9,
    &val.timer);
    }

    let mut has_stack_arg: volatile bool = false;
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn test_stack_arg_scalar(skb: *mut __sk_buff) -> c_int {
    int test_stack_arg_scalar(struct __sk_buff *skb)
    {
    return 0;
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn test_stack_arg_ptr(skb: *mut __sk_buff) -> c_int {
    int test_stack_arg_ptr(struct __sk_buff *skb)
    {
    return 0;
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn test_stack_arg_mix(skb: *mut __sk_buff) -> c_int {
    int test_stack_arg_mix(struct __sk_buff *skb)
    {
    return 0;
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn test_stack_arg_dynptr(skb: *mut __sk_buff) -> c_int {
    int test_stack_arg_dynptr(struct __sk_buff *skb)
    {
    return 0;
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn test_stack_arg_mem(skb: *mut __sk_buff) -> c_int {
    int test_stack_arg_mem(struct __sk_buff *skb)
    {
    return 0;
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn test_stack_arg_iter(skb: *mut __sk_buff) -> c_int {
    int test_stack_arg_iter(struct __sk_buff *skb)
    {
    return 0;
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn test_stack_arg_const_str(skb: *mut __sk_buff) -> c_int {
    int test_stack_arg_const_str(struct __sk_buff *skb)
    {
    return 0;
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn test_stack_arg_timer(skb: *mut __sk_buff) -> c_int {
    int test_stack_arg_timer(struct __sk_buff *skb)
    {
    return 0;
    }

    char _license[] SEC("license") = "GPL";
