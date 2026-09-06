//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/rbtree.c
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
// Copyright (c) 2022 Meta Platforms, Inc. and affiliates.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct node_data {
    pub key: c_long,
    pub data: c_long,
    pub node: bpf_rb_node,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct root_nested_inner {
    pub glock: bpf_spin_lock,
    pub node): bpf_rb_root root __contains(node_data,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct root_nested {
    pub inner: root_nested_inner,
}

    let mut less_callback_ran: c_long = -1;
    let mut removed_key: c_long = -1;
    long first_data[2] = {-1, -1};

    private(A) struct bpf_spin_lock glock;
    private(A) struct bpf_rb_root groot __contains(node_data, node);
    private(A) struct bpf_rb_root groot_array[2] __contains(node_data, node);
    private(A) struct bpf_rb_root groot_array_one[1] __contains(node_data, node);
    private(B) struct root_nested groot_nested;
#[no_mangle]
unsafe extern "C" fn less(a: *mut bpf_rb_node, b: *const bpf_rb_node) -> bool {
    static bool less(struct bpf_rb_node *a, const struct bpf_rb_node *b)
    {
    struct node_data *node_a;
    struct node_data *node_b;
    node_a = container_of(a, struct node_data, node);
    node_b = container_of(b, struct node_data, node);
    less_callback_ran = 1;
    return node_a.key < node_b.key;
    }
#[no_mangle]
unsafe extern "C" fn __add_three(root: *mut bpf_rb_root, lock: *mut bpf_spin_lock) -> c_long {
    static long __add_three(struct bpf_rb_root *root, struct bpf_spin_lock *lock)
    {
    struct node_data *n, *m;
    n = bpf_obj_new(typeof(*n));
    if (!n)
    return 1;
    n.key = 5;
    m = bpf_obj_new(typeof(*m));
    if (!m) {
    bpf_obj_drop(n);
    return 2;
    }
    m.key = 1;
    bpf_spin_lock(lock);
    bpf_rbtree_add(root, &n.node, less);
    bpf_rbtree_add(root, &m.node, less);
    bpf_spin_unlock(lock);
    n = bpf_obj_new(typeof(*n));
    if (!n)
    return 3;
    n.key = 3;
    bpf_spin_lock(lock);
    bpf_rbtree_add(root, &n.node, less);
    bpf_spin_unlock(lock);
    return 0;
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn rbtree_add_nodes(ctx: *mut c_void) -> c_long {
    long rbtree_add_nodes(void *ctx)
    {
    return __add_three(&groot, &glock);
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn rbtree_add_nodes_nested(ctx: *mut c_void) -> c_long {
    long rbtree_add_nodes_nested(void *ctx)
    {
    return __add_three(&groot_nested.inner.root, &groot_nested.inner.glock);
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn rbtree_add_and_remove(ctx: *mut c_void) -> c_long {
    long rbtree_add_and_remove(void *ctx)
    {
    struct bpf_rb_node *res = core::ptr::null_mut();
    struct node_data *n, *m = core::ptr::null_mut();
    n = bpf_obj_new(typeof(*n));
    if (!n)
    goto err_out;
    n.key = 5;
    m = bpf_obj_new(typeof(*m));
    if (!m)
    goto err_out;
    m.key = 3;
    bpf_spin_lock(&glock);
    bpf_rbtree_add(&groot, &n.node, less);
    bpf_rbtree_add(&groot, &m.node, less);
    res = bpf_rbtree_remove(&groot, &n.node);
    bpf_spin_unlock(&glock);
    if (!res)
    return 1;
    n = container_of(res, struct node_data, node);
    removed_key = n.key;
    bpf_obj_drop(n);
    return 0;
    err_out:
    if (n)
    bpf_obj_drop(n);
    if (m)
    bpf_obj_drop(m);
    return 1;
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn rbtree_add_and_remove_array(ctx: *mut c_void) -> c_long {
    long rbtree_add_and_remove_array(void *ctx)
    {
    struct bpf_rb_node *res1 = core::ptr::null_mut(), *res2 = core::ptr::null_mut(), *res3 = core::ptr::null_mut();
    struct node_data *nodes[3][2] = {{core::ptr::null_mut(), core::ptr::null_mut()}, {core::ptr::null_mut(), core::ptr::null_mut()}, {core::ptr::null_mut(), core::ptr::null_mut()}};
    struct node_data *n;
    let mut k1: c_long = -1, k2 = -1, k3 = -1;
    int i, j;
    for (i = 0; i < 3; i++) {
    for (j = 0; j < 2; j++) {
    nodes[i][j] = bpf_obj_new(typeof(*nodes[i][j]));
    if (!nodes[i][j])
    goto err_out;
    nodes[i][j].key = i * 2 + j;
    }
    }
    bpf_spin_lock(&glock);
    for (i = 0; i < 2; i++)
    for (j = 0; j < 2; j++)
    bpf_rbtree_add(&groot_array[i], &nodes[i][j].node, less);
    for (j = 0; j < 2; j++)
    bpf_rbtree_add(&groot_array_one[0], &nodes[2][j].node, less);
    res1 = bpf_rbtree_remove(&groot_array[0], &nodes[0][0].node);
    res2 = bpf_rbtree_remove(&groot_array[1], &nodes[1][0].node);
    res3 = bpf_rbtree_remove(&groot_array_one[0], &nodes[2][0].node);
    bpf_spin_unlock(&glock);
    if (res1) {
    n = container_of(res1, struct node_data, node);
    k1 = n.key;
    bpf_obj_drop(n);
    }
    if (res2) {
    n = container_of(res2, struct node_data, node);
    k2 = n.key;
    bpf_obj_drop(n);
    }
    if (res3) {
    n = container_of(res3, struct node_data, node);
    k3 = n.key;
    bpf_obj_drop(n);
    }
    if (k1 != 0 || k2 != 2 || k3 != 4)
    return 2;
    return 0;
    err_out:
    for (i = 0; i < 3; i++) {
    for (j = 0; j < 2; j++) {
    if (nodes[i][j])
    bpf_obj_drop(nodes[i][j]);
    }
    }
    return 1;
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn rbtree_first_and_remove(ctx: *mut c_void) -> c_long {
    long rbtree_first_and_remove(void *ctx)
    {
    struct bpf_rb_node *res = core::ptr::null_mut();
    struct node_data *n, *m, *o;
    n = bpf_obj_new(typeof(*n));
    if (!n)
    return 1;
    n.key = 3;
    n.data = 4;
    m = bpf_obj_new(typeof(*m));
    if (!m)
    goto err_out;
    m.key = 5;
    m.data = 6;
    o = bpf_obj_new(typeof(*o));
    if (!o)
    goto err_out;
    o.key = 1;
    o.data = 2;
    bpf_spin_lock(&glock);
    bpf_rbtree_add(&groot, &n.node, less);
    bpf_rbtree_add(&groot, &m.node, less);
    bpf_rbtree_add(&groot, &o.node, less);
    res = bpf_rbtree_first(&groot);
    if (!res) {
    bpf_spin_unlock(&glock);
    return 2;
    }
    o = container_of(res, struct node_data, node);
    first_data[0] = o.data;
    res = bpf_rbtree_remove(&groot, &o.node);
    bpf_spin_unlock(&glock);
    if (!res)
    return 5;
    o = container_of(res, struct node_data, node);
    removed_key = o.key;
    bpf_obj_drop(o);
    bpf_spin_lock(&glock);
    res = bpf_rbtree_first(&groot);
    if (!res) {
    bpf_spin_unlock(&glock);
    return 3;
    }
    o = container_of(res, struct node_data, node);
    first_data[1] = o.data;
    bpf_spin_unlock(&glock);
    return 0;
    err_out:
    if (n)
    bpf_obj_drop(n);
    if (m)
    bpf_obj_drop(m);
    return 1;
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn rbtree_api_release_aliasing(ctx: *mut c_void) -> c_long {
    long rbtree_api_release_aliasing(void *ctx)
    {
    struct node_data *n, *m, *o;
    struct bpf_rb_node *res, *res2;
    n = bpf_obj_new(typeof(*n));
    if (!n)
    return 1;
    n.key = 41;
    n.data = 42;
    bpf_spin_lock(&glock);
    bpf_rbtree_add(&groot, &n.node, less);
    bpf_spin_unlock(&glock);
    bpf_spin_lock(&glock);
// m and o point to the same node,
// but verifier doesn't know this
//
    res = bpf_rbtree_first(&groot);
    if (!res)
    goto err_out;
    o = container_of(res, struct node_data, node);
    res = bpf_rbtree_first(&groot);
    if (!res)
    goto err_out;
    m = container_of(res, struct node_data, node);
    res = bpf_rbtree_remove(&groot, &m.node);
// Retval of previous remove returns an owning reference to m,
// which is the same node non-owning ref o is pointing at.
// We can safely try to remove o as the second rbtree_remove will
// return NULL since the node isn't in a tree.
//
// Previously we relied on the verifier type system + rbtree_remove
// invalidating non-owning refs to ensure that rbtree_remove couldn't
// fail, but now rbtree_remove does runtime checking so we no longer
// invalidate non-owning refs after remove.
//
    res2 = bpf_rbtree_remove(&groot, &o.node);
    bpf_spin_unlock(&glock);
    if (res) {
    o = container_of(res, struct node_data, node);
    first_data[0] = o.data;
    bpf_obj_drop(o);
    }
    if (res2) {
// The second remove fails, so res2 is null and this doesn't
// execute
//
    m = container_of(res2, struct node_data, node);
    first_data[1] = m.data;
    bpf_obj_drop(m);
    }
    return 0;
    err_out:
    bpf_spin_unlock(&glock);
    return 1;
    }
    char _license[] SEC("license") = "GPL";
