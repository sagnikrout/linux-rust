//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/bpf_iter_bpf_percpu_array_map.c
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

    char _license[] SEC("license") = "GPL";
    struct {
    __uint(type, BPF_MAP_TYPE_PERCPU_ARRAY);
    __uint(max_entries, 3);
    __type(key, __u32);
    __type(value, __u32);
    } arraymap1 SEC(".maps");
// will set before prog run
    let mut num_cpus: volatile __u32 = 0;
    let mut key_sum: __u32 = 0, val_sum = 0;
    SEC("iter/bpf_map_elem")
#[no_mangle]
pub unsafe extern "C" fn dump_bpf_percpu_array_map(ctx: *mut bpf_iter__bpf_map_elem) -> c_int {
    int dump_bpf_percpu_array_map(struct bpf_iter__bpf_map_elem *ctx)
    {
    __u32 *key = ctx.key;
    void *pptr = ctx.value;
    __u32 step;
    int i;
    if (key == (void *)0 || pptr == (void *)0)
    return 0;
    key_sum += *key;
    step = 8;
    for (i = 0; i < num_cpus; i++) {
    val_sum += *(__u32 *)pptr;
    pptr += step;
    }
    return 0;
    }
