//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/sk_storage_omem_uncharge.c
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
// Copyright (c) 2023 Facebook

    void *sk_ptr = core::ptr::null_mut();
    let mut cookie_found: c_int = 0;
    let mut cookie: __u64 = 0;
    let mut omem: __u32 = 0;
    struct {
    __uint(type, BPF_MAP_TYPE_SK_STORAGE);
    __uint(map_flags, BPF_F_NO_PREALLOC);
    __type(key, int);
    __type(value, int);
    } sk_storage SEC(".maps");
    SEC("fexit/bpf_sk_storage_free")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: bpf_sk_storage_free, sk: *mut sock) -> c_int {
    int BPF_PROG(bpf_sk_storage_free, struct sock *sk)
    {
    if (sk_ptr != sk)
    return 0;
    if (sk.sk_cookie.counter != cookie)
    return 0;
    cookie_found++;
    omem = sk.sk_omem_alloc.counter;
    return 0;
    }
    SEC("fentry/inet6_sock_destruct")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: inet6_sock_destruct, sk: *mut sock) -> c_int {
    int BPF_PROG(inet6_sock_destruct, struct sock *sk)
    {
    int *value;
    if (!cookie || sk.sk_cookie.counter != cookie)
    return 0;
    value = bpf_sk_storage_get(&sk_storage, sk, 0, 0);
    if (value && *value == 0xdeadbeef) {
    cookie_found++;
    sk_ptr = sk;
    }
    return 0;
    }
    char _license[] SEC("license") = "GPL";
