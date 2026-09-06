//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/refcounted_kptr_fail.c
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct node_acquire {
    pub key: c_long,
    pub data: c_long,
    pub node: bpf_rb_node,
    pub refcount: bpf_refcount,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct node_refcounted {
    pub key: c_long,
    pub list: bpf_list_node,
    pub refcount: bpf_refcount,
}

    extern void bpf_rcu_read_lock(void) __ksym;
    extern void bpf_rcu_read_unlock(void) __ksym;

    private(A) struct bpf_spin_lock glock;
    private(A) struct bpf_rb_root groot __contains(node_acquire, node);
    private(B) struct bpf_spin_lock lock;
    private(B) struct bpf_list_head head __contains(node_refcounted, list);
#[no_mangle]
unsafe extern "C" fn less(a: *mut bpf_rb_node, b: *const bpf_rb_node) -> bool {
    static bool less(struct bpf_rb_node *a, const struct bpf_rb_node *b)
    {
    struct node_acquire *node_a;
    struct node_acquire *node_b;
    node_a = container_of(a, struct node_acquire, node);
    node_b = container_of(b, struct node_acquire, node);
    return node_a.key < node_b.key;
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn __msg(alloc_insn={{[0-9]+}}": "Unreleased reference id=4) -> __failure {
    __failure __msg("Unreleased reference id=4 alloc_insn={{[0-9]+}}")
#[no_mangle]
pub unsafe extern "C" fn rbtree_refcounted_node_ref_escapes(ctx: *mut c_void) -> c_long {
    long rbtree_refcounted_node_ref_escapes(void *ctx)
    {
    struct node_acquire *n, *m;
    n = bpf_obj_new(typeof(*n));
    if (!n)
    return 1;
    bpf_spin_lock(&glock);
    bpf_rbtree_add(&groot, &n.node, less);
// m becomes an owning ref but is never drop'd or added to a tree
    m = bpf_refcount_acquire(n);
    bpf_spin_unlock(&glock);
    if (!m)
    return 2;
    m.key = 2;
    return 0;
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn __msg(R1": "Possibly NULL pointer passed to trusted) -> __failure {
    __failure __msg("Possibly core::ptr::null_mut() pointer passed to trusted R1")
    __msg("requires a non-core::ptr::null_mut() value of type (void *)")
#[no_mangle]
pub unsafe extern "C" fn refcount_acquire_maybe_null(ctx: *mut c_void) -> c_long {
    long refcount_acquire_maybe_null(void *ctx)
    {
    struct node_acquire *n, *m;
    n = bpf_obj_new(typeof(*n));
// Intentionally not testing !n
// it's MAYBE_NULL for refcount_acquire
//
    m = bpf_refcount_acquire(n);
    if (m)
    bpf_obj_drop(m);
    if (n)
    bpf_obj_drop(n);
    return 0;
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn __msg(ref": "R1 is neither owning or non-owning) -> __failure {
    __failure __msg("R1 is neither owning or non-owning ref")
    __msg("expects a pointer to a BPF-managed refcounted object, but R1 is a context pointer")
#[no_mangle]
pub unsafe extern "C" fn refcount_acquire_non_object(ctx: *mut c_void) -> c_long {
    long refcount_acquire_non_object(void *ctx)
    {
    return bpf_refcount_acquire(ctx) != core::ptr::null_mut();
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn __msg(alloc_insn={{[0-9]+}}": "Unreleased reference id=3) -> __failure {
    __failure __msg("Unreleased reference id=3 alloc_insn={{[0-9]+}}")
#[no_mangle]
pub unsafe extern "C" fn rbtree_refcounted_node_ref_escapes_owning_input(ctx: *mut c_void) -> c_long {
    long rbtree_refcounted_node_ref_escapes_owning_input(void *ctx)
    {
    struct node_acquire *n, *m;
    n = bpf_obj_new(typeof(*n));
    if (!n)
    return 1;
// m becomes an owning ref but is never drop'd or added to a tree
    m = bpf_refcount_acquire(n);
    m.key = 2;
    bpf_spin_lock(&glock);
    bpf_rbtree_add(&groot, &n.node, less);
    bpf_spin_unlock(&glock);
    return 0;
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn __msg(R1": "dereference of modified ptr_ ptr) -> __failure {
    __failure __msg("dereference of modified ptr_ ptr R1")
#[no_mangle]
pub unsafe extern "C" fn refcount_acquire_list_node_offset(ctx: *mut c_void) -> c_long {
    long refcount_acquire_list_node_offset(void *ctx)
    {
    struct node_refcounted *node, *base, *ref;
    struct bpf_list_node *list_node;
    node = bpf_obj_new(typeof(*node));
    if (!node)
    return 1;
    bpf_spin_lock(&lock);
    bpf_list_push_front(&head, &node.list);
    list_node = bpf_list_pop_front(&head);
    bpf_spin_unlock(&lock);
    if (!list_node)
    return 2;
    base = container_of(list_node, struct node_refcounted, list);
    ref = bpf_refcount_acquire(list_node);
    if (ref)
    bpf_obj_drop(ref);
    bpf_obj_drop(base);
    return 0;
    }
    SEC("?fentry.s/" SYS_PREFIX "sys_getpgid")
#[no_mangle]
pub unsafe extern "C" fn __msg(lock": "function calls are not allowed while holding a) -> __failure {
    __failure __msg("function calls are not allowed while holding a lock")
    int BPF_PROG(rbtree_fail_sleepable_lock_across_rcu,
    struct file *file, struct kobject *kobj,
    struct bin_attribute *bin_attr, char *buf, loff_t off, size_t len)
    {
    struct node_acquire *n;
    n = bpf_obj_new(typeof(*n));
    if (!n)
    return 0;
// spin_{lock,unlock} are in different RCU CS
    bpf_rcu_read_lock();
    bpf_spin_lock(&glock);
    bpf_rbtree_add(&groot, &n.node, less);
    bpf_rcu_read_unlock();
    bpf_rcu_read_lock();
    bpf_spin_unlock(&glock);
    bpf_rcu_read_unlock();
    return 0;
    }
    char _license[] SEC("license") = "GPL";
