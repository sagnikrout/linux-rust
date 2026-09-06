//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/udp_limit.c
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


// SPDX-License-Identifier: GPL-2.0-only

    let mut invocations: c_int = 0, in_use = 0;
    struct {
    __uint(type, BPF_MAP_TYPE_SK_STORAGE);
    __uint(map_flags, BPF_F_NO_PREALLOC);
    __type(key, int);
    __type(value, int);
    } sk_map SEC(".maps");
    SEC("cgroup/sock_create")
#[no_mangle]
pub unsafe extern "C" fn sock(ctx: *mut bpf_sock) -> c_int {
    int sock(struct bpf_sock *ctx)
    {
    int *sk_storage;
    if (ctx.type != SOCK_DGRAM)
    return 1;
    sk_storage = bpf_sk_storage_get(&sk_map, ctx, 0,
    BPF_SK_STORAGE_GET_F_CREATE);
    if (!sk_storage)
    return 0;
// sk_storage = 0xdeadbeef;
    __sync_fetch_and_add(&invocations, 1);
    if (in_use > 0) {
// BPF_CGROUP_INET_SOCK_RELEASE is _not_ called
// when we return an error from the BPF
// program!
//
    return 0;
    }
    __sync_fetch_and_add(&in_use, 1);
    return 1;
    }
    SEC("cgroup/sock_release")
#[no_mangle]
pub unsafe extern "C" fn sock_release(ctx: *mut bpf_sock) -> c_int {
    int sock_release(struct bpf_sock *ctx)
    {
    int *sk_storage;
    if (ctx.type != SOCK_DGRAM)
    return 1;
    sk_storage = bpf_sk_storage_get(&sk_map, ctx, 0, 0);
    if (!sk_storage || *sk_storage != 0xdeadbeef)
    return 0;
    __sync_fetch_and_add(&invocations, 1);
    __sync_fetch_and_add(&in_use, -1);
    return 1;
    }
