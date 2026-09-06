//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_map_in_map.c
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
// Copyright (c) 2018 Facebook

    struct {
    __uint(type, BPF_MAP_TYPE_ARRAY_OF_MAPS);
    __uint(max_entries, 1);
    __uint(map_flags, 0);
    __type(key, __u32);
    __type(value, __u32);
    } mim_array SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_HASH_OF_MAPS);
    __uint(max_entries, 1);
    __uint(map_flags, 0);
    __type(key, int);
    __type(value, __u32);
    } mim_hash SEC(".maps");
// The following three maps are used to test
// perf_event_array map can be an inner
// map of hash/array_of_maps.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_event_array {
    pub BPF_MAP_TYPE_PERF_EVENT_ARRAY): __uint(type,,
    pub __u32): __type(key,,
    pub __u32): __type(value,,
    pub SEC(".maps"): } inner_map0,
    struct {
    pub BPF_MAP_TYPE_ARRAY_OF_MAPS): __uint(type,,
    pub 1): __uint(max_entries,,
    pub __u32): __type(key,,
    pub perf_event_array): __array(values, struct,
    } mim_array_pe SEC(".maps") = {
    pub {&inner_map0}}: .values =,
    struct {
    pub BPF_MAP_TYPE_HASH_OF_MAPS): __uint(type,,
    pub 1): __uint(max_entries,,
    pub __u32): __type(key,,
    pub perf_event_array): __array(values, struct,
    } mim_hash_pe SEC(".maps") = {
    pub {&inner_map0}}: .values =,
    SEC("xdp")
#[no_mangle]
pub unsafe extern "C" fn xdp_mimtest0(ctx: *mut xdp_md) -> c_int {
    int xdp_mimtest0(struct xdp_md *ctx)
    {
    pub 123: int value =,
    pub value_p: *mut c_int,
    pub 0: int key =,
    pub map: *mut c_void,
    pub &key): map = bpf_map_lookup_elem(&mim_array,,
    if (!map)
    pub XDP_DROP: return,
    pub 0): bpf_map_update_elem(map, &key, &value,,
    pub &key): value_p = bpf_map_lookup_elem(map,,
    if (!value_p || *value_p != 123)
    pub XDP_DROP: return,
    pub &key): map = bpf_map_lookup_elem(&mim_hash,,
    if (!map)
    pub XDP_DROP: return,
    pub 0): bpf_map_update_elem(map, &key, &value,,
    pub XDP_PASS: return,
    }
    pub "GPL": char _license[] SEC("license") =,
