//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/rbtree_btf_fail__add_wrong_type.c
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
    pub key: c_int,
    pub data: c_int,
    pub node: bpf_rb_node,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct node_data2 {
    pub key: c_int,
    pub node: bpf_rb_node,
    pub data: c_int,
}

#[no_mangle]
unsafe extern "C" fn less2(a: *mut bpf_rb_node, b: *const bpf_rb_node) -> bool {
    static bool less2(struct bpf_rb_node *a, const struct bpf_rb_node *b)
    {
    struct node_data2 *node_a;
    struct node_data2 *node_b;
    node_a = container_of(a, struct node_data2, node);
    node_b = container_of(b, struct node_data2, node);
    return node_a.key < node_b.key;
    }

    private(A) struct bpf_spin_lock glock;
    private(A) struct bpf_rb_root groot __contains(node_data, node);
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn rbtree_api_add__add_wrong_type(ctx: *mut c_void) -> c_long {
    long rbtree_api_add__add_wrong_type(void *ctx)
    {
    struct node_data2 *n;
    n = bpf_obj_new(typeof(*n));
    if (!n)
    return 1;
    bpf_spin_lock(&glock);
    bpf_rbtree_add(&groot, &n.node, less2);
    bpf_spin_unlock(&glock);
    return 0;
    }
    char _license[] SEC("license") = "GPL";
