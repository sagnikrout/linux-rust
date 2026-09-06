//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/rbtree_fail.c
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
pub struct node_data {
    pub key: c_long,
    pub data: c_long,
    pub node: bpf_rb_node,
}

    private(A) struct bpf_spin_lock glock;
    private(A) struct bpf_rb_root groot __contains(node_data, node);
    private(A) struct bpf_rb_root groot2 __contains(node_data, node);
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
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn __msg(bpf_rb_root": "bpf_spin_lock at off=16 must be held for) -> __failure {
    __failure __msg("bpf_spin_lock at off=16 must be held for bpf_rb_root")
#[no_mangle]
pub unsafe extern "C" fn rbtree_api_nolock_add(ctx: *mut c_void) -> c_long {
    long rbtree_api_nolock_add(void *ctx)
    {
    struct node_data *n;
    n = bpf_obj_new(typeof(*n));
    if (!n)
    return 1;
    bpf_rbtree_add(&groot, &n.node, less);
    return 0;
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn __msg(bpf_rb_root": "bpf_spin_lock at off=16 must be held for) -> __failure {
    __failure __msg("bpf_spin_lock at off=16 must be held for bpf_rb_root")
#[no_mangle]
pub unsafe extern "C" fn rbtree_api_nolock_remove(ctx: *mut c_void) -> c_long {
    long rbtree_api_nolock_remove(void *ctx)
    {
    struct node_data *n;
    n = bpf_obj_new(typeof(*n));
    if (!n)
    return 1;
    bpf_spin_lock(&glock);
    bpf_rbtree_add(&groot, &n.node, less);
    bpf_spin_unlock(&glock);
    bpf_rbtree_remove(&groot, &n.node);
    return 0;
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn __msg(bpf_rb_root": "bpf_spin_lock at off=16 must be held for) -> __failure {
    __failure __msg("bpf_spin_lock at off=16 must be held for bpf_rb_root")
#[no_mangle]
pub unsafe extern "C" fn rbtree_api_nolock_first(ctx: *mut c_void) -> c_long {
    long rbtree_api_nolock_first(void *ctx)
    {
    bpf_rbtree_first(&groot);
    return 0;
    }
    SEC("?tc")
    __retval(0)
#[no_mangle]
pub unsafe extern "C" fn rbtree_api_remove_unadded_node(ctx: *mut c_void) -> c_long {
    long rbtree_api_remove_unadded_node(void *ctx)
    {
    struct node_data *n, *m;
    struct bpf_rb_node *res_n, *res_m;
    n = bpf_obj_new(typeof(*n));
    if (!n)
    return 1;
    m = bpf_obj_new(typeof(*m));
    if (!m) {
    bpf_obj_drop(n);
    return 1;
    }
    bpf_spin_lock(&glock);
    bpf_rbtree_add(&groot, &n.node, less);
    res_n = bpf_rbtree_remove(&groot, &n.node);
    res_m = bpf_rbtree_remove(&groot, &m.node);
    bpf_spin_unlock(&glock);
    bpf_obj_drop(m);
    if (res_n)
    bpf_obj_drop(container_of(res_n, struct node_data, node));
    if (res_m) {
    bpf_obj_drop(container_of(res_m, struct node_data, node));
// m was not added to the rbtree
    return 2;
    }
    return 0;
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn __msg(alloc_insn={{[0-9]+}}": "Unreleased reference id=3) -> __failure {
    __failure __msg("Unreleased reference id=3 alloc_insn={{[0-9]+}}")
#[no_mangle]
pub unsafe extern "C" fn rbtree_api_remove_no_drop(ctx: *mut c_void) -> c_long {
    long rbtree_api_remove_no_drop(void *ctx)
    {
    struct bpf_rb_node *res;
    struct node_data *n;
    bpf_spin_lock(&glock);
    res = bpf_rbtree_first(&groot);
    if (!res)
    goto unlock_err;
    res = bpf_rbtree_remove(&groot, res);
    if (res) {
    n = container_of(res, struct node_data, node);
    __sink(n);
    }
    bpf_spin_unlock(&glock);
// if (res) { bpf_obj_drop(n); } is missing here
    return 0;
    unlock_err:
    bpf_spin_unlock(&glock);
    return 1;
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn __msg(object": "R2 expected pointer to allocated) -> __failure {
    __failure __msg("R2 expected pointer to allocated object")
#[no_mangle]
pub unsafe extern "C" fn rbtree_api_add_to_multiple_trees(ctx: *mut c_void) -> c_long {
    long rbtree_api_add_to_multiple_trees(void *ctx)
    {
    struct node_data *n;
    n = bpf_obj_new(typeof(*n));
    if (!n)
    return 1;
    bpf_spin_lock(&glock);
    bpf_rbtree_add(&groot, &n.node, less);
// This add should fail since n already in groot's tree
    bpf_rbtree_add(&groot2, &n.node, less);
    bpf_spin_unlock(&glock);
    return 0;
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn __msg(R2": "Possibly NULL pointer passed to trusted) -> __failure {
    __failure __msg("Possibly core::ptr::null_mut() pointer passed to trusted R2")
#[no_mangle]
pub unsafe extern "C" fn rbtree_api_use_unchecked_remove_retval(ctx: *mut c_void) -> c_long {
    long rbtree_api_use_unchecked_remove_retval(void *ctx)
    {
    struct bpf_rb_node *res;
    bpf_spin_lock(&glock);
    res = bpf_rbtree_first(&groot);
    if (!res)
    goto err_out;
    res = bpf_rbtree_remove(&groot, res);
    bpf_spin_unlock(&glock);
    bpf_spin_lock(&glock);
// Must check res for NULL before using in rbtree_add below
    bpf_rbtree_add(&groot, res, less);
    bpf_spin_unlock(&glock);
    return 0;
    err_out:
    bpf_spin_unlock(&glock);
    return 1;
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn __msg(pointer": "bpf_rbtree_remove can only take non-owning or refcounted bpf_rb_node) -> __failure {
    __failure __msg("bpf_rbtree_remove can only take non-owning or refcounted bpf_rb_node pointer")
#[no_mangle]
pub unsafe extern "C" fn rbtree_api_add_release_unlock_escape(ctx: *mut c_void) -> c_long {
    long rbtree_api_add_release_unlock_escape(void *ctx)
    {
    struct node_data *n;
    n = bpf_obj_new(typeof(*n));
    if (!n)
    return 1;
    bpf_spin_lock(&glock);
    bpf_rbtree_add(&groot, &n.node, less);
    bpf_spin_unlock(&glock);
    bpf_spin_lock(&glock);
// After add() in previous critical section, n should be
// release_on_unlock and released after previous spin_unlock,
// so should not be possible to use it here
//
    bpf_rbtree_remove(&groot, &n.node);
    bpf_spin_unlock(&glock);
    return 0;
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn __msg(pointer": "bpf_rbtree_remove can only take non-owning or refcounted bpf_rb_node) -> __failure {
    __failure __msg("bpf_rbtree_remove can only take non-owning or refcounted bpf_rb_node pointer")
#[no_mangle]
pub unsafe extern "C" fn rbtree_api_first_release_unlock_escape(ctx: *mut c_void) -> c_long {
    long rbtree_api_first_release_unlock_escape(void *ctx)
    {
    struct bpf_rb_node *res;
    struct node_data *n;
    bpf_spin_lock(&glock);
    res = bpf_rbtree_first(&groot);
    if (!res) {
    bpf_spin_unlock(&glock);
    return 1;
    }
    n = container_of(res, struct node_data, node);
    bpf_spin_unlock(&glock);
    bpf_spin_lock(&glock);
// After first() in previous critical section, n should be
// release_on_unlock and released after previous spin_unlock,
// so should not be possible to use it here
//
    bpf_rbtree_remove(&groot, &n.node);
    bpf_spin_unlock(&glock);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn less__bad_fn_call_add(a: *mut bpf_rb_node, b: *const bpf_rb_node) -> bool {
    static bool less__bad_fn_call_add(struct bpf_rb_node *a, const struct bpf_rb_node *b)
    {
    struct node_data *node_a;
    struct node_data *node_b;
    node_a = container_of(a, struct node_data, node);
    node_b = container_of(b, struct node_data, node);
    bpf_rbtree_add(&groot, &node_a.node, less);
    return node_a.key < node_b.key;
    }
#[no_mangle]
unsafe extern "C" fn less__bad_fn_call_remove(a: *mut bpf_rb_node, b: *const bpf_rb_node) -> bool {
    static bool less__bad_fn_call_remove(struct bpf_rb_node *a, const struct bpf_rb_node *b)
    {
    struct node_data *node_a;
    struct node_data *node_b;
    node_a = container_of(a, struct node_data, node);
    node_b = container_of(b, struct node_data, node);
    bpf_rbtree_remove(&groot, &node_a.node);
    return node_a.key < node_b.key;
    }
#[no_mangle]
unsafe extern "C" fn less__bad_fn_call_first_unlock_after(a: *mut bpf_rb_node, b: *const bpf_rb_node) -> bool {
    static bool less__bad_fn_call_first_unlock_after(struct bpf_rb_node *a, const struct bpf_rb_node *b)
    {
    struct node_data *node_a;
    struct node_data *node_b;
    node_a = container_of(a, struct node_data, node);
    node_b = container_of(b, struct node_data, node);
    bpf_rbtree_first(&groot);
    bpf_spin_unlock(&glock);
    return node_a.key < node_b.key;
    }
    static __always_inline
#[no_mangle]
pub unsafe extern "C" fn add_with_cb(a: *mut bool (cb)(struct bpf_rb_node, b): *const bpf_rb_node) -> c_long {
    long add_with_cb(bool (cb)(struct bpf_rb_node *a, const struct bpf_rb_node *b))
    {
    struct node_data *n;
    n = bpf_obj_new(typeof(*n));
    if (!n)
    return 1;
    bpf_spin_lock(&glock);
    bpf_rbtree_add(&groot, &n.node, cb);
    bpf_spin_unlock(&glock);
    return 0;
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn __msg(object": "R2 expected pointer to allocated) -> __failure {
    __failure __msg("R2 expected pointer to allocated object")
#[no_mangle]
pub unsafe extern "C" fn rbtree_api_add_bad_cb_bad_fn_call_add(ctx: *mut c_void) -> c_long {
    long rbtree_api_add_bad_cb_bad_fn_call_add(void *ctx)
    {
    return add_with_cb(less__bad_fn_call_add);
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn __msg(cb": "rbtree_remove not allowed in rbtree) -> __failure {
    __failure __msg("rbtree_remove not allowed in rbtree cb")
#[no_mangle]
pub unsafe extern "C" fn rbtree_api_add_bad_cb_bad_fn_call_remove(ctx: *mut c_void) -> c_long {
    long rbtree_api_add_bad_cb_bad_fn_call_remove(void *ctx)
    {
    return add_with_cb(less__bad_fn_call_remove);
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn __msg(spin_{lock: "can't, cb": unlock} in rbtree) -> __failure {
    __failure __msg("can't spin_{lock,unlock} in rbtree cb")
#[no_mangle]
pub unsafe extern "C" fn rbtree_api_add_bad_cb_bad_fn_call_first_unlock_after(ctx: *mut c_void) -> c_long {
    long rbtree_api_add_bad_cb_bad_fn_call_first_unlock_after(void *ctx)
    {
    return add_with_cb(less__bad_fn_call_first_unlock_after);
    }
    char _license[] SEC("license") = "GPL";
