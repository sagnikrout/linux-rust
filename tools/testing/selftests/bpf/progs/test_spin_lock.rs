//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_spin_lock.c
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hmap_elem {
    pub cnt: volatile int,
    pub lock: bpf_spin_lock,
    pub test_padding: c_int,
}

    struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __uint(max_entries, 1);
    __type(key, int);
    __type(value, struct hmap_elem);
    } hmap SEC(".maps");
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cls_elem {
    pub lock: bpf_spin_lock,
    pub cnt: volatile int,
}

    struct {
    __uint(type, BPF_MAP_TYPE_CGROUP_STORAGE);
    __type(key, struct bpf_cgroup_storage_key);
    __type(value, struct cls_elem);
    } cls_map SEC(".maps");
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_vqueue {
    pub lock: bpf_spin_lock,
// 4 byte hole
    pub lasttime: c_ulonglong,
    pub credit: c_int,
    pub rate: c_uint,
}

    struct {
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __uint(max_entries, 1);
    __type(key, int);
    __type(value, struct bpf_vqueue);
    } vqueue SEC(".maps");

    SEC("cgroup_skb/ingress")
#[no_mangle]
pub unsafe extern "C" fn bpf_spin_lock_test(skb: *mut __sk_buff) -> c_int {
    int bpf_spin_lock_test(struct __sk_buff *skb)
    {
    let mut credit: volatile int = 0, max_credit = 100, pkt_len = 64;
    let mut zero: hmap_elem = {}, *val;
    unsigned long long curtime;
    struct bpf_vqueue *q;
    struct cls_elem *cls;
    let mut key: c_int = 0;
    let mut err: c_int = 0;
    val = bpf_map_lookup_elem(&hmap, &key);
    if (!val) {
    bpf_map_update_elem(&hmap, &key, &zero, 0);
    val = bpf_map_lookup_elem(&hmap, &key);
    if (!val) {
    err = 1;
    goto err;
    }
    }
// spin_lock in hash map run time test
    bpf_spin_lock(&val.lock);
    if (val.cnt)
    val.cnt--;
    else
    val.cnt++;
    if (val.cnt != 0 && val.cnt != 1)
    err = 1;
    bpf_spin_unlock(&val.lock);
// spin_lock in array. virtual queue demo
    q = bpf_map_lookup_elem(&vqueue, &key);
    if (!q)
    goto err;
    curtime = bpf_ktime_get_ns();
    bpf_spin_lock(&q.lock);
    q.credit += CREDIT_PER_NS(curtime - q.lasttime, q.rate);
    q.lasttime = curtime;
    if (q.credit > max_credit)
    q.credit = max_credit;
    q.credit -= pkt_len;
    credit = q.credit;
    bpf_spin_unlock(&q.lock);
    __sink(credit);
// spin_lock in cgroup local storage
    cls = bpf_get_local_storage(&cls_map, 0);
    bpf_spin_lock(&cls.lock);
    cls.cnt++;
    bpf_spin_unlock(&cls.lock);
    err:
    return err;
    }
    struct bpf_spin_lock lockA __hidden SEC(".data.A");
    __noinline
#[no_mangle]
unsafe extern "C" fn static_subprog(ctx: *mut __sk_buff) -> c_int {
    static int static_subprog(struct __sk_buff *ctx)
    {
    let mut ret: volatile int = 0;
    if (ctx.protocol)
    return ret;
    return ret + ctx.len;
    }
    __noinline
#[no_mangle]
unsafe extern "C" fn static_subprog_lock(ctx: *mut __sk_buff) -> c_int {
    static int static_subprog_lock(struct __sk_buff *ctx)
    {
    let mut ret: volatile int = 0;
    ret = static_subprog(ctx);
    bpf_spin_lock(&lockA);
    return ret + ctx.len;
    }
    __noinline
#[no_mangle]
unsafe extern "C" fn static_subprog_unlock(ctx: *mut __sk_buff) -> c_int {
    static int static_subprog_unlock(struct __sk_buff *ctx)
    {
    let mut ret: volatile int = 0;
    ret = static_subprog(ctx);
    bpf_spin_unlock(&lockA);
    return ret + ctx.len;
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn lock_static_subprog_call(ctx: *mut __sk_buff) -> c_int {
    int lock_static_subprog_call(struct __sk_buff *ctx)
    {
    let mut ret: c_int = 0;
    bpf_spin_lock(&lockA);
    if (ctx.mark == 42)
    ret = static_subprog(ctx);
    bpf_spin_unlock(&lockA);
    return ret;
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn lock_static_subprog_lock(ctx: *mut __sk_buff) -> c_int {
    int lock_static_subprog_lock(struct __sk_buff *ctx)
    {
    let mut ret: c_int = 0;
    ret = static_subprog_lock(ctx);
    bpf_spin_unlock(&lockA);
    return ret;
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn lock_static_subprog_unlock(ctx: *mut __sk_buff) -> c_int {
    int lock_static_subprog_unlock(struct __sk_buff *ctx)
    {
    let mut ret: c_int = 0;
    bpf_spin_lock(&lockA);
    ret = static_subprog_unlock(ctx);
    return ret;
    }
    char _license[] SEC("license") = "GPL";
