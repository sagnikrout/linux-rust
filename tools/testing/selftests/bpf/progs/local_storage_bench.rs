//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/local_storage_bench.c
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
// Copyright (c) 2022 Meta Platforms, Inc. and affiliates.

pub const HASHMAP_SZ: c_int = 4194304;
    struct {
    __uint(type, BPF_MAP_TYPE_ARRAY_OF_MAPS);
    __uint(max_entries, 1000);
    __type(key, int);
    __type(value, int);
    __array(values, struct {
    __uint(type, BPF_MAP_TYPE_TASK_STORAGE);
    __uint(map_flags, BPF_F_NO_PREALLOC);
    __type(key, int);
    __type(value, int);
    });
    } array_of_local_storage_maps SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_ARRAY_OF_MAPS);
    __uint(max_entries, 1000);
    __type(key, int);
    __type(value, int);
    __array(values, struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __uint(max_entries, HASHMAP_SZ);
    __type(key, int);
    __type(value, int);
    });
    } array_of_hash_maps SEC(".maps");
    long important_hits;
    long hits;
// set from user-space
    const volatile unsigned int use_hashmap;
    const volatile unsigned int hashmap_num_keys;
    const volatile unsigned int num_maps;
    const volatile unsigned int interleave;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct loop_ctx {
    pub task: *mut task_struct,
    pub loop_hits: c_long,
    pub loop_important_hits: c_long,
}

#[no_mangle]
unsafe extern "C" fn do_lookup(elem: c_uint, lctx: *mut loop_ctx) -> c_int {
    static int do_lookup(unsigned int elem, struct loop_ctx *lctx)
    {
    void *map, *inner_map;
    let mut idx: c_int = 0;
    if (use_hashmap)
    map = &array_of_hash_maps;
    else
    map = &array_of_local_storage_maps;
    inner_map = bpf_map_lookup_elem(map, &elem);
    if (!inner_map)
    return -1;
    if (use_hashmap) {
    idx = bpf_get_prandom_u32() % hashmap_num_keys;
    bpf_map_lookup_elem(inner_map, &idx);
    } else {
    bpf_task_storage_get(inner_map, lctx.task, &idx,
    BPF_LOCAL_STORAGE_GET_F_CREATE);
    }
    lctx.loop_hits++;
    if (!elem)
    lctx.loop_important_hits++;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn loop(index: u32, ctx: *mut c_void) -> c_long {
    static long loop(u32 index, void *ctx)
    {
    struct loop_ctx *lctx = (struct loop_ctx *)ctx;
    let mut map_idx: c_uint = index % num_maps;
    do_lookup(map_idx, lctx);
    if (interleave && map_idx % 3 == 0)
    do_lookup(0, lctx);
    return 0;
    }
    SEC("fentry/" SYS_PREFIX "sys_getpgid")
#[no_mangle]
pub unsafe extern "C" fn get_local(ctx: *mut c_void) -> c_int {
    int get_local(void *ctx)
    {
    struct loop_ctx lctx;
    lctx.task = bpf_get_current_task_btf();
    lctx.loop_hits = 0;
    lctx.loop_important_hits = 0;
    bpf_loop(10000, &loop, &lctx, 0);
    __sync_add_and_fetch(&hits, lctx.loop_hits);
    __sync_add_and_fetch(&important_hits, lctx.loop_important_hits);
    return 0;
    }
    char _license[] SEC("license") = "GPL";
