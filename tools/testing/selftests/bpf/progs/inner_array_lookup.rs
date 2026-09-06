//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/inner_array_lookup.c
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


// SPDX-License-Identifier: GPL-2.0-only

#[repr(C)]
#[derive(Copy, Clone)]
pub struct inner_map {
    pub BPF_MAP_TYPE_ARRAY): __uint(type,,
    pub 5): __uint(max_entries,,
    pub int): __type(key,,
    pub int): __type(value,,
    pub SEC(".maps"): } inner_map1,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct outer_map {
    pub BPF_MAP_TYPE_HASH_OF_MAPS): __uint(type,,
    pub 3): __uint(max_entries,,
    pub int): __type(key,,
    pub inner_map): __array(values, struct,
    } outer_map1 SEC(".maps") = {
    .values = {
    [2] = &inner_map1,
    },
}

    SEC("raw_tp/sys_enter")
#[no_mangle]
pub unsafe extern "C" fn handle__sys_enter(ctx: *mut c_void) -> c_int {
    int handle__sys_enter(void *ctx)
    {
    let mut outer_key: c_int = 2, inner_key = 3;
    int *val;
    void *map;
    map = bpf_map_lookup_elem(&outer_map1, &outer_key);
    if (!map)
    return 1;
    val = bpf_map_lookup_elem(map, &inner_key);
    if (!val)
    return 1;
    if (*val == 1)
// val = 2;
    return 0;
    }
    char _license[] SEC("license") = "GPL";
