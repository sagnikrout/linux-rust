//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/for_each_hash_modify.c
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
// Copyright (c) 2025 Intel Corporation

    char _license[] SEC("license") = "GPL";
    struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __uint(max_entries, 128);
    __type(key, __u64);
    __type(value, __u64);
    } hashmap SEC(".maps");
#[no_mangle]
unsafe extern "C" fn cb(map: *mut bpf_map, key: *mut __u64, val: *mut __u64, arg: *mut c_void) -> c_int {
    static int cb(struct bpf_map *map, __u64 *key, __u64 *val, void *arg)
    {
    bpf_map_delete_elem(map, key);
    bpf_map_update_elem(map, key, val, 0);
    return 0;
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn test_pkt_access(skb: *mut __sk_buff) -> c_int {
    int test_pkt_access(struct __sk_buff *skb)
    {
    (void)skb;
    bpf_for_each_map_elem(&hashmap, cb, core::ptr::null_mut(), 0);
    return 0;
    }
