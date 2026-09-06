//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/iter_buf_null_fail.c
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
// Copyright (c) 2026 Qi Tang

    char _license[] SEC("license") = "GPL";
// Verify that the verifier rejects direct access to nullable PTR_TO_BUF.
    SEC("iter/bpf_map_elem")
#[no_mangle]
pub unsafe extern "C" fn __msg(access": "invalid mem) -> __failure {
    __failure __msg("invalid mem access")
#[no_mangle]
pub unsafe extern "C" fn iter_buf_null_deref(ctx: *mut bpf_iter__bpf_map_elem) -> c_int {
    int iter_buf_null_deref(struct bpf_iter__bpf_map_elem *ctx)
    {
//
// ctx->key is PTR_TO_BUF | PTR_MAYBE_NULL | MEM_RDONLY.
// Direct access without null check must be rejected.
//
    let mut v: volatile __u32 = *(__u32 *)ctx.key;
    (void)v;
    return 0;
    }
// Verify that access after a null check is still accepted.
    SEC("iter/bpf_map_elem")
    __success
#[no_mangle]
pub unsafe extern "C" fn iter_buf_null_check_ok(ctx: *mut bpf_iter__bpf_map_elem) -> c_int {
    int iter_buf_null_check_ok(struct bpf_iter__bpf_map_elem *ctx)
    {
    __u32 *key = ctx.key;
    if (!key)
    return 0;
    let mut v: volatile __u32 = *key;
    (void)v;
    return 0;
    }
