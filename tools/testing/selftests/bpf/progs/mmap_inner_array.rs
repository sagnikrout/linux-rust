//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/mmap_inner_array.c
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
// Copyright (c) 2024 Meta Platforms, Inc. and affiliates.

    char _license[] SEC("license") = "GPL";
#[repr(C)]
#[derive(Copy, Clone)]
pub struct inner_array_type {
    pub BPF_MAP_TYPE_ARRAY): __uint(type,,
    pub BPF_F_MMAPABLE): __uint(map_flags,,
    pub __u32): __type(key,,
    pub __u64): __type(value,,
    pub 1): __uint(max_entries,,
    pub SEC(".maps"): } inner_array,
    struct {
    pub BPF_MAP_TYPE_HASH_OF_MAPS): __uint(type,,
    pub 4): __uint(key_size,,
    pub 4): __uint(value_size,,
    pub 1): __uint(max_entries,,
    pub inner_array_type): __array(values, struct,
    pub SEC(".maps"): } outer_map,
    pub 0: int pid =,
    pub 0x13572468: __u64 match_value =,
    pub false: bool done =,
    pub false: bool pid_match =,
    pub false: bool outer_map_match =,
    SEC("fentry/" SYS_PREFIX "sys_nanosleep")
#[no_mangle]
pub unsafe extern "C" fn add_to_list_in_inner_array(ctx: *mut c_void) -> c_int {
    int add_to_list_in_inner_array(void *ctx)
    {
    pub 0: __u32 curr_pid, zero =,
    pub map: *mut bpf_map,
    pub value: *mut __u64,
    pub (u32)bpf_get_current_pid_tgid(): curr_pid =,
    if (done || curr_pid != pid)
    pub 0: return,
    pub true: pid_match =,
    pub &curr_pid): map = bpf_map_lookup_elem(&outer_map,,
    if (!map)
    pub 0: return,
    pub true: outer_map_match =,
    pub &zero): value = bpf_map_lookup_elem(map,,
    if (!value)
    pub 0: return,
// value = match_value;
    pub true: done =,
    pub 0: return,
    }
