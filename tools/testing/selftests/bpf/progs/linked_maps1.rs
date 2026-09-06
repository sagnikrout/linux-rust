//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/linked_maps1.c
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct my_key {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct my_value {
    struct {
    pub BPF_MAP_TYPE_HASH): __uint(type,,
    pub my_key): __type(key, struct,
    pub my_value): __type(value, struct,
    pub 16): __uint(max_entries,,
    pub SEC(".maps"): } map1,
// Matches map2 definition in linked_maps2.c. Order of the attributes doesn't
// matter.
//
    typedef struct {
    pub 8): __uint(max_entries,,
    pub int): __type(key,,
    pub int): __type(value,,
    pub BPF_MAP_TYPE_ARRAY): __uint(type,,
    pub map2_t: },
    pub SEC(".maps"): extern map2_t map2,
// This should be the winning map definition, but we have no way of verifying,
// so we just make sure that it links and works without errors
//
    struct {
    pub BPF_MAP_TYPE_ARRAY): __uint(type,,
    pub int): __type(key,,
    pub int): __type(value,,
    pub 16): __uint(max_entries,,
    pub SEC(".maps"): } map_weak __weak,
    pub output_first1: c_int,
    pub output_second1: c_int,
    pub output_weak1: c_int,
    SEC("raw_tp/sys_enter")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: handler_enter1) -> c_int {
    int BPF_PROG(handler_enter1)
    {
// update values with key = 1
    pub 1: int key = 1, val =,
    pub }: my_key key_= { .x = 1,
    pub }: my_value val_= { .x = 1000,
    pub 0): bpf_map_update_elem(&map1, &key_struct, &val_struct,,
    pub 0): bpf_map_update_elem(&map2, &key, &val,,
    pub 0): bpf_map_update_elem(&map_weak, &key, &val,,
    pub 0: return,
    }
    SEC("raw_tp/sys_exit")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: handler_exit1) -> c_int {
    int BPF_PROG(handler_exit1)
    {
// lookup values with key = 2, set in another file
    pub val: *mut int key = 2,,
    pub }: my_key key_= { .x = 2,
    pub value_struct: *mut my_value,
    pub &key_struct): value_struct = bpf_map_lookup_elem(&map1,,
    if (value_struct)
    pub value_struct->x: output_first1 =,
    pub &key): val = bpf_map_lookup_elem(&map2,,
    if (val)
    pub val: *mut output_second1 =,
    pub &key): val = bpf_map_lookup_elem(&map_weak,,
    if (val)
    pub val: *mut output_weak1 =,
    pub 0: return,
    }
    pub "GPL": char LICENSE[] SEC("license") =,
