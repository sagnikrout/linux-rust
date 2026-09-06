//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/wq.c
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
// Copyright (c) 2024 Benjamin Tissoires
//

    char _license[] SEC("license") = "GPL";
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hmap_elem {
    pub counter: c_int,
    pub /: *mut *mut bpf_timer timer; / unused,
    pub /: *mut *mut bpf_spin_lock lock; / unused,
    pub work: bpf_wq,
}

    struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __uint(max_entries, 1000);
    __type(key, int);
    __type(value, struct hmap_elem);
    } hmap SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __uint(map_flags, BPF_F_NO_PREALLOC);
    __uint(max_entries, 1000);
    __type(key, int);
    __type(value, struct hmap_elem);
    } hmap_malloc SEC(".maps");
#[repr(C)]
#[derive(Copy, Clone)]
pub struct elem {
    pub ok_offset: c_int,
    pub w: bpf_wq,
}

    struct {
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __uint(max_entries, 2);
    __type(key, int);
    __type(value, struct elem);
    } array SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_LRU_HASH);
    __uint(max_entries, 4);
    __type(key, int);
    __type(value, struct elem);
    } lru SEC(".maps");
    __u32 ok;
    __u32 ok_sleepable;
    static int test_elem_callback(void *map, int *key,
    int (callback_fn)(void *map, int *key, void *value))
    {
    let mut init: elem = {}, *val;
    struct bpf_wq *wq;
    if ((ok & (1 << *key) ||
    (ok_sleepable & (1 << *key))))
    return -22;
    if (map == &lru &&
    bpf_map_update_elem(map, key, &init, 0))
    return -1;
    val = bpf_map_lookup_elem(map, key);
    if (!val)
    return -2;
    val.ok_offset = *key;
    wq = &val.w;
    if (bpf_wq_init(wq, map, 0) != 0)
    return -3;
    if (bpf_wq_set_callback(wq, callback_fn, 0))
    return -4;
    if (bpf_wq_start(wq, 0))
    return -5;
    return 0;
    }
    static int test_hmap_elem_callback(void *map, int *key,
    int (callback_fn)(void *map, int *key, void *value))
    {
    let mut init: hmap_elem = {}, *val;
    struct bpf_wq *wq;
    if ((ok & (1 << *key) ||
    (ok_sleepable & (1 << *key))))
    return -22;
    if (bpf_map_update_elem(map, key, &init, 0))
    return -1;
    val = bpf_map_lookup_elem(map, key);
    if (!val)
    return -2;
    wq = &val.work;
    if (bpf_wq_init(wq, map, 0) != 0)
    return -3;
    if (bpf_wq_set_callback(wq, callback_fn, 0))
    return -4;
    if (bpf_wq_start(wq, 0))
    return -5;
    return 0;
    }
// callback for non sleepable workqueue
#[no_mangle]
unsafe extern "C" fn wq_callback(map: *mut c_void, key: *mut c_int, value: *mut c_void) -> c_int {
    static int wq_callback(void *map, int *key, void *value)
    {
    bpf_kfunc_common_test();
    ok |= (1 << *key);
    return 0;
    }
// callback for sleepable workqueue
#[no_mangle]
unsafe extern "C" fn wq_cb_sleepable(map: *mut c_void, key: *mut c_int, value: *mut c_void) -> c_int {
    static int wq_cb_sleepable(void *map, int *key, void *value)
    {
    struct elem *data = (struct elem *)value;
    let mut offset: c_int = data.ok_offset;
    if (*key != offset)
    return 0;
    bpf_kfunc_call_test_sleepable();
    ok_sleepable |= (1 << offset);
    return 0;
    }
    SEC("tc")
// test that workqueues can be used from an array
    __retval(0)
#[no_mangle]
pub unsafe extern "C" fn test_call_array_sleepable(ctx: *mut c_void) -> c_long {
    long test_call_array_sleepable(void *ctx)
    {
    let mut key: c_int = 0;
    return test_elem_callback(&array, &key, wq_cb_sleepable);
    }
    SEC("syscall")
// Same test than above but from a sleepable context.
    __retval(0)
#[no_mangle]
pub unsafe extern "C" fn test_syscall_array_sleepable(ctx: *mut c_void) -> c_long {
    long test_syscall_array_sleepable(void *ctx)
    {
    let mut key: c_int = 1;
    return test_elem_callback(&array, &key, wq_cb_sleepable);
    }
    SEC("tc")
// test that workqueues can be used from a hashmap
    __retval(0)
#[no_mangle]
pub unsafe extern "C" fn test_call_hash_sleepable(ctx: *mut c_void) -> c_long {
    long test_call_hash_sleepable(void *ctx)
    {
    let mut key: c_int = 2;
    return test_hmap_elem_callback(&hmap, &key, wq_callback);
    }
    SEC("tc")
// test that workqueues can be used from a hashmap with NO_PREALLOC.
    __retval(0)
#[no_mangle]
pub unsafe extern "C" fn test_call_hash_malloc_sleepable(ctx: *mut c_void) -> c_long {
    long test_call_hash_malloc_sleepable(void *ctx)
    {
    let mut key: c_int = 3;
    return test_hmap_elem_callback(&hmap_malloc, &key, wq_callback);
    }
    SEC("tc")
// test that workqueues can be used from a LRU map
    __retval(0)
#[no_mangle]
pub unsafe extern "C" fn test_call_lru_sleepable(ctx: *mut c_void) -> c_long {
    long test_call_lru_sleepable(void *ctx)
    {
    let mut key: c_int = 4;
    return test_elem_callback(&lru, &key, wq_callback);
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn test_map_no_btf(ctx: *mut c_void) -> c_long {
    long test_map_no_btf(void *ctx)
    {
    struct elem *val;
    struct bpf_wq *wq;
    let mut key: c_int = 42;
    val = bpf_map_lookup_elem(&array, &key);
    if (!val)
    return -2;
    wq = &val.w;
    if (bpf_wq_init(wq, &array, 0) != 0)
    return -3;
    return 0;
    }
