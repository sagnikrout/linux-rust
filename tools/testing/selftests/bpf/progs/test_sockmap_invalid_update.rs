//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_sockmap_invalid_update.c
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

    struct {
    __uint(type, BPF_MAP_TYPE_SOCKMAP);
    __uint(max_entries, 1);
    __type(key, __u32);
    __type(value, __u64);
    } map SEC(".maps");
    SEC("sockops")
#[no_mangle]
pub unsafe extern "C" fn bpf_sockmap(skops: *mut bpf_sock_ops) -> c_int {
    int bpf_sockmap(struct bpf_sock_ops *skops)
    {
    let mut key: __u32 = 0;
    if (skops.sk)
    bpf_map_update_elem(&map, &key, skops.sk, 0);
    return 0;
    }
    char _license[] SEC("license") = "GPL";
