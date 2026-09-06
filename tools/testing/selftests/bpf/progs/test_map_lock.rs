//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_map_lock.c
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
// Copyright (c) 2019 Facebook

pub const VAR_NUM: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hmap_elem {
    pub lock: bpf_spin_lock,
    pub var: [c_int; VAR_NUM],
}

    struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __uint(max_entries, 1);
    __type(key, __u32);
    __type(value, struct hmap_elem);
    } hash_map SEC(".maps");
#[repr(C)]
#[derive(Copy, Clone)]
pub struct array_elem {
    pub lock: bpf_spin_lock,
    pub var: [c_int; VAR_NUM],
}

    struct {
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __uint(max_entries, 1);
    __type(key, int);
    __type(value, struct array_elem);
    } array_map SEC(".maps");
    SEC("cgroup/skb")
#[no_mangle]
pub unsafe extern "C" fn bpf_map_lock_test(skb: *mut __sk_buff) -> c_int {
    int bpf_map_lock_test(struct __sk_buff *skb)
    {
    struct hmap_elem *val;
    let mut rnd: c_int = bpf_get_prandom_u32();
    let mut key: c_int = 0, err = 1, i;
    struct array_elem *q;
    val = bpf_map_lookup_elem(&hash_map, &key);
    if (!val)
    goto err;
// spin_lock in hash map
    bpf_spin_lock(&val.lock);
    for (i = 0; i < VAR_NUM; i++)
    val.var[i] = rnd;
    bpf_spin_unlock(&val.lock);
// spin_lock in array
    q = bpf_map_lookup_elem(&array_map, &key);
    if (!q)
    goto err;
    bpf_spin_lock(&q.lock);
    for (i = 0; i < VAR_NUM; i++)
    q.var[i] = rnd;
    bpf_spin_unlock(&q.lock);
    err = 0;
    err:
    return err;
    }
    char _license[] SEC("license") = "GPL";
