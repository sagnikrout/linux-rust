//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/kfunc_implicit_args_tracing.c
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

    extern int bpf_kfunc_implicit_arg(int a) __weak __ksym;
    char _license[] SEC("license") = "GPL";
// Shared arg checks; reports arg count and aux, returns 1 on success.
    static __always_inline __u64
    check_implicit_args(void *ctx, __u64 *arg_cnt, __u64 *aux_arg)
    {
    let mut a: __u64 = 0, aux = 0, z = 0;
    __u64 result;
    __s64 err;
// arg_cnt = bpf_get_func_arg_cnt(ctx);
    result = *arg_cnt == 2;
    err = bpf_get_func_arg(ctx, 0, &a);
    result &= err == 0 && (int)a == 5;
    err = bpf_get_func_arg(ctx, 1, &aux);
// aux_arg = aux;
    result &= err == 0 && aux != 0;
    err = bpf_get_func_arg(ctx, 2, &z);
    result &= err == -EINVAL;
    return result;
    }
    __u64 fentry_result;
    __u64 fentry_arg_cnt;
    __u64 fentry_aux_arg;
    SEC("fentry/bpf_kfunc_implicit_arg")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: trace_implicit_arg_fentry) -> c_int {
    int BPF_PROG(trace_implicit_arg_fentry)
    {
    let mut ret: __u64 = 0;
    __s64 err;
    fentry_result = check_implicit_args(ctx, &fentry_arg_cnt, &fentry_aux_arg);
    err = bpf_get_func_ret(ctx, &ret);
    fentry_result &= err == -EOPNOTSUPP;
    return 0;
    }
    __u64 fexit_result;
    __u64 fexit_arg_cnt;
    __u64 fexit_aux_arg;
    SEC("fexit/bpf_kfunc_implicit_arg")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: trace_implicit_arg_fexit) -> c_int {
    int BPF_PROG(trace_implicit_arg_fexit)
    {
    let mut ret: __u64 = 0;
    __s64 err;
    fexit_result = check_implicit_args(ctx, &fexit_arg_cnt, &fexit_aux_arg);
    err = bpf_get_func_ret(ctx, &ret);
    fexit_result &= err == 0 && ret == 5;
    return 0;
    }
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn trigger_implicit_arg(ctx: *mut c_void) -> c_int {
    int trigger_implicit_arg(void *ctx)
    {
    return bpf_kfunc_implicit_arg(5);
    }
