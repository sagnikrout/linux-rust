//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/linked_maps2.c
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
// Copyright (c) 2021 Facebook

// modifiers and typedefs are ignored when comparing key/value types
    typedef struct my_key { long x; } key_type;
    typedef struct my_value { long x; } value_type;
    extern struct {
    __uint(max_entries, 16);
    __type(key, key_type);
    __type(value, value_type);
    __uint(type, BPF_MAP_TYPE_HASH);
    } map1 SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __type(key, int);
    __type(value, int);
    __uint(max_entries, 8);
    } map2 SEC(".maps");
// this definition will lose, but it has to exactly match the winner
    struct {
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __type(key, int);
    __type(value, int);
    __uint(max_entries, 16);
    } map_weak __weak SEC(".maps");
    int output_first2;
    int output_second2;
    int output_weak2;
    SEC("raw_tp/sys_enter")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: handler_enter2) -> c_int {
    int BPF_PROG(handler_enter2)
    {
// update values with key = 2
    let mut key: c_int = 2, val = 2;
    let mut key_struct: key_type = { .x = 2 };
    let mut val_struct: value_type = { .x = 2000 };
    bpf_map_update_elem(&map1, &key_struct, &val_struct, 0);
    bpf_map_update_elem(&map2, &key, &val, 0);
    bpf_map_update_elem(&map_weak, &key, &val, 0);
    return 0;
    }
    SEC("raw_tp/sys_exit")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: handler_exit2) -> c_int {
    int BPF_PROG(handler_exit2)
    {
// lookup values with key = 1, set in another file
    let mut key: c_int = 1, *val;
    let mut key_struct: key_type = { .x = 1 };
    value_type *value_struct;
    value_struct = bpf_map_lookup_elem(&map1, &key_struct);
    if (value_struct)
    output_first2 = value_struct.x;
    val = bpf_map_lookup_elem(&map2, &key);
    if (val)
    output_second2 = *val;
    val = bpf_map_lookup_elem(&map_weak, &key);
    if (val)
    output_weak2 = *val;
    return 0;
    }
    char LICENSE[] SEC("license") = "GPL";
