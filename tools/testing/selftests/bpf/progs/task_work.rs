//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/task_work.c
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

    char _license[] SEC("license") = "GPL";
    const void *user_ptr = core::ptr::null_mut();
#[repr(C)]
#[derive(Copy, Clone)]
pub struct elem {
    pub data: [c_char; 128],
    pub tw: bpf_task_work,
}

    struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __uint(map_flags, BPF_F_NO_PREALLOC);
    __uint(max_entries, 1);
    __type(key, int);
    __type(value, struct elem);
    } hmap SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __uint(max_entries, 1);
    __type(key, int);
    __type(value, struct elem);
    } arrmap SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_LRU_HASH);
    __uint(max_entries, 1);
    __type(key, int);
    __type(value, struct elem);
    } lrumap SEC(".maps");
#[no_mangle]
unsafe extern "C" fn process_work(map: *mut bpf_map, key: *mut c_void, value: *mut c_void) -> c_int {
    static int process_work(struct bpf_map *map, void *key, void *value)
    {
    struct elem *work = value;
    bpf_copy_from_user_str(work.data, sizeof(work.data), (const void *)user_ptr, 0);
    return 0;
    }
    let mut key: c_int = 0;
    SEC("perf_event")
#[no_mangle]
pub unsafe extern "C" fn oncpu_hash_map(args: *mut pt_regs) -> c_int {
    int oncpu_hash_map(struct pt_regs *args)
    {
    let mut empty_work: elem = { .data = { 0 } };
    struct elem *work;
    struct task_struct *task;
    int err;
    task = bpf_get_current_task_btf();
    err = bpf_map_update_elem(&hmap, &key, &empty_work, BPF_NOEXIST);
    if (err)
    return 0;
    work = bpf_map_lookup_elem(&hmap, &key);
    if (!work)
    return 0;
    bpf_task_work_schedule_resume(task, &work.tw, &hmap, process_work);
    return 0;
    }
    SEC("perf_event")
#[no_mangle]
pub unsafe extern "C" fn oncpu_array_map(args: *mut pt_regs) -> c_int {
    int oncpu_array_map(struct pt_regs *args)
    {
    struct elem *work;
    struct task_struct *task;
    task = bpf_get_current_task_btf();
    work = bpf_map_lookup_elem(&arrmap, &key);
    if (!work)
    return 0;
    bpf_task_work_schedule_signal(task, &work.tw, &arrmap, process_work);
    return 0;
    }
    SEC("perf_event")
#[no_mangle]
pub unsafe extern "C" fn oncpu_lru_map(args: *mut pt_regs) -> c_int {
    int oncpu_lru_map(struct pt_regs *args)
    {
    let mut empty_work: elem = { .data = { 0 } };
    struct elem *work;
    struct task_struct *task;
    int err;
    task = bpf_get_current_task_btf();
    work = bpf_map_lookup_elem(&lrumap, &key);
    if (work)
    return 0;
    err = bpf_map_update_elem(&lrumap, &key, &empty_work, BPF_NOEXIST);
    if (err)
    return 0;
    work = bpf_map_lookup_elem(&lrumap, &key);
    if (!work || work.data[0])
    return 0;
    bpf_task_work_schedule_resume(task, &work.tw, &lrumap, process_work);
    return 0;
    }
