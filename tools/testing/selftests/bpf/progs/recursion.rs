//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/recursion.c
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

    char _license[] SEC("license") = "GPL";
    struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __uint(max_entries, 1);
    __type(key, int);
    __type(value, long);
    } hash1 SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __uint(max_entries, 1);
    __type(key, int);
    __type(value, long);
    } hash2 SEC(".maps");
    let mut pass1: c_int = 0;
    let mut pass2: c_int = 0;
    SEC("fentry/htab_map_delete_elem")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: on_delete, map: *mut bpf_map) -> c_int {
    int BPF_PROG(on_delete, struct bpf_map *map)
    {
    let mut key: c_int = 0;
    if (map == (void *)&hash1) {
    pass1++;
    return 0;
    }
    if (map == (void *)&hash2) {
    pass2++;
    bpf_map_delete_elem(&hash2, &key);
    return 0;
    }
    return 0;
    }
