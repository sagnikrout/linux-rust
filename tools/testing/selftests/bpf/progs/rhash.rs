//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/rhash.c
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
// Copyright (c) 2026 Meta Platforms, Inc. and affiliates.

pub const ENOENT: c_int = 2;
pub const EEXIST: c_int = 17;
    char _license[] SEC("license") = "GPL";
    int err;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct elem {
    pub arr: [c_char; 128],
    pub val: c_int,
}

    struct {
    __uint(type, BPF_MAP_TYPE_RHASH);
    __uint(map_flags, BPF_F_NO_PREALLOC);
    __uint(max_entries, 128);
    __type(key, int);
    __type(value, struct elem);
    } rhmap SEC(".maps");
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn test_rhash_lookup_update(ctx: *mut c_void) -> c_int {
    int test_rhash_lookup_update(void *ctx)
    {
    let mut key: c_int = 5;
    let mut empty: elem = {.val = 3, .arr = {0}};
    struct elem *e;
    err = 1;
    e = bpf_map_lookup_elem(&rhmap, &key);
    if (e)
    return 1;
    err = bpf_map_update_elem(&rhmap, &key, &empty, BPF_NOEXIST);
    if (err)
    return 1;
    e = bpf_map_lookup_elem(&rhmap, &key);
    if (!e || e.val != empty.val) {
    err = 2;
    return 2;
    }
    err = 0;
    return 0;
    }
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn test_rhash_update_delete(ctx: *mut c_void) -> c_int {
    int test_rhash_update_delete(void *ctx)
    {
    let mut key: c_int = 6;
    let mut empty: elem = {.val = 4, .arr = {0}};
    struct elem *e;
    err = 1;
    e = bpf_map_lookup_elem(&rhmap, &key);
    if (e)
    return 1;
    err = bpf_map_update_elem(&rhmap, &key, &empty, BPF_NOEXIST);
    if (err)
    return 2;
    err = bpf_map_delete_elem(&rhmap, &key);
    if (err)
    return 3;
    e = bpf_map_lookup_elem(&rhmap, &key);
    if (e) {
    err = 4;
    return 4;
    }
    err = 0;
    return 0;
    }
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn test_rhash_update_elements(ctx: *mut c_void) -> c_int {
    int test_rhash_update_elements(void *ctx)
    {
    let mut key: c_int = 0;
    let mut empty: elem = {.val = 4, .arr = {0}};
    struct elem *e;
    int i;
    err = 1;
    for (i = 0; i < 128; ++i) {
    key = i;
    e = bpf_map_lookup_elem(&rhmap, &key);
    if (e)
    return 1;
    empty.val = key;
    err = bpf_map_update_elem(&rhmap, &key, &empty, BPF_NOEXIST);
    if (err)
    return 2;
    e = bpf_map_lookup_elem(&rhmap, &key);
    if (!e || e.val != key) {
    err = 4;
    return 4;
    }
    }
    for (i = 0; i < 128; ++i) {
    key = i;
    err = bpf_map_delete_elem(&rhmap, &key);
    if (err)
    return 3;
    e = bpf_map_lookup_elem(&rhmap, &key);
    if (e) {
    err = 5;
    return 5;
    }
    }
    err = 0;
    return 0;
    }
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn test_rhash_update_exist(ctx: *mut c_void) -> c_int {
    int test_rhash_update_exist(void *ctx)
    {
    let mut key: c_int = 10;
    let mut val1: elem = {.val = 100, .arr = {0}};
    let mut val2: elem = {.val = 200, .arr = {0}};
    struct elem *e;
    int ret;
    err = 1;
// BPF_EXIST on non-existent key should fail with -ENOENT
    ret = bpf_map_update_elem(&rhmap, &key, &val1, BPF_EXIST);
    if (ret != -ENOENT)
    return 1;
// Insert element first
    ret = bpf_map_update_elem(&rhmap, &key, &val1, BPF_NOEXIST);
    if (ret)
    return 2;
// Verify initial value
    e = bpf_map_lookup_elem(&rhmap, &key);
    if (!e || e.val != 100)
    return 3;
// BPF_EXIST on existing key should succeed and update value
    ret = bpf_map_update_elem(&rhmap, &key, &val2, BPF_EXIST);
    if (ret)
    return 4;
// Verify value was updated
    e = bpf_map_lookup_elem(&rhmap, &key);
    if (!e || e.val != 200)
    return 5;
// Cleanup
    bpf_map_delete_elem(&rhmap, &key);
    err = 0;
    return 0;
    }
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn test_rhash_update_any(ctx: *mut c_void) -> c_int {
    int test_rhash_update_any(void *ctx)
    {
    let mut key: c_int = 11;
    let mut val1: elem = {.val = 111, .arr = {0}};
    let mut val2: elem = {.val = 222, .arr = {0}};
    struct elem *e;
    int ret;
    err = 1;
// BPF_ANY on non-existent key should insert
    ret = bpf_map_update_elem(&rhmap, &key, &val1, BPF_ANY);
    if (ret)
    return 1;
    e = bpf_map_lookup_elem(&rhmap, &key);
    if (!e || e.val != 111)
    return 2;
// BPF_ANY on existing key should update
    ret = bpf_map_update_elem(&rhmap, &key, &val2, BPF_ANY);
    if (ret)
    return 3;
    e = bpf_map_lookup_elem(&rhmap, &key);
    if (!e || e.val != 222)
    return 4;
// Cleanup
    bpf_map_delete_elem(&rhmap, &key);
    err = 0;
    return 0;
    }
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn test_rhash_noexist_duplicate(ctx: *mut c_void) -> c_int {
    int test_rhash_noexist_duplicate(void *ctx)
    {
    let mut key: c_int = 12;
    let mut val: elem = {.val = 600, .arr = {0}};
    int ret;
    err = 1;
// Insert element
    ret = bpf_map_update_elem(&rhmap, &key, &val, BPF_NOEXIST);
    if (ret)
    return 1;
// Try to insert again with BPF_NOEXIST - should fail with -EEXIST
    ret = bpf_map_update_elem(&rhmap, &key, &val, BPF_NOEXIST);
    if (ret != -EEXIST)
    return 2;
// Cleanup
    bpf_map_delete_elem(&rhmap, &key);
    err = 0;
    return 0;
    }
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn test_rhash_delete_nonexistent(ctx: *mut c_void) -> c_int {
    int test_rhash_delete_nonexistent(void *ctx)
    {
    let mut key: c_int = 99999;
    int ret;
    err = 1;
// Delete non-existent key should return -ENOENT
    ret = bpf_map_delete_elem(&rhmap, &key);
    if (ret != -ENOENT)
    return 1;
    err = 0;
    return 0;
    }
