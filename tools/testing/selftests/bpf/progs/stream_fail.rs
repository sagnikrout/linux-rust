//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/stream_fail.c
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

    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn __msg(passed": "Possibly NULL pointer) -> __failure {
    __failure __msg("Possibly core::ptr::null_mut() pointer passed")
#[no_mangle]
pub unsafe extern "C" fn stream_vprintk_null_arg(ctx: *mut c_void) -> c_int {
    int stream_vprintk_null_arg(void *ctx)
    {
    bpf_stream_vprintk(BPF_STDOUT, "", core::ptr::null_mut(), 0);
    return 0;
    }
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn __msg(expected=": "R3 type=scalar) -> __failure {
    __failure __msg("R3 type=scalar expected=")
#[no_mangle]
pub unsafe extern "C" fn stream_vprintk_scalar_arg(ctx: *mut c_void) -> c_int {
    int stream_vprintk_scalar_arg(void *ctx)
    {
    bpf_stream_vprintk(BPF_STDOUT, "", (void *)46, 0);
    return 0;
    }
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn __msg(string": "R2 doesn't point to a const) -> __failure {
    __failure __msg("R2 doesn't point to a const string")
#[no_mangle]
pub unsafe extern "C" fn stream_vprintk_string_arg(ctx: *mut c_void) -> c_int {
    int stream_vprintk_string_arg(void *ctx)
    {
    bpf_stream_vprintk(BPF_STDOUT, ctx, core::ptr::null_mut(), 0);
    return 0;
    }
    char _license[] SEC("license") = "GPL";
