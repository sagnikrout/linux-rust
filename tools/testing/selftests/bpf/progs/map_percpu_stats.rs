//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/map_percpu_stats.c
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

    __u32 target_id;
    __s64 bpf_map_sum_elem_count(const struct bpf_map *map) __ksym;
    SEC("iter/bpf_map")
#[no_mangle]
pub unsafe extern "C" fn dump_bpf_map(ctx: *mut bpf_iter__bpf_map) -> c_int {
    int dump_bpf_map(struct bpf_iter__bpf_map *ctx)
    {
    struct seq_file *seq = ctx.meta.seq;
    struct bpf_map *map = ctx.map;
    if (map && map.id == target_id)
    BPF_SEQ_PRINTF(seq, "%lld", bpf_map_sum_elem_count(map));
    return 0;
    }
    char _license[] SEC("license") = "GPL";
