//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/fd_htab_lookup.c
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
// Copyright (C) 2025. Huawei Technologies Co., Ltd

    char _license[] SEC("license") = "GPL";
#[repr(C)]
#[derive(Copy, Clone)]
pub struct inner_map_type {
    pub BPF_MAP_TYPE_ARRAY): __uint(type,,
    pub 4): __uint(key_size,,
    pub 4): __uint(value_size,,
    pub 1): __uint(max_entries,,
    pub SEC(".maps"): } inner_map,
    struct {
    pub BPF_MAP_TYPE_HASH_OF_MAPS): __uint(type,,
    pub 64): __uint(max_entries,,
    pub int): __type(key,,
    pub int): __type(value,,
    pub inner_map_type): __array(values, struct,
    } outer_map SEC(".maps") = {
    .values = {
    [0] = &inner_map,
    },
}
