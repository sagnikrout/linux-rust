//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/bpf_loop.c
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

    char _license[] SEC("license") = "GPL";
#[repr(C)]
#[derive(Copy, Clone)]
pub struct callback_ctx {
    pub output: c_int,
}

    struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __uint(max_entries, 32);
    __type(key, int);
    __type(value, int);
    } map1 SEC(".maps");
// These should be set by the user program
    u32 nested_callback_nr_loops;
    let mut stop_index: u32 = -1;
    u32 nr_loops;
    int pid;
    int callback_selector;
// Making these global variables so that the userspace program
// can verify the output through the skeleton
//
    int nr_loops_returned;
    int g_output;
    int err;
#[no_mangle]
unsafe extern "C" fn callback(index: __u32, data: *mut c_void) -> c_int {
    static int callback(__u32 index, void *data)
    {
    struct callback_ctx *ctx = data;
    if (index >= stop_index)
    return 1;
    ctx.output += index;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn empty_callback(index: __u32, data: *mut c_void) -> c_int {
    static int empty_callback(__u32 index, void *data)
    {
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn nested_callback2(index: __u32, data: *mut c_void) -> c_int {
    static int nested_callback2(__u32 index, void *data)
    {
    nr_loops_returned += bpf_loop(nested_callback_nr_loops, callback, data, 0);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn nested_callback1(index: __u32, data: *mut c_void) -> c_int {
    static int nested_callback1(__u32 index, void *data)
    {
    bpf_loop(nested_callback_nr_loops, nested_callback2, data, 0);
    return 0;
    }
    SEC("fentry/" SYS_PREFIX "sys_nanosleep")
#[no_mangle]
pub unsafe extern "C" fn test_prog(ctx: *mut c_void) -> c_int {
    int test_prog(void *ctx)
    {
    let mut data: callback_ctx = {};
    if (bpf_get_current_pid_tgid() >> 32 != pid)
    return 0;
    nr_loops_returned = bpf_loop(nr_loops, callback, &data, 0);
    if (nr_loops_returned < 0)
    err = nr_loops_returned;
    else
    g_output = data.output;
    return 0;
    }
    SEC("fentry/" SYS_PREFIX "sys_nanosleep")
#[no_mangle]
pub unsafe extern "C" fn prog_null_ctx(ctx: *mut c_void) -> c_int {
    int prog_null_ctx(void *ctx)
    {
    if (bpf_get_current_pid_tgid() >> 32 != pid)
    return 0;
    nr_loops_returned = bpf_loop(nr_loops, empty_callback, core::ptr::null_mut(), 0);
    return 0;
    }
    SEC("fentry/" SYS_PREFIX "sys_nanosleep")
#[no_mangle]
pub unsafe extern "C" fn prog_invalid_flags(ctx: *mut c_void) -> c_int {
    int prog_invalid_flags(void *ctx)
    {
    let mut data: callback_ctx = {};
    if (bpf_get_current_pid_tgid() >> 32 != pid)
    return 0;
    err = bpf_loop(nr_loops, callback, &data, 1);
    return 0;
    }
    SEC("fentry/" SYS_PREFIX "sys_nanosleep")
#[no_mangle]
pub unsafe extern "C" fn prog_nested_calls(ctx: *mut c_void) -> c_int {
    int prog_nested_calls(void *ctx)
    {
    let mut data: callback_ctx = {};
    if (bpf_get_current_pid_tgid() >> 32 != pid)
    return 0;
    nr_loops_returned = 0;
    bpf_loop(nr_loops, nested_callback1, &data, 0);
    g_output = data.output;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn callback_set_f0(i: c_int, ctx: *mut c_void) -> c_int {
    static int callback_set_f0(int i, void *ctx)
    {
    g_output = 0xF0;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn callback_set_0f(i: c_int, ctx: *mut c_void) -> c_int {
    static int callback_set_0f(int i, void *ctx)
    {
    g_output = 0x0F;
    return 0;
    }
//
// non-constant callback is a corner case for bpf_loop inline logic
//
    SEC("fentry/" SYS_PREFIX "sys_nanosleep")
#[no_mangle]
pub unsafe extern "C" fn prog_non_constant_callback(ctx: *mut c_void) -> c_int {
    int prog_non_constant_callback(void *ctx)
    {
    if (bpf_get_current_pid_tgid() >> 32 != pid)
    return 0;
    int (*callback)(int i, void *ctx);
    g_output = 0;
    if (callback_selector == 0x0F)
    callback = callback_set_0f;
    else
    callback = callback_set_f0;
    bpf_loop(1, callback, core::ptr::null_mut(), 0);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn stack_check_inner_callback(ctx: *mut c_void) -> c_int {
    static int stack_check_inner_callback(void *ctx)
    {
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn map1_lookup_elem(key: c_int) -> c_int {
    static int map1_lookup_elem(int key)
    {
    int *val = bpf_map_lookup_elem(&map1, &key);
    return val ? *val : -1;
    }
#[no_mangle]
unsafe extern "C" fn map1_update_elem(key: c_int, val: c_int) {
    static void map1_update_elem(int key, int val)
    {
    bpf_map_update_elem(&map1, &key, &val, BPF_ANY);
    }
#[no_mangle]
unsafe extern "C" fn stack_check_outer_callback(ctx: *mut c_void) -> c_int {
    static int stack_check_outer_callback(void *ctx)
    {
    let mut a: c_int = map1_lookup_elem(1);
    let mut b: c_int = map1_lookup_elem(2);
    let mut c: c_int = map1_lookup_elem(3);
    let mut d: c_int = map1_lookup_elem(4);
    let mut e: c_int = map1_lookup_elem(5);
    let mut f: c_int = map1_lookup_elem(6);
    bpf_loop(1, stack_check_inner_callback, core::ptr::null_mut(), 0);
    map1_update_elem(1, a + 1);
    map1_update_elem(2, b + 1);
    map1_update_elem(3, c + 1);
    map1_update_elem(4, d + 1);
    map1_update_elem(5, e + 1);
    map1_update_elem(6, f + 1);
    return 0;
    }
// Some of the local variables in stack_check and
// stack_check_outer_callback would be allocated on stack by
// compiler. This test should verify that stack content for these
// variables is preserved between calls to bpf_loop (might be an issue
// if loop inlining allocates stack slots incorrectly).
//
    SEC("fentry/" SYS_PREFIX "sys_nanosleep")
#[no_mangle]
pub unsafe extern "C" fn stack_check(ctx: *mut c_void) -> c_int {
    int stack_check(void *ctx)
    {
    if (bpf_get_current_pid_tgid() >> 32 != pid)
    return 0;
    let mut a: c_int = map1_lookup_elem(7);
    let mut b: c_int = map1_lookup_elem(8);
    let mut c: c_int = map1_lookup_elem(9);
    let mut d: c_int = map1_lookup_elem(10);
    let mut e: c_int = map1_lookup_elem(11);
    let mut f: c_int = map1_lookup_elem(12);
    bpf_loop(1, stack_check_outer_callback, core::ptr::null_mut(), 0);
    map1_update_elem(7,  a + 1);
    map1_update_elem(8, b + 1);
    map1_update_elem(9, c + 1);
    map1_update_elem(10, d + 1);
    map1_update_elem(11, e + 1);
    map1_update_elem(12, f + 1);
    return 0;
    }
