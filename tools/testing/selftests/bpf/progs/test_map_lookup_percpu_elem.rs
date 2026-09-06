//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_map_lookup_percpu_elem.c
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
// Copyright (c) 2022 Bytedance

    let mut percpu_array_elem_sum: __u64 = 0;
    let mut percpu_hash_elem_sum: __u64 = 0;
    let mut percpu_lru_hash_elem_sum: __u64 = 0;
    const volatile int nr_cpus;
    const volatile int my_pid;
    struct {
    __uint(type, BPF_MAP_TYPE_PERCPU_ARRAY);
    __uint(max_entries, 1);
    __type(key, __u32);
    __type(value, __u64);
    } percpu_array_map SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_PERCPU_HASH);
    __uint(max_entries, 1);
    __type(key, __u64);
    __type(value, __u64);
    } percpu_hash_map SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_LRU_PERCPU_HASH);
    __uint(max_entries, 1);
    __type(key, __u64);
    __type(value, __u64);
    } percpu_lru_hash_map SEC(".maps");
#[repr(C)]
#[derive(Copy, Clone)]
pub struct read_percpu_elem_ctx {
    pub map: *mut c_void,
    pub sum: __u64,
}

#[no_mangle]
unsafe extern "C" fn read_percpu_elem_callback(index: __u32, ctx: *mut read_percpu_elem_ctx) -> c_int {
    static int read_percpu_elem_callback(__u32 index, struct read_percpu_elem_ctx *ctx)
    {
    let mut key: __u64 = 0;
    __u64 *value;
    value = bpf_map_lookup_percpu_elem(ctx.map, &key, index);
    if (value)
    ctx.sum += *value;
    return 0;
    }
    SEC("tp/syscalls/sys_enter_getuid")
#[no_mangle]
pub unsafe extern "C" fn sysenter_getuid(ctx: *const c_void) -> c_int {
    int sysenter_getuid(const void *ctx)
    {
    struct read_percpu_elem_ctx map_ctx;
    if (my_pid != (bpf_get_current_pid_tgid() >> 32))
    return 0;
    map_ctx.map = &percpu_array_map;
    map_ctx.sum = 0;
    bpf_loop(nr_cpus, read_percpu_elem_callback, &map_ctx, 0);
    percpu_array_elem_sum = map_ctx.sum;
    map_ctx.map = &percpu_hash_map;
    map_ctx.sum = 0;
    bpf_loop(nr_cpus, read_percpu_elem_callback, &map_ctx, 0);
    percpu_hash_elem_sum = map_ctx.sum;
    map_ctx.map = &percpu_lru_hash_map;
    map_ctx.sum = 0;
    bpf_loop(nr_cpus, read_percpu_elem_callback, &map_ctx, 0);
    percpu_lru_hash_elem_sum = map_ctx.sum;
    return 0;
    }
    char _license[] SEC("license") = "GPL";
