//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_sk_storage_trace_itself.c
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

    struct {
    __uint(type, BPF_MAP_TYPE_SK_STORAGE);
    __uint(map_flags, BPF_F_NO_PREALLOC);
    __type(key, int);
    __type(value, int);
    } sk_stg_map SEC(".maps");
    SEC("fentry/bpf_sk_storage_free")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: trace_bpf_sk_storage_free, sk: *mut sock) -> c_int {
    int BPF_PROG(trace_bpf_sk_storage_free, struct sock *sk)
    {
    int *value;
    value = bpf_sk_storage_get(&sk_stg_map, sk, 0,
    BPF_SK_STORAGE_GET_F_CREATE);
    if (value)
// value = 1;
    return 0;
    }
    char _license[] SEC("license") = "GPL";
