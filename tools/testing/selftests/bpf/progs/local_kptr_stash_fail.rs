//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/local_kptr_stash_fail.c
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct node_data {
    pub key: c_long,
    pub data: c_long,
    pub node: bpf_rb_node,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct map_value {
    pub node: *mut node_data __kptr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct node_data2 {
    pub key: [c_long; 4],
}

// This is necessary so that LLVM generates BTF for node_data struct
// If it's not included, a fwd reference for node_data will be generated but
// no struct. Example BTF of "node" field in map_value when not included:
//
// [10] PTR '(anon)' type_id=35
// [34] FWD 'node_data' fwd_kind=struct
// [35] TYPE_TAG 'kptr_ref' type_id=34
//
    struct node_data *just_here_because_btf_bug;
    struct {
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __type(key, int);
    __type(value, struct map_value);
    __uint(max_entries, 2);
    } some_nodes SEC(".maps");
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn __msg(access: "invalid kptr, expected=ptr_node_data": R2 type=ptr_node_data2) -> __failure {
    __failure __msg("invalid kptr access, R2 type=ptr_node_data2 expected=ptr_node_data")
#[no_mangle]
pub unsafe extern "C" fn stash_rb_nodes(ctx: *mut c_void) -> c_long {
    long stash_rb_nodes(void *ctx)
    {
    struct map_value *mapval;
    struct node_data2 *res;
    let mut idx: c_int = 0;
    mapval = bpf_map_lookup_elem(&some_nodes, &idx);
    if (!mapval)
    return 1;
    res = bpf_obj_new(typeof(*res));
    if (!res)
    return 1;
    res.key[0] = 40;
    res = bpf_kptr_xchg(&mapval.node, res);
    if (res)
    bpf_obj_drop(res);
    return 0;
    }
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn __msg(func": "R1 must have zero offset when passed to release) -> __failure {
    __failure __msg("R1 must have zero offset when passed to release func")
#[no_mangle]
pub unsafe extern "C" fn drop_rb_node_off(ctx: *mut c_void) -> c_long {
    long drop_rb_node_off(void *ctx)
    {
    struct map_value *mapval;
    struct node_data *res;
    let mut idx: c_int = 0;
    mapval = bpf_map_lookup_elem(&some_nodes, &idx);
    if (!mapval)
    return 1;
    res = bpf_obj_new(typeof(*res));
    if (!res)
    return 1;
// Try releasing with graph node offset
    bpf_obj_drop(&res.node);
    return 0;
    }
    char _license[] SEC("license") = "GPL";
