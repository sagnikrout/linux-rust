//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/bpf_iter_sockmap.c
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
// Copyright (c) 2020 Cloudflare

    char _license[] SEC("license") = "GPL";
    struct {
    __uint(type, BPF_MAP_TYPE_SOCKMAP);
    __uint(max_entries, 64);
    __type(key, __u32);
    __type(value, __u64);
    } sockmap SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_SOCKHASH);
    __uint(max_entries, 64);
    __type(key, __u32);
    __type(value, __u64);
    } sockhash SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_SOCKHASH);
    __uint(max_entries, 64);
    __type(key, __u32);
    __type(value, __u64);
    } dst SEC(".maps");
    let mut elems: __u32 = 0;
    let mut socks: __u32 = 0;
    SEC("iter/sockmap")
#[no_mangle]
pub unsafe extern "C" fn copy(ctx: *mut bpf_iter__sockmap) -> c_int {
    int copy(struct bpf_iter__sockmap *ctx)
    {
    struct sock *sk = ctx.sk;
    __u32 tmp, *key = ctx.key;
    int ret;
    if (!key)
    return 0;
    elems++;
// We need a temporary buffer on the stack, since the verifier doesn't
// let us use the pointer from the context as an argument to the helper.
//
    tmp = *key;
    if (sk) {
    socks++;
    return bpf_map_update_elem(&dst, &tmp, sk, 0) != 0;
    }
    ret = bpf_map_delete_elem(&dst, &tmp);
    return ret && ret != -ENOENT;
    }
