//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/bloom_filter_map.c
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
// Copyright (c) 2021 Facebook

    char _license[] SEC("license") = "GPL";
    struct bpf_map;
    struct {
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __type(key, __u32);
    __type(value, __u32);
    __uint(max_entries, 1000);
    } map_random_data SEC(".maps");
#[repr(C)]
#[derive(Copy, Clone)]
pub struct map_bloom_type {
    pub BPF_MAP_TYPE_BLOOM_FILTER): __uint(type,,
    pub __u32): __type(value,,
    pub 10000): __uint(max_entries,,
    pub 5): __uint(map_extra,,
    pub SEC(".maps"): } map_bloom,
    struct {
    pub BPF_MAP_TYPE_ARRAY_OF_MAPS): __uint(type,,
    pub int): __type(key,,
    pub int): __type(value,,
    pub 1): __uint(max_entries,,
    pub map_bloom_type): __array(values, struct,
    pub SEC(".maps"): } outer_map,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct callback_ctx {
    pub map: *mut bpf_map,
}

    let mut error: c_int = 0;
    static __u64
    check_elem(struct bpf_map *map, __u32 *key, __u32 *val,
    struct callback_ctx *data)
    {
    int err;
    err = bpf_map_peek_elem(data.map, val);
    if (err) {
    error |= 1;
    return 1; /* stop the iteration */
    }
    return 0;
    }
    SEC("fentry/" SYS_PREFIX "sys_getpgid")
#[no_mangle]
pub unsafe extern "C" fn inner_map(ctx: *mut c_void) -> c_int {
    int inner_map(void *ctx)
    {
    struct bpf_map *inner_map;
    struct callback_ctx data;
    let mut key: c_int = 0;
    inner_map = bpf_map_lookup_elem(&outer_map, &key);
    if (!inner_map) {
    error |= 2;
    return 0;
    }
    data.map = inner_map;
    bpf_for_each_map_elem(&map_random_data, check_elem, &data, 0);
    return 0;
    }
    SEC("fentry/" SYS_PREFIX "sys_getpgid")
#[no_mangle]
pub unsafe extern "C" fn check_bloom(ctx: *mut c_void) -> c_int {
    int check_bloom(void *ctx)
    {
    struct callback_ctx data;
    data.map = (struct bpf_map *)&map_bloom;
    bpf_for_each_map_elem(&map_random_data, check_elem, &data, 0);
    return 0;
    }
