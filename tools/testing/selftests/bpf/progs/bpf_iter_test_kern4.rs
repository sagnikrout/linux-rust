//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/bpf_iter_test_kern4.c
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
    let mut map1_id: __u32 = 0, map2_id = 0;
    let mut map1_accessed: __u32 = 0, map2_accessed = 0;
    let mut map1_seqnum: __u64 = 0, map2_seqnum1 = 0, map2_seqnum2 = 0;
    volatile const __u32 print_len;
    volatile const __u32 ret1;
    SEC("iter/bpf_map")
#[no_mangle]
pub unsafe extern "C" fn dump_bpf_map(ctx: *mut bpf_iter__bpf_map) -> c_int {
    int dump_bpf_map(struct bpf_iter__bpf_map *ctx)
    {
    struct seq_file *seq = ctx.meta.seq;
    struct bpf_map *map = ctx.map;
    __u64 seq_num;
    int i, ret = 0;
    if (map == (void *)0)
    return 0;
// only dump map1_id and map2_id
    if (map.id != map1_id && map.id != map2_id)
    return 0;
    seq_num = ctx.meta.seq_num;
    if (map.id == map1_id) {
    map1_seqnum = seq_num;
    map1_accessed++;
    }
    if (map.id == map2_id) {
    if (map2_accessed == 0) {
    map2_seqnum1 = seq_num;
    if (ret1)
    ret = 1;
    } else {
    map2_seqnum2 = seq_num;
    }
    map2_accessed++;
    }
// fill seq_file buffer
    for (i = 0; i < (int)print_len; i++)
    bpf_seq_write(seq, &seq_num, sizeof(seq_num));
    return ret;
    }
