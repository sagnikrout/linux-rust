//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/btf__stack_arg_precision.c
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
pub unsafe extern "C" fn subprog_call_mem_kfunc(a: c_long, b: c_long, c: c_long, d: c_long, e: c_long, size: c_long) -> c_long {
    long subprog_call_mem_kfunc(long a, long b, long c, long d, long e, long size)
    {
    char buf[8] = {};
    return bpf_kfunc_call_stack_arg_mem(a, b, c, d, e, buf, size);
    }

#[no_mangle]
pub unsafe extern "C" fn subprog_call_mem_kfunc() -> c_long {
    long subprog_call_mem_kfunc(void)
    {
    return 0;
    }
