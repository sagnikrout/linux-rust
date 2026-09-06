//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/bpf_hashmap_full_update_bench.c
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

    char _license[] SEC("license") = "GPL";
pub const MAX_ENTRIES: c_int = 1000;
    struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __type(key, u32);
    __type(value, u64);
    __uint(max_entries, MAX_ENTRIES);
    } hash_map_bench SEC(".maps");
    u64 __attribute__((__aligned__(256))) percpu_time[256];
    u64 nr_loops;
#[no_mangle]
unsafe extern "C" fn loop_update_callback(index: __u32, key: *mut u32) -> c_int {
    static int loop_update_callback(__u32 index, u32 *key)
    {
    let mut init_val: u64 = 1;
    bpf_map_update_elem(&hash_map_bench, key, &init_val, BPF_ANY);
    return 0;
    }
    SEC("fentry/" SYS_PREFIX "sys_getpgid")
#[no_mangle]
pub unsafe extern "C" fn benchmark(ctx: *mut c_void) -> c_int {
    int benchmark(void *ctx)
    {
    let mut cpu: u32 = bpf_get_smp_processor_id();
    let mut key: u32 = cpu + MAX_ENTRIES;
    let mut start_time: u64 = bpf_ktime_get_ns();
    bpf_loop(nr_loops, loop_update_callback, &key, 0);
    percpu_time[cpu & 255] = bpf_ktime_get_ns() - start_time;
    return 0;
    }
