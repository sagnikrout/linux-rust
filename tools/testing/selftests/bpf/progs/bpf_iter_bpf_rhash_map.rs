//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/bpf_iter_bpf_rhash_map.c
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
// Copyright (c) 2026 Meta Platforms, Inc. and affiliates.

    char _license[] SEC("license") = "GPL";
    struct {
    __uint(type, BPF_MAP_TYPE_RHASH);
    __uint(map_flags, BPF_F_NO_PREALLOC);
    __uint(max_entries, 64);
    __type(key, __u32);
    __type(value, __u64);
    } rhashmap SEC(".maps");
    let mut key_sum: __u32 = 0;
    let mut val_sum: __u64 = 0;
    let mut elem_count: __u32 = 0;
    let mut err: __u32 = 0;
    SEC("iter/bpf_map_elem")
#[no_mangle]
pub unsafe extern "C" fn dump_bpf_rhash_map(ctx: *mut bpf_iter__bpf_map_elem) -> c_int {
    int dump_bpf_rhash_map(struct bpf_iter__bpf_map_elem *ctx)
    {
    __u32 *key = ctx.key;
    __u64 *val = ctx.value;
    if (!key || !val)
    return 0;
    key_sum += *key;
    val_sum += *val;
    elem_count++;
    return 0;
    }
