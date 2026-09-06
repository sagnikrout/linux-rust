//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/rbtree_search_kptr.c
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
// Copyright (c) 2026 KylinSoft Corporation.

pub const NR_NODES: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct node_data {
    pub data: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tree_node {
    pub node: bpf_rb_node,
    pub key: u64,
    pub node_data: *mut *mut node_data __kptr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tree_node_ref {
    pub ref: bpf_refcount,
    pub node: bpf_rb_node,
    pub key: u64,
    pub node_data: *mut *mut node_data __kptr,
}

    private(A) struct bpf_rb_root root __contains(tree_node, node);
    private(A) struct bpf_spin_lock lock;
    private(B) struct bpf_rb_root root_r __contains(tree_node_ref, node);
    private(B) struct bpf_spin_lock lock_r;
#[no_mangle]
unsafe extern "C" fn less(a: *mut bpf_rb_node, b: *const bpf_rb_node) -> bool {
    static bool less(struct bpf_rb_node *a, const struct bpf_rb_node *b)
    {
    struct tree_node *node_a, *node_b;
    node_a = container_of(a, struct tree_node, node);
    node_b = container_of(b, struct tree_node, node);
    return node_a.key < node_b.key;
    }
    SEC("syscall")
    __retval(0)
#[no_mangle]
pub unsafe extern "C" fn rbtree_search_kptr(ctx: *mut c_void) -> c_long {
    long rbtree_search_kptr(void *ctx)
    {
    struct tree_node *tnode;
    struct bpf_rb_node *rb_n;
    struct node_data __kptr * node_data;
    let mut lookup_key: c_int = NR_NODES / 2;
    let mut lookup_data: c_int = NR_NODES / 2;
    int i, data, ret = 0;
    for (i = 0; i < NR_NODES && can_loop; i++) {
    tnode = bpf_obj_new(typeof(*tnode));
    if (!tnode)
    return __LINE__;
    node_data = bpf_obj_new(typeof(*node_data));
    if (!node_data) {
    bpf_obj_drop(tnode);
    return __LINE__;
    }
    tnode.key = i;
    node_data.data = i;
    node_data = bpf_kptr_xchg(&tnode.node_data, node_data);
    if (node_data)
    bpf_obj_drop(node_data);
    bpf_spin_lock(&lock);
    bpf_rbtree_add(&root, &tnode.node, less);
    bpf_spin_unlock(&lock);
    }
    bpf_spin_lock(&lock);
    rb_n = bpf_rbtree_root(&root);
    while (rb_n && can_loop) {
    tnode = container_of(rb_n, struct tree_node, node);
    node_data = bpf_kptr_xchg(&tnode.node_data, core::ptr::null_mut());
    if (!node_data) {
    ret = __LINE__;
    goto fail;
    }
    data = node_data.data;
    node_data = bpf_kptr_xchg(&tnode.node_data, node_data);
    if (node_data) {
    bpf_spin_unlock(&lock);
    bpf_obj_drop(node_data);
    return __LINE__;
    }
    if (lookup_key == tnode.key) {
    if (data == lookup_data)
    break;
    ret = __LINE__;
    goto fail;
    }
    if (lookup_key < tnode.key)
    rb_n = bpf_rbtree_left(&root, rb_n);
    else
    rb_n = bpf_rbtree_right(&root, rb_n);
    }
    bpf_spin_unlock(&lock);
    while (can_loop) {
    bpf_spin_lock(&lock);
    rb_n = bpf_rbtree_first(&root);
    if (!rb_n) {
    bpf_spin_unlock(&lock);
    return 0;
    }
    rb_n = bpf_rbtree_remove(&root, rb_n);
    if (!rb_n) {
    ret = __LINE__;
    goto fail;
    }
    bpf_spin_unlock(&lock);
    tnode = container_of(rb_n, struct tree_node, node);
    node_data = bpf_kptr_xchg(&tnode.node_data, core::ptr::null_mut());
    if (node_data)
    bpf_obj_drop(node_data);
    bpf_obj_drop(tnode);
    }
    return 0;
    fail:
    bpf_spin_unlock(&lock);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn less_r(a: *mut bpf_rb_node, b: *const bpf_rb_node) -> bool {
    static bool less_r(struct bpf_rb_node *a, const struct bpf_rb_node *b)
    {
    struct tree_node_ref *node_a, *node_b;
    node_a = container_of(a, struct tree_node_ref, node);
    node_b = container_of(b, struct tree_node_ref, node);
    return node_a.key < node_b.key;
    }
    SEC("syscall")
    __retval(0)
#[no_mangle]
pub unsafe extern "C" fn rbtree_search_kptr_ref(ctx: *mut c_void) -> c_long {
    long rbtree_search_kptr_ref(void *ctx)
    {
    struct tree_node_ref *tnode_r, *tnode_m;
    struct bpf_rb_node *rb_n;
    struct node_data __kptr * node_data;
    let mut lookup_key: c_int = NR_NODES / 2;
    let mut lookup_data: c_int = NR_NODES / 2;
    int i, data, ret = 0;
    for (i = 0; i < NR_NODES && can_loop; i++) {
    tnode_r = bpf_obj_new(typeof(*tnode_r));
    if (!tnode_r)
    return __LINE__;
    node_data = bpf_obj_new(typeof(*node_data));
    if (!node_data) {
    bpf_obj_drop(tnode_r);
    return __LINE__;
    }
    tnode_r.key = i;
    node_data.data = i;
    node_data = bpf_kptr_xchg(&tnode_r.node_data, node_data);
    if (node_data)
    bpf_obj_drop(node_data);
// Unused reference
    tnode_m = bpf_refcount_acquire(tnode_r);
    if (!tnode_m)
    return __LINE__;
    bpf_spin_lock(&lock_r);
    bpf_rbtree_add(&root_r, &tnode_r.node, less_r);
    bpf_spin_unlock(&lock_r);
    bpf_obj_drop(tnode_m);
    }
    bpf_spin_lock(&lock_r);
    rb_n = bpf_rbtree_root(&root_r);
    while (rb_n && can_loop) {
    tnode_r = container_of(rb_n, struct tree_node_ref, node);
    node_data = bpf_kptr_xchg(&tnode_r.node_data, core::ptr::null_mut());
    if (!node_data) {
    ret = __LINE__;
    goto fail;
    }
    data = node_data.data;
    node_data = bpf_kptr_xchg(&tnode_r.node_data, node_data);
    if (node_data) {
    bpf_spin_unlock(&lock_r);
    bpf_obj_drop(node_data);
    return __LINE__;
    }
    if (lookup_key == tnode_r.key) {
    if (data == lookup_data)
    break;
    ret = __LINE__;
    goto fail;
    }
    if (lookup_key < tnode_r.key)
    rb_n = bpf_rbtree_left(&root_r, rb_n);
    else
    rb_n = bpf_rbtree_right(&root_r, rb_n);
    }
    bpf_spin_unlock(&lock_r);
    while (can_loop) {
    bpf_spin_lock(&lock_r);
    rb_n = bpf_rbtree_first(&root_r);
    if (!rb_n) {
    bpf_spin_unlock(&lock_r);
    return 0;
    }
    rb_n = bpf_rbtree_remove(&root_r, rb_n);
    if (!rb_n) {
    ret = __LINE__;
    goto fail;
    }
    bpf_spin_unlock(&lock_r);
    tnode_r = container_of(rb_n, struct tree_node_ref, node);
    node_data = bpf_kptr_xchg(&tnode_r.node_data, core::ptr::null_mut());
    if (node_data)
    bpf_obj_drop(node_data);
    bpf_obj_drop(tnode_r);
    }
    return 0;
    fail:
    bpf_spin_unlock(&lock_r);
    return ret;
    }
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn __msg(expected=map_value: "R1 type=scalar, _arg: ptr_, _arg: ptr_") -> __failure {
    __failure __msg("R1 type=scalar expected=map_value, ptr_, ptr_")
#[no_mangle]
pub unsafe extern "C" fn non_own_ref_kptr_xchg_no_lock(ctx: *mut c_void) -> c_long {
    long non_own_ref_kptr_xchg_no_lock(void *ctx)
    {
    struct tree_node *tnode;
    struct bpf_rb_node *rb_n;
    struct node_data __kptr * node_data;
    int data;
    bpf_spin_lock(&lock);
    rb_n = bpf_rbtree_first(&root);
    if (!rb_n) {
    bpf_spin_unlock(&lock);
    return __LINE__;
    }
    bpf_spin_unlock(&lock);
    tnode = container_of(rb_n, struct tree_node, node);
    node_data = bpf_kptr_xchg(&tnode.node_data, core::ptr::null_mut());
    if (!node_data)
    return __LINE__;
    data = node_data.data;
    if (data < 0)
    return __LINE__;
    node_data = bpf_kptr_xchg(&tnode.node_data, node_data);
    if (node_data)
    return __LINE__;
    return 0;
    }
    char _license[] SEC("license") = "GPL";
