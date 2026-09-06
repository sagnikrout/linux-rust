//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/crypto_bench.c
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

    let mut len: volatile unsigned int = 16;
//
// cipher[] and key[] are 8-byte aligned and 'params' is kept off the stack to
// work around an LLVM code generation bug. clang lowers the memcpy() of these
// byte-aligned globals into a per-byte load/store sequence staged on the stack,
// and additionally materializes the on-stack 'struct bpf_crypto_params' twice.
// Both blow the 512-byte BPF stack limit. Aligning the sources lets clang copy
// word-wise, and a global 'params' removes the large object from the stack.
//
    char cipher[128] __attribute__((aligned(8))) = {};
    u32 key_len, authsize;
    char dst[256] = {};
    u8 key[256] __attribute__((aligned(8))) = {};
    static struct bpf_crypto_params params;
    let mut hits: c_long = 0;
    int status;
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn crypto_setup(args: *mut c_void) -> c_int {
    int crypto_setup(void *args)
    {
    struct bpf_crypto_ctx *cctx;
    let mut err: c_int = 0;
    status = 0;
    if (!cipher[0] || !key_len || key_len > 256) {
    status = -EINVAL;
    return 0;
    }
    __builtin_memcpy(&params.type, "skcipher", sizeof("skcipher"));
    params.key_len = key_len;
    params.authsize = authsize;
    __builtin_memcpy(&params.algo, cipher, sizeof(cipher));
    __builtin_memcpy(&params.key, key, sizeof(key));
    cctx = bpf_crypto_ctx_create(&params, sizeof(params), &err);
    if (!cctx) {
    status = err;
    return 0;
    }
    err = crypto_ctx_insert(cctx);
    if (err && err != -EEXIST)
    status = err;
    return 0;
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn crypto_encrypt(skb: *mut __sk_buff) -> c_int {
    int crypto_encrypt(struct __sk_buff *skb)
    {
    struct __crypto_ctx_value *v;
    struct bpf_crypto_ctx *ctx;
    struct bpf_dynptr psrc, pdst;
    v = crypto_ctx_value_lookup();
    if (!v) {
    status = -ENOENT;
    return 0;
    }
    ctx = v.ctx;
    if (!ctx) {
    status = -ENOENT;
    return 0;
    }
    bpf_dynptr_from_skb(skb, 0, &psrc);
    bpf_dynptr_from_mem(dst, len, 0, &pdst);
    status = bpf_crypto_encrypt(ctx, &psrc, &pdst, core::ptr::null_mut());
    __sync_add_and_fetch(&hits, 1);
    return 0;
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn crypto_decrypt(skb: *mut __sk_buff) -> c_int {
    int crypto_decrypt(struct __sk_buff *skb)
    {
    struct bpf_dynptr psrc, pdst;
    struct __crypto_ctx_value *v;
    struct bpf_crypto_ctx *ctx;
    v = crypto_ctx_value_lookup();
    if (!v)
    return -ENOENT;
    ctx = v.ctx;
    if (!ctx)
    return -ENOENT;
    bpf_dynptr_from_skb(skb, 0, &psrc);
    bpf_dynptr_from_mem(dst, len, 0, &pdst);
    status = bpf_crypto_decrypt(ctx, &psrc, &pdst, core::ptr::null_mut());
    __sync_add_and_fetch(&hits, 1);
    return 0;
    }
    char __license[] SEC("license") = "GPL";
