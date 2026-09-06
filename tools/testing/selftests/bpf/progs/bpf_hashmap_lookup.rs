//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/bpf_hashmap_lookup.c
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
// Copyright (c) 2023 Isovalent

    char _license[] SEC("license") = "GPL";
    struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    } hash_map_bench SEC(".maps");
// The number of slots to store times
pub const NR_SLOTS: c_int = 32;
pub const NR_CPUS: c_int = 256;

// Configured by userspace
    u64 nr_entries;
    u64 nr_loops;
    u32 __attribute__((__aligned__(8))) key[NR_CPUS];
// Filled by us
    u64 __attribute__((__aligned__(256))) percpu_times_index[NR_CPUS];
    u64 __attribute__((__aligned__(256))) percpu_times[NR_CPUS][NR_SLOTS];
#[no_mangle]
pub unsafe extern "C" fn patch_key(i: u32) {
    static inline void patch_key(u32 i)
    {

    key[0] = i + 1;

    key[0] = __builtin_bswap32(i + 1);

// the rest of key is random and is configured by userspace
    }
#[no_mangle]
unsafe extern "C" fn lookup_callback(index: __u32, unused: *mut u32) -> c_int {
    static int lookup_callback(__u32 index, u32 *unused)
    {
    patch_key(index);
    return bpf_map_lookup_elem(&hash_map_bench, key) ? 0 : 1;
    }
#[no_mangle]
unsafe extern "C" fn loop_lookup_callback(index: __u32, unused: *mut u32) -> c_int {
    static int loop_lookup_callback(__u32 index, u32 *unused)
    {
    return bpf_loop(nr_entries, lookup_callback, core::ptr::null_mut(), 0) ? 0 : 1;
    }
    SEC("fentry/" SYS_PREFIX "sys_getpgid")
#[no_mangle]
pub unsafe extern "C" fn benchmark(ctx: *mut c_void) -> c_int {
    int benchmark(void *ctx)
    {
    let mut cpu: u32 = bpf_get_smp_processor_id();
    u32 times_index;
    u64 start_time;
    times_index = percpu_times_index[cpu & CPU_MASK] % NR_SLOTS;
    start_time = bpf_ktime_get_ns();
    bpf_loop(nr_loops, loop_lookup_callback, core::ptr::null_mut(), 0);
    percpu_times[cpu & CPU_MASK][times_index] = bpf_ktime_get_ns() - start_time;
    percpu_times_index[cpu & CPU_MASK] += 1;
    return 0;
    }
