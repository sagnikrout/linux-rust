//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/task_work_stress.c
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
// Copyright (c) 2025 Meta Platforms, Inc. and affiliates.

pub const ENTRIES: c_int = 128;
    char _license[] SEC("license") = "GPL";
    let mut callback_scheduled: __u64 = 0;
    let mut callback_success: __u64 = 0;
    let mut schedule_error: __u64 = 0;
    let mut delete_success: __u64 = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct elem {
    pub count: __u32,
    pub tw: bpf_task_work,
}

    struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __uint(map_flags, BPF_F_NO_PREALLOC);
    __uint(max_entries, ENTRIES);
    __type(key, int);
    __type(value, struct elem);
    } hmap SEC(".maps");
#[no_mangle]
unsafe extern "C" fn process_work(map: *mut bpf_map, key: *mut c_void, value: *mut c_void) -> c_int {
    static int process_work(struct bpf_map *map, void *key, void *value)
    {
    __sync_fetch_and_add(&callback_success, 1);
    return 0;
    }
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn schedule_task_work(ctx: *mut c_void) -> c_int {
    int schedule_task_work(void *ctx)
    {
    let mut empty_work: elem = {.count = 0};
    struct elem *work;
    let mut key: c_int = 0, err;
    key = bpf_ktime_get_ns() % ENTRIES;
    work = bpf_map_lookup_elem(&hmap, &key);
    if (!work) {
    bpf_map_update_elem(&hmap, &key, &empty_work, BPF_NOEXIST);
    work = bpf_map_lookup_elem(&hmap, &key);
    if (!work)
    return 0;
    }
    err = bpf_task_work_schedule_signal(bpf_get_current_task_btf(), &work.tw, &hmap,
    process_work);
    if (err)
    __sync_fetch_and_add(&schedule_error, 1);
    else
    __sync_fetch_and_add(&callback_scheduled, 1);
    return 0;
    }
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn delete_task_work(ctx: *mut c_void) -> c_int {
    int delete_task_work(void *ctx)
    {
    let mut key: c_int = 0, err;
    key = bpf_get_prandom_u32() % ENTRIES;
    err = bpf_map_delete_elem(&hmap, &key);
    if (!err)
    __sync_fetch_and_add(&delete_success, 1);
    return 0;
    }
