//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/bpf_iter_bpf_hash_map.c
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
#[repr(C)]
#[derive(Copy, Clone)]
pub struct key_t {
    pub a: c_int,
    pub b: c_int,
    pub c: c_int,
}

    struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __uint(max_entries, 3);
    __type(key, struct key_t);
    __type(value, __u64);
    } hashmap1 SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __uint(max_entries, 3);
    __type(key, __u64);
    __type(value, __u64);
    } hashmap2 SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __uint(max_entries, 3);
    __type(key, struct key_t);
    __type(value, __u32);
    } hashmap3 SEC(".maps");
// will set before prog run
    let mut in_test_mode: bool = 0;
// will collect results during prog run
    let mut key_sum_a: __u32 = 0, key_sum_b = 0, key_sum_c = 0;
    let mut val_sum: __u64 = 0;
    SEC("iter/bpf_map_elem")
#[no_mangle]
pub unsafe extern "C" fn dump_bpf_hash_map(ctx: *mut bpf_iter__bpf_map_elem) -> c_int {
    int dump_bpf_hash_map(struct bpf_iter__bpf_map_elem *ctx)
    {
    struct seq_file *seq = ctx.meta.seq;
    let mut seq_num: __u32 = ctx.meta.seq_num;
    struct bpf_map *map = ctx.map;
    struct key_t *key = ctx.key;
    struct key_t tmp_key;
    __u64 *val = ctx.value;
    let mut tmp_val: __u64 = 0;
    int ret;
    if (in_test_mode) {
// test mode is used by selftests to
// test functionality of bpf_hash_map iter.
//
// the above hashmap1 will have correct size
// and will be accepted, hashmap2 and hashmap3
// should be rejected due to smaller key/value
// size.
//
    if (key == (void *)0 || val == (void *)0)
    return 0;
// update the value and then delete the <key, value> pair.
// it should not impact the existing 'val' which is still
// accessible under rcu.
//
    __builtin_memcpy(&tmp_key, key, sizeof(struct key_t));
    ret = bpf_map_update_elem(&hashmap1, &tmp_key, &tmp_val, 0);
    if (ret)
    return 0;
    ret = bpf_map_delete_elem(&hashmap1, &tmp_key);
    if (ret)
    return 0;
    key_sum_a += key.a;
    key_sum_b += key.b;
    key_sum_c += key.c;
    val_sum += *val;
    return 0;
    }
// non-test mode, the map is prepared with the
// below bpftool command sequence:
// bpftool map create /sys/fs/bpf/m1 type hash \
// key 12 value 8 entries 3 name map1
// bpftool map update id 77 key 0 0 0 1 0 0 0 0 0 0 0 1 \
// value 0 0 0 1 0 0 0 1
// bpftool map update id 77 key 0 0 0 1 0 0 0 0 0 0 0 2 \
// value 0 0 0 1 0 0 0 2
// The bpftool iter command line:
// bpftool iter pin ./bpf_iter_bpf_hash_map.o /sys/fs/bpf/p1 \
// map id 77
// The below output will be:
// map dump starts
// 77: (1000000 0 2000000) (200000001000000)
// 77: (1000000 0 1000000) (100000001000000)
// map dump ends
//
    if (seq_num == 0)
    BPF_SEQ_PRINTF(seq, "map dump starts\n");
    if (key == (void *)0 || val == (void *)0) {
    BPF_SEQ_PRINTF(seq, "map dump ends\n");
    return 0;
    }
    BPF_SEQ_PRINTF(seq, "%d: (%x %d %x) (%llx)\n", map.id,
    key.a, key.b, key.c, *val);
    return 0;
    }
    SEC("iter.s/bpf_map_elem")
#[no_mangle]
pub unsafe extern "C" fn sleepable_dummy_dump(ctx: *mut bpf_iter__bpf_map_elem) -> c_int {
    int sleepable_dummy_dump(struct bpf_iter__bpf_map_elem *ctx)
    {
    if (ctx.meta.seq_num == 0)
    BPF_SEQ_PRINTF(ctx.meta.seq, "map dump starts\n");
    return 0;
    }
