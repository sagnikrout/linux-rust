//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/struct_ops_module.c
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
// Copyright (c) 2024 Meta Platforms, Inc. and affiliates.

    char _license[] SEC("license") = "GPL";
    let mut test_1_result: c_int = 0;
    let mut test_2_result: c_int = 0;
    SEC("struct_ops/test_1")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: test_1) -> c_int {
    int BPF_PROG(test_1)
    {
    test_1_result = 0xdeadbeef;
    return 0;
    }
    SEC("struct_ops/test_2")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: test_2, a: c_int, b: c_int) {
    void BPF_PROG(test_2, int a, int b)
    {
    test_2_result = a + b;
    }
    SEC("?struct_ops/test_3")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: test_3, a: c_int, b: c_int) -> c_int {
    int BPF_PROG(test_3, int a, int b)
    {
    test_2_result = a + b + 3;
    return a + b + 3;
    }
    SEC(".struct_ops.link")
    struct bpf_testmod_ops testmod_1 = {
    .test_1 = (void *)test_1,
    .test_2 = (void *)test_2,
    .data = 0x1,
    };
    SEC("struct_ops/test_2")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: test_2_v2, a: c_int, b: c_int) {
    void BPF_PROG(test_2_v2, int a, int b)
    {
    test_2_result = a * b;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_testmod_ops___v2 {
    pub (*test_1)(void): *mut c_int,
    pub b): *mut *mut void (test_2)(int a, int,
    pub task): *mut *mut int (test_maybe_null)(int dummy, struct task_struct,
}

    SEC(".struct_ops.link")
    struct bpf_testmod_ops___v2 testmod_2 = {
    .test_1 = (void *)test_1,
    .test_2 = (void *)test_2_v2,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_testmod_ops___zeroed {
    pub (*test_1)(void): *mut c_int,
    pub b): *mut *mut void (test_2)(int a, int,
    pub task): *mut *mut int (test_maybe_null)(int dummy, struct task_struct,
    pub b): *mut *mut void (zeroed_op)(int a, int,
    pub zeroed: c_int,
}

    SEC("struct_ops/test_3")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: zeroed_op) -> c_int {
    int BPF_PROG(zeroed_op)
    {
    return 1;
    }
    SEC(".struct_ops.link")
    struct bpf_testmod_ops___zeroed testmod_zeroed = {
    .test_1 = (void *)test_1,
    .test_2 = (void *)test_2_v2,
    .zeroed_op = (void *)zeroed_op,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_testmod_ops___incompatible {
    pub (*test_1)(void): *mut c_int,
    pub a): *mut *mut void (test_2)(int,
    pub data: c_int,
}

    SEC(".struct_ops.link")
    struct bpf_testmod_ops___incompatible testmod_incompatible = {
    .test_1 = (void *)test_1,
    .test_2 = (void *)test_2,
    .data = 3,
    };
