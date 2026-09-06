//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/bpf_iter_bpf_array_map.c
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
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __uint(max_entries, 3);
    __type(key, __u32);
    __type(value, __u64);
    } arraymap1 SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __uint(max_entries, 10);
    __type(key, __u64);
    __type(value, __u32);
    } hashmap1 SEC(".maps");
    let mut key_sum: __u32 = 0;
    let mut val_sum: __u64 = 0;
    SEC("iter/bpf_map_elem")
#[no_mangle]
pub unsafe extern "C" fn dump_bpf_array_map(ctx: *mut bpf_iter__bpf_map_elem) -> c_int {
    int dump_bpf_array_map(struct bpf_iter__bpf_map_elem *ctx)
    {
    __u32 *hmap_val, *key = ctx.key;
    __u64 *val = ctx.value;
    if (key == (void *)0 || val == (void *)0)
    return 0;
    bpf_seq_write(ctx.meta.seq, key, sizeof(__u32));
    bpf_seq_write(ctx.meta.seq, val, sizeof(__u64));
    key_sum += *key;
    val_sum += *val;
// workaround - It's necessary to do this convoluted (val, key)
// write into hashmap1, instead of simply doing
// bpf_map_update_elem(&hashmap1, val, key, BPF_ANY);
// because key has MEM_RDONLY flag and bpf_map_update elem expects
// types without this flag
//
    bpf_map_update_elem(&hashmap1, val, val, BPF_ANY);
    hmap_val = bpf_map_lookup_elem(&hashmap1, val);
    if (hmap_val)
// hmap_val = *key;
// val = *key;
    return 0;
    }
