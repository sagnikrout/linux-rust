//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/map_in_map_btf.c
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
pub struct node_data {
    pub data: __u64,
    pub node: bpf_list_node,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct map_value {
    pub node): bpf_list_head head __contains(node_data,,
    pub lock: bpf_spin_lock,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct inner_array_type {
    pub BPF_MAP_TYPE_ARRAY): __uint(type,,
    pub int): __type(key,,
    pub map_value): __type(value, struct,
    pub 1): __uint(max_entries,,
    pub SEC(".maps"): } inner_array,
    struct {
    pub BPF_MAP_TYPE_ARRAY_OF_MAPS): __uint(type,,
    pub 4): __uint(key_size,,
    pub 4): __uint(value_size,,
    pub 1): __uint(max_entries,,
    pub inner_array_type): __array(values, struct,
    } outer_array SEC(".maps") = {
    .values = {
    [0] = &inner_array,
    },
}

    char _license[] SEC("license") = "GPL";
    let mut pid: c_int = 0;
    let mut done: bool = false;
    SEC("fentry/" SYS_PREFIX "sys_nanosleep")
#[no_mangle]
pub unsafe extern "C" fn add_to_list_in_inner_array(ctx: *mut c_void) -> c_int {
    int add_to_list_in_inner_array(void *ctx)
    {
    struct map_value *value;
    struct node_data *new;
    struct bpf_map *map;
    let mut zero: c_int = 0;
    if (done || (u32)bpf_get_current_pid_tgid() != pid)
    return 0;
    map = bpf_map_lookup_elem(&outer_array, &zero);
    if (!map)
    return 0;
    value = bpf_map_lookup_elem(map, &zero);
    if (!value)
    return 0;
    new = bpf_obj_new(typeof(*new));
    if (!new)
    return 0;
    bpf_spin_lock(&value.lock);
    bpf_list_push_back(&value.head, &new.node);
    bpf_spin_unlock(&value.lock);
    done = true;
    return 0;
    }
