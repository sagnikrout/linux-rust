//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/bpf_iter_bpf_sk_storage_map.c
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
// Copyright (c) 2020 Facebook

    char _license[] SEC("license") = "GPL";
    struct {
    __uint(type, BPF_MAP_TYPE_SK_STORAGE);
    __uint(map_flags, BPF_F_NO_PREALLOC);
    __type(key, int);
    __type(value, int);
    } sk_stg_map SEC(".maps");
    let mut val_sum: __u32 = 0;
    let mut ipv6_sk_count: __u32 = 0;
    let mut to_add_val: __u32 = 0;
    SEC("iter/bpf_sk_storage_map")
#[no_mangle]
pub unsafe extern "C" fn rw_bpf_sk_storage_map(ctx: *mut bpf_iter__bpf_sk_storage_map) -> c_int {
    int rw_bpf_sk_storage_map(struct bpf_iter__bpf_sk_storage_map *ctx)
    {
    struct sock *sk = ctx.sk;
    __u32 *val = ctx.value;
    if (sk == core::ptr::null_mut() || val == core::ptr::null_mut())
    return 0;
    if (sk.sk_family == AF_INET6)
    ipv6_sk_count++;
    val_sum += *val;
// val += to_add_val;
    return 0;
    }
    SEC("iter/bpf_sk_storage_map")
#[no_mangle]
pub unsafe extern "C" fn oob_write_bpf_sk_storage_map(ctx: *mut bpf_iter__bpf_sk_storage_map) -> c_int {
    int oob_write_bpf_sk_storage_map(struct bpf_iter__bpf_sk_storage_map *ctx)
    {
    struct sock *sk = ctx.sk;
    __u32 *val = ctx.value;
    if (sk == core::ptr::null_mut() || val == core::ptr::null_mut())
    return 0;
// (val + 1) = 0xdeadbeef;
    return 0;
    }
