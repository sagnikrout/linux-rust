//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/struct_ops_refcounted_fail__tail_call.c
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
// Copyright (c) 2025 Meta Platforms, Inc. and affiliates.

    char _license[] SEC("license") = "GPL";
    struct {
    __uint(type, BPF_MAP_TYPE_PROG_ARRAY);
    __uint(max_entries, 1);
    __uint(key_size, sizeof(__u32));
    __uint(value_size, sizeof(__u32));
    } prog_array SEC(".maps");
// Test that the verifier rejects a program with referenced kptr arguments
// that tail call
//
    SEC("struct_ops/test_refcounted")
#[no_mangle]
pub unsafe extern "C" fn __msg(call": "program with __ref argument cannot tail) -> __failure {
    __failure __msg("program with __ref argument cannot tail call")
#[no_mangle]
pub unsafe extern "C" fn refcounted_fail__tail_call(ctx: *mut c_ulonglong) -> c_int {
    int refcounted_fail__tail_call(unsigned long long *ctx)
    {
    struct task_struct *task = (struct task_struct *)ctx[1];
    bpf_task_release(task);
    bpf_tail_call(ctx, &prog_array, 0);
    return 0;
    }
    SEC(".struct_ops.link")
    struct bpf_testmod_ops testmod_ref_acquire = {
    .test_refcounted = (void *)refcounted_fail__tail_call,
    };
