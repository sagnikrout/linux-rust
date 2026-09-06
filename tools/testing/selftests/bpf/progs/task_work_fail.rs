//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/task_work_fail.c
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
pub unsafe extern "C" fn __msg(R3": "doesn't match map pointer in) -> __failure {
    __failure __msg("doesn't match map pointer in R3")
#[no_mangle]
pub unsafe extern "C" fn mismatch_map(args: *mut pt_regs) -> c_int {
    int mismatch_map(struct pt_regs *args)
    {
    struct elem *work;
    struct task_struct *task;
    task = bpf_get_current_task_btf();
    work = bpf_map_lookup_elem(&arrmap, &key);
    if (!work)
    return 0;
    bpf_task_work_schedule_resume(task, &work.tw, &hmap, process_work);
    return 0;
    }
    SEC("perf_event")
#[no_mangle]
pub unsafe extern "C" fn __msg(value": "R2 doesn't point to a map) -> __failure {
    __failure __msg("R2 doesn't point to a map value")
#[no_mangle]
pub unsafe extern "C" fn no_map_task_work(args: *mut pt_regs) -> c_int {
    int no_map_task_work(struct pt_regs *args)
    {
    struct task_struct *task;
    struct bpf_task_work tw;
    task = bpf_get_current_task_btf();
    bpf_task_work_schedule_resume(task, &tw, &hmap, process_work);
    return 0;
    }
    SEC("perf_event")
#[no_mangle]
pub unsafe extern "C" fn __msg(R2": "Possibly NULL pointer passed to trusted) -> __failure {
    __failure __msg("Possibly core::ptr::null_mut() pointer passed to trusted R2")
#[no_mangle]
pub unsafe extern "C" fn task_work_null(args: *mut pt_regs) -> c_int {
    int task_work_null(struct pt_regs *args)
    {
    struct task_struct *task;
    task = bpf_get_current_task_btf();
    bpf_task_work_schedule_resume(task, core::ptr::null_mut(), &hmap, process_work);
    return 0;
    }
    SEC("perf_event")
#[no_mangle]
pub unsafe extern "C" fn __msg(R3": "Possibly NULL pointer passed to trusted) -> __failure {
    __failure __msg("Possibly core::ptr::null_mut() pointer passed to trusted R3")
#[no_mangle]
pub unsafe extern "C" fn map_null(args: *mut pt_regs) -> c_int {
    int map_null(struct pt_regs *args)
    {
    struct elem *work;
    struct task_struct *task;
    task = bpf_get_current_task_btf();
    work = bpf_map_lookup_elem(&arrmap, &key);
    if (!work)
    return 0;
    bpf_task_work_schedule_resume(task, &work.tw, core::ptr::null_mut(), process_work);
    return 0;
    }
