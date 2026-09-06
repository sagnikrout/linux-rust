//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/access_map_in_map.c
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
// Copyright (C) 2023. Huawei Technologies Co., Ltd

#[repr(C)]
#[derive(Copy, Clone)]
pub struct inner_map_type {
    pub BPF_MAP_TYPE_ARRAY): __uint(type,,
    pub 4): __uint(key_size,,
    pub 4): __uint(value_size,,
    pub 1): __uint(max_entries,,
    pub SEC(".maps"): } inner_map,
    struct {
    pub BPF_MAP_TYPE_ARRAY_OF_MAPS): __uint(type,,
    pub int): __type(key,,
    pub int): __type(value,,
    pub 1): __uint(max_entries,,
    pub inner_map_type): __array(values, struct,
    } outer_array_map SEC(".maps") = {
    .values = {
    [0] = &inner_map,
    },
}

    struct {
    __uint(type, BPF_MAP_TYPE_HASH_OF_MAPS);
    __type(key, int);
    __type(value, int);
    __uint(max_entries, 1);
    __array(values, struct inner_map_type);
    } outer_htab_map SEC(".maps") = {
    .values = {
    [0] = &inner_map,
    },
    };
    char _license[] SEC("license") = "GPL";
    let mut tgid: c_int = 0;
#[no_mangle]
unsafe extern "C" fn acc_map_in_map(outer_map: *mut c_void) -> c_int {
    static int acc_map_in_map(void *outer_map)
    {
    int i, key, value = 0xdeadbeef;
    void *inner_map;
    if ((bpf_get_current_pid_tgid() >> 32) != tgid)
    return 0;
// Find nonexistent inner map
    key = 1;
    inner_map = bpf_map_lookup_elem(outer_map, &key);
    if (inner_map)
    return 0;
// Find the old inner map
    key = 0;
    inner_map = bpf_map_lookup_elem(outer_map, &key);
    if (!inner_map)
    return 0;
// Wait for the old inner map to be replaced
    for (i = 0; i < 2048; i++)
    bpf_map_update_elem(inner_map, &key, &value, 0);
    return 0;
    }
    SEC("?kprobe/" SYS_PREFIX "sys_getpgid")
#[no_mangle]
pub unsafe extern "C" fn access_map_in_array(ctx: *mut c_void) -> c_int {
    int access_map_in_array(void *ctx)
    {
    return acc_map_in_map(&outer_array_map);
    }
    SEC("?fentry.s/" SYS_PREFIX "sys_getpgid")
#[no_mangle]
pub unsafe extern "C" fn sleepable_access_map_in_array(ctx: *mut c_void) -> c_int {
    int sleepable_access_map_in_array(void *ctx)
    {
    return acc_map_in_map(&outer_array_map);
    }
    SEC("?kprobe/" SYS_PREFIX "sys_getpgid")
#[no_mangle]
pub unsafe extern "C" fn access_map_in_htab(ctx: *mut c_void) -> c_int {
    int access_map_in_htab(void *ctx)
    {
    return acc_map_in_map(&outer_htab_map);
    }
    SEC("?fentry.s/" SYS_PREFIX "sys_getpgid")
#[no_mangle]
pub unsafe extern "C" fn sleepable_access_map_in_htab(ctx: *mut c_void) -> c_int {
    int sleepable_access_map_in_htab(void *ctx)
    {
    return acc_map_in_map(&outer_htab_map);
    }
