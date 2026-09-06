//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_btf_map_in_map.c
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
// Copyright (c) 2020 Facebook

#[repr(C)]
#[derive(Copy, Clone)]
pub struct inner_map {
    pub BPF_MAP_TYPE_ARRAY): __uint(type,,
    pub 1): __uint(max_entries,,
    pub int): __type(key,,
    pub int): __type(value,,
    } inner_map1 SEC(".maps"),
    pub SEC(".maps"): inner_map2,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct inner_map_sz2 {
    pub BPF_MAP_TYPE_ARRAY): __uint(type,,
    pub 2): __uint(max_entries,,
    pub int): __type(key,,
    pub int): __type(value,,
    pub SEC(".maps"): } inner_map_sz2,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct outer_arr {
    pub BPF_MAP_TYPE_ARRAY_OF_MAPS): __uint(type,,
    pub 3): __uint(max_entries,,
    pub int): __type(key,,
    pub int): __type(value,,
// it's possible to use anonymous struct as inner map definition here
    __array(values, struct {
    pub BPF_MAP_TYPE_ARRAY): __uint(type,,
// changing max_entries to 2 will fail during load
// due to incompatibility with inner_map definition
    pub 1): __uint(max_entries,,
    pub int): __type(key,,
    pub int): __type(value,,
    } outer_arr SEC(".maps") = {
// (void *) cast is necessary because we didn't use `struct inner_map`
// in __inner(values, ...)
// Actually, a conscious effort is required to screw up initialization
// of inner map slots, which is a great thing!
//
    .values = { (void *)&inner_map1, 0, (void *)&inner_map2 },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct inner_map_sz3 {
    pub BPF_MAP_TYPE_ARRAY): __uint(type,,
    pub BPF_F_INNER_MAP): __uint(map_flags,,
    pub 3): __uint(max_entries,,
    pub int): __type(key,,
    pub int): __type(value,,
    } inner_map3 SEC(".maps"),
    pub SEC(".maps"): inner_map4,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct inner_map_sz4 {
    pub BPF_MAP_TYPE_ARRAY): __uint(type,,
    pub BPF_F_INNER_MAP): __uint(map_flags,,
    pub 5): __uint(max_entries,,
    pub int): __type(key,,
    pub int): __type(value,,
    pub SEC(".maps"): } inner_map5,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct outer_arr_dyn {
    pub BPF_MAP_TYPE_ARRAY_OF_MAPS): __uint(type,,
    pub 3): __uint(max_entries,,
    pub int): __type(key,,
    pub int): __type(value,,
    __array(values, struct {
    pub BPF_MAP_TYPE_ARRAY): __uint(type,,
    pub BPF_F_INNER_MAP): __uint(map_flags,,
    pub 1): __uint(max_entries,,
    pub int): __type(key,,
    pub int): __type(value,,
    } outer_arr_dyn SEC(".maps") = {
    .values = {
    [0] = (void *)&inner_map3,
    [1] = (void *)&inner_map4,
    [2] = (void *)&inner_map5,
    },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct outer_hash {
    pub BPF_MAP_TYPE_HASH_OF_MAPS): __uint(type,,
    pub 5): __uint(max_entries,,
    pub int): __type(key,,
// Here everything works flawlessly due to reuse of struct inner_map
// and compiler will complain at the attempt to use non-inner_map
// references below. This is great experience.
//
    pub inner_map): __array(values, struct,
    } outer_hash SEC(".maps") = {
    .values = {
    [0] = &inner_map2,
    [4] = &inner_map1,
    },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sockarr_sz1 {
    pub BPF_MAP_TYPE_REUSEPORT_SOCKARRAY): __uint(type,,
    pub 1): __uint(max_entries,,
    pub int): __type(key,,
    pub int): __type(value,,
    pub SEC(".maps"): } sockarr_sz1,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sockarr_sz2 {
    pub BPF_MAP_TYPE_REUSEPORT_SOCKARRAY): __uint(type,,
    pub 2): __uint(max_entries,,
    pub int): __type(key,,
    pub int): __type(value,,
    pub SEC(".maps"): } sockarr_sz2,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct outer_sockarr_sz1 {
    pub BPF_MAP_TYPE_ARRAY_OF_MAPS): __uint(type,,
    pub 1): __uint(max_entries,,
    pub int): __type(key,,
    pub int): __type(value,,
    pub sockarr_sz1): __array(values, struct,
    } outer_sockarr SEC(".maps") = {
    .values = { (void *)&sockarr_sz1 },
}

    let mut input: c_int = 0;
    SEC("raw_tp/sys_enter")
#[no_mangle]
pub unsafe extern "C" fn handle__sys_enter(ctx: *mut c_void) -> c_int {
    int handle__sys_enter(void *ctx)
    {
    struct inner_map *inner_map;
    let mut key: c_int = 0, val;
    inner_map = bpf_map_lookup_elem(&outer_arr, &key);
    if (!inner_map)
    return 1;
    val = input;
    bpf_map_update_elem(inner_map, &key, &val, 0);
    inner_map = bpf_map_lookup_elem(&outer_hash, &key);
    if (!inner_map)
    return 1;
    val = input + 1;
    bpf_map_update_elem(inner_map, &key, &val, 0);
    inner_map = bpf_map_lookup_elem(&outer_arr_dyn, &key);
    if (!inner_map)
    return 1;
    val = input + 2;
    bpf_map_update_elem(inner_map, &key, &val, 0);
    return 0;
    }
    char _license[] SEC("license") = "GPL";
