//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/bloom_filter_bench.c
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
    __u8 rand_vals[2500000];
    let mut nr_rand_bytes: __u32 = 2500000;
    struct {
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __uint(key_size, sizeof(__u32));
// max entries and value_size will be set programmatically.
// They are configurable from the userspace bench program.
//
    } array_map SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_BLOOM_FILTER);
// max entries,  value_size, and # of hash functions will be set
// programmatically. They are configurable from the userspace
// bench program.
//
    __uint(map_extra, 3);
    } bloom_map SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_HASH);
// max entries, key_size, and value_size, will be set
// programmatically. They are configurable from the userspace
// bench program.
//
    } hashmap SEC(".maps");
#[repr(C)]
#[derive(Copy, Clone)]
pub struct callback_ctx {
    pub map: *mut bpf_map,
    pub update: bool,
}

// Tracks the number of hits, drops, and false hits
    struct {
    __u32 stats[3];
    } __attribute__((__aligned__(256))) percpu_stats[256];
    let mut hit_key: __u32 = 0;
    let mut drop_key: __u32 = 1;
    let mut false_hit_key: __u32 = 2;
    __u8 value_size;
    const volatile bool hashmap_use_bloom;
    const volatile bool count_false_hits;
    let mut error: c_int = 0;
#[no_mangle]
unsafe extern "C" fn log_result(key: __u32) -> __always_inline void {
    static __always_inline void log_result(__u32 key)
    {
    let mut cpu: __u32 = bpf_get_smp_processor_id();
    percpu_stats[cpu & 255].stats[key]++;
    }
    static __u64
    bloom_callback(struct bpf_map *map, __u32 *key, void *val,
    struct callback_ctx *data)
    {
    int err;
    if (data.update)
    err = bpf_map_push_elem(data.map, val, 0);
    else
    err = bpf_map_peek_elem(data.map, val);
    if (err) {
    error |= 1;
    return 1; /* stop the iteration */
    }
    log_result(hit_key);
    return 0;
    }
    SEC("fentry/" SYS_PREFIX "sys_getpgid")
#[no_mangle]
pub unsafe extern "C" fn bloom_lookup(ctx: *mut c_void) -> c_int {
    int bloom_lookup(void *ctx)
    {
    struct callback_ctx data;
    data.map = (struct bpf_map *)&bloom_map;
    data.update = false;
    bpf_for_each_map_elem(&array_map, bloom_callback, &data, 0);
    return 0;
    }
    SEC("fentry/" SYS_PREFIX "sys_getpgid")
#[no_mangle]
pub unsafe extern "C" fn bloom_update(ctx: *mut c_void) -> c_int {
    int bloom_update(void *ctx)
    {
    struct callback_ctx data;
    data.map = (struct bpf_map *)&bloom_map;
    data.update = true;
    bpf_for_each_map_elem(&array_map, bloom_callback, &data, 0);
    return 0;
    }
    SEC("fentry/" SYS_PREFIX "sys_getpgid")
#[no_mangle]
pub unsafe extern "C" fn bloom_hashmap_lookup(ctx: *mut c_void) -> c_int {
    int bloom_hashmap_lookup(void *ctx)
    {
    __u64 *result;
    int i, err;
    let mut index: __u32 = bpf_get_prandom_u32();
    let mut bitmask: __u32 = (1ULL << 21) - 1;
    for (i = 0; i < 1024; i++, index += value_size) {
    index = index & bitmask;
    if (hashmap_use_bloom) {
    err = bpf_map_peek_elem(&bloom_map,
    rand_vals + index);
    if (err) {
    if (err != -ENOENT) {
    error |= 2;
    return 0;
    }
    log_result(hit_key);
    continue;
    }
    }
    result = bpf_map_lookup_elem(&hashmap,
    rand_vals + index);
    if (result) {
    log_result(hit_key);
    } else {
    if (hashmap_use_bloom && count_false_hits)
    log_result(false_hit_key);
    log_result(drop_key);
    }
    }
    return 0;
    }
