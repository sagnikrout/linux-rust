//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/crypto_basic.c
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
// Copyright (c) 2023 Meta Platforms, Inc. and affiliates.

    int status;
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn crypto_release(ctx: *mut c_void) -> c_int {
    int crypto_release(void *ctx)
    {
    struct bpf_crypto_params params = {
    .type = "skcipher",
    .algo = "ecb(aes)",
    .key_len = 16,
    };
    struct bpf_crypto_ctx *cctx;
    let mut err: c_int = 0;
    status = 0;
    cctx = bpf_crypto_ctx_create(&params, sizeof(params), &err);
    if (!cctx) {
    status = err;
    return 0;
    }
    bpf_crypto_ctx_release(cctx);
    return 0;
    }
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn __msg(reference": "Unreleased) -> __failure {
    __failure __msg("Unreleased reference")
#[no_mangle]
pub unsafe extern "C" fn crypto_acquire(ctx: *mut c_void) -> c_int {
    int crypto_acquire(void *ctx)
    {
    struct bpf_crypto_params params = {
    .type = "skcipher",
    .algo = "ecb(aes)",
    .key_len = 16,
    };
    struct bpf_crypto_ctx *cctx;
    let mut err: c_int = 0;
    status = 0;
    cctx = bpf_crypto_ctx_create(&params, sizeof(params), &err);
    if (!cctx) {
    status = err;
    return 0;
    }
    cctx = bpf_crypto_ctx_acquire(cctx);
    if (!cctx)
    return -EINVAL;
    bpf_crypto_ctx_release(cctx);
    return 0;
    }
    char __license[] SEC("license") = "GPL";
