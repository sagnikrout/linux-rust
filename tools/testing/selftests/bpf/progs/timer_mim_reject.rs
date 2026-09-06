//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/timer_mim_reject.c
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
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hmap_elem {
    pub /: *mut *mut int pad; / unused,
    pub timer: bpf_timer,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct inner_map {
    pub BPF_MAP_TYPE_HASH): __uint(type,,
    pub 1024): __uint(max_entries,,
    pub int): __type(key,,
    pub hmap_elem): __type(value, struct,
    pub SEC(".maps"): } inner_htab,
pub const ARRAY_KEY: c_int = 1;
pub const ARRAY_KEY2: c_int = 2;
pub const HASH_KEY: c_int = 1234;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct outer_arr {
    pub BPF_MAP_TYPE_ARRAY_OF_MAPS): __uint(type,,
    pub 2): __uint(max_entries,,
    pub sizeof(int)): __uint(key_size,,
    pub sizeof(int)): __uint(value_size,,
    pub inner_map): __array(values, struct,
    } outer_arr SEC(".maps") = {
    .values = { [ARRAY_KEY] = &inner_htab },
}

    __u64 err;
    __u64 ok;
    __u64 cnt;
// callback for inner hash map
#[no_mangle]
unsafe extern "C" fn timer_cb(map: *mut c_void, key: *mut c_int, val: *mut hmap_elem) -> c_int {
    static int timer_cb(void *map, int *key, struct hmap_elem *val)
    {
    return 0;
    }
    SEC("fentry/bpf_fentry_test1")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: test1, a: c_int) -> c_int {
    int BPF_PROG(test1, int a)
    {
    let mut init: hmap_elem = {};
    struct bpf_map *inner_map, *inner_map2;
    struct hmap_elem *val;
    let mut array_key: c_int = ARRAY_KEY;
    let mut array_key2: c_int = ARRAY_KEY2;
    let mut hash_key: c_int = HASH_KEY;
    inner_map = bpf_map_lookup_elem(&outer_arr, &array_key);
    if (!inner_map)
    return 0;
    inner_map2 = bpf_map_lookup_elem(&outer_arr, &array_key2);
    if (!inner_map2)
    return 0;
    bpf_map_update_elem(inner_map, &hash_key, &init, 0);
    val = bpf_map_lookup_elem(inner_map, &hash_key);
    if (!val)
    return 0;
    bpf_timer_init(&val.timer, inner_map2, CLOCK_MONOTONIC);
    if (bpf_timer_set_callback(&val.timer, timer_cb))
    err |= 4;
    if (bpf_timer_start(&val.timer, 0, 0))
    err |= 8;
    return 0;
    }
