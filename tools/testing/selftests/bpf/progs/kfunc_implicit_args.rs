//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/kfunc_implicit_args.c
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
    extern int bpf_kfunc_implicit_arg_impl(int a, struct bpf_prog_aux *aux) __weak __ksym; /* illegal */
    extern int bpf_kfunc_implicit_arg_legacy(int a, int b) __weak __ksym;
    extern int bpf_kfunc_implicit_arg_legacy_impl(int a, int b, struct bpf_prog_aux *aux) __weak __ksym;
    char _license[] SEC("license") = "GPL";
    SEC("syscall")
    __retval(5)
#[no_mangle]
pub unsafe extern "C" fn test_kfunc_implicit_arg(ctx: *mut c_void) -> c_int {
    int test_kfunc_implicit_arg(void *ctx)
    {
    return bpf_kfunc_implicit_arg(5);
    }
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn __msg(bpf_kfunc_implicit_arg_impl": "cannot find address for kernel function) -> __failure {
    __failure __msg("cannot find address for kernel function bpf_kfunc_implicit_arg_impl")
#[no_mangle]
pub unsafe extern "C" fn test_kfunc_implicit_arg_impl_illegal(ctx: *mut c_void) -> c_int {
    int test_kfunc_implicit_arg_impl_illegal(void *ctx)
    {
    return bpf_kfunc_implicit_arg_impl(5, core::ptr::null_mut());
    }
    SEC("syscall")
    __retval(7)
#[no_mangle]
pub unsafe extern "C" fn test_kfunc_implicit_arg_legacy(ctx: *mut c_void) -> c_int {
    int test_kfunc_implicit_arg_legacy(void *ctx)
    {
    return bpf_kfunc_implicit_arg_legacy(3, 4);
    }
    SEC("syscall")
    __retval(11)
#[no_mangle]
pub unsafe extern "C" fn test_kfunc_implicit_arg_legacy_impl(ctx: *mut c_void) -> c_int {
    int test_kfunc_implicit_arg_legacy_impl(void *ctx)
    {
    return bpf_kfunc_implicit_arg_legacy_impl(5, 6, core::ptr::null_mut());
    }
