//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/map_kptr_race.c
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct map_value {
    pub ref_ptr: *mut prog_test_ref_kfunc __kptr,
}

    struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __uint(map_flags, BPF_F_NO_PREALLOC);
    __type(key, int);
    __type(value, struct map_value);
    __uint(max_entries, 1);
    } race_hash_map SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_PERCPU_HASH);
    __uint(map_flags, BPF_F_NO_PREALLOC);
    __type(key, int);
    __type(value, struct map_value);
    __uint(max_entries, 1);
    } race_percpu_hash_map SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_SK_STORAGE);
    __uint(map_flags, BPF_F_NO_PREALLOC);
    __type(key, int);
    __type(value, struct map_value);
    } race_sk_ls_map SEC(".maps");
    int num_of_refs;
    int sk_ls_leak_done;
    int target_map_id;
    int map_freed;
    const volatile int nr_cpus;
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn test_htab_leak(skb: *mut __sk_buff) -> c_int {
    int test_htab_leak(struct __sk_buff *skb)
    {
    struct prog_test_ref_kfunc *p, *old;
    let mut val: map_value = {};
    struct map_value *v;
    let mut key: c_int = 0;
    if (bpf_map_update_elem(&race_hash_map, &key, &val, BPF_ANY))
    return 1;
    v = bpf_map_lookup_elem(&race_hash_map, &key);
    if (!v)
    return 2;
    p = bpf_kfunc_call_test_acquire(&(unsigned long){0});
    if (!p)
    return 3;
    old = bpf_kptr_xchg(&v.ref_ptr, p);
    if (old)
    bpf_kfunc_call_test_release(old);
    bpf_map_delete_elem(&race_hash_map, &key);
    p = bpf_kfunc_call_test_acquire(&(unsigned long){0});
    if (!p)
    return 4;
    old = bpf_kptr_xchg(&v.ref_ptr, p);
    if (old)
    bpf_kfunc_call_test_release(old);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn fill_percpu_kptr(v: *mut map_value) -> c_int {
    static int fill_percpu_kptr(struct map_value *v)
    {
    struct prog_test_ref_kfunc *p, *old;
    p = bpf_kfunc_call_test_acquire(&(unsigned long){0});
    if (!p)
    return 1;
    old = bpf_kptr_xchg(&v.ref_ptr, p);
    if (old)
    bpf_kfunc_call_test_release(old);
    return 0;
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn test_percpu_htab_leak(skb: *mut __sk_buff) -> c_int {
    int test_percpu_htab_leak(struct __sk_buff *skb)
    {
    struct map_value *v, *arr[16] = {};
    let mut val: map_value = {};
    let mut key: c_int = 0;
    let mut err: c_int = 0;
    if (bpf_map_update_elem(&race_percpu_hash_map, &key, &val, BPF_ANY))
    return 1;
    for (int i = 0; i < nr_cpus; i++) {
    v = bpf_map_lookup_percpu_elem(&race_percpu_hash_map, &key, i);
    if (!v)
    return 2;
    arr[i] = v;
    }
    bpf_map_delete_elem(&race_percpu_hash_map, &key);
    for (int i = 0; i < nr_cpus; i++) {
    v = arr[i];
    err = fill_percpu_kptr(v);
    if (err)
    return 3;
    }
    return 0;
    }
    SEC("tp_btf/inet_sock_set_state")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: test_sk_ls_leak, sk: *mut sock, oldstate: c_int, newstate: c_int) -> c_int {
    int BPF_PROG(test_sk_ls_leak, struct sock *sk, int oldstate, int newstate)
    {
    struct prog_test_ref_kfunc *p, *old;
    struct map_value *v;
    if (newstate != BPF_TCP_SYN_SENT)
    return 0;
    if (sk_ls_leak_done)
    return 0;
    v = bpf_sk_storage_get(&race_sk_ls_map, sk, core::ptr::null_mut(),
    BPF_SK_STORAGE_GET_F_CREATE);
    if (!v)
    return 0;
    p = bpf_kfunc_call_test_acquire(&(unsigned long){0});
    if (!p)
    return 0;
    old = bpf_kptr_xchg(&v.ref_ptr, p);
    if (old)
    bpf_kfunc_call_test_release(old);
    bpf_sk_storage_delete(&race_sk_ls_map, sk);
    p = bpf_kfunc_call_test_acquire(&(unsigned long){0});
    if (!p)
    return 0;
    old = bpf_kptr_xchg(&v.ref_ptr, p);
    if (old)
    bpf_kfunc_call_test_release(old);
    sk_ls_leak_done = 1;
    return 0;
    }
    long target_map_ptr;
    SEC("fentry/bpf_map_put")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: map_put, map: *mut bpf_map) -> c_int {
    int BPF_PROG(map_put, struct bpf_map *map)
    {
    if (target_map_id && map.id == (u32)target_map_id)
    target_map_ptr = (long)map;
    return 0;
    }
    SEC("fexit/htab_map_free")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: htab_map_free, map: *mut bpf_map) -> c_int {
    int BPF_PROG(htab_map_free, struct bpf_map *map)
    {
    if (target_map_ptr && (long)map == target_map_ptr)
    map_freed = 1;
    return 0;
    }
    SEC("fexit/bpf_sk_storage_map_free")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: sk_map_free, map: *mut bpf_map) -> c_int {
    int BPF_PROG(sk_map_free, struct bpf_map *map)
    {
    if (target_map_ptr && (long)map == target_map_ptr)
    map_freed = 1;
    return 0;
    }
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn count_ref(ctx: *mut c_void) -> c_int {
    int count_ref(void *ctx)
    {
    struct prog_test_ref_kfunc *p;
    let mut arg: c_ulong = 0;
    p = bpf_kfunc_call_test_acquire(&arg);
    if (!p)
    return 1;
    num_of_refs = p.cnt.refs.counter;
    bpf_kfunc_call_test_release(p);
    return 0;
    }
    char _license[] SEC("license") = "GPL";
