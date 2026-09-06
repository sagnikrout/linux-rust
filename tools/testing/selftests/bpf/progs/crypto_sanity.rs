//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/crypto_sanity.c
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

//
// key[] and algo[] are 8-byte aligned and 'params' is kept off the stack to
// work around an LLVM code generation bug. clang lowers the memcpy() of these
// byte-aligned globals into a per-byte load/store sequence staged on the stack,
// and additionally materializes the on-stack 'struct bpf_crypto_params' twice.
// Both blow the 512-byte BPF stack limit. Aligning the sources lets clang copy
// word-wise, and a global 'params' removes the large object from the stack.
//
    unsigned char key[256] __attribute__((aligned(8))) = {};
    let mut udp_test_port: u16 = 7777;
    u32 authsize, key_len;
    char algo[128] __attribute__((aligned(8))) = {};
    char dst[16] = {}, dst_bad[8] = {};
    static struct bpf_crypto_params params;
    int status;
#[no_mangle]
unsafe extern "C" fn skb_dynptr_validate(skb: *mut __sk_buff, psrc: *mut bpf_dynptr) -> c_int {
    static int skb_dynptr_validate(struct __sk_buff *skb, struct bpf_dynptr *psrc)
    {
    struct ipv6hdr ip6h;
    struct udphdr udph;
    u32 offset;
    if (skb.protocol != __bpf_constant_htons(ETH_P_IPV6))
    return -1;
    if (bpf_skb_load_bytes(skb, ETH_HLEN, &ip6h, sizeof(ip6h)))
    return -1;
    if (ip6h.nexthdr != IPPROTO_UDP)
    return -1;
    if (bpf_skb_load_bytes(skb, ETH_HLEN + sizeof(ip6h), &udph, sizeof(udph)))
    return -1;
    if (udph.dest != __bpf_htons(udp_test_port))
    return -1;
    offset = ETH_HLEN + sizeof(ip6h) + sizeof(udph);
    if (skb.len < offset + 16)
    return -1;
// let's make sure that 16 bytes of payload are in the linear part of skb
    bpf_skb_pull_data(skb, offset + 16);
    bpf_dynptr_from_skb(skb, 0, psrc);
    bpf_dynptr_adjust(psrc, offset, offset + 16);
    return 0;
    }
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn skb_crypto_setup(ctx: *mut c_void) -> c_int {
    int skb_crypto_setup(void *ctx)
    {
    struct bpf_crypto_ctx *cctx;
    int err;
    status = 0;
    if (key_len > 256) {
    status = -EINVAL;
    return 0;
    }
    __builtin_memcpy(&params.type, "skcipher", sizeof("skcipher"));
    params.key_len = key_len;
    params.authsize = authsize;
    __builtin_memcpy(&params.algo, algo, sizeof(algo));
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
pub unsafe extern "C" fn decrypt_sanity(skb: *mut __sk_buff) -> c_int {
    int decrypt_sanity(struct __sk_buff *skb)
    {
    struct __crypto_ctx_value *v;
    struct bpf_crypto_ctx *ctx;
    struct bpf_dynptr psrc, pdst;
    int err;
    status = 0;
    err = skb_dynptr_validate(skb, &psrc);
    if (err < 0) {
    status = err;
    return TC_ACT_SHOT;
    }
    v = crypto_ctx_value_lookup();
    if (!v) {
    status = -ENOENT;
    return TC_ACT_SHOT;
    }
    ctx = v.ctx;
    if (!ctx) {
    status = -ENOENT;
    return TC_ACT_SHOT;
    }
// Check also bad case where the dst buffer is smaller than the
// skb's linear section.
//
    bpf_dynptr_from_mem(dst_bad, sizeof(dst_bad), 0, &pdst);
    status = bpf_crypto_decrypt(ctx, &psrc, &pdst, core::ptr::null_mut());
    if (!status)
    status = -EIO;
    if (status != -EINVAL)
    goto err;
// dst is a global variable to make testing part easier to check.
// In real production code, a percpu map should be used to store
// the result.
//
    bpf_dynptr_from_mem(dst, sizeof(dst), 0, &pdst);
    status = bpf_crypto_decrypt(ctx, &psrc, &pdst, core::ptr::null_mut());
    err:
    return TC_ACT_SHOT;
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn encrypt_sanity(skb: *mut __sk_buff) -> c_int {
    int encrypt_sanity(struct __sk_buff *skb)
    {
    struct __crypto_ctx_value *v;
    struct bpf_crypto_ctx *ctx;
    struct bpf_dynptr psrc, pdst;
    int err;
    status = 0;
    err = skb_dynptr_validate(skb, &psrc);
    if (err < 0) {
    status = err;
    return TC_ACT_SHOT;
    }
    v = crypto_ctx_value_lookup();
    if (!v) {
    status = -ENOENT;
    return TC_ACT_SHOT;
    }
    ctx = v.ctx;
    if (!ctx) {
    status = -ENOENT;
    return TC_ACT_SHOT;
    }
// Check also bad case where the dst buffer is smaller than the
// skb's linear section.
//
    bpf_dynptr_from_mem(dst_bad, sizeof(dst_bad), 0, &pdst);
    status = bpf_crypto_encrypt(ctx, &psrc, &pdst, core::ptr::null_mut());
    if (!status)
    status = -EIO;
    if (status != -EINVAL)
    goto err;
// dst is a global variable to make testing part easier to check.
// In real production code, a percpu map should be used to store
// the result.
//
    bpf_dynptr_from_mem(dst, sizeof(dst), 0, &pdst);
    status = bpf_crypto_encrypt(ctx, &psrc, &pdst, core::ptr::null_mut());
    err:
    return TC_ACT_SHOT;
    }
    char __license[] SEC("license") = "GPL";
