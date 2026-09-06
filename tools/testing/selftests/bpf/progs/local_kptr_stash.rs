//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/local_kptr_stash.c
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
// Copyright (c) 2023 Meta Platforms, Inc. and affiliates.

    struct plain_local;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct node_data {
    pub key: c_long,
    pub data: c_long,
    pub stashed_in_local_kptr: *mut *mut plain_local __kptr,
    pub node: bpf_rb_node,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct refcounted_node {
    pub data: c_long,
    pub rb_node: bpf_rb_node,
    pub refcount: bpf_refcount,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stash {
    pub l: bpf_spin_lock,
    pub stashed: *mut refcounted_node __kptr,
}

    struct {
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __type(key, int);
    __type(value, struct stash);
    __uint(max_entries, 10);
    } refcounted_node_stash SEC(".maps");
#[repr(C)]
#[derive(Copy, Clone)]
pub struct plain_local {
    pub key: c_long,
    pub data: c_long,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct local_with_root {
    pub key: c_long,
    pub l: bpf_spin_lock,
    pub node): bpf_rb_root r __contains(node_data,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct map_value {
    pub not_kptr: *mut prog_test_ref_kfunc,
    pub val: *mut prog_test_ref_kfunc __kptr,
    pub node: *mut node_data __kptr,
    pub plain: *mut plain_local __kptr,
    pub local_root: *mut local_with_root __kptr,
}

// This is necessary so that LLVM generates BTF for node_data struct
// If it's not included, a fwd reference for node_data will be generated but
// no struct. Example BTF of "node" field in map_value when not included:
//
// [10] PTR '(anon)' type_id=35
// [34] FWD 'node_data' fwd_kind=struct
// [35] TYPE_TAG 'kptr_ref' type_id=34
//
// (with no node_data struct defined)
// Had to do the same w/ bpf_kfunc_call_test_release below
//
    struct node_data *just_here_because_btf_bug;
    struct refcounted_node *just_here_because_btf_bug2;
    struct {
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __type(key, int);
    __type(value, struct map_value);
    __uint(max_entries, 2);
    } some_nodes SEC(".maps");
#[no_mangle]
unsafe extern "C" fn less(a: *mut bpf_rb_node, b: *const bpf_rb_node) -> bool {
    static bool less(struct bpf_rb_node *a, const struct bpf_rb_node *b)
    {
    struct node_data *node_a;
    struct node_data *node_b;
    node_a = container_of(a, struct node_data, node);
    node_b = container_of(b, struct node_data, node);
    return node_a.key < node_b.key;
    }
#[no_mangle]
unsafe extern "C" fn create_and_stash(idx: c_int, val: c_int) -> c_int {
    static int create_and_stash(int idx, int val)
    {
    struct plain_local *inner_local_kptr;
    struct map_value *mapval;
    struct node_data *res;
    mapval = bpf_map_lookup_elem(&some_nodes, &idx);
    if (!mapval)
    return 1;
    inner_local_kptr = bpf_obj_new(typeof(*inner_local_kptr));
    if (!inner_local_kptr)
    return 2;
    res = bpf_obj_new(typeof(*res));
    if (!res) {
    bpf_obj_drop(inner_local_kptr);
    return 3;
    }
    res.key = val;
    inner_local_kptr = bpf_kptr_xchg(&res.stashed_in_local_kptr, inner_local_kptr);
    if (inner_local_kptr) {
// Should never happen, we just obj_new'd res
    bpf_obj_drop(inner_local_kptr);
    bpf_obj_drop(res);
    return 4;
    }
    res = bpf_kptr_xchg(&mapval.node, res);
    if (res)
    bpf_obj_drop(res);
    return 0;
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn stash_rb_nodes(ctx: *mut c_void) -> c_long {
    long stash_rb_nodes(void *ctx)
    {
    return create_and_stash(0, 41) ?: create_and_stash(1, 42);
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn stash_plain(ctx: *mut c_void) -> c_long {
    long stash_plain(void *ctx)
    {
    struct map_value *mapval;
    struct plain_local *res;
    let mut idx: c_int = 0;
    mapval = bpf_map_lookup_elem(&some_nodes, &idx);
    if (!mapval)
    return 1;
    res = bpf_obj_new(typeof(*res));
    if (!res)
    return 1;
    res.key = 41;
    res = bpf_kptr_xchg(&mapval.plain, res);
    if (res)
    bpf_obj_drop(res);
    return 0;
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn stash_local_with_root(ctx: *mut c_void) -> c_long {
    long stash_local_with_root(void *ctx)
    {
    struct local_with_root *res;
    struct map_value *mapval;
    struct node_data *n;
    let mut idx: c_int = 0;
    mapval = bpf_map_lookup_elem(&some_nodes, &idx);
    if (!mapval)
    return 1;
    res = bpf_obj_new(typeof(*res));
    if (!res)
    return 2;
    res.key = 41;
    n = bpf_obj_new(typeof(*n));
    if (!n) {
    bpf_obj_drop(res);
    return 3;
    }
    bpf_spin_lock(&res.l);
    bpf_rbtree_add(&res.r, &n.node, less);
    bpf_spin_unlock(&res.l);
    res = bpf_kptr_xchg(&mapval.local_root, res);
    if (res) {
    bpf_obj_drop(res);
    return 4;
    }
    return 0;
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn unstash_rb_node(ctx: *mut c_void) -> c_long {
    long unstash_rb_node(void *ctx)
    {
    struct plain_local *inner_local_kptr = core::ptr::null_mut();
    struct map_value *mapval;
    struct node_data *res;
    long retval;
    let mut key: c_int = 1;
    mapval = bpf_map_lookup_elem(&some_nodes, &key);
    if (!mapval)
    return 1;
    res = bpf_kptr_xchg(&mapval.node, core::ptr::null_mut());
    if (res) {
    inner_local_kptr = bpf_kptr_xchg(&res.stashed_in_local_kptr, inner_local_kptr);
    if (!inner_local_kptr) {
    bpf_obj_drop(res);
    return 1;
    }
    bpf_obj_drop(inner_local_kptr);
    retval = res.key;
    bpf_obj_drop(res);
    return retval;
    }
    return 1;
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn stash_test_ref_kfunc(ctx: *mut c_void) -> c_long {
    long stash_test_ref_kfunc(void *ctx)
    {
    struct prog_test_ref_kfunc *res;
    struct map_value *mapval;
    let mut key: c_int = 0;
    mapval = bpf_map_lookup_elem(&some_nodes, &key);
    if (!mapval)
    return 1;
    res = bpf_kptr_xchg(&mapval.val, core::ptr::null_mut());
    if (res)
    bpf_kfunc_call_test_release(res);
    return 0;
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn refcount_acquire_without_unstash(ctx: *mut c_void) -> c_long {
    long refcount_acquire_without_unstash(void *ctx)
    {
    struct refcounted_node *p;
    struct stash *s;
    let mut ret: c_int = 0;
    s = bpf_map_lookup_elem(&refcounted_node_stash, &ret);
    if (!s)
    return 1;
    if (!s.stashed)
// refcount_acquire failure is expected when no refcounted_node
// has been stashed before this program executes
//
    return 2;
    p = bpf_refcount_acquire(s.stashed);
    if (!p)
    return 3;
    ret = s.stashed ? s.stashed.data : -1;
    bpf_obj_drop(p);
    return ret;
    }
// Helper for refcount_acquire_without_unstash test
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn stash_refcounted_node(ctx: *mut c_void) -> c_long {
    long stash_refcounted_node(void *ctx)
    {
    struct refcounted_node *p;
    struct stash *s;
    let mut key: c_int = 0;
    s = bpf_map_lookup_elem(&refcounted_node_stash, &key);
    if (!s)
    return 1;
    p = bpf_obj_new(typeof(*p));
    if (!p)
    return 2;
    p.data = 42;
    p = bpf_kptr_xchg(&s.stashed, p);
    if (p) {
    bpf_obj_drop(p);
    return 3;
    }
    return 0;
    }
    char _license[] SEC("license") = "GPL";
