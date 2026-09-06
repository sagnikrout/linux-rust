//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/stack_arg.c
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

pub const CLOCK_MONOTONIC: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct timer_elem {
    pub timer: bpf_timer,
}

    struct {
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __uint(max_entries, 1);
    __type(key, int);
    __type(value, struct timer_elem);
    } timer_map SEC(".maps");
    int timer_result;

    defined(__BPF_FEATURE_STACK_ARGUMENT)
    let mut has_stack_arg: volatile bool = true;
    __noinline static int static_func_many_args(int a, int b, int c, int d,
    int e, int f, int g, int h,
    int i, int j)
    {
    return a + b + c + d + e + f + g + h + i + j;
    }
#[no_mangle]
pub unsafe extern "C" fn global_calls_many_args(a: c_int, b: c_int, c: c_int) -> __noinline int {
    __noinline int global_calls_many_args(int a, int b, int c)
    {
    return static_func_many_args(a, b, c, a + 3, a + 4, a + 5, a + 6,
    a + 7, a + 8, a + 9);
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn test_global_many_args() -> c_int {
    int test_global_many_args(void)
    {
    return global_calls_many_args(1, 2, 3);
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct test_data {
    pub x: c_long,
    pub y: c_long,
}

// 1+2+3+4+5+6+7+8+9+10+20 = 75
    __noinline static long func_with_ptr_stack_arg(long a, long b, long c, long d,
    long e, long f, long g, long h,
    long i, struct test_data *p)
    {
    return a + b + c + d + e + f + g + h + i + p.x + p.y;
    }
#[no_mangle]
pub unsafe extern "C" fn global_ptr_stack_arg(a: c_long, b: c_long, c: c_long, d: c_long, e: c_long) -> __noinline long {
    __noinline long global_ptr_stack_arg(long a, long b, long c, long d, long e)
    {
    let mut data: test_data = { .x = 10, .y = 20 };
    return func_with_ptr_stack_arg(a, b, c, d, e, a + 5, a + 6, a + 7,
    a + 8, &data);
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn test_bpf2bpf_ptr_stack_arg() -> c_int {
    int test_bpf2bpf_ptr_stack_arg(void)
    {
    return global_ptr_stack_arg(1, 2, 3, 4, 5);
    }
// 1+2+3+4+5+6+7+10+8+20 = 66
    __noinline static long func_with_mix_stack_args(long a, long b, long c, long d,
    long e, long f, long g,
    struct test_data *p,
    long h, struct test_data *q)
    {
    return a + b + c + d + e + f + g + p.x + h + q.y;
    }
#[no_mangle]
pub unsafe extern "C" fn global_mix_stack_args(a: c_long, b: c_long, c: c_long, d: c_long, e: c_long) -> __noinline long {
    __noinline long global_mix_stack_args(long a, long b, long c, long d, long e)
    {
    let mut p: test_data = { .x = 10 };
    let mut q: test_data = { .y = 20 };
    return func_with_mix_stack_args(a, b, c, d, e, e + 1, e + 2, &p,
    e + 3, &q);
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn test_bpf2bpf_mix_stack_args() -> c_int {
    int test_bpf2bpf_mix_stack_args(void)
    {
    return global_mix_stack_args(1, 2, 3, 4, 5);
    }
//
// Nesting test: func_outer calls func_inner, both with struct pointer
// as stack arg.
//
// func_inner: (a+1)+...+(i+1) + p->x + p->y
// = 2+3+4+5+6+7+8+9+10+10+20 = 84
//
    __noinline static long func_inner_ptr(long a, long b, long c, long d,
    long e, long f, long g, long h,
    long i, struct test_data *p)
    {
    return a + b + c + d + e + f + g + h + i + p.x + p.y;
    }
    __noinline static long func_outer_ptr(long a, long b, long c, long d,
    long e, long f, long g, long h,
    long i, struct test_data *p)
    {
    return func_inner_ptr(a + 1, b + 1, c + 1, d + 1, e + 1,
    f + 1, g + 1, h + 1, i + 1, p);
    }
#[no_mangle]
pub unsafe extern "C" fn global_nesting_ptr(a: c_long, b: c_long, c: c_long, d: c_long, e: c_long) -> __noinline long {
    __noinline long global_nesting_ptr(long a, long b, long c, long d, long e)
    {
    let mut data: test_data = { .x = 10, .y = 20 };
    return func_outer_ptr(a, b, c, d, e, a + 5, a + 6, a + 7, a + 8,
    &data);
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn test_bpf2bpf_nesting_stack_arg() -> c_int {
    int test_bpf2bpf_nesting_stack_arg(void)
    {
    return global_nesting_ptr(1, 2, 3, 4, 5);
    }
// 1+2+3+4+5+6+7+8+9+sizeof(pkt_v4) = 45+54 = 99
    __noinline static long func_with_dynptr(long a, long b, long c, long d,
    long e, long f, long g, long h,
    long i, struct bpf_dynptr *ptr)
    {
    return a + b + c + d + e + f + g + h + i + bpf_dynptr_size(ptr);
    }
    __noinline long global_dynptr_stack_arg(void *ctx __arg_ctx, long a, long b,
    long c, long d)
    {
    struct bpf_dynptr ptr;
    bpf_dynptr_from_skb(ctx, 0, &ptr);
    return func_with_dynptr(a, b, c, d, d + 1, d + 2, d + 3, d + 4,
    d + 5, &ptr);
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn test_bpf2bpf_dynptr_stack_arg(skb: *mut __sk_buff) -> c_int {
    int test_bpf2bpf_dynptr_stack_arg(struct __sk_buff *skb)
    {
    return global_dynptr_stack_arg(skb, 1, 2, 3, 4);
    }
// foo1: a+b+c+d+e+f+g+h+i+j
    __noinline static int foo1(int a, int b, int c, int d, int e,
    int f, int g, int h, int i, int j)
    {
    return a + b + c + d + e + f + g + h + i + j;
    }
// foo2: a+b+c+d+e+f+g+h+i+j+k+l
    __noinline static int foo2(int a, int b, int c, int d, int e,
    int f, int g, int h, int i, int j,
    int k, int l)
    {
    return a + b + c + d + e + f + g + h + i + j + k + l;
    }
// global_two_callees calls foo1 (5 stack args) and foo2 (7 stack args).
// The outgoing stack arg area is sized for foo2 (the larger callee).
// Stores for foo1 are a subset of the area used by foo2.
// Result: foo1(1..10) + foo2(1..12) = 55 + 78 = 133
//
// Pass a-e through so the compiler can't constant-fold the stack args away.
//
#[no_mangle]
pub unsafe extern "C" fn global_two_callees(a: c_int, b: c_int, c: c_int, d: c_int, e: c_int) -> __noinline int {
    __noinline int global_two_callees(int a, int b, int c, int d, int e)
    {
    int ret;
    ret = foo1(a, b, c, d, e, a + 5, a + 6, a + 7, a + 8, a + 9);
    ret += foo2(a, b, c, d, e, a + 5, a + 6, a + 7, a + 8, a + 9,
    a + 10, a + 11);
    return ret;
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn test_two_callees() -> c_int {
    int test_two_callees(void)
    {
    return global_two_callees(1, 2, 3, 4, 5);
    }
    let mut timer_base: volatile int = 10;
#[no_mangle]
unsafe extern "C" fn timer_cb_many_args(map: *mut c_void, key: *mut c_int, timer: *mut bpf_timer) -> c_int {
    static int timer_cb_many_args(void *map, int *key, struct bpf_timer *timer)
    {
    let mut v: c_int = timer_base;
    timer_result = static_func_many_args(v, v * 2, v * 3, v * 4, v * 5,
    v * 6, v * 7, v * 8, v * 9,
    v * 10);
    return 0;
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn test_async_cb_many_args() -> c_int {
    int test_async_cb_many_args(void)
    {
    struct timer_elem *elem;
    let mut key: c_int = 0;
    elem = bpf_map_lookup_elem(&timer_map, &key);
    if (!elem)
    return -1;
    bpf_timer_init(&elem.timer, &timer_map, CLOCK_MONOTONIC);
    bpf_timer_set_callback(&elem.timer, timer_cb_many_args);
    bpf_timer_start(&elem.timer, 1, 0);
    return 0;
    }

    let mut has_stack_arg: volatile bool = false;
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn test_global_many_args() -> c_int {
    int test_global_many_args(void)
    {
    return 0;
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn test_bpf2bpf_ptr_stack_arg() -> c_int {
    int test_bpf2bpf_ptr_stack_arg(void)
    {
    return 0;
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn test_bpf2bpf_mix_stack_args() -> c_int {
    int test_bpf2bpf_mix_stack_args(void)
    {
    return 0;
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn test_bpf2bpf_nesting_stack_arg() -> c_int {
    int test_bpf2bpf_nesting_stack_arg(void)
    {
    return 0;
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn test_bpf2bpf_dynptr_stack_arg(skb: *mut __sk_buff) -> c_int {
    int test_bpf2bpf_dynptr_stack_arg(struct __sk_buff *skb)
    {
    return 0;
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn test_two_callees() -> c_int {
    int test_two_callees(void)
    {
    return 0;
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn test_async_cb_many_args() -> c_int {
    int test_async_cb_many_args(void)
    {
    return 0;
    }

    char _license[] SEC("license") = "GPL";
