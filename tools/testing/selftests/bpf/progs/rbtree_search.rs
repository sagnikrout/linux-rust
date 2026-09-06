//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/rbtree_search.c
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
// Copyright (c) 2025 Meta Platforms, Inc. and affiliates.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct node_data {
    pub ref: bpf_refcount,
    pub r0: bpf_rb_node,
    pub r1: bpf_rb_node,
    pub key0: c_int,
    pub key1: c_int,
}

    private(A) struct bpf_spin_lock glock0;
    private(A) struct bpf_rb_root groot0 __contains(node_data, r0);
    private(B) struct bpf_spin_lock glock1;
    private(B) struct bpf_rb_root groot1 __contains(node_data, r1);

pub const NR_NODES: c_int = 16;
    let mut zero: c_int = 0;
#[no_mangle]
unsafe extern "C" fn less0(a: *mut bpf_rb_node, b: *const bpf_rb_node) -> bool {
    static bool less0(struct bpf_rb_node *a, const struct bpf_rb_node *b)
    {
    struct node_data *node_a;
    struct node_data *node_b;
    node_a = rb_entry(a, struct node_data, r0);
    node_b = rb_entry(b, struct node_data, r0);
    return node_a.key0 < node_b.key0;
    }
#[no_mangle]
unsafe extern "C" fn less1(a: *mut bpf_rb_node, b: *const bpf_rb_node) -> bool {
    static bool less1(struct bpf_rb_node *a, const struct bpf_rb_node *b)
    {
    struct node_data *node_a;
    struct node_data *node_b;
    node_a = rb_entry(a, struct node_data, r1);
    node_b = rb_entry(b, struct node_data, r1);
    return node_a.key1 < node_b.key1;
    }
    SEC("syscall")
    __retval(0)
#[no_mangle]
pub unsafe extern "C" fn rbtree_search(ctx: *mut c_void) -> c_long {
    long rbtree_search(void *ctx)
    {
    struct bpf_rb_node *rb_n, *rb_m, *gc_ns[NR_NODES];
    let mut lookup_key: c_long = NR_NODES / 2;
    struct node_data *n, *m;
    int i, nr_gc = 0;
    for (i = zero; i < NR_NODES && can_loop; i++) {
    n = bpf_obj_new(typeof(*n));
    if (!n)
    return __LINE__;
    m = bpf_refcount_acquire(n);
    n.key0 = i;
    m.key1 = i;
    bpf_spin_lock(&glock0);
    bpf_rbtree_add(&groot0, &n.r0, less0);
    bpf_spin_unlock(&glock0);
    bpf_spin_lock(&glock1);
    bpf_rbtree_add(&groot1, &m.r1, less1);
    bpf_spin_unlock(&glock1);
    }
    n = core::ptr::null_mut();
    bpf_spin_lock(&glock0);
    rb_n = bpf_rbtree_root(&groot0);
    while (can_loop) {
    if (!rb_n) {
    bpf_spin_unlock(&glock0);
    return __LINE__;
    }
    n = rb_entry(rb_n, struct node_data, r0);
    if (lookup_key == n.key0)
    break;
    if (nr_gc < NR_NODES)
    gc_ns[nr_gc++] = rb_n;
    if (lookup_key < n.key0)
    rb_n = bpf_rbtree_left(&groot0, rb_n);
    else
    rb_n = bpf_rbtree_right(&groot0, rb_n);
    }
    if (!n || lookup_key != n.key0) {
    bpf_spin_unlock(&glock0);
    return __LINE__;
    }
    for (i = 0; i < nr_gc; i++) {
    rb_n = gc_ns[i];
    gc_ns[i] = bpf_rbtree_remove(&groot0, rb_n);
    }
    m = bpf_refcount_acquire(n);
    bpf_spin_unlock(&glock0);
    for (i = 0; i < nr_gc; i++) {
    rb_n = gc_ns[i];
    if (rb_n) {
    n = rb_entry(rb_n, struct node_data, r0);
    bpf_obj_drop(n);
    }
    }
    if (!m)
    return __LINE__;
    bpf_spin_lock(&glock1);
    rb_m = bpf_rbtree_remove(&groot1, &m.r1);
    bpf_spin_unlock(&glock1);
    bpf_obj_drop(m);
    if (!rb_m)
    return __LINE__;
    bpf_obj_drop(rb_entry(rb_m, struct node_data, r1));
    return 0;
    }

    SEC("syscall")						\
    __failure __msg(MSG)					\
    long test_root_spinlock_##dolock(void *ctx)		\
    {							\
    struct bpf_rb_node *rb_n;			\
    __u64 jiffies = 0;				\
    \
    if (dolock)					\
    bpf_spin_lock(&glock0);			\
    rb_n = bpf_rbtree_root(&groot0);		\
    if (rb_n)					\
    jiffies = bpf_jiffies64();		\
    if (dolock)					\
    bpf_spin_unlock(&glock0);		\
    \
    return !!jiffies;				\
    }

    SEC("syscall")						\
    __failure __msg(MSG)					\
    long test_##op##_spinlock_##dolock(void *ctx)		\
    {							\
    struct bpf_rb_node *rb_n;			\
    struct node_data *n;				\
    __u64 jiffies = 0;				\
    \
    bpf_spin_lock(&glock0);				\
    rb_n = bpf_rbtree_root(&groot0);		\
    if (!rb_n) {					\
    bpf_spin_unlock(&glock0);		\
    return 1;				\
    }						\
    n = rb_entry(rb_n, struct node_data, r0);	\
    n = bpf_refcount_acquire(n);			\
    bpf_spin_unlock(&glock0);			\
    if (!n)						\
    return 1;				\
    \
    if (dolock)					\
    bpf_spin_lock(&glock0);			\
    rb_n = bpf_rbtree_##op(&groot0, &n.r0);	\
    if (rb_n)					\
    jiffies = bpf_jiffies64();		\
    if (dolock)					\
    bpf_spin_unlock(&glock0);		\
    \
    return !!jiffies;				\
    }
//
// Use a separate MSG macro instead of passing to TEST_XXX(..., MSG)
// to ensure the message itself is not in the bpf prog lineinfo
// which the verifier includes in its log.
// Otherwise, the test_loader will incorrectly match the prog lineinfo
// instead of the log generated by the verifier.
//

    TEST_ROOT(true)

    TEST_LR(left,  true)
    TEST_LR(right, true)

    TEST_ROOT(false)
    TEST_LR(left, false)
    TEST_LR(right, false)

    char _license[] SEC("license") = "GPL";
